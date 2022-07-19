#ifndef ICU4XGregorianDateTime_H
#define ICU4XGregorianDateTime_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct ICU4XGregorianDateTime ICU4XGregorianDateTime;
#ifdef __cplusplus
} // namespace capi
#endif
#include "diplomat_result_box_ICU4XGregorianDateTime_ICU4XError.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

diplomat_result_box_ICU4XGregorianDateTime_ICU4XError ICU4XGregorianDateTime_try_new(int32_t year, uint8_t month, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second);
void ICU4XGregorianDateTime_destroy(ICU4XGregorianDateTime* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
