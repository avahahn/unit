use opentelemetry::global::BoxedSpan;
use opentelemetry::trace::{
    Span, SpanBuilder, SpanKind, TraceId, Tracer, TracerProvider,
};
use opentelemetry::{global, KeyValue};
use opentelemetry_otlp::Protocol::Grpc;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::trace::Config;
use opentelemetry_sdk::Resource;
use std::ffi::{CStr, CString};
use std::ptr;
use std::sync::{Arc, OnceLock};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

// otel_endpoints are hardcoded for phase 1 purposes.
const OTEL_HTTP_TRACES_ENDPOINT: &str = "http://lgtm:4318/v1/traces";
const OTEL_GRPC_TRACES_ENDPOINT: &str = "http://lgtm:4317/";

// Stored sender channel to send spans or a shutdown message to within the
// Tokio runtime.
static SPAN_TX: OnceLock<Sender<SpanMessage>> = OnceLock::new();

// Message type to send on the channel. Either a span or a shutdown message for
// graceful termination of the tokio runtime.
enum SpanMessage {
    Span {
        s: Arc<BoxedSpan>
    },
    Shutdown,
}


// potentially returns an error message
#[no_mangle]
unsafe fn nxt_otel_init(log_callback: unsafe extern "C" fn(*mut i8)) {
    // Create a new mpsc channel. Tokio runtime gets receiver, the send
    // trace function gets sender.
    let (tx, rx): (Sender<SpanMessage>, Receiver<SpanMessage>) = mpsc::channel(32);

    // Store the sender so the other function can also reach it.
    SPAN_TX.get_or_init(|| tx);

    // spawn a new thread with the tokio runtime and forget about it.
    // This function will return that allows the C code to carry on
    // doing its thing, whereas the runtime function is a long lived
    // process that only exits when a shutdown message is sent.
    std::thread::spawn(move || runtime(log_callback, rx));
}


// function that we wrap around Tokio's runtime code. This is long lived,
// which means it stops only when a shutdown signal is sent to the rx
// channel, or we terminate the process and leave memory all over.
#[tokio::main]
async unsafe fn runtime(log_callback: unsafe extern "C" fn(*mut i8), mut rx: Receiver<SpanMessage>) {
    let otlp_exporter = opentelemetry_otlp::new_exporter()
        .http()
        .with_endpoint(OTEL_GRPC_TRACES_ENDPOINT)
        .with_protocol(Grpc)
        .with_timeout(Duration::new(10, 0));

    // Then pass it into pipeline builder
    let res = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_trace_config(Config::default().with_resource(
            Resource::new(vec![KeyValue::new(
                opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                "NGINX Unit",
            )]),
        ))
        .with_exporter(otlp_exporter)
        .install_simple();


    match res {
        Err(e) => {
            let msg = CString::from_vec_unchecked(e.to_string().as_bytes().to_vec());
            log_callback(msg.into_raw() as _)
        }
        Ok(t) => {
            global::set_tracer_provider(t);

            // GLOBAL_TRACER_PROVIDER.get_or_init(move || t);
            let msg = CString::from_vec_unchecked("otel exporter has been initialised".as_bytes().to_vec());
            log_callback(msg.into_raw() as _);
        }
    }

    // this is the block that keeps this function running until it gets shut down.
    // @see https://tokio.rs/tokio/tutorial/channels for the inspiration.
    while let Some(message) = rx.recv().await {
        match message {
            SpanMessage::Shutdown => {
                eprintln!("it was a shutdown");
                break;
            }
            SpanMessage::Span { s: _s } => {
                // do nothing, because the point is for this _s var to be dropped here
                // rather than where it was sent from.
            }
        }
    }
}

// it's on the caller to pass in a buf of proper length
#[no_mangle]
pub unsafe fn nxt_otel_copy_traceparent(buf: *mut i8, span: *const BoxedSpan) {
    if buf.is_null() || span.is_null() {
        return;
    }

    let traceparent = format!(
        "00-{:032x}-{:016x}-{:02x}",
        (*span).span_context().trace_id(), // 16 chars, 32 hex
        (*span).span_context().span_id(),  // 8 byte, 16 hex
        (*span).span_context().trace_flags()  // 1 char, 2 hex
    );

    assert_eq!(traceparent.len(), 55);

    ptr::copy_nonoverlapping(
        traceparent.as_bytes().as_ptr(),
        buf as _,
        55,
    );
    // set null terminator
    *buf.add(55) = b'\0' as _;
}

#[no_mangle]
pub unsafe fn nxt_otel_add_event_to_trace(
    trace: *mut BoxedSpan,
    key: *mut i8,
    val: *mut i8,
) {
    if !key.is_null() && !val.is_null() && !trace.is_null() {
        let key = CStr::from_ptr(key as _).to_string_lossy();
        let val = CStr::from_ptr(val as _).to_string_lossy();

        (*trace)
            .add_event(String::from("Unit Attribute"), vec![KeyValue::new(key, val)]);
    }
}

#[no_mangle]
pub unsafe fn nxt_otel_get_or_create_trace(trace_id: *mut i8) -> *mut BoxedSpan {
    let mut trace_key = None;
    let trace_cstr: &CStr;
    if !trace_id.is_null() {
        trace_cstr = CStr::from_ptr(trace_id as _);
        if let Ok(id) = TraceId::from_hex(&trace_cstr.to_string_lossy()) {
            trace_key = Some(id);
        }
    }

    let span = global::tracer_provider().tracer("NGINX Unit").build(SpanBuilder {
            trace_id: trace_key,
            span_kind: Some(SpanKind::Server),
            ..Default::default()
        });

    Arc::<BoxedSpan>::into_raw(Arc::new(span)) as *mut BoxedSpan
}

#[no_mangle]
#[tokio::main]
pub async unsafe fn nxt_otel_send_trace(trace: *mut BoxedSpan) {
    // damage nothing on an improper call
    if trace.is_null() {
        eprintln!("trace was null, returning");
        return;
    }

    /* memory needs to be accounted for via arc here
     * see the final return statement from
     * nxt_otel_get_or_create_trace
     */
    let arc_span = Arc::from_raw(trace);

    // Instead of dropping the reference at the end of this function
    // we'll send the entire Arc through the channel to the long
    // running process that will drop it there. The reason we need to
    // drop it there, rather than here is because that code block is
    // within the tokio runtime context with the mpsc channels still
    // open, whereas if we tried to do it here, it would fail for
    // a number of different reasons:
    // - channel closed
    // - not a tokio runtime
    // - different tokio runtime
    SPAN_TX.get().unwrap().try_send(SpanMessage::Span { s: arc_span }).unwrap();
}

/// Function to send a shutdown signal to the tokio runtime.
/// The receive loop will break and exit.
/// It might be better to close the channels here instead.
#[no_mangle]
pub fn nxt_otel_shutdown_tracer() {
    eprintln!("SENDER - sending shutdown code");
    SPAN_TX.get().unwrap().try_send(SpanMessage::Shutdown).unwrap();
}
