#ifndef ICU4XWeekCalculator_H
#define ICU4XWeekCalculator_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XDataError.d.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XIsoWeekday.d.h"
#include "ICU4XLocale.d.h"
#include "ICU4XWeekendContainsDay.d.h"

#include "ICU4XWeekCalculator.d.h"






typedef struct ICU4XWeekCalculator_create_result {union {ICU4XWeekCalculator* ok; ICU4XDataError err;}; bool is_ok;} ICU4XWeekCalculator_create_result;
ICU4XWeekCalculator_create_result ICU4XWeekCalculator_create(const ICU4XDataProvider* provider, const ICU4XLocale* locale);

ICU4XWeekCalculator* ICU4XWeekCalculator_create_from_first_day_of_week_and_min_week_days(ICU4XIsoWeekday first_weekday, uint8_t min_week_days);

ICU4XIsoWeekday ICU4XWeekCalculator_first_weekday(const ICU4XWeekCalculator* self);

uint8_t ICU4XWeekCalculator_min_week_days(const ICU4XWeekCalculator* self);

ICU4XWeekendContainsDay ICU4XWeekCalculator_weekend(const ICU4XWeekCalculator* self);


void ICU4XWeekCalculator_destroy(ICU4XWeekCalculator* self);





#endif // ICU4XWeekCalculator_H
