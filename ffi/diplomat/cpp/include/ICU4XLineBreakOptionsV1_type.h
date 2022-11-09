#ifndef ICU4XLineBreakOptionsV1_type_H
#define ICU4XLineBreakOptionsV1_type_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XLineBreakRule_type.h"
#include "ICU4XWordBreakRule_type.h"
#ifdef __cplusplus
namespace capi {
#endif // __cplusplus

typedef struct ICU4XLineBreakOptionsV1 {
    ICU4XLineBreakRule line_break_rule;
    ICU4XWordBreakRule word_break_rule;
    bool ja_zh;
} ICU4XLineBreakOptionsV1;
#ifdef __cplusplus
} // namespace capi
#endif // __cplusplus
#endif // ICU4XLineBreakOptionsV1_type_H
