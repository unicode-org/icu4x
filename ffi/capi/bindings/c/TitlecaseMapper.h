#ifndef TitlecaseMapper_H
#define TitlecaseMapper_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"
#include "Locale.d.h"
#include "TitlecaseOptionsV1.d.h"

#include "TitlecaseMapper.d.h"






typedef struct ICU4XTitlecaseMapper_create_result {union {TitlecaseMapper* ok; DataError err;}; bool is_ok;} ICU4XTitlecaseMapper_create_result;
ICU4XTitlecaseMapper_create_result ICU4XTitlecaseMapper_create(const DataProvider* provider);

void ICU4XTitlecaseMapper_titlecase_segment_v1(const TitlecaseMapper* self, const char* s_data, size_t s_len, const Locale* locale, TitlecaseOptionsV1 options, DiplomatWrite* write);


void ICU4XTitlecaseMapper_destroy(TitlecaseMapper* self);





#endif // TitlecaseMapper_H
