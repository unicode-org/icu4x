#ifndef result_void_void_H
#define result_void_void_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif
typedef struct decimal_ffi_result_void_void {
    union {
        uint8_t ok[0];
        uint8_t err[0];
    };
    bool is_ok;
} decimal_ffi_result_void_void;
#ifdef __cplusplus
}
#endif
#endif
