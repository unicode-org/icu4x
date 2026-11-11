#ifndef DurationFormatter_H
#define DurationFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataProvider.d.h"
#include "Duration.d.h"
#include "DurationFormatterError.d.h"
#include "DurationFormatterOptions.d.h"
#include "Locale.d.h"
#include "Parts.d.h"

#include "DurationFormatter.d.h"






typedef struct icu4x_DurationFormatter_create_mv1_result {union {DurationFormatter* ok; DurationFormatterError err;}; bool is_ok;} icu4x_DurationFormatter_create_mv1_result;
icu4x_DurationFormatter_create_mv1_result icu4x_DurationFormatter_create_mv1(const Locale* locale, DurationFormatterOptions options);

typedef struct icu4x_DurationFormatter_create_with_provider_mv1_result {union {DurationFormatter* ok; DurationFormatterError err;}; bool is_ok;} icu4x_DurationFormatter_create_with_provider_mv1_result;
icu4x_DurationFormatter_create_with_provider_mv1_result icu4x_DurationFormatter_create_with_provider_mv1(const DataProvider* provider, const Locale* locale, DurationFormatterOptions options);

void icu4x_DurationFormatter_format_mv1(const DurationFormatter* self, Duration duration, DiplomatWrite* write);

void icu4x_DurationFormatter_format_utf8_to_parts_mv1(const DurationFormatter* self, Duration duration, Parts* parts, DiplomatWrite* write);

void icu4x_DurationFormatter_format_utf16_to_parts_mv1(const DurationFormatter* self, Duration duration, Parts* parts, DiplomatWrite* write);

void icu4x_DurationFormatter_destroy_mv1(DurationFormatter* self);





#endif // DurationFormatter_H
