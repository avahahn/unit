use std::collections::HashMap;
use std::sync::{Mutex, Arc, OnceLock};
use std::ffi::{CStr, CString};
use std::ptr;

use opentelemetry::trace::{SpanKind, SpanBuilder, SpanId, TraceId, Span, Tracer};
use opentelemetry_sdk::trace::{Span as SpanImpl, Tracer as TracerImpl};
use lazy_static::lazy_static;

// otel_endpoint is hardcoded for proof of concept purposes.
const OTEL_TRACES_ENDPOINT: &str = "http://lgtm:4318/v1/traces";

lazy_static!{
    static ref GLOBAL_REF_CACHE: Mutex<HashMap<SpanId, Arc<SpanImpl>>> =
        Mutex::new(HashMap::new());
}

static GLOBAL_TRACER: OnceLock<TracerImpl> = OnceLock::new();

// potentially returns an error message
// TODO: pass in a callback to log error instead of broken bullshit
#[no_mangle]
pub unsafe fn nxt_otel_init(log_callback: unsafe extern "C" fn(*mut i8)) {
    // TODO configure endpoint here :)
    let otlp_exporter = opentelemetry_otlp::new_exporter()
        .http();
    // Then pass it into pipeline builder
    let res = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(otlp_exporter)
        .install_simple();
    // unwrap
    match res {
        Err(e) => log_callback(
            CString::from_vec_unchecked(
                e.to_string()
                    .as_bytes()
                    .to_vec()
            ).into_raw()
        ),
        Ok(tracer) => {
            GLOBAL_TRACER.get_or_init(move || tracer);
        }
    }
}

// its on the caller to pass in a buf of proper length
#[no_mangle]
pub unsafe fn nxt_otel_copy_traceparent(buf: *mut i8, span: *const SpanImpl) {
    if buf.is_null() || span.is_null() {
        return;
    }

    let traceparent = format!(
        "00-{}-{}-{:x}",
        (*span).span_context().trace_id(),
        (*span).span_context().trace_id(),
        (*span).span_context().trace_flags()
    );

    assert!(traceparent.len() == 52);

    std::ptr::copy_nonoverlapping(traceparent.as_bytes().as_ptr(), buf as _, 52);
    // set null terminator
    *buf.add(42) = b'\0' as _;
}

/* WARNING
 * Blocks until mutex held
 * DO NOT CALL FROM REQUEST PROCESSING THREAD
 */
#[inline(always)]
fn cache_new_span(span: Arc<SpanImpl>) {
    let id = span.span_context().span_id();
    { // CRITICAL SECTION
        let _ = GLOBAL_REF_CACHE
            .lock()
            .unwrap()
            .insert(id, span);
    }
}

/* WARNING
 * Blocks until mutex held
 * DO NOT CALL FROM REQUEST PROCESSING THREAD
 */
#[inline(always)]
fn drop_cached_span_if_exists(span: Arc<SpanImpl>) {
    let id = span.span_context().span_id();
    { // CRITICAL SECTION
        let _ = GLOBAL_REF_CACHE
            .lock()
            .unwrap()
            .remove(&id);
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
    let trace_cstr: &CStr;
    if !trace_id.is_null() {
        trace_cstr = CStr::from_ptr(trace_id);
        if let Ok(id) = TraceId::from_hex(&trace_cstr.to_string_lossy()) {
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
    cache_new_span(arc_span.clone());

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

    // simple exporter will export spans when dropped
    // aka at end of this function
    drop_cached_span_if_exists(arc_span);

    /* One final thing we can do here is check
     * the strong count of the Arc. If it is not
     * now one, we can decrement manually to ensure
     * that is goes out of scope here.
     */
}
