#ifndef diplomat_result_uint16_t_void_D_H
#define diplomat_result_uint16_t_void_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


typedef struct diplomat_result_uint16_t_void {
  union {
    uint16_t ok;
  };
  bool is_ok;
} diplomat_result_uint16_t_void;

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // diplomat_result_uint16_t_void_D_H
