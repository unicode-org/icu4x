#ifndef ICU4XTitlecaseMapper_H
#define ICU4XTitlecaseMapper_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XDataProvider.h"
#include "ICU4XLocale.d.h"
#include "ICU4XLocale.h"
#include "ICU4XTitlecaseOptionsV1.d.h"
#include "ICU4XTitlecaseOptionsV1.h"
#include "diplomat_result_box_ICU4XTitlecaseMapper_ICU4XError.d.h"

#include "ICU4XTitlecaseMapper.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


diplomat_result_box_ICU4XTitlecaseMapper_ICU4XError ICU4XTitlecaseMapper_create(const ICU4XDataProvider* provider);

void ICU4XTitlecaseMapper_titlecase_segment_v1(const ICU4XTitlecaseMapper* self, const char* s_data, size_t s_len, const ICU4XLocale* locale, ICU4XTitlecaseOptionsV1 options, DiplomatWrite* write);

void ICU4XTitlecaseMapper_destroy(ICU4XTitlecaseMapper* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XTitlecaseMapper_H
