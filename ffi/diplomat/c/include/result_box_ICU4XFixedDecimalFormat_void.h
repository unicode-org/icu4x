#ifndef result_box_ICU4XFixedDecimalFormat_void_H
#define result_box_ICU4XFixedDecimalFormat_void_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif
typedef struct ICU4XFixedDecimalFormat ICU4XFixedDecimalFormat;
typedef struct decimal_ffi_result_box_ICU4XFixedDecimalFormat_void {
    union {
        ICU4XFixedDecimalFormat* ok;
    };
    bool is_ok;
} decimal_ffi_result_box_ICU4XFixedDecimalFormat_void;
#ifdef __cplusplus
}
#endif
#endif
