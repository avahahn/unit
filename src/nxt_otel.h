/*
 * Copyright (C) F5, Inc.
 */

#if !defined _NXT_OTEL_H_INCLUDED_ && defined NXT_OTEL
#define _NXT_OTEL_H_INCLUDED_

extern struct nxt_otel_span_s;
extern struct nxt_otel_trace_s;

typedef struct nxt_otel_span_t  nxt_otel_span_s;
typedef struct nxt_otel_trace_t nxt_otel_trace_s;

/* nxt_otel_state_t
 * cache of data needed per request
 */
typedef struct {
  char *trace_key;
} nxt_otel_state_t;

int nxt_otel_library_linkable();
int nxt_otel_link_library();

#endif // _NXT_OTEL_H_INCLUDED_
