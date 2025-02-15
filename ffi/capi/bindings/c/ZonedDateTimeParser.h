#ifndef ZonedDateTimeParser_H
#define ZonedDateTimeParser_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "Calendar.d.h"
#include "CalendarParseError.d.h"
#include "DataError.d.h"
#include "DataProvider.d.h"
#include "ZonedDateTime.d.h"
#include "ZonedIsoDateTime.d.h"

#include "ZonedDateTimeParser.d.h"






ZonedDateTimeParser* icu4x_ZonedDateTimeParser_create_mv1(void);

typedef struct icu4x_ZonedDateTimeParser_create_with_provider_mv1_result {union {ZonedDateTimeParser* ok; DataError err;}; bool is_ok;} icu4x_ZonedDateTimeParser_create_with_provider_mv1_result;
icu4x_ZonedDateTimeParser_create_with_provider_mv1_result icu4x_ZonedDateTimeParser_create_with_provider_mv1(const DataProvider* provider);

typedef struct icu4x_ZonedDateTimeParser_try_iso_from_str_mv1_result {union {ZonedIsoDateTime ok; CalendarParseError err;}; bool is_ok;} icu4x_ZonedDateTimeParser_try_iso_from_str_mv1_result;
icu4x_ZonedDateTimeParser_try_iso_from_str_mv1_result icu4x_ZonedDateTimeParser_try_iso_from_str_mv1(const ZonedDateTimeParser* self, DiplomatStringView v);

typedef struct icu4x_ZonedDateTimeParser_try_from_str_mv1_result {union {ZonedDateTime ok; CalendarParseError err;}; bool is_ok;} icu4x_ZonedDateTimeParser_try_from_str_mv1_result;
icu4x_ZonedDateTimeParser_try_from_str_mv1_result icu4x_ZonedDateTimeParser_try_from_str_mv1(const ZonedDateTimeParser* self, DiplomatStringView v, const Calendar* calendar);


void icu4x_ZonedDateTimeParser_destroy_mv1(ZonedDateTimeParser* self);





#endif // ZonedDateTimeParser_H
