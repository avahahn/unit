use opentelemetry::{
    global,
    trace::{Tracer, TracerProvider as _},
};

use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::Arc;

use opentelemetry::trace::Span;
use opentelemetry_sdk::trace::{SpanEvents, TracerProvider};
use std::os::unix::raw::time_t;

// not the prettiest looking type for sure
static global_ref_cache: Arc<Mutex<HashMap<String, Arc::<Span>>>> =
    Arc::new(Mutex::new(Hashmap::new()));

/* WARNING
 * Blocks until mutex held
 * DO NOT CALL FROM REQUEST PROCESSING THREAD
 */
#[inline(always)]
fn cache_new_span(trace_id: &str, span: Arc<Span>) {
    let trace_key = trace_id.into_owned();
    let span_owned = Arc::clone(span);
    { // CRITICAL SECTION
        let _ = Arc::clone(global_ref_cache)
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
    let trace_key = trace_id.into_owned();
    { // CRITICAL SECTION
        let _ = Arc::clone(global_ref_cache)
            .lock()
            .unwrap()
            .remove(trace_key);
    }
}

struct SpanId(String);
struct TraceId(String);

struct Attribute((String, String));
pub(crate) struct Event {
    name: String,
    timestamp: time_t,
    attributes: Vec<Attribute>,
}

#[no_mangle]
pub(crate) struct Span {
    name: String,
    parent_id: Some(SpanId),
    span_id: SpanId,
    trace_id: TraceId,
    start_time: time_t,
    end_time: time_t,
    attributes: Vec<Attribute>,
    events: Vec<Event>,
}
