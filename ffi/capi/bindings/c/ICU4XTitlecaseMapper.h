#ifndef ICU4XTitlecaseMapper_H
#define ICU4XTitlecaseMapper_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XDataError.d.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XLocale.d.h"
#include "ICU4XTitlecaseOptionsV1.d.h"

#include "ICU4XTitlecaseMapper.d.h"






struct ICU4XTitlecaseMapper_create_result {union {ICU4XTitlecaseMapper* ok; ICU4XDataError err;}; bool is_ok;};
struct ICU4XTitlecaseMapper_create_result ICU4XTitlecaseMapper_create(const ICU4XDataProvider* provider);

void ICU4XTitlecaseMapper_titlecase_segment_v1(const ICU4XTitlecaseMapper* self, const char* s_data, size_t s_len, const ICU4XLocale* locale, ICU4XTitlecaseOptionsV1 options, DiplomatWrite* write);


void ICU4XTitlecaseMapper_destroy(ICU4XTitlecaseMapper* self);





#endif // ICU4XTitlecaseMapper_H
