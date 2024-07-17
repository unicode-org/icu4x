#ifndef LocaleExpander_H
#define LocaleExpander_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataProvider.d.h"
#include "Error.d.h"
#include "Locale.d.h"
#include "TransformResult.d.h"

#include "LocaleExpander.d.h"






typedef struct icu4x_LocaleExpander_create_mv1_result {union {LocaleExpander* ok; Error err;}; bool is_ok;} icu4x_LocaleExpander_create_mv1_result;
icu4x_LocaleExpander_create_mv1_result icu4x_LocaleExpander_create_mv1(const DataProvider* provider);

typedef struct icu4x_LocaleExpander_create_extended_mv1_result {union {LocaleExpander* ok; Error err;}; bool is_ok;} icu4x_LocaleExpander_create_extended_mv1_result;
icu4x_LocaleExpander_create_extended_mv1_result icu4x_LocaleExpander_create_extended_mv1(const DataProvider* provider);

TransformResult icu4x_LocaleExpander_maximize_mv1(const LocaleExpander* self, Locale* locale);

TransformResult icu4x_LocaleExpander_minimize_mv1(const LocaleExpander* self, Locale* locale);

TransformResult icu4x_LocaleExpander_minimize_favor_script_mv1(const LocaleExpander* self, Locale* locale);


void icu4x_LocaleExpander_destroy_mv1(LocaleExpander* self);





#endif // LocaleExpander_H
