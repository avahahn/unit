use std::collections::HashMap;
use std::sync::{Mutex, Arc, OnceLock};
use std::ffi::{CString, CStr};
use std::borrow::Cow;
use std::ptr;

use opentelemetry::trace::{SpanKind, SpanBuilder, TraceId};
use opentelemetry_sdk::trace::{Span as SpanImpl, Tracer as TracerImpl};
use opentelemetry::trace::{Span, Tracer};
use lazy_static::lazy_static;

lazy_static!{
    static ref GLOBAL_REF_CACHE: Mutex<HashMap<String, Arc<SpanImpl>>> =
        Mutex::new(HashMap::new());
}

static GLOBAL_TRACER: OnceLock<TracerImpl> = OnceLock::new();


// potentially returns an error message
// TODO: pass in a callback to log error instead of broken bullshit
#[no_mangle]
fn nxt_otel_init() -> *mut i8 {
    // First, create a OTLP exporter builder. Configure it as you need.
    // TODO configure endpoint here :)
    let otlp_exporter = opentelemetry_otlp::new_exporter().tonic();
    // Then pass it into pipeline builder
    let res = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(otlp_exporter)
        .install_simple();
    // unwrap
    match res {
        Err(e) => unsafe {
            CString::from_vec_unchecked(
                e.to_string()
                    .as_bytes()
                    .to_vec()
            ).into_raw()
        },
        Ok(tracer) => {
            GLOBAL_TRACER.get_or_init(move || tracer);
            ptr::null::<i8>() as *mut i8
        }
    }
}

/* WARNING
 * Blocks until mutex held
 * DO NOT CALL FROM REQUEST PROCESSING THREAD
 */
#[inline(always)]
fn cache_new_span(trace_id: &str, span: Arc<SpanImpl>) {
    let trace_key = String::from(trace_id);
    let span_owned = Arc::clone(&span);
    { // CRITICAL SECTION
        let _ = GLOBAL_REF_CACHE
            .lock()
            .unwrap()
            .insert(trace_key, span_owned);
    }
}

/* WARNING
 * Blocks until mutex held
 * DO NOT CALL FROM REQUEST PROCESSING THREAD
 */
#[inline(always)]
fn drop_cached_span_if_exists(trace_id: &str) {
    let trace_key = String::from(trace_id);
    { // CRITICAL SECTION
        let _ = GLOBAL_REF_CACHE
            .lock()
            .unwrap()
            .remove(&trace_key);
    }
}

#[no_mangle]
pub unsafe fn nxt_otel_add_event_to_trace(
    _key: *mut i8,
    _val: *mut i8,
    _trace_id: *mut i8
) {
    // damage nothing on an improper call
    if !_key.is_null() &&
        !_val.is_null() &&
        !_trace_id.is_null() {
            // TODO: generate an event attached to the span
            todo!()
        }
}

#[no_mangle]
pub unsafe fn nxt_otel_get_or_create_trace(
    trace_id: *mut i8
) -> *mut SpanImpl {
    let mut trace_key = None;
    let trace_cstr: CString;
    let trace_str: Cow<str>;
    if !trace_id.is_null() {
        trace_cstr = CString::from_raw(trace_id);
        trace_str = trace_cstr.to_string_lossy();
        if let Ok(id) = TraceId::from_hex(&trace_str) {
            trace_key = Some(id);
        }
    }

    let span: SpanImpl;
    if let Some(tracer) = GLOBAL_TRACER.get() {
        span = tracer.build(SpanBuilder {
            trace_id: trace_key,
            span_kind: Some(SpanKind::Server),
            ..Default::default()
        });
    } else {
        return ptr::null::<SpanImpl>() as *mut SpanImpl;
    }

    let arc_span = Arc::new(span);
    let new_cstr = CString::new(
        arc_span.span_context()
            .span_id()
            .to_string()
    ).unwrap();
    let new_key = new_cstr
        .to_string_lossy();
    cache_new_span(&new_key, arc_span.clone());

    // this reference accounted for in
    // nxt_otel_send_trace
    return Arc::<SpanImpl>::into_raw(arc_span) as *mut SpanImpl;
}

#[no_mangle]
pub unsafe fn nxt_otel_send_trace(trace: *mut SpanImpl) {
    // damage nothing on an improper call
    if trace.is_null() {
        return;
    }

    /* memory needs to be accounted for via arc here
     * see the final return statement from
     * nxt_otel_get_or_create_trace
     */
    let arc_span = Arc::from_raw(trace);
    let cstr = CString::new(
        arc_span.span_context()
            .span_id()
            .to_string()
    ).unwrap();
    let key = cstr.to_string_lossy();

    // simple exporter will export spans when dropped
    // aka at end of this function
    drop_cached_span_if_exists(&key);

    /* One final thing we can do here is check
     * the strong count of the Arc. If it is not
     * now one, we can decrement manually to ensure
     * that is goes out of scope here.
     */
}
