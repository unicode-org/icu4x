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






typedef struct ICU4XMetazoneCalculator_create_result {union {MetazoneCalculator* ok; DataError err;}; bool is_ok;} ICU4XMetazoneCalculator_create_result;
ICU4XMetazoneCalculator_create_result ICU4XMetazoneCalculator_create(const DataProvider* provider);


void ICU4XMetazoneCalculator_destroy(MetazoneCalculator* self);





#endif // MetazoneCalculator_H
