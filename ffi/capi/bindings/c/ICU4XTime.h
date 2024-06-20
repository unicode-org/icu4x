#ifndef ICU4XTime_H
#define ICU4XTime_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XCalendarError.d.h"

#include "ICU4XTime.d.h"






struct ICU4XTime_create_result {union {ICU4XTime* ok; ICU4XCalendarError err;}; bool is_ok;};
struct ICU4XTime_create_result ICU4XTime_create(uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond);

struct ICU4XTime_create_midnight_result {union {ICU4XTime* ok; ICU4XCalendarError err;}; bool is_ok;};
struct ICU4XTime_create_midnight_result ICU4XTime_create_midnight();

uint8_t ICU4XTime_hour(const ICU4XTime* self);

uint8_t ICU4XTime_minute(const ICU4XTime* self);

uint8_t ICU4XTime_second(const ICU4XTime* self);

uint32_t ICU4XTime_nanosecond(const ICU4XTime* self);


void ICU4XTime_destroy(ICU4XTime* self);





#endif // ICU4XTime_H
