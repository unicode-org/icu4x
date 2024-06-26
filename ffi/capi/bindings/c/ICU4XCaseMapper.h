#ifndef ICU4XCaseMapper_H
#define ICU4XCaseMapper_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XCodePointSetBuilder.d.h"
#include "ICU4XDataError.d.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XLocale.d.h"
#include "ICU4XTitlecaseOptionsV1.d.h"

#include "ICU4XCaseMapper.d.h"






typedef struct ICU4XCaseMapper_create_result {union {ICU4XCaseMapper* ok; ICU4XDataError err;}; bool is_ok;} ICU4XCaseMapper_create_result;
ICU4XCaseMapper_create_result ICU4XCaseMapper_create(const ICU4XDataProvider* provider);

void ICU4XCaseMapper_lowercase(const ICU4XCaseMapper* self, const char* s_data, size_t s_len, const ICU4XLocale* locale, DiplomatWrite* write);

void ICU4XCaseMapper_uppercase(const ICU4XCaseMapper* self, const char* s_data, size_t s_len, const ICU4XLocale* locale, DiplomatWrite* write);

void ICU4XCaseMapper_titlecase_segment_with_only_case_data_v1(const ICU4XCaseMapper* self, const char* s_data, size_t s_len, const ICU4XLocale* locale, ICU4XTitlecaseOptionsV1 options, DiplomatWrite* write);

void ICU4XCaseMapper_fold(const ICU4XCaseMapper* self, const char* s_data, size_t s_len, DiplomatWrite* write);

void ICU4XCaseMapper_fold_turkic(const ICU4XCaseMapper* self, const char* s_data, size_t s_len, DiplomatWrite* write);

void ICU4XCaseMapper_add_case_closure_to(const ICU4XCaseMapper* self, char32_t c, ICU4XCodePointSetBuilder* builder);

char32_t ICU4XCaseMapper_simple_lowercase(const ICU4XCaseMapper* self, char32_t ch);

char32_t ICU4XCaseMapper_simple_uppercase(const ICU4XCaseMapper* self, char32_t ch);

char32_t ICU4XCaseMapper_simple_titlecase(const ICU4XCaseMapper* self, char32_t ch);

char32_t ICU4XCaseMapper_simple_fold(const ICU4XCaseMapper* self, char32_t ch);

char32_t ICU4XCaseMapper_simple_fold_turkic(const ICU4XCaseMapper* self, char32_t ch);


void ICU4XCaseMapper_destroy(ICU4XCaseMapper* self);





#endif // ICU4XCaseMapper_H
