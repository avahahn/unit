/*
 * Copyright (C) F5, Inc.
 */

#include <nxt_http_parse.h>

#if !defined _NXT_OTEL_H_INCLUDED_ && defined NXT_OTEL
#define _NXT_OTEL_H_INCLUDED_

extern struct nxt_otel_span_s;
extern struct nxt_otel_trace_s;

typedef struct nxt_otel_span_t  nxt_otel_span_s;
typedef struct nxt_otel_trace_t nxt_otel_trace_s;

/* nxt_otel_status_t
 * more efficient than a single handler state struct
 */
typedef enum {
    // 0 = uninitialized and/or unset status
    NXT_OTEL_INIT_STATE = 1,
    NXT_OTEL_HEADER_STATE,
    NXT_OTEL_BODY_STATE,
    NXT_OTEL_COLLECT_STATE,
    NXT_OTEL_ERROR_STATE,
} nxt_otel_status_t;

/* nxt_otel_t
 * cache of trace data needed per request and
 * includes indicator as to current flow state
 */
typedef struct {
    char              *version;
    char              *trace_id;
    char              *parent_id;
    char              *trace_flags;
    nxt_otel_status_t status;
    nxt_str_t         trace_state;
} nxt_otel_state_t;

int nxt_otel_library_linkable();
int nxt_otel_link_library();
void nxt_otel_test_and_call_state(nxt_http_request_t *);
nxt_int_t nxt_otel_parse_traceparent(void *ctx, nxt_http_field_t *field, uintptr_t data);
nxt_int_t nxt_otel_parse_tracestate(void *ctx, nxt_http_field_t *field, uintptr_t data);

#endif // _NXT_OTEL_H_INCLUDED_
