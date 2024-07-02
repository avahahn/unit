
/*
 * Copyright (C) F5, Inc.
 */

#include <nxt_router.h>
#include <nxt_http.h>
#include <nxt_otel.h>
#include <nxt_mp.h>
#include <nxt_work_queue.h>
#include <nxt_main.h>

static inline void nxt_otel_trace_and_span_init(nxt_task_t *t, nxt_http_request_t *);
static inline void nxt_otel_span_collect(nxt_task_t *, nxt_http_request_t *);
static void nxt_otel_send_trace_and_span_data(nxt_task_t *, void *, void *);
static void nxt_otel_span_add_header(nxt_http_request_t *);
static void nxt_otel_span_add_body(nxt_http_request_t *);
static void nxt_otel_error(nxt_http_request_t *);

/*int nxt_otel_library_linkable()
{
  // todo
}

int nxt_otel_link_library()
{
  // todo
}
*/

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
nxt_otel_state_transition(nxt_otel_state_t *state, nxt_otel_status_t status)
{
    if (status == NXT_OTEL_ERROR_STATE || state->status != NXT_OTEL_ERROR_STATE) {
        state->status = status;
    }
}

static inline void
nxt_otel_trace_and_span_init(nxt_task_t *t, nxt_http_request_t *r)
{
    nxt_http_field_t *f;
    u_char *val, *name;

    r->otel->trace =
      nxt_otel_get_or_create_trace(r->otel->trace_id);
    if (!r->otel->trace) {
      nxt_otel_state_transition(r->otel, NXT_OTEL_ERROR_STATE);
      return;
    }

    name = nxt_mp_zalloc(r->mem_pool, 11);
    val = nxt_mp_zalloc(r->mem_pool, 56);
    if (!val || !name) {
      /* let it go blank here.
       * span still gets populated and sent
       * but data is not propagated to peer or app.
       *
       * TODO: Log this
       */
      return;
    }

    nxt_memcpy(name, "traceparent", 11);
    nxt_otel_copy_traceparent(val, r->otel->trace);

    f = nxt_list_add(r->resp.fields);
    if (f) {
      f->name = val;
      f->name_length = 11;
      f->value = val;
      f->value_length = 56;
    }

    nxt_otel_state_transition(r->otel, NXT_OTEL_HEADER_STATE);
}

static void
nxt_otel_span_add_header(nxt_http_request_t *r)
{
    /* TODO:
     * 1. extract specific headers (not all of r->fields)
     * 2. use rust library func to put them in new span
     */

    nxt_otel_state_transition(r->otel, NXT_OTEL_BODY_STATE);
}

static void
nxt_otel_span_add_body(nxt_http_request_t *r)
{
    /* TODO:
     * 1. extract body length and total request processing time
     * 2. use rust library func to put these in new span
     */

    nxt_otel_state_transition(r->otel, NXT_OTEL_COLLECT_STATE);
}

static inline void
nxt_otel_span_collect(nxt_task_t *t, nxt_http_request_t *r)
{
    nxt_work_queue_add(&t->thread->engine->fast_work_queue,
                       nxt_otel_send_trace_and_span_data, t, r, NULL);
    nxt_otel_state_transition(r->otel, 0);
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
nxt_otel_send_trace_and_span_data(nxt_task_t *task, void *obj, void *data)
{
    nxt_http_request_t *r;
    r = obj;

    if (!r->otel->trace) {
        // nxt_otel_find_or_set_trace has not finished.
        // requeue and return.
        nxt_otel_span_collect(task, r);
        return;
    }

    nxt_otel_state_transition(r->otel, 0);
    nxt_otel_send_trace(r->otel->trace);
    r->otel->trace = NULL;
}

nxt_int_t
nxt_otel_parse_traceparent(void *ctx, nxt_http_field_t *field, uintptr_t data)
{
    nxt_http_request_t *r;
    char             *cursor, *copy;

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
    if (field->value_length != 54) {
        goto error_state;
    }

    /* strtok and strtok_r are destructive.
     * make a second copy of the field in memory.
     */
    copy = nxt_mp_zalloc(r->mem_pool, field->value_length+1);
    if (copy == NULL) {
        goto error_state;
    }
    memcpy(copy, field->value, field->value_length);

    /* From "man strtok_r":
     *   On some implementations, *saveptr is required to be NULL on the
     *   first call to strtok_r() that is being used to parse str.
     */
    cursor = NULL;

    r->otel->version = (u_char *) strtok_r(copy, "-", &cursor);
    r->otel->trace_id = (u_char *) strtok_r(NULL, "-", &cursor);
    r->otel->parent_id = (u_char *) strtok_r(NULL, "-", &cursor);
    r->otel->trace_flags = (u_char *) strtok_r(NULL, "-", &cursor);

    if (!r->otel->version ||
        !r->otel->trace_id ||
        !r->otel->parent_id ||
        !r->otel->trace_flags) {
        goto error_state;
    }

    return NXT_OK;

 error_state:
    nxt_otel_state_transition(r->otel, NXT_OTEL_ERROR_STATE);
    return NXT_ERROR;
}

nxt_int_t
nxt_otel_parse_tracestate(void *ctx, nxt_http_field_t *field, uintptr_t data)
{
    nxt_http_request_t *r;
    nxt_str_t     s;
    nxt_http_field_t *f;

    s.length = field->value_length;
    s.start = field->value;
    r = ctx;
    r->otel->trace_state = s;

    // maybe someday this should get sent down into the otel lib

    f = nxt_list_add(r->resp.fields);
    if (f) {
      *f = *field;
    }

    return NXT_OK;
}
