#ifndef ICU4XMetaZoneCalculator_H
#define ICU4XMetaZoneCalculator_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct ICU4XMetaZoneCalculator ICU4XMetaZoneCalculator;
#ifdef __cplusplus
} // namespace capi
#endif
#include "ICU4XDataProvider.h"
#include "diplomat_result_box_ICU4XMetaZoneCalculator_ICU4XError.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

diplomat_result_box_ICU4XMetaZoneCalculator_ICU4XError ICU4XMetaZoneCalculator_try_new(const ICU4XDataProvider* provider);
void ICU4XMetaZoneCalculator_destroy(ICU4XMetaZoneCalculator* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
