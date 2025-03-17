#ifndef NeoDateFormatter_H
#define NeoDateFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataProvider.d.h"
#include "Date.d.h"
#include "DateTimeAlignment.d.h"
#include "DateTimeFormatterLoadError.d.h"
#include "DateTimeLength.d.h"
#include "DateTimeMismatchedCalendarError.d.h"
#include "IsoDate.d.h"
#include "Locale.d.h"
#include "YearStyle.d.h"

#include "NeoDateFormatter.d.h"






typedef struct icu4x_NeoDateFormatter_create_d_mv1_result {union {NeoDateFormatter* ok; DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatter_create_d_mv1_result;
icu4x_NeoDateFormatter_create_d_mv1_result icu4x_NeoDateFormatter_create_d_mv1(const Locale* locale, DateTimeLength_option length, DateTimeAlignment_option alignment);

typedef struct icu4x_NeoDateFormatter_create_d_with_provider_mv1_result {union {NeoDateFormatter* ok; DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatter_create_d_with_provider_mv1_result;
icu4x_NeoDateFormatter_create_d_with_provider_mv1_result icu4x_NeoDateFormatter_create_d_with_provider_mv1(const DataProvider* provider, const Locale* locale, DateTimeLength_option length, DateTimeAlignment_option alignment);

typedef struct icu4x_NeoDateFormatter_create_md_mv1_result {union {NeoDateFormatter* ok; DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatter_create_md_mv1_result;
icu4x_NeoDateFormatter_create_md_mv1_result icu4x_NeoDateFormatter_create_md_mv1(const Locale* locale, DateTimeLength_option length, DateTimeAlignment_option alignment);

typedef struct icu4x_NeoDateFormatter_create_md_with_provider_mv1_result {union {NeoDateFormatter* ok; DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatter_create_md_with_provider_mv1_result;
icu4x_NeoDateFormatter_create_md_with_provider_mv1_result icu4x_NeoDateFormatter_create_md_with_provider_mv1(const DataProvider* provider, const Locale* locale, DateTimeLength_option length, DateTimeAlignment_option alignment);

typedef struct icu4x_NeoDateFormatter_create_ymd_mv1_result {union {NeoDateFormatter* ok; DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatter_create_ymd_mv1_result;
icu4x_NeoDateFormatter_create_ymd_mv1_result icu4x_NeoDateFormatter_create_ymd_mv1(const Locale* locale, DateTimeLength_option length, DateTimeAlignment_option alignment, YearStyle_option year_style);

typedef struct icu4x_NeoDateFormatter_create_ymd_with_provider_mv1_result {union {NeoDateFormatter* ok; DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatter_create_ymd_with_provider_mv1_result;
icu4x_NeoDateFormatter_create_ymd_with_provider_mv1_result icu4x_NeoDateFormatter_create_ymd_with_provider_mv1(const DataProvider* provider, const Locale* locale, DateTimeLength_option length, DateTimeAlignment_option alignment, YearStyle_option year_style);

typedef struct icu4x_NeoDateFormatter_create_de_mv1_result {union {NeoDateFormatter* ok; DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatter_create_de_mv1_result;
icu4x_NeoDateFormatter_create_de_mv1_result icu4x_NeoDateFormatter_create_de_mv1(const Locale* locale, DateTimeLength_option length, DateTimeAlignment_option alignment);

typedef struct icu4x_NeoDateFormatter_create_de_with_provider_mv1_result {union {NeoDateFormatter* ok; DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatter_create_de_with_provider_mv1_result;
icu4x_NeoDateFormatter_create_de_with_provider_mv1_result icu4x_NeoDateFormatter_create_de_with_provider_mv1(const DataProvider* provider, const Locale* locale, DateTimeLength_option length, DateTimeAlignment_option alignment);

typedef struct icu4x_NeoDateFormatter_create_mde_mv1_result {union {NeoDateFormatter* ok; DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatter_create_mde_mv1_result;
icu4x_NeoDateFormatter_create_mde_mv1_result icu4x_NeoDateFormatter_create_mde_mv1(const Locale* locale, DateTimeLength_option length, DateTimeAlignment_option alignment);

typedef struct icu4x_NeoDateFormatter_create_mde_with_provider_mv1_result {union {NeoDateFormatter* ok; DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatter_create_mde_with_provider_mv1_result;
icu4x_NeoDateFormatter_create_mde_with_provider_mv1_result icu4x_NeoDateFormatter_create_mde_with_provider_mv1(const DataProvider* provider, const Locale* locale, DateTimeLength_option length, DateTimeAlignment_option alignment);

typedef struct icu4x_NeoDateFormatter_create_ymde_mv1_result {union {NeoDateFormatter* ok; DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatter_create_ymde_mv1_result;
icu4x_NeoDateFormatter_create_ymde_mv1_result icu4x_NeoDateFormatter_create_ymde_mv1(const Locale* locale, DateTimeLength_option length, DateTimeAlignment_option alignment, YearStyle_option year_style);

typedef struct icu4x_NeoDateFormatter_create_ymde_with_provider_mv1_result {union {NeoDateFormatter* ok; DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatter_create_ymde_with_provider_mv1_result;
icu4x_NeoDateFormatter_create_ymde_with_provider_mv1_result icu4x_NeoDateFormatter_create_ymde_with_provider_mv1(const DataProvider* provider, const Locale* locale, DateTimeLength_option length, DateTimeAlignment_option alignment, YearStyle_option year_style);

typedef struct icu4x_NeoDateFormatter_create_e_mv1_result {union {NeoDateFormatter* ok; DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatter_create_e_mv1_result;
icu4x_NeoDateFormatter_create_e_mv1_result icu4x_NeoDateFormatter_create_e_mv1(const Locale* locale, DateTimeLength_option length);

typedef struct icu4x_NeoDateFormatter_create_e_with_provider_mv1_result {union {NeoDateFormatter* ok; DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatter_create_e_with_provider_mv1_result;
icu4x_NeoDateFormatter_create_e_with_provider_mv1_result icu4x_NeoDateFormatter_create_e_with_provider_mv1(const DataProvider* provider, const Locale* locale, DateTimeLength_option length);

typedef struct icu4x_NeoDateFormatter_create_m_mv1_result {union {NeoDateFormatter* ok; DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatter_create_m_mv1_result;
icu4x_NeoDateFormatter_create_m_mv1_result icu4x_NeoDateFormatter_create_m_mv1(const Locale* locale, DateTimeLength_option length, DateTimeAlignment_option alignment);

typedef struct icu4x_NeoDateFormatter_create_m_with_provider_mv1_result {union {NeoDateFormatter* ok; DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatter_create_m_with_provider_mv1_result;
icu4x_NeoDateFormatter_create_m_with_provider_mv1_result icu4x_NeoDateFormatter_create_m_with_provider_mv1(const DataProvider* provider, const Locale* locale, DateTimeLength_option length, DateTimeAlignment_option alignment);

typedef struct icu4x_NeoDateFormatter_create_ym_mv1_result {union {NeoDateFormatter* ok; DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatter_create_ym_mv1_result;
icu4x_NeoDateFormatter_create_ym_mv1_result icu4x_NeoDateFormatter_create_ym_mv1(const Locale* locale, DateTimeLength_option length, DateTimeAlignment_option alignment, YearStyle_option year_style);

typedef struct icu4x_NeoDateFormatter_create_ym_with_provider_mv1_result {union {NeoDateFormatter* ok; DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatter_create_ym_with_provider_mv1_result;
icu4x_NeoDateFormatter_create_ym_with_provider_mv1_result icu4x_NeoDateFormatter_create_ym_with_provider_mv1(const DataProvider* provider, const Locale* locale, DateTimeLength_option length, DateTimeAlignment_option alignment, YearStyle_option year_style);

typedef struct icu4x_NeoDateFormatter_create_y_mv1_result {union {NeoDateFormatter* ok; DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatter_create_y_mv1_result;
icu4x_NeoDateFormatter_create_y_mv1_result icu4x_NeoDateFormatter_create_y_mv1(const Locale* locale, DateTimeLength_option length, DateTimeAlignment_option alignment, YearStyle_option year_style);

typedef struct icu4x_NeoDateFormatter_create_y_with_provider_mv1_result {union {NeoDateFormatter* ok; DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatter_create_y_with_provider_mv1_result;
icu4x_NeoDateFormatter_create_y_with_provider_mv1_result icu4x_NeoDateFormatter_create_y_with_provider_mv1(const DataProvider* provider, const Locale* locale, DateTimeLength_option length, DateTimeAlignment_option alignment, YearStyle_option year_style);

void icu4x_NeoDateFormatter_format_iso_mv1(const NeoDateFormatter* self, const IsoDate* date, DiplomatWrite* write);

typedef struct icu4x_NeoDateFormatter_format_same_calendar_mv1_result {union { DateTimeMismatchedCalendarError err;}; bool is_ok;} icu4x_NeoDateFormatter_format_same_calendar_mv1_result;
icu4x_NeoDateFormatter_format_same_calendar_mv1_result icu4x_NeoDateFormatter_format_same_calendar_mv1(const NeoDateFormatter* self, const Date* date, DiplomatWrite* write);


void icu4x_NeoDateFormatter_destroy_mv1(NeoDateFormatter* self);





#endif // NeoDateFormatter_H
