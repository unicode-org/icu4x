#ifndef diplomat_result_box_ICU4XLineBreakSegmenter_void_H
#define diplomat_result_box_ICU4XLineBreakSegmenter_void_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif
typedef struct ICU4XLineBreakSegmenter ICU4XLineBreakSegmenter;
typedef struct diplomat_result_box_ICU4XLineBreakSegmenter_void {
    union {
        ICU4XLineBreakSegmenter* ok;
    };
    bool is_ok;
} diplomat_result_box_ICU4XLineBreakSegmenter_void;
#ifdef __cplusplus
}
#endif
#endif
