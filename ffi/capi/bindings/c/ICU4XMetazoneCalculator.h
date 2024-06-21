#ifndef ICU4XMetazoneCalculator_H
#define ICU4XMetazoneCalculator_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XDataError.d.h"
#include "ICU4XDataProvider.d.h"

#include "ICU4XMetazoneCalculator.d.h"






typedef struct ICU4XMetazoneCalculator_create_result {union {ICU4XMetazoneCalculator* ok; ICU4XDataError err;}; bool is_ok;} ICU4XMetazoneCalculator_create_result;
ICU4XMetazoneCalculator_create_result ICU4XMetazoneCalculator_create(const ICU4XDataProvider* provider);


void ICU4XMetazoneCalculator_destroy(ICU4XMetazoneCalculator* self);





#endif // ICU4XMetazoneCalculator_H
