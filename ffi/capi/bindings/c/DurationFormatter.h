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
#include "DurationFormatterOptionsV1.d.h"
#include "Locale.d.h"

#include "DurationFormatter.d.h"






typedef struct icu4x_DurationFormatter_create_v1_mv1_result {union {DurationFormatter* ok; DurationFormatterError err;}; bool is_ok;} icu4x_DurationFormatter_create_v1_mv1_result;
icu4x_DurationFormatter_create_v1_mv1_result icu4x_DurationFormatter_create_v1_mv1(const Locale* locale, DurationFormatterOptionsV1 options);

typedef struct icu4x_DurationFormatter_create_v1_with_provider_mv1_result {union {DurationFormatter* ok; DurationFormatterError err;}; bool is_ok;} icu4x_DurationFormatter_create_v1_with_provider_mv1_result;
icu4x_DurationFormatter_create_v1_with_provider_mv1_result icu4x_DurationFormatter_create_v1_with_provider_mv1(const DataProvider* provider, const Locale* locale, DurationFormatterOptionsV1 options);

void icu4x_DurationFormatter_format_mv1(const DurationFormatter* self, Duration duration, DiplomatWrite* write);

void icu4x_DurationFormatter_destroy_mv1(DurationFormatter* self);





#endif // DurationFormatter_H
