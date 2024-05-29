#ifndef ICU4XCaseMapper_H
#define ICU4XCaseMapper_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XCodePointSetBuilder.d.h"
#include "ICU4XCodePointSetBuilder.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XDataProvider.h"
#include "ICU4XLocale.d.h"
#include "ICU4XLocale.h"
#include "ICU4XTitlecaseOptionsV1.d.h"
#include "ICU4XTitlecaseOptionsV1.h"
#include "diplomat_result_box_ICU4XCaseMapper_ICU4XDataError.d.h"

#include "ICU4XCaseMapper.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


diplomat_result_box_ICU4XCaseMapper_ICU4XDataError ICU4XCaseMapper_create(const ICU4XDataProvider* provider);

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


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XCaseMapper_H
