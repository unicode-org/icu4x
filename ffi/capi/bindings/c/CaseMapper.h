#ifndef CaseMapper_H
#define CaseMapper_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "CodePointSetBuilder.d.h"
#include "DataError.d.h"
#include "DataProvider.d.h"
#include "Locale.d.h"
#include "TitlecaseOptionsV1.d.h"

#include "CaseMapper.d.h"






typedef struct ICU4XCaseMapper_create_result {union {CaseMapper* ok; DataError err;}; bool is_ok;} ICU4XCaseMapper_create_result;
ICU4XCaseMapper_create_result ICU4XCaseMapper_create(const DataProvider* provider);

void ICU4XCaseMapper_lowercase(const CaseMapper* self, const char* s_data, size_t s_len, const Locale* locale, DiplomatWrite* write);

void ICU4XCaseMapper_uppercase(const CaseMapper* self, const char* s_data, size_t s_len, const Locale* locale, DiplomatWrite* write);

void ICU4XCaseMapper_titlecase_segment_with_only_case_data_v1(const CaseMapper* self, const char* s_data, size_t s_len, const Locale* locale, TitlecaseOptionsV1 options, DiplomatWrite* write);

void ICU4XCaseMapper_fold(const CaseMapper* self, const char* s_data, size_t s_len, DiplomatWrite* write);

void ICU4XCaseMapper_fold_turkic(const CaseMapper* self, const char* s_data, size_t s_len, DiplomatWrite* write);

void ICU4XCaseMapper_add_case_closure_to(const CaseMapper* self, char32_t c, CodePointSetBuilder* builder);

char32_t ICU4XCaseMapper_simple_lowercase(const CaseMapper* self, char32_t ch);

char32_t ICU4XCaseMapper_simple_uppercase(const CaseMapper* self, char32_t ch);

char32_t ICU4XCaseMapper_simple_titlecase(const CaseMapper* self, char32_t ch);

char32_t ICU4XCaseMapper_simple_fold(const CaseMapper* self, char32_t ch);

char32_t ICU4XCaseMapper_simple_fold_turkic(const CaseMapper* self, char32_t ch);


void ICU4XCaseMapper_destroy(CaseMapper* self);





#endif // CaseMapper_H
