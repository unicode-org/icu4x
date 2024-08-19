#ifndef MetazoneCalculator_H
#define MetazoneCalculator_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"

#include "MetazoneCalculator.d.h"






typedef struct icu4x_MetazoneCalculator_create_mv1_result {union {MetazoneCalculator* ok; DataError err;}; bool is_ok;} icu4x_MetazoneCalculator_create_mv1_result;
icu4x_MetazoneCalculator_create_mv1_result icu4x_MetazoneCalculator_create_mv1(const DataProvider* provider);


void icu4x_MetazoneCalculator_destroy_mv1(MetazoneCalculator* self);





#endif // MetazoneCalculator_H
