#ifndef result_void_ICU4XLocaleError_H
#define result_void_ICU4XLocaleError_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif
#include "ICU4XLocaleError.h"
typedef struct locale_ffi_result_void_ICU4XLocaleError {
    union {
        ICU4XLocaleError err;
    };
    bool is_ok;
} locale_ffi_result_void_ICU4XLocaleError;
#ifdef __cplusplus
}
#endif
#endif
