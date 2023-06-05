#ifndef PR21_COMMON_INC
#define PR21_COMMON_INC

#include <stddef.h>

enum {
  PR21_POLL_PENDING,
  PR21_POLL_READY
};

typedef int32_t pr21_poll_state_t;

typedef struct {
  pr21_poll_state_t state;
} pr21_poll_void_t;

typedef struct {
  pr21_poll_state_t state;
  int32_t value;
} pr21_poll_int_t;

typedef struct {
  pr21_poll_state_t state;
  float value;
} pr21_poll_float_t;

typedef struct {
  pr21_poll_state_t state;
  int32_t value1;
  int32_t value2;
} pr21_poll_tuple_ii_t;

typedef struct {
  pr21_poll_state_t state;
  int32_t value1;
  float value2;
} pr21_poll_tuple_if_t;

typedef struct {
  pr21_poll_state_t state;
  float value1;
  int32_t value2;
} pr21_poll_tuple_fi_t;

typedef struct {
  pr21_poll_state_t state;
  float value1;
  float value2;
} pr21_poll_tuple_ff_t;

#endif /* PR21_COMMON_INC */
