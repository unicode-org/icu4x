#ifndef diplomat_result_str_ref_void_H
#define diplomat_result_str_ref_void_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif
typedef struct diplomat_result_str_ref_void {
    union {
        DiplomatStringView ok;
    };
    bool is_ok;
} diplomat_result_str_ref_void;
#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
