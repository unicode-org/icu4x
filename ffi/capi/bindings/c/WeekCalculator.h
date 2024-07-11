#ifndef WeekCalculator_H
#define WeekCalculator_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"
#include "IsoWeekday.d.h"
#include "Locale.d.h"
#include "WeekendContainsDay.d.h"

#include "WeekCalculator.d.h"






typedef struct ICU4XWeekCalculator_create_result {union {WeekCalculator* ok; DataError err;}; bool is_ok;} ICU4XWeekCalculator_create_result;
ICU4XWeekCalculator_create_result ICU4XWeekCalculator_create(const DataProvider* provider, const Locale* locale);

WeekCalculator* ICU4XWeekCalculator_create_from_first_day_of_week_and_min_week_days(IsoWeekday first_weekday, uint8_t min_week_days);

IsoWeekday ICU4XWeekCalculator_first_weekday(const WeekCalculator* self);

uint8_t ICU4XWeekCalculator_min_week_days(const WeekCalculator* self);

WeekendContainsDay ICU4XWeekCalculator_weekend(const WeekCalculator* self);


void ICU4XWeekCalculator_destroy(WeekCalculator* self);





#endif // WeekCalculator_H
