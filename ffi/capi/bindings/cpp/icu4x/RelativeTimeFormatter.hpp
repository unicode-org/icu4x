#ifndef ICU4X_RelativeTimeFormatter_HPP
#define ICU4X_RelativeTimeFormatter_HPP

#include "RelativeTimeFormatter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "Decimal.hpp"
#include "Locale.hpp"
#include "RelativeTimeFormatterOptionsV1.hpp"
#include "diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    extern "C" {

    typedef struct icu4x_RelativeTimeFormatter_create_long_second_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_long_second_mv1_result;
    icu4x_RelativeTimeFormatter_create_long_second_mv1_result icu4x_RelativeTimeFormatter_create_long_second_mv1(const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_long_second_with_provider_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_long_second_with_provider_mv1_result;
    icu4x_RelativeTimeFormatter_create_long_second_with_provider_mv1_result icu4x_RelativeTimeFormatter_create_long_second_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_long_minute_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_long_minute_mv1_result;
    icu4x_RelativeTimeFormatter_create_long_minute_mv1_result icu4x_RelativeTimeFormatter_create_long_minute_mv1(const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_long_minute_with_provider_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_long_minute_with_provider_mv1_result;
    icu4x_RelativeTimeFormatter_create_long_minute_with_provider_mv1_result icu4x_RelativeTimeFormatter_create_long_minute_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_long_hour_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_long_hour_mv1_result;
    icu4x_RelativeTimeFormatter_create_long_hour_mv1_result icu4x_RelativeTimeFormatter_create_long_hour_mv1(const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_long_hour_with_provider_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_long_hour_with_provider_mv1_result;
    icu4x_RelativeTimeFormatter_create_long_hour_with_provider_mv1_result icu4x_RelativeTimeFormatter_create_long_hour_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_long_day_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_long_day_mv1_result;
    icu4x_RelativeTimeFormatter_create_long_day_mv1_result icu4x_RelativeTimeFormatter_create_long_day_mv1(const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_long_day_with_provider_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_long_day_with_provider_mv1_result;
    icu4x_RelativeTimeFormatter_create_long_day_with_provider_mv1_result icu4x_RelativeTimeFormatter_create_long_day_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_long_week_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_long_week_mv1_result;
    icu4x_RelativeTimeFormatter_create_long_week_mv1_result icu4x_RelativeTimeFormatter_create_long_week_mv1(const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_long_week_with_provider_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_long_week_with_provider_mv1_result;
    icu4x_RelativeTimeFormatter_create_long_week_with_provider_mv1_result icu4x_RelativeTimeFormatter_create_long_week_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_long_month_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_long_month_mv1_result;
    icu4x_RelativeTimeFormatter_create_long_month_mv1_result icu4x_RelativeTimeFormatter_create_long_month_mv1(const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_long_month_with_provider_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_long_month_with_provider_mv1_result;
    icu4x_RelativeTimeFormatter_create_long_month_with_provider_mv1_result icu4x_RelativeTimeFormatter_create_long_month_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_long_quarter_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_long_quarter_mv1_result;
    icu4x_RelativeTimeFormatter_create_long_quarter_mv1_result icu4x_RelativeTimeFormatter_create_long_quarter_mv1(const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_long_quarter_with_provider_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_long_quarter_with_provider_mv1_result;
    icu4x_RelativeTimeFormatter_create_long_quarter_with_provider_mv1_result icu4x_RelativeTimeFormatter_create_long_quarter_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_long_year_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_long_year_mv1_result;
    icu4x_RelativeTimeFormatter_create_long_year_mv1_result icu4x_RelativeTimeFormatter_create_long_year_mv1(const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_long_year_with_provider_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_long_year_with_provider_mv1_result;
    icu4x_RelativeTimeFormatter_create_long_year_with_provider_mv1_result icu4x_RelativeTimeFormatter_create_long_year_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_short_second_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_short_second_mv1_result;
    icu4x_RelativeTimeFormatter_create_short_second_mv1_result icu4x_RelativeTimeFormatter_create_short_second_mv1(const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_short_second_with_provider_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_short_second_with_provider_mv1_result;
    icu4x_RelativeTimeFormatter_create_short_second_with_provider_mv1_result icu4x_RelativeTimeFormatter_create_short_second_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_short_minute_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_short_minute_mv1_result;
    icu4x_RelativeTimeFormatter_create_short_minute_mv1_result icu4x_RelativeTimeFormatter_create_short_minute_mv1(const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_short_minute_with_provider_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_short_minute_with_provider_mv1_result;
    icu4x_RelativeTimeFormatter_create_short_minute_with_provider_mv1_result icu4x_RelativeTimeFormatter_create_short_minute_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_short_hour_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_short_hour_mv1_result;
    icu4x_RelativeTimeFormatter_create_short_hour_mv1_result icu4x_RelativeTimeFormatter_create_short_hour_mv1(const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_short_hour_with_provider_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_short_hour_with_provider_mv1_result;
    icu4x_RelativeTimeFormatter_create_short_hour_with_provider_mv1_result icu4x_RelativeTimeFormatter_create_short_hour_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_short_day_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_short_day_mv1_result;
    icu4x_RelativeTimeFormatter_create_short_day_mv1_result icu4x_RelativeTimeFormatter_create_short_day_mv1(const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_short_day_with_provider_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_short_day_with_provider_mv1_result;
    icu4x_RelativeTimeFormatter_create_short_day_with_provider_mv1_result icu4x_RelativeTimeFormatter_create_short_day_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_short_week_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_short_week_mv1_result;
    icu4x_RelativeTimeFormatter_create_short_week_mv1_result icu4x_RelativeTimeFormatter_create_short_week_mv1(const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_short_week_with_provider_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_short_week_with_provider_mv1_result;
    icu4x_RelativeTimeFormatter_create_short_week_with_provider_mv1_result icu4x_RelativeTimeFormatter_create_short_week_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_short_month_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_short_month_mv1_result;
    icu4x_RelativeTimeFormatter_create_short_month_mv1_result icu4x_RelativeTimeFormatter_create_short_month_mv1(const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_short_month_with_provider_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_short_month_with_provider_mv1_result;
    icu4x_RelativeTimeFormatter_create_short_month_with_provider_mv1_result icu4x_RelativeTimeFormatter_create_short_month_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_short_quarter_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_short_quarter_mv1_result;
    icu4x_RelativeTimeFormatter_create_short_quarter_mv1_result icu4x_RelativeTimeFormatter_create_short_quarter_mv1(const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_short_quarter_with_provider_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_short_quarter_with_provider_mv1_result;
    icu4x_RelativeTimeFormatter_create_short_quarter_with_provider_mv1_result icu4x_RelativeTimeFormatter_create_short_quarter_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_short_year_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_short_year_mv1_result;
    icu4x_RelativeTimeFormatter_create_short_year_mv1_result icu4x_RelativeTimeFormatter_create_short_year_mv1(const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_short_year_with_provider_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_short_year_with_provider_mv1_result;
    icu4x_RelativeTimeFormatter_create_short_year_with_provider_mv1_result icu4x_RelativeTimeFormatter_create_short_year_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_narrow_second_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_narrow_second_mv1_result;
    icu4x_RelativeTimeFormatter_create_narrow_second_mv1_result icu4x_RelativeTimeFormatter_create_narrow_second_mv1(const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_narrow_second_with_provider_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_narrow_second_with_provider_mv1_result;
    icu4x_RelativeTimeFormatter_create_narrow_second_with_provider_mv1_result icu4x_RelativeTimeFormatter_create_narrow_second_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_narrow_minute_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_narrow_minute_mv1_result;
    icu4x_RelativeTimeFormatter_create_narrow_minute_mv1_result icu4x_RelativeTimeFormatter_create_narrow_minute_mv1(const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_narrow_minute_with_provider_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_narrow_minute_with_provider_mv1_result;
    icu4x_RelativeTimeFormatter_create_narrow_minute_with_provider_mv1_result icu4x_RelativeTimeFormatter_create_narrow_minute_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_narrow_hour_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_narrow_hour_mv1_result;
    icu4x_RelativeTimeFormatter_create_narrow_hour_mv1_result icu4x_RelativeTimeFormatter_create_narrow_hour_mv1(const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_narrow_hour_with_provider_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_narrow_hour_with_provider_mv1_result;
    icu4x_RelativeTimeFormatter_create_narrow_hour_with_provider_mv1_result icu4x_RelativeTimeFormatter_create_narrow_hour_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_narrow_day_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_narrow_day_mv1_result;
    icu4x_RelativeTimeFormatter_create_narrow_day_mv1_result icu4x_RelativeTimeFormatter_create_narrow_day_mv1(const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_narrow_day_with_provider_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_narrow_day_with_provider_mv1_result;
    icu4x_RelativeTimeFormatter_create_narrow_day_with_provider_mv1_result icu4x_RelativeTimeFormatter_create_narrow_day_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_narrow_week_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_narrow_week_mv1_result;
    icu4x_RelativeTimeFormatter_create_narrow_week_mv1_result icu4x_RelativeTimeFormatter_create_narrow_week_mv1(const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_narrow_week_with_provider_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_narrow_week_with_provider_mv1_result;
    icu4x_RelativeTimeFormatter_create_narrow_week_with_provider_mv1_result icu4x_RelativeTimeFormatter_create_narrow_week_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_narrow_month_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_narrow_month_mv1_result;
    icu4x_RelativeTimeFormatter_create_narrow_month_mv1_result icu4x_RelativeTimeFormatter_create_narrow_month_mv1(const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_narrow_month_with_provider_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_narrow_month_with_provider_mv1_result;
    icu4x_RelativeTimeFormatter_create_narrow_month_with_provider_mv1_result icu4x_RelativeTimeFormatter_create_narrow_month_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_narrow_quarter_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_narrow_quarter_mv1_result;
    icu4x_RelativeTimeFormatter_create_narrow_quarter_mv1_result icu4x_RelativeTimeFormatter_create_narrow_quarter_mv1(const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_narrow_quarter_with_provider_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_narrow_quarter_with_provider_mv1_result;
    icu4x_RelativeTimeFormatter_create_narrow_quarter_with_provider_mv1_result icu4x_RelativeTimeFormatter_create_narrow_quarter_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_narrow_year_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_narrow_year_mv1_result;
    icu4x_RelativeTimeFormatter_create_narrow_year_mv1_result icu4x_RelativeTimeFormatter_create_narrow_year_mv1(const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    typedef struct icu4x_RelativeTimeFormatter_create_narrow_year_with_provider_mv1_result {union {icu4x::capi::RelativeTimeFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_RelativeTimeFormatter_create_narrow_year_with_provider_mv1_result;
    icu4x_RelativeTimeFormatter_create_narrow_year_with_provider_mv1_result icu4x_RelativeTimeFormatter_create_narrow_year_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::RelativeTimeFormatterOptionsV1 options);

    void icu4x_RelativeTimeFormatter_format_mv1(const icu4x::capi::RelativeTimeFormatter* self, const icu4x::capi::Decimal* value, icu4x::diplomat::capi::DiplomatWrite* write);

    void icu4x_RelativeTimeFormatter_destroy_mv1(RelativeTimeFormatter* self);

    } // extern "C"
} // namespace capi
} // namespace

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_long_second(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_long_second_mv1(locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_long_second_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_long_second_with_provider_mv1(provider.AsFFI(),
        locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_long_minute(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_long_minute_mv1(locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_long_minute_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_long_minute_with_provider_mv1(provider.AsFFI(),
        locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_long_hour(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_long_hour_mv1(locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_long_hour_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_long_hour_with_provider_mv1(provider.AsFFI(),
        locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_long_day(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_long_day_mv1(locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_long_day_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_long_day_with_provider_mv1(provider.AsFFI(),
        locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_long_week(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_long_week_mv1(locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_long_week_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_long_week_with_provider_mv1(provider.AsFFI(),
        locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_long_month(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_long_month_mv1(locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_long_month_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_long_month_with_provider_mv1(provider.AsFFI(),
        locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_long_quarter(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_long_quarter_mv1(locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_long_quarter_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_long_quarter_with_provider_mv1(provider.AsFFI(),
        locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_long_year(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_long_year_mv1(locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_long_year_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_long_year_with_provider_mv1(provider.AsFFI(),
        locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_short_second(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_short_second_mv1(locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_short_second_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_short_second_with_provider_mv1(provider.AsFFI(),
        locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_short_minute(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_short_minute_mv1(locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_short_minute_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_short_minute_with_provider_mv1(provider.AsFFI(),
        locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_short_hour(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_short_hour_mv1(locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_short_hour_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_short_hour_with_provider_mv1(provider.AsFFI(),
        locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_short_day(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_short_day_mv1(locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_short_day_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_short_day_with_provider_mv1(provider.AsFFI(),
        locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_short_week(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_short_week_mv1(locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_short_week_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_short_week_with_provider_mv1(provider.AsFFI(),
        locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_short_month(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_short_month_mv1(locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_short_month_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_short_month_with_provider_mv1(provider.AsFFI(),
        locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_short_quarter(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_short_quarter_mv1(locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_short_quarter_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_short_quarter_with_provider_mv1(provider.AsFFI(),
        locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_short_year(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_short_year_mv1(locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_short_year_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_short_year_with_provider_mv1(provider.AsFFI(),
        locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_narrow_second(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_narrow_second_mv1(locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_narrow_second_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_narrow_second_with_provider_mv1(provider.AsFFI(),
        locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_narrow_minute(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_narrow_minute_mv1(locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_narrow_minute_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_narrow_minute_with_provider_mv1(provider.AsFFI(),
        locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_narrow_hour(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_narrow_hour_mv1(locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_narrow_hour_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_narrow_hour_with_provider_mv1(provider.AsFFI(),
        locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_narrow_day(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_narrow_day_mv1(locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_narrow_day_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_narrow_day_with_provider_mv1(provider.AsFFI(),
        locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_narrow_week(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_narrow_week_mv1(locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_narrow_week_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_narrow_week_with_provider_mv1(provider.AsFFI(),
        locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_narrow_month(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_narrow_month_mv1(locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_narrow_month_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_narrow_month_with_provider_mv1(provider.AsFFI(),
        locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_narrow_quarter(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_narrow_quarter_mv1(locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_narrow_quarter_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_narrow_quarter_with_provider_mv1(provider.AsFFI(),
        locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_narrow_year(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_narrow_year_mv1(locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> icu4x::RelativeTimeFormatter::create_narrow_year_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options) {
    auto result = icu4x::capi::icu4x_RelativeTimeFormatter_create_narrow_year_with_provider_mv1(provider.AsFFI(),
        locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::RelativeTimeFormatter>>(std::unique_ptr<icu4x::RelativeTimeFormatter>(icu4x::RelativeTimeFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline std::string icu4x::RelativeTimeFormatter::format(const icu4x::Decimal& value) const {
    std::string output;
    icu4x::diplomat::capi::DiplomatWrite write = icu4x::diplomat::WriteFromString(output);
    icu4x::capi::icu4x_RelativeTimeFormatter_format_mv1(this->AsFFI(),
        value.AsFFI(),
        &write);
    return output;
}
template<typename W>
inline void icu4x::RelativeTimeFormatter::format_write(const icu4x::Decimal& value, W& writeable) const {
    icu4x::diplomat::capi::DiplomatWrite write = icu4x::diplomat::WriteTrait<W>::Construct(writeable);
    icu4x::capi::icu4x_RelativeTimeFormatter_format_mv1(this->AsFFI(),
        value.AsFFI(),
        &write);
}

inline const icu4x::capi::RelativeTimeFormatter* icu4x::RelativeTimeFormatter::AsFFI() const {
    return reinterpret_cast<const icu4x::capi::RelativeTimeFormatter*>(this);
}

inline icu4x::capi::RelativeTimeFormatter* icu4x::RelativeTimeFormatter::AsFFI() {
    return reinterpret_cast<icu4x::capi::RelativeTimeFormatter*>(this);
}

inline const icu4x::RelativeTimeFormatter* icu4x::RelativeTimeFormatter::FromFFI(const icu4x::capi::RelativeTimeFormatter* ptr) {
    return reinterpret_cast<const icu4x::RelativeTimeFormatter*>(ptr);
}

inline icu4x::RelativeTimeFormatter* icu4x::RelativeTimeFormatter::FromFFI(icu4x::capi::RelativeTimeFormatter* ptr) {
    return reinterpret_cast<icu4x::RelativeTimeFormatter*>(ptr);
}

inline void icu4x::RelativeTimeFormatter::operator delete(void* ptr) {
    icu4x::capi::icu4x_RelativeTimeFormatter_destroy_mv1(reinterpret_cast<icu4x::capi::RelativeTimeFormatter*>(ptr));
}


#endif // ICU4X_RelativeTimeFormatter_HPP
