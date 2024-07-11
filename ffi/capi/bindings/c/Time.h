#ifndef Time_H
#define Time_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "CalendarError.d.h"

#include "Time.d.h"






typedef struct ICU4XTime_create_result {union {Time* ok; CalendarError err;}; bool is_ok;} ICU4XTime_create_result;
ICU4XTime_create_result ICU4XTime_create(uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond);

typedef struct ICU4XTime_create_midnight_result {union {Time* ok; CalendarError err;}; bool is_ok;} ICU4XTime_create_midnight_result;
ICU4XTime_create_midnight_result ICU4XTime_create_midnight();

uint8_t ICU4XTime_hour(const Time* self);

uint8_t ICU4XTime_minute(const Time* self);

uint8_t ICU4XTime_second(const Time* self);

uint32_t ICU4XTime_nanosecond(const Time* self);


void ICU4XTime_destroy(Time* self);





#endif // Time_H
