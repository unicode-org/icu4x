#ifndef icu4x_NeoDateFormatter_HPP
#define icu4x_NeoDateFormatter_HPP

#include "NeoDateFormatter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../diplomat_runtime.hpp"
#include "DataProvider.hpp"
#include "Date.hpp"
#include "DateTimeAlignment.hpp"
#include "DateTimeFormatterLoadError.hpp"
#include "DateTimeLength.hpp"
#include "DateTimeMismatchedCalendarError.hpp"
#include "IsoDate.hpp"
#include "Locale.hpp"
#include "YearStyle.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_NeoDateFormatter_create_d_mv1_result {union {icu4x::capi::NeoDateFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatter_create_d_mv1_result;
    icu4x_NeoDateFormatter_create_d_mv1_result icu4x_NeoDateFormatter_create_d_mv1(const icu4x::capi::Locale* locale, icu4x::capi::DateTimeLength_option length, icu4x::capi::DateTimeAlignment_option alignment);
    
    typedef struct icu4x_NeoDateFormatter_create_d_with_provider_mv1_result {union {icu4x::capi::NeoDateFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatter_create_d_with_provider_mv1_result;
    icu4x_NeoDateFormatter_create_d_with_provider_mv1_result icu4x_NeoDateFormatter_create_d_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::DateTimeLength_option length, icu4x::capi::DateTimeAlignment_option alignment);
    
    typedef struct icu4x_NeoDateFormatter_create_md_mv1_result {union {icu4x::capi::NeoDateFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatter_create_md_mv1_result;
    icu4x_NeoDateFormatter_create_md_mv1_result icu4x_NeoDateFormatter_create_md_mv1(const icu4x::capi::Locale* locale, icu4x::capi::DateTimeLength_option length, icu4x::capi::DateTimeAlignment_option alignment);
    
    typedef struct icu4x_NeoDateFormatter_create_md_with_provider_mv1_result {union {icu4x::capi::NeoDateFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatter_create_md_with_provider_mv1_result;
    icu4x_NeoDateFormatter_create_md_with_provider_mv1_result icu4x_NeoDateFormatter_create_md_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::DateTimeLength_option length, icu4x::capi::DateTimeAlignment_option alignment);
    
    typedef struct icu4x_NeoDateFormatter_create_ymd_mv1_result {union {icu4x::capi::NeoDateFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatter_create_ymd_mv1_result;
    icu4x_NeoDateFormatter_create_ymd_mv1_result icu4x_NeoDateFormatter_create_ymd_mv1(const icu4x::capi::Locale* locale, icu4x::capi::DateTimeLength_option length, icu4x::capi::DateTimeAlignment_option alignment, icu4x::capi::YearStyle_option year_style);
    
    typedef struct icu4x_NeoDateFormatter_create_ymd_with_provider_mv1_result {union {icu4x::capi::NeoDateFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatter_create_ymd_with_provider_mv1_result;
    icu4x_NeoDateFormatter_create_ymd_with_provider_mv1_result icu4x_NeoDateFormatter_create_ymd_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::DateTimeLength_option length, icu4x::capi::DateTimeAlignment_option alignment, icu4x::capi::YearStyle_option year_style);
    
    typedef struct icu4x_NeoDateFormatter_create_de_mv1_result {union {icu4x::capi::NeoDateFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatter_create_de_mv1_result;
    icu4x_NeoDateFormatter_create_de_mv1_result icu4x_NeoDateFormatter_create_de_mv1(const icu4x::capi::Locale* locale, icu4x::capi::DateTimeLength_option length, icu4x::capi::DateTimeAlignment_option alignment);
    
    typedef struct icu4x_NeoDateFormatter_create_de_with_provider_mv1_result {union {icu4x::capi::NeoDateFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatter_create_de_with_provider_mv1_result;
    icu4x_NeoDateFormatter_create_de_with_provider_mv1_result icu4x_NeoDateFormatter_create_de_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::DateTimeLength_option length, icu4x::capi::DateTimeAlignment_option alignment);
    
    typedef struct icu4x_NeoDateFormatter_create_mde_mv1_result {union {icu4x::capi::NeoDateFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatter_create_mde_mv1_result;
    icu4x_NeoDateFormatter_create_mde_mv1_result icu4x_NeoDateFormatter_create_mde_mv1(const icu4x::capi::Locale* locale, icu4x::capi::DateTimeLength_option length, icu4x::capi::DateTimeAlignment_option alignment);
    
    typedef struct icu4x_NeoDateFormatter_create_mde_with_provider_mv1_result {union {icu4x::capi::NeoDateFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatter_create_mde_with_provider_mv1_result;
    icu4x_NeoDateFormatter_create_mde_with_provider_mv1_result icu4x_NeoDateFormatter_create_mde_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::DateTimeLength_option length, icu4x::capi::DateTimeAlignment_option alignment);
    
    typedef struct icu4x_NeoDateFormatter_create_ymde_mv1_result {union {icu4x::capi::NeoDateFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatter_create_ymde_mv1_result;
    icu4x_NeoDateFormatter_create_ymde_mv1_result icu4x_NeoDateFormatter_create_ymde_mv1(const icu4x::capi::Locale* locale, icu4x::capi::DateTimeLength_option length, icu4x::capi::DateTimeAlignment_option alignment, icu4x::capi::YearStyle_option year_style);
    
    typedef struct icu4x_NeoDateFormatter_create_ymde_with_provider_mv1_result {union {icu4x::capi::NeoDateFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatter_create_ymde_with_provider_mv1_result;
    icu4x_NeoDateFormatter_create_ymde_with_provider_mv1_result icu4x_NeoDateFormatter_create_ymde_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::DateTimeLength_option length, icu4x::capi::DateTimeAlignment_option alignment, icu4x::capi::YearStyle_option year_style);
    
    typedef struct icu4x_NeoDateFormatter_create_e_mv1_result {union {icu4x::capi::NeoDateFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatter_create_e_mv1_result;
    icu4x_NeoDateFormatter_create_e_mv1_result icu4x_NeoDateFormatter_create_e_mv1(const icu4x::capi::Locale* locale, icu4x::capi::DateTimeLength_option length);
    
    typedef struct icu4x_NeoDateFormatter_create_e_with_provider_mv1_result {union {icu4x::capi::NeoDateFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatter_create_e_with_provider_mv1_result;
    icu4x_NeoDateFormatter_create_e_with_provider_mv1_result icu4x_NeoDateFormatter_create_e_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::DateTimeLength_option length);
    
    typedef struct icu4x_NeoDateFormatter_create_m_mv1_result {union {icu4x::capi::NeoDateFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatter_create_m_mv1_result;
    icu4x_NeoDateFormatter_create_m_mv1_result icu4x_NeoDateFormatter_create_m_mv1(const icu4x::capi::Locale* locale, icu4x::capi::DateTimeLength_option length, icu4x::capi::DateTimeAlignment_option alignment);
    
    typedef struct icu4x_NeoDateFormatter_create_m_with_provider_mv1_result {union {icu4x::capi::NeoDateFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatter_create_m_with_provider_mv1_result;
    icu4x_NeoDateFormatter_create_m_with_provider_mv1_result icu4x_NeoDateFormatter_create_m_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::DateTimeLength_option length, icu4x::capi::DateTimeAlignment_option alignment);
    
    typedef struct icu4x_NeoDateFormatter_create_ym_mv1_result {union {icu4x::capi::NeoDateFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatter_create_ym_mv1_result;
    icu4x_NeoDateFormatter_create_ym_mv1_result icu4x_NeoDateFormatter_create_ym_mv1(const icu4x::capi::Locale* locale, icu4x::capi::DateTimeLength_option length, icu4x::capi::DateTimeAlignment_option alignment, icu4x::capi::YearStyle_option year_style);
    
    typedef struct icu4x_NeoDateFormatter_create_ym_with_provider_mv1_result {union {icu4x::capi::NeoDateFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatter_create_ym_with_provider_mv1_result;
    icu4x_NeoDateFormatter_create_ym_with_provider_mv1_result icu4x_NeoDateFormatter_create_ym_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::DateTimeLength_option length, icu4x::capi::DateTimeAlignment_option alignment, icu4x::capi::YearStyle_option year_style);
    
    typedef struct icu4x_NeoDateFormatter_create_y_mv1_result {union {icu4x::capi::NeoDateFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatter_create_y_mv1_result;
    icu4x_NeoDateFormatter_create_y_mv1_result icu4x_NeoDateFormatter_create_y_mv1(const icu4x::capi::Locale* locale, icu4x::capi::DateTimeLength_option length, icu4x::capi::DateTimeAlignment_option alignment, icu4x::capi::YearStyle_option year_style);
    
    typedef struct icu4x_NeoDateFormatter_create_y_with_provider_mv1_result {union {icu4x::capi::NeoDateFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatter_create_y_with_provider_mv1_result;
    icu4x_NeoDateFormatter_create_y_with_provider_mv1_result icu4x_NeoDateFormatter_create_y_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::DateTimeLength_option length, icu4x::capi::DateTimeAlignment_option alignment, icu4x::capi::YearStyle_option year_style);
    
    void icu4x_NeoDateFormatter_format_iso_mv1(const icu4x::capi::NeoDateFormatter* self, const icu4x::capi::IsoDate* date, diplomat::capi::DiplomatWrite* write);
    
    typedef struct icu4x_NeoDateFormatter_format_same_calendar_mv1_result {union { icu4x::capi::DateTimeMismatchedCalendarError err;}; bool is_ok;} icu4x_NeoDateFormatter_format_same_calendar_mv1_result;
    icu4x_NeoDateFormatter_format_same_calendar_mv1_result icu4x_NeoDateFormatter_format_same_calendar_mv1(const icu4x::capi::NeoDateFormatter* self, const icu4x::capi::Date* date, diplomat::capi::DiplomatWrite* write);
    
    
    void icu4x_NeoDateFormatter_destroy_mv1(NeoDateFormatter* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateFormatter::create_d(const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment) {
  auto result = icu4x::capi::icu4x_NeoDateFormatter_create_d_mv1(locale.AsFFI(),
    length.has_value() ? (icu4x::capi::DateTimeLength_option{ { length.value().AsFFI() }, true }) : (icu4x::capi::DateTimeLength_option{ {}, false }),
    alignment.has_value() ? (icu4x::capi::DateTimeAlignment_option{ { alignment.value().AsFFI() }, true }) : (icu4x::capi::DateTimeAlignment_option{ {}, false }));
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateFormatter>>(std::unique_ptr<icu4x::NeoDateFormatter>(icu4x::NeoDateFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateFormatter::create_d_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment) {
  auto result = icu4x::capi::icu4x_NeoDateFormatter_create_d_with_provider_mv1(provider.AsFFI(),
    locale.AsFFI(),
    length.has_value() ? (icu4x::capi::DateTimeLength_option{ { length.value().AsFFI() }, true }) : (icu4x::capi::DateTimeLength_option{ {}, false }),
    alignment.has_value() ? (icu4x::capi::DateTimeAlignment_option{ { alignment.value().AsFFI() }, true }) : (icu4x::capi::DateTimeAlignment_option{ {}, false }));
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateFormatter>>(std::unique_ptr<icu4x::NeoDateFormatter>(icu4x::NeoDateFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateFormatter::create_md(const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment) {
  auto result = icu4x::capi::icu4x_NeoDateFormatter_create_md_mv1(locale.AsFFI(),
    length.has_value() ? (icu4x::capi::DateTimeLength_option{ { length.value().AsFFI() }, true }) : (icu4x::capi::DateTimeLength_option{ {}, false }),
    alignment.has_value() ? (icu4x::capi::DateTimeAlignment_option{ { alignment.value().AsFFI() }, true }) : (icu4x::capi::DateTimeAlignment_option{ {}, false }));
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateFormatter>>(std::unique_ptr<icu4x::NeoDateFormatter>(icu4x::NeoDateFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateFormatter::create_md_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment) {
  auto result = icu4x::capi::icu4x_NeoDateFormatter_create_md_with_provider_mv1(provider.AsFFI(),
    locale.AsFFI(),
    length.has_value() ? (icu4x::capi::DateTimeLength_option{ { length.value().AsFFI() }, true }) : (icu4x::capi::DateTimeLength_option{ {}, false }),
    alignment.has_value() ? (icu4x::capi::DateTimeAlignment_option{ { alignment.value().AsFFI() }, true }) : (icu4x::capi::DateTimeAlignment_option{ {}, false }));
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateFormatter>>(std::unique_ptr<icu4x::NeoDateFormatter>(icu4x::NeoDateFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateFormatter::create_ymd(const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment, std::optional<icu4x::YearStyle> year_style) {
  auto result = icu4x::capi::icu4x_NeoDateFormatter_create_ymd_mv1(locale.AsFFI(),
    length.has_value() ? (icu4x::capi::DateTimeLength_option{ { length.value().AsFFI() }, true }) : (icu4x::capi::DateTimeLength_option{ {}, false }),
    alignment.has_value() ? (icu4x::capi::DateTimeAlignment_option{ { alignment.value().AsFFI() }, true }) : (icu4x::capi::DateTimeAlignment_option{ {}, false }),
    year_style.has_value() ? (icu4x::capi::YearStyle_option{ { year_style.value().AsFFI() }, true }) : (icu4x::capi::YearStyle_option{ {}, false }));
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateFormatter>>(std::unique_ptr<icu4x::NeoDateFormatter>(icu4x::NeoDateFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateFormatter::create_ymd_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment, std::optional<icu4x::YearStyle> year_style) {
  auto result = icu4x::capi::icu4x_NeoDateFormatter_create_ymd_with_provider_mv1(provider.AsFFI(),
    locale.AsFFI(),
    length.has_value() ? (icu4x::capi::DateTimeLength_option{ { length.value().AsFFI() }, true }) : (icu4x::capi::DateTimeLength_option{ {}, false }),
    alignment.has_value() ? (icu4x::capi::DateTimeAlignment_option{ { alignment.value().AsFFI() }, true }) : (icu4x::capi::DateTimeAlignment_option{ {}, false }),
    year_style.has_value() ? (icu4x::capi::YearStyle_option{ { year_style.value().AsFFI() }, true }) : (icu4x::capi::YearStyle_option{ {}, false }));
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateFormatter>>(std::unique_ptr<icu4x::NeoDateFormatter>(icu4x::NeoDateFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateFormatter::create_de(const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment) {
  auto result = icu4x::capi::icu4x_NeoDateFormatter_create_de_mv1(locale.AsFFI(),
    length.has_value() ? (icu4x::capi::DateTimeLength_option{ { length.value().AsFFI() }, true }) : (icu4x::capi::DateTimeLength_option{ {}, false }),
    alignment.has_value() ? (icu4x::capi::DateTimeAlignment_option{ { alignment.value().AsFFI() }, true }) : (icu4x::capi::DateTimeAlignment_option{ {}, false }));
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateFormatter>>(std::unique_ptr<icu4x::NeoDateFormatter>(icu4x::NeoDateFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateFormatter::create_de_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment) {
  auto result = icu4x::capi::icu4x_NeoDateFormatter_create_de_with_provider_mv1(provider.AsFFI(),
    locale.AsFFI(),
    length.has_value() ? (icu4x::capi::DateTimeLength_option{ { length.value().AsFFI() }, true }) : (icu4x::capi::DateTimeLength_option{ {}, false }),
    alignment.has_value() ? (icu4x::capi::DateTimeAlignment_option{ { alignment.value().AsFFI() }, true }) : (icu4x::capi::DateTimeAlignment_option{ {}, false }));
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateFormatter>>(std::unique_ptr<icu4x::NeoDateFormatter>(icu4x::NeoDateFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateFormatter::create_mde(const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment) {
  auto result = icu4x::capi::icu4x_NeoDateFormatter_create_mde_mv1(locale.AsFFI(),
    length.has_value() ? (icu4x::capi::DateTimeLength_option{ { length.value().AsFFI() }, true }) : (icu4x::capi::DateTimeLength_option{ {}, false }),
    alignment.has_value() ? (icu4x::capi::DateTimeAlignment_option{ { alignment.value().AsFFI() }, true }) : (icu4x::capi::DateTimeAlignment_option{ {}, false }));
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateFormatter>>(std::unique_ptr<icu4x::NeoDateFormatter>(icu4x::NeoDateFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateFormatter::create_mde_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment) {
  auto result = icu4x::capi::icu4x_NeoDateFormatter_create_mde_with_provider_mv1(provider.AsFFI(),
    locale.AsFFI(),
    length.has_value() ? (icu4x::capi::DateTimeLength_option{ { length.value().AsFFI() }, true }) : (icu4x::capi::DateTimeLength_option{ {}, false }),
    alignment.has_value() ? (icu4x::capi::DateTimeAlignment_option{ { alignment.value().AsFFI() }, true }) : (icu4x::capi::DateTimeAlignment_option{ {}, false }));
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateFormatter>>(std::unique_ptr<icu4x::NeoDateFormatter>(icu4x::NeoDateFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateFormatter::create_ymde(const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment, std::optional<icu4x::YearStyle> year_style) {
  auto result = icu4x::capi::icu4x_NeoDateFormatter_create_ymde_mv1(locale.AsFFI(),
    length.has_value() ? (icu4x::capi::DateTimeLength_option{ { length.value().AsFFI() }, true }) : (icu4x::capi::DateTimeLength_option{ {}, false }),
    alignment.has_value() ? (icu4x::capi::DateTimeAlignment_option{ { alignment.value().AsFFI() }, true }) : (icu4x::capi::DateTimeAlignment_option{ {}, false }),
    year_style.has_value() ? (icu4x::capi::YearStyle_option{ { year_style.value().AsFFI() }, true }) : (icu4x::capi::YearStyle_option{ {}, false }));
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateFormatter>>(std::unique_ptr<icu4x::NeoDateFormatter>(icu4x::NeoDateFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateFormatter::create_ymde_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment, std::optional<icu4x::YearStyle> year_style) {
  auto result = icu4x::capi::icu4x_NeoDateFormatter_create_ymde_with_provider_mv1(provider.AsFFI(),
    locale.AsFFI(),
    length.has_value() ? (icu4x::capi::DateTimeLength_option{ { length.value().AsFFI() }, true }) : (icu4x::capi::DateTimeLength_option{ {}, false }),
    alignment.has_value() ? (icu4x::capi::DateTimeAlignment_option{ { alignment.value().AsFFI() }, true }) : (icu4x::capi::DateTimeAlignment_option{ {}, false }),
    year_style.has_value() ? (icu4x::capi::YearStyle_option{ { year_style.value().AsFFI() }, true }) : (icu4x::capi::YearStyle_option{ {}, false }));
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateFormatter>>(std::unique_ptr<icu4x::NeoDateFormatter>(icu4x::NeoDateFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateFormatter::create_e(const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length) {
  auto result = icu4x::capi::icu4x_NeoDateFormatter_create_e_mv1(locale.AsFFI(),
    length.has_value() ? (icu4x::capi::DateTimeLength_option{ { length.value().AsFFI() }, true }) : (icu4x::capi::DateTimeLength_option{ {}, false }));
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateFormatter>>(std::unique_ptr<icu4x::NeoDateFormatter>(icu4x::NeoDateFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateFormatter::create_e_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length) {
  auto result = icu4x::capi::icu4x_NeoDateFormatter_create_e_with_provider_mv1(provider.AsFFI(),
    locale.AsFFI(),
    length.has_value() ? (icu4x::capi::DateTimeLength_option{ { length.value().AsFFI() }, true }) : (icu4x::capi::DateTimeLength_option{ {}, false }));
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateFormatter>>(std::unique_ptr<icu4x::NeoDateFormatter>(icu4x::NeoDateFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateFormatter::create_m(const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment) {
  auto result = icu4x::capi::icu4x_NeoDateFormatter_create_m_mv1(locale.AsFFI(),
    length.has_value() ? (icu4x::capi::DateTimeLength_option{ { length.value().AsFFI() }, true }) : (icu4x::capi::DateTimeLength_option{ {}, false }),
    alignment.has_value() ? (icu4x::capi::DateTimeAlignment_option{ { alignment.value().AsFFI() }, true }) : (icu4x::capi::DateTimeAlignment_option{ {}, false }));
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateFormatter>>(std::unique_ptr<icu4x::NeoDateFormatter>(icu4x::NeoDateFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateFormatter::create_m_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment) {
  auto result = icu4x::capi::icu4x_NeoDateFormatter_create_m_with_provider_mv1(provider.AsFFI(),
    locale.AsFFI(),
    length.has_value() ? (icu4x::capi::DateTimeLength_option{ { length.value().AsFFI() }, true }) : (icu4x::capi::DateTimeLength_option{ {}, false }),
    alignment.has_value() ? (icu4x::capi::DateTimeAlignment_option{ { alignment.value().AsFFI() }, true }) : (icu4x::capi::DateTimeAlignment_option{ {}, false }));
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateFormatter>>(std::unique_ptr<icu4x::NeoDateFormatter>(icu4x::NeoDateFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateFormatter::create_ym(const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment, std::optional<icu4x::YearStyle> year_style) {
  auto result = icu4x::capi::icu4x_NeoDateFormatter_create_ym_mv1(locale.AsFFI(),
    length.has_value() ? (icu4x::capi::DateTimeLength_option{ { length.value().AsFFI() }, true }) : (icu4x::capi::DateTimeLength_option{ {}, false }),
    alignment.has_value() ? (icu4x::capi::DateTimeAlignment_option{ { alignment.value().AsFFI() }, true }) : (icu4x::capi::DateTimeAlignment_option{ {}, false }),
    year_style.has_value() ? (icu4x::capi::YearStyle_option{ { year_style.value().AsFFI() }, true }) : (icu4x::capi::YearStyle_option{ {}, false }));
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateFormatter>>(std::unique_ptr<icu4x::NeoDateFormatter>(icu4x::NeoDateFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateFormatter::create_ym_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment, std::optional<icu4x::YearStyle> year_style) {
  auto result = icu4x::capi::icu4x_NeoDateFormatter_create_ym_with_provider_mv1(provider.AsFFI(),
    locale.AsFFI(),
    length.has_value() ? (icu4x::capi::DateTimeLength_option{ { length.value().AsFFI() }, true }) : (icu4x::capi::DateTimeLength_option{ {}, false }),
    alignment.has_value() ? (icu4x::capi::DateTimeAlignment_option{ { alignment.value().AsFFI() }, true }) : (icu4x::capi::DateTimeAlignment_option{ {}, false }),
    year_style.has_value() ? (icu4x::capi::YearStyle_option{ { year_style.value().AsFFI() }, true }) : (icu4x::capi::YearStyle_option{ {}, false }));
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateFormatter>>(std::unique_ptr<icu4x::NeoDateFormatter>(icu4x::NeoDateFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateFormatter::create_y(const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment, std::optional<icu4x::YearStyle> year_style) {
  auto result = icu4x::capi::icu4x_NeoDateFormatter_create_y_mv1(locale.AsFFI(),
    length.has_value() ? (icu4x::capi::DateTimeLength_option{ { length.value().AsFFI() }, true }) : (icu4x::capi::DateTimeLength_option{ {}, false }),
    alignment.has_value() ? (icu4x::capi::DateTimeAlignment_option{ { alignment.value().AsFFI() }, true }) : (icu4x::capi::DateTimeAlignment_option{ {}, false }),
    year_style.has_value() ? (icu4x::capi::YearStyle_option{ { year_style.value().AsFFI() }, true }) : (icu4x::capi::YearStyle_option{ {}, false }));
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateFormatter>>(std::unique_ptr<icu4x::NeoDateFormatter>(icu4x::NeoDateFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateFormatter::create_y_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment, std::optional<icu4x::YearStyle> year_style) {
  auto result = icu4x::capi::icu4x_NeoDateFormatter_create_y_with_provider_mv1(provider.AsFFI(),
    locale.AsFFI(),
    length.has_value() ? (icu4x::capi::DateTimeLength_option{ { length.value().AsFFI() }, true }) : (icu4x::capi::DateTimeLength_option{ {}, false }),
    alignment.has_value() ? (icu4x::capi::DateTimeAlignment_option{ { alignment.value().AsFFI() }, true }) : (icu4x::capi::DateTimeAlignment_option{ {}, false }),
    year_style.has_value() ? (icu4x::capi::YearStyle_option{ { year_style.value().AsFFI() }, true }) : (icu4x::capi::YearStyle_option{ {}, false }));
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateFormatter>>(std::unique_ptr<icu4x::NeoDateFormatter>(icu4x::NeoDateFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline std::string icu4x::NeoDateFormatter::format_iso(const icu4x::IsoDate& date) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  icu4x::capi::icu4x_NeoDateFormatter_format_iso_mv1(this->AsFFI(),
    date.AsFFI(),
    &write);
  return output;
}

inline diplomat::result<std::string, icu4x::DateTimeMismatchedCalendarError> icu4x::NeoDateFormatter::format_same_calendar(const icu4x::Date& date) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = icu4x::capi::icu4x_NeoDateFormatter_format_same_calendar_mv1(this->AsFFI(),
    date.AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, icu4x::DateTimeMismatchedCalendarError>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, icu4x::DateTimeMismatchedCalendarError>(diplomat::Err<icu4x::DateTimeMismatchedCalendarError>(icu4x::DateTimeMismatchedCalendarError::FromFFI(result.err)));
}

inline const icu4x::capi::NeoDateFormatter* icu4x::NeoDateFormatter::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::NeoDateFormatter*>(this);
}

inline icu4x::capi::NeoDateFormatter* icu4x::NeoDateFormatter::AsFFI() {
  return reinterpret_cast<icu4x::capi::NeoDateFormatter*>(this);
}

inline const icu4x::NeoDateFormatter* icu4x::NeoDateFormatter::FromFFI(const icu4x::capi::NeoDateFormatter* ptr) {
  return reinterpret_cast<const icu4x::NeoDateFormatter*>(ptr);
}

inline icu4x::NeoDateFormatter* icu4x::NeoDateFormatter::FromFFI(icu4x::capi::NeoDateFormatter* ptr) {
  return reinterpret_cast<icu4x::NeoDateFormatter*>(ptr);
}

inline void icu4x::NeoDateFormatter::operator delete(void* ptr) {
  icu4x::capi::icu4x_NeoDateFormatter_destroy_mv1(reinterpret_cast<icu4x::capi::NeoDateFormatter*>(ptr));
}


#endif // icu4x_NeoDateFormatter_HPP
