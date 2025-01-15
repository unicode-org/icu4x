#ifndef IxdtfParser_H
#define IxdtfParser_H

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

#include "IxdtfParser.d.h"






IxdtfParser* icu4x_IxdtfParser_create_mv1(void);

typedef struct icu4x_IxdtfParser_create_with_provider_mv1_result {union {IxdtfParser* ok; DataError err;}; bool is_ok;} icu4x_IxdtfParser_create_with_provider_mv1_result;
icu4x_IxdtfParser_create_with_provider_mv1_result icu4x_IxdtfParser_create_with_provider_mv1(const DataProvider* provider);

typedef struct icu4x_IxdtfParser_try_iso_from_str_mv1_result {union {ZonedIsoDateTime ok; CalendarParseError err;}; bool is_ok;} icu4x_IxdtfParser_try_iso_from_str_mv1_result;
icu4x_IxdtfParser_try_iso_from_str_mv1_result icu4x_IxdtfParser_try_iso_from_str_mv1(const IxdtfParser* self, DiplomatStringView v);

typedef struct icu4x_IxdtfParser_try_from_str_mv1_result {union {ZonedDateTime ok; CalendarParseError err;}; bool is_ok;} icu4x_IxdtfParser_try_from_str_mv1_result;
icu4x_IxdtfParser_try_from_str_mv1_result icu4x_IxdtfParser_try_from_str_mv1(const IxdtfParser* self, DiplomatStringView v, const Calendar* calendar);


void icu4x_IxdtfParser_destroy_mv1(IxdtfParser* self);





#endif // IxdtfParser_H
