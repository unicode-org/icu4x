#ifndef ZoneOffsetCalculator_H
#define ZoneOffsetCalculator_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"

#include "ZoneOffsetCalculator.d.h"






typedef struct icu4x_ZoneOffsetCalculator_create_mv1_result {union {ZoneOffsetCalculator* ok; DataError err;}; bool is_ok;} icu4x_ZoneOffsetCalculator_create_mv1_result;
icu4x_ZoneOffsetCalculator_create_mv1_result icu4x_ZoneOffsetCalculator_create_mv1(const DataProvider* provider);


void icu4x_ZoneOffsetCalculator_destroy_mv1(ZoneOffsetCalculator* self);





#endif // ZoneOffsetCalculator_H
