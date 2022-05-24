#ifndef result_box_ICU4XBidi_void_H
#define result_box_ICU4XBidi_void_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif
typedef struct ICU4XBidi ICU4XBidi;
typedef struct bidi_ffi_result_box_ICU4XBidi_void {
    union {
        ICU4XBidi* ok;
    };
    bool is_ok;
} bidi_ffi_result_box_ICU4XBidi_void;
#ifdef __cplusplus
}
#endif
#endif
