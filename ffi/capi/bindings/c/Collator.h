#ifndef Collator_H
#define Collator_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "CollatorOptionsV1.d.h"
#include "CollatorResolvedOptionsV1.d.h"
#include "DataError.d.h"
#include "DataProvider.d.h"
#include "Locale.d.h"

#include "Collator.d.h"






typedef struct ICU4XCollator_create_v1_result {union {Collator* ok; DataError err;}; bool is_ok;} ICU4XCollator_create_v1_result;
ICU4XCollator_create_v1_result ICU4XCollator_create_v1(const DataProvider* provider, const Locale* locale, CollatorOptionsV1 options);

int8_t ICU4XCollator_compare_utf16_(const Collator* self, const char16_t* left_data, size_t left_len, const char16_t* right_data, size_t right_len);

int8_t ICU4XCollator_compare_(const Collator* self, const char* left_data, size_t left_len, const char* right_data, size_t right_len);

CollatorResolvedOptionsV1 ICU4XCollator_resolved_options(const Collator* self);


void ICU4XCollator_destroy(Collator* self);





#endif // Collator_H
