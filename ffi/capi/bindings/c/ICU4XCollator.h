#ifndef ICU4XCollator_H
#define ICU4XCollator_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XCollatorOptionsV1.d.h"
#include "ICU4XCollatorResolvedOptionsV1.d.h"
#include "ICU4XDataError.d.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XLocale.d.h"

#include "ICU4XCollator.d.h"






struct ICU4XCollator_create_v1_result {union {ICU4XCollator* ok; ICU4XDataError err;}; bool is_ok;};
struct ICU4XCollator_create_v1_result ICU4XCollator_create_v1(const ICU4XDataProvider* provider, const ICU4XLocale* locale, ICU4XCollatorOptionsV1 options);

int8_t ICU4XCollator_compare_utf16_(const ICU4XCollator* self, const char16_t* left_data, size_t left_len, const char16_t* right_data, size_t right_len);

int8_t ICU4XCollator_compare_(const ICU4XCollator* self, const char* left_data, size_t left_len, const char* right_data, size_t right_len);

ICU4XCollatorResolvedOptionsV1 ICU4XCollator_resolved_options(const ICU4XCollator* self);


void ICU4XCollator_destroy(ICU4XCollator* self);





#endif // ICU4XCollator_H
