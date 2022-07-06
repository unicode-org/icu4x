#ifndef ICU4XGregorianDateTime_H
#define ICU4XGregorianDateTime_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ICU4XGregorianDateTime ICU4XGregorianDateTime;
#include "diplomat_result_box_ICU4XGregorianDateTime_ICU4XError.h"

diplomat_result_box_ICU4XGregorianDateTime_ICU4XError ICU4XGregorianDateTime_new_gregorian_datetime(int32_t year, uint8_t month, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second);
void ICU4XGregorianDateTime_destroy(ICU4XGregorianDateTime* self);

#ifdef __cplusplus
}
#endif
#endif
