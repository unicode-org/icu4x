#ifndef ICU4XIsoDate_H
#define ICU4XIsoDate_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct ICU4XIsoDate ICU4XIsoDate;
#ifdef __cplusplus
} // namespace capi
#endif
#include "diplomat_result_box_ICU4XIsoDate_ICU4XError.h"
#include "ICU4XCalendar.h"
#include "ICU4XDate.h"
#include "ICU4XIsoWeekday.h"
#include "diplomat_result_void_ICU4XError.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

diplomat_result_box_ICU4XIsoDate_ICU4XError ICU4XIsoDate_try_new(int32_t year, uint8_t month, uint8_t day);

ICU4XDate* ICU4XIsoDate_to_calendar(const ICU4XIsoDate* self, const ICU4XCalendar* calendar);

ICU4XDate* ICU4XIsoDate_to_any(const ICU4XIsoDate* self);

uint32_t ICU4XIsoDate_day_of_month(const ICU4XIsoDate* self);

ICU4XIsoWeekday ICU4XIsoDate_day_of_week(const ICU4XIsoDate* self);

uint32_t ICU4XIsoDate_ordinal_month(const ICU4XIsoDate* self);

diplomat_result_void_ICU4XError ICU4XIsoDate_month_code(const ICU4XIsoDate* self, DiplomatWriteable* write);

int32_t ICU4XIsoDate_year_in_era(const ICU4XIsoDate* self);

diplomat_result_void_ICU4XError ICU4XIsoDate_era(const ICU4XIsoDate* self, DiplomatWriteable* write);

uint8_t ICU4XIsoDate_months_in_year(const ICU4XIsoDate* self);

uint8_t ICU4XIsoDate_days_in_month(const ICU4XIsoDate* self);

uint32_t ICU4XIsoDate_days_in_year(const ICU4XIsoDate* self);
void ICU4XIsoDate_destroy(ICU4XIsoDate* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
