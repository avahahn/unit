/*
 * Copyright (C) F5, Inc.
 */

#include <nxt_http_parse.h>
#include <nxt_errno.h>
#include <nxt_lvlhsh.h>
#include <nxt_mp.h>
#include <nxt_work_queue.h>
#include <nxt_otel.h>
#include <nxt_main.h>
#include <nxt_http.h>

int nxt_otel_library_linkable()
{
  // todo
}

int nxt_otel_link_library()
{
  // todo
}

inline void
nxt_otel_test_and_call_state(nxt_task_t *t, nxt_http_request_t *r)
{
    // catches null state and unset flow status
    if (!r->otel || !r->otel->status) {
        return;
    }

    switch (r->otel->status) {
    case NXT_OTEL_INIT_STATE:
        nxt_otel_trace_and_span_init(t, r);
        break;
    case NXT_OTEL_HEADER_STATE:
        nxt_otel_span_add_header(r);
        break;
    case NXT_OTEL_BODY_STATE:
        nxt_otel_span_add_body(r);
        break;
    case NXT_OTEL_COLLECT_STATE:
        nxt_otel_span_collect(t, r);
        break;
    case NXT_OTEL_ERROR_STATE:
        nxt_otel_error(r);
        break;
    }
}

static inline void
nxt_otel_state_transition(nxt_otel_state_t *state, nxt_otel_status_t *status)
{
    if (status == NXT_OTEL_ERROR_STATE || state->status != NXT_OTEL_ERROR_STATE) {
        state->status = status;
    }
}

static inline void
nxt_otel_trace_and_span_init(nxt_task_t *t, nxt_http_request_t *r)
{
    nxt_work_queue_add(&t->thread->engine->fast_work_queue,
                       nxt_otel_find_or_set_trace, t, r);
    nxt_otel_state_transition(r, NXT_OTEL_HEADER_STATE);
}

static void
nxt_otel_span_add_header(nxt_http_request_t *r)
{
    /* TODO:
     * 1. extract specific headers (not all of r->fields)
     * 2. use rust library func to put them in new span
     */

    nxt_otel_state_transition(r, NXT_OTEL_BODY_STATE);
}

static void
nxt_otel_span_add_body(nxt_http_request_t *r)
{
    /* TODO:
     * 1. extract body length and total request processing time
     * 2. use rust library func to put these in new span
     */

    nxt_otel_state_transition(r, NXT_OTEL_COLLECT_STATE);
}

static inline void
nxt_otel_span_collect(nxt_task_t *t, nxt_http_request_t *r)
{
    nxt_work_queue_add(&t->thread->engine->fast_work_queue,
                       nxt_otel_send_trace_and_span_data, t, r);
    nxt_otel_state_transition(r, NULL)
}

static void
nxt_otel_error(nxt_http_request_t *r)
{
    // purposefully not using state transition helper
    r->otel->status = 0;

    /* TODO:
     * 1. log error
     * 2. trigger rust library func to release any references to trace or span data
     * 3. make null the reference to trace in r->otel
     */
}

static void
nxt_otel_find_or_set_trace(nxt_task_t *task, void *obj, void *data)
{
    nxt_request_t *r;

    r = obj;

    /* Do not fetch a new trace ID if we failed to parse one
     * present in the request headers.
     */
    if (r->otel->status == NXT_OTEL_ERROR_STATE) {
        return;
    }

    /* Do not fetch a new trace ID if we successfully parsed
     * one present in the request headers.
     */
    if(r->otel->trace_id) {
        goto init;
    }

    /* TODO: make a new trace using rust lib function
     *   a. if fail, set the otel->state to nxt_otel_error_state directly
     *   b. if success, set r->otel->(all the fields)
     *   c. fall through to init
     */

 init:
    /* TODO: do we need to initialize any state in rust lib?
     *   if not, remove this case and exit instead.
     */
}

static void
nxt_otel_send_trace_and_span_data(nxt_task_t *task, void *obj, void *data)
{
    nxt_request_t *r;
    r = obj;

    if (!r->otel->trace_key) {
        // nxt_otel_find_or_set_trace has not finished.
        // requeue and return.
        nxt_otel_span_collect(r);
        return;
    }

    nxt_otel_state_transition(r, NULL);

    /* TODO:
     * 1. call Rust library func to send traces to collector
     * 2. make null the reference to trace itself
     */
}

nxt_int_t
nxt_otel_parse_traceparent(void *ctx, nxt_http_field_t *field, uintptr_t data)
{
    nxt_request_t *r;
    char          *cursor, *copy;

    /* For information on parsing the traceparent header:
     * https://www.w3.org/TR/trace-context/#traceparent-header
     * A summary of the traceparent header value format follows:
     * Traceparent: "$a-$b-$c-$d"
     *   a. version (2 hex digits) (ff is forbidden)
     *   b. trace_id (32 hex digits) (all zeroes forbidden)
     *   c. parent_id (16 hex digits) (all zeroes forbidden)
     *   d. flags (2 hex digits)
     */

    r = ctx;
    if (field->value_length != 55) {
        goto error_state;
    }

    /* strtok and strtok_r are destructive.
     * make a second copy of the field in memory.
     */
    copy = nxt_mp_zalloc(r->mem_pool, field->value_length+1);
    if (copy == NULL) {
        goto error_state;
    }
    strncpy(copy, field->value, field->value_length);

    /* From "man strtok_r":
     *   On some implementations, *saveptr is required to be NULL on the
     *   first call to strtok_r() that is being used to parse str.
     */
    cursor = NULL;
    r->otel->version = strtok_r(copy, "-", &cursor);
    r->otel->trace_id = strtok_r(NULL, "-", &cursor);
    r->otel->parent_id = strtok_r(NULL, "-", &cursor);
    r->otel->trace_flags = strtok_r(NULL, "-", &cursor);

    if (!r->otel->version ||
        !r->otel->trace_id ||
        !r->otel->parent_id ||
        !r->otel->trace_flags) {
        goto error_state;
    }

    return NXT_OK;

 error_state:
    nxt_otel_state_transition(r->otel, NXT_OTEL_ERROR_STATE);
    return;
}

nxt_int_t
nxt_otel_parse_tracestate(void *ctx, nxt_http_field_t *field, uintptr_t data)
{
    nxt_request_t *r;
    nxt_str_t     s;

    s.length = field->value_length;
    s.start = field->value;
    r = ctx;
    r->otel->trace_state = s;

    return NXT_OK;
}
