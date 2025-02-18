#ifndef ZonedDateTime_H
#define ZonedDateTime_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "Calendar.d.h"
#include "CalendarParseError.d.h"
#include "IanaParser.d.h"
#include "UtcOffsetCalculator.d.h"

#include "ZonedDateTime.d.h"






typedef struct icu4x_ZonedDateTime_try_from_str_mv1_result {union {ZonedDateTime ok; CalendarParseError err;}; bool is_ok;} icu4x_ZonedDateTime_try_from_str_mv1_result;
icu4x_ZonedDateTime_try_from_str_mv1_result icu4x_ZonedDateTime_try_from_str_mv1(DiplomatStringView v, const Calendar* calendar, const IanaParser* iana_parser, const UtcOffsetCalculator* offset_calculator);






#endif // ZonedDateTime_H
