#ifndef NeoZonedDateTimeFormatter_H
#define NeoZonedDateTimeFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DateTimeWriteError.d.h"
#include "IsoDate.d.h"
#include "Time.d.h"
#include "TimeZoneInfo.d.h"

#include "NeoZonedDateTimeFormatter.d.h"






typedef struct icu4x_NeoZonedDateTimeFormatter_format_iso_mv1_result {union { DateTimeWriteError err;}; bool is_ok;} icu4x_NeoZonedDateTimeFormatter_format_iso_mv1_result;
icu4x_NeoZonedDateTimeFormatter_format_iso_mv1_result icu4x_NeoZonedDateTimeFormatter_format_iso_mv1(const NeoZonedDateTimeFormatter* self, const IsoDate* date, const Time* time, const TimeZoneInfo* zone, DiplomatWrite* write);


void icu4x_NeoZonedDateTimeFormatter_destroy_mv1(NeoZonedDateTimeFormatter* self);





#endif // NeoZonedDateTimeFormatter_H
