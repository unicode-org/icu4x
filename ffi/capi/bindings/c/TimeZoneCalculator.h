#ifndef TimeZoneCalculator_H
#define TimeZoneCalculator_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"

#include "TimeZoneCalculator.d.h"






typedef struct icu4x_TimeZoneCalculator_create_mv1_result {union {TimeZoneCalculator* ok; DataError err;}; bool is_ok;} icu4x_TimeZoneCalculator_create_mv1_result;
icu4x_TimeZoneCalculator_create_mv1_result icu4x_TimeZoneCalculator_create_mv1(const DataProvider* provider);


void icu4x_TimeZoneCalculator_destroy_mv1(TimeZoneCalculator* self);





#endif // TimeZoneCalculator_H
