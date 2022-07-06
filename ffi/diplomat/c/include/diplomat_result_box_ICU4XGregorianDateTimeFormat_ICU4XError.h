#ifndef diplomat_result_box_ICU4XGregorianDateTimeFormat_ICU4XError_H
#define diplomat_result_box_ICU4XGregorianDateTimeFormat_ICU4XError_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif
typedef struct ICU4XGregorianDateTimeFormat ICU4XGregorianDateTimeFormat;
#include "ICU4XError.h"
typedef struct diplomat_result_box_ICU4XGregorianDateTimeFormat_ICU4XError {
    union {
        ICU4XGregorianDateTimeFormat* ok;
        ICU4XError err;
    };
    bool is_ok;
} diplomat_result_box_ICU4XGregorianDateTimeFormat_ICU4XError;
#ifdef __cplusplus
}
#endif
#endif
