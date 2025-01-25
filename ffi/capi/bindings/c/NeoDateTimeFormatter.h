#ifndef NeoDateTimeFormatter_H
#define NeoDateTimeFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DateTimeAlignment.d.h"
#include "DateTimeFormatterLoadError.d.h"
#include "IsoDate.d.h"
#include "Locale.d.h"
#include "NeoDateTimeLength.d.h"
#include "Time.d.h"
#include "TimePrecision.d.h"

#include "NeoDateTimeFormatter.d.h"






typedef struct icu4x_NeoDateTimeFormatter_create_dt_mv1_result {union {NeoDateTimeFormatter* ok; DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateTimeFormatter_create_dt_mv1_result;
icu4x_NeoDateTimeFormatter_create_dt_mv1_result icu4x_NeoDateTimeFormatter_create_dt_mv1(const Locale* locale, NeoDateTimeLength length, TimePrecision time_precision, DateTimeAlignment alignment);

typedef struct icu4x_NeoDateTimeFormatter_create_mdt_mv1_result {union {NeoDateTimeFormatter* ok; DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateTimeFormatter_create_mdt_mv1_result;
icu4x_NeoDateTimeFormatter_create_mdt_mv1_result icu4x_NeoDateTimeFormatter_create_mdt_mv1(const Locale* locale, NeoDateTimeLength length, TimePrecision time_precision, DateTimeAlignment alignment);

void icu4x_NeoDateTimeFormatter_format_iso_mv1(const NeoDateTimeFormatter* self, const IsoDate* date, const Time* time, DiplomatWrite* write);


void icu4x_NeoDateTimeFormatter_destroy_mv1(NeoDateTimeFormatter* self);





#endif // NeoDateTimeFormatter_H
