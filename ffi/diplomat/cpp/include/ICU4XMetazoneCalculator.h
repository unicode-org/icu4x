#ifndef ICU4XMetazoneCalculator_H
#define ICU4XMetazoneCalculator_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XMetazoneCalculator_type.h"
#include "ICU4XDataProvider_type.h"
#include "diplomat_result_box_ICU4XMetazoneCalculator_ICU4XError.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

diplomat_result_box_ICU4XMetazoneCalculator_ICU4XError ICU4XMetazoneCalculator_create(const ICU4XDataProvider* provider);
void ICU4XMetazoneCalculator_destroy(ICU4XMetazoneCalculator* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus
#endif // ICU4XMetazoneCalculator_H
