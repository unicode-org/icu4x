#ifndef ICU4XLogger_H
#define ICU4XLogger_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XLogger_type.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

bool ICU4XLogger_init_simple_logger();
void ICU4XLogger_destroy(ICU4XLogger* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus
#endif // ICU4XLogger_H
