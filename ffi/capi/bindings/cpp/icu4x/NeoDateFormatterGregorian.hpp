#ifndef icu4x_NeoDateFormatterGregorian_HPP
#define icu4x_NeoDateFormatterGregorian_HPP

#include "NeoDateFormatterGregorian.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../diplomat_runtime.hpp"
#include "DataProvider.hpp"
#include "DateTimeAlignment.hpp"
#include "DateTimeFormatterLoadError.hpp"
#include "DateTimeLength.hpp"
#include "IsoDate.hpp"
#include "Locale.hpp"
#include "YearStyle.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_NeoDateFormatterGregorian_create_d_mv1_result {union {icu4x::capi::NeoDateFormatterGregorian* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatterGregorian_create_d_mv1_result;
    icu4x_NeoDateFormatterGregorian_create_d_mv1_result icu4x_NeoDateFormatterGregorian_create_d_mv1(const icu4x::capi::Locale* locale, icu4x::capi::DateTimeLength_option length, icu4x::capi::DateTimeAlignment_option alignment);
    
    typedef struct icu4x_NeoDateFormatterGregorian_create_d_with_provider_mv1_result {union {icu4x::capi::NeoDateFormatterGregorian* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatterGregorian_create_d_with_provider_mv1_result;
    icu4x_NeoDateFormatterGregorian_create_d_with_provider_mv1_result icu4x_NeoDateFormatterGregorian_create_d_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::DateTimeLength_option length, icu4x::capi::DateTimeAlignment_option alignment);
    
    typedef struct icu4x_NeoDateFormatterGregorian_create_md_mv1_result {union {icu4x::capi::NeoDateFormatterGregorian* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatterGregorian_create_md_mv1_result;
    icu4x_NeoDateFormatterGregorian_create_md_mv1_result icu4x_NeoDateFormatterGregorian_create_md_mv1(const icu4x::capi::Locale* locale, icu4x::capi::DateTimeLength_option length, icu4x::capi::DateTimeAlignment_option alignment);
    
    typedef struct icu4x_NeoDateFormatterGregorian_create_md_with_provider_mv1_result {union {icu4x::capi::NeoDateFormatterGregorian* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatterGregorian_create_md_with_provider_mv1_result;
    icu4x_NeoDateFormatterGregorian_create_md_with_provider_mv1_result icu4x_NeoDateFormatterGregorian_create_md_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::DateTimeLength_option length, icu4x::capi::DateTimeAlignment_option alignment);
    
    typedef struct icu4x_NeoDateFormatterGregorian_create_ymd_mv1_result {union {icu4x::capi::NeoDateFormatterGregorian* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatterGregorian_create_ymd_mv1_result;
    icu4x_NeoDateFormatterGregorian_create_ymd_mv1_result icu4x_NeoDateFormatterGregorian_create_ymd_mv1(const icu4x::capi::Locale* locale, icu4x::capi::DateTimeLength_option length, icu4x::capi::DateTimeAlignment_option alignment, icu4x::capi::YearStyle_option year_style);
    
    typedef struct icu4x_NeoDateFormatterGregorian_create_ymd_with_provider_mv1_result {union {icu4x::capi::NeoDateFormatterGregorian* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatterGregorian_create_ymd_with_provider_mv1_result;
    icu4x_NeoDateFormatterGregorian_create_ymd_with_provider_mv1_result icu4x_NeoDateFormatterGregorian_create_ymd_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::DateTimeLength_option length, icu4x::capi::DateTimeAlignment_option alignment, icu4x::capi::YearStyle_option year_style);
    
    typedef struct icu4x_NeoDateFormatterGregorian_create_de_mv1_result {union {icu4x::capi::NeoDateFormatterGregorian* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatterGregorian_create_de_mv1_result;
    icu4x_NeoDateFormatterGregorian_create_de_mv1_result icu4x_NeoDateFormatterGregorian_create_de_mv1(const icu4x::capi::Locale* locale, icu4x::capi::DateTimeLength_option length, icu4x::capi::DateTimeAlignment_option alignment);
    
    typedef struct icu4x_NeoDateFormatterGregorian_create_de_with_provider_mv1_result {union {icu4x::capi::NeoDateFormatterGregorian* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatterGregorian_create_de_with_provider_mv1_result;
    icu4x_NeoDateFormatterGregorian_create_de_with_provider_mv1_result icu4x_NeoDateFormatterGregorian_create_de_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::DateTimeLength_option length, icu4x::capi::DateTimeAlignment_option alignment);
    
    typedef struct icu4x_NeoDateFormatterGregorian_create_mde_mv1_result {union {icu4x::capi::NeoDateFormatterGregorian* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatterGregorian_create_mde_mv1_result;
    icu4x_NeoDateFormatterGregorian_create_mde_mv1_result icu4x_NeoDateFormatterGregorian_create_mde_mv1(const icu4x::capi::Locale* locale, icu4x::capi::DateTimeLength_option length, icu4x::capi::DateTimeAlignment_option alignment);
    
    typedef struct icu4x_NeoDateFormatterGregorian_create_mde_with_provider_mv1_result {union {icu4x::capi::NeoDateFormatterGregorian* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatterGregorian_create_mde_with_provider_mv1_result;
    icu4x_NeoDateFormatterGregorian_create_mde_with_provider_mv1_result icu4x_NeoDateFormatterGregorian_create_mde_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::DateTimeLength_option length, icu4x::capi::DateTimeAlignment_option alignment);
    
    typedef struct icu4x_NeoDateFormatterGregorian_create_ymde_mv1_result {union {icu4x::capi::NeoDateFormatterGregorian* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatterGregorian_create_ymde_mv1_result;
    icu4x_NeoDateFormatterGregorian_create_ymde_mv1_result icu4x_NeoDateFormatterGregorian_create_ymde_mv1(const icu4x::capi::Locale* locale, icu4x::capi::DateTimeLength_option length, icu4x::capi::DateTimeAlignment_option alignment, icu4x::capi::YearStyle_option year_style);
    
    typedef struct icu4x_NeoDateFormatterGregorian_create_ymde_with_provider_mv1_result {union {icu4x::capi::NeoDateFormatterGregorian* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatterGregorian_create_ymde_with_provider_mv1_result;
    icu4x_NeoDateFormatterGregorian_create_ymde_with_provider_mv1_result icu4x_NeoDateFormatterGregorian_create_ymde_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::DateTimeLength_option length, icu4x::capi::DateTimeAlignment_option alignment, icu4x::capi::YearStyle_option year_style);
    
    typedef struct icu4x_NeoDateFormatterGregorian_create_e_mv1_result {union {icu4x::capi::NeoDateFormatterGregorian* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatterGregorian_create_e_mv1_result;
    icu4x_NeoDateFormatterGregorian_create_e_mv1_result icu4x_NeoDateFormatterGregorian_create_e_mv1(const icu4x::capi::Locale* locale, icu4x::capi::DateTimeLength_option length);
    
    typedef struct icu4x_NeoDateFormatterGregorian_create_e_with_provider_mv1_result {union {icu4x::capi::NeoDateFormatterGregorian* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatterGregorian_create_e_with_provider_mv1_result;
    icu4x_NeoDateFormatterGregorian_create_e_with_provider_mv1_result icu4x_NeoDateFormatterGregorian_create_e_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::DateTimeLength_option length);
    
    typedef struct icu4x_NeoDateFormatterGregorian_create_m_mv1_result {union {icu4x::capi::NeoDateFormatterGregorian* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatterGregorian_create_m_mv1_result;
    icu4x_NeoDateFormatterGregorian_create_m_mv1_result icu4x_NeoDateFormatterGregorian_create_m_mv1(const icu4x::capi::Locale* locale, icu4x::capi::DateTimeLength_option length, icu4x::capi::DateTimeAlignment_option alignment);
    
    typedef struct icu4x_NeoDateFormatterGregorian_create_m_with_provider_mv1_result {union {icu4x::capi::NeoDateFormatterGregorian* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatterGregorian_create_m_with_provider_mv1_result;
    icu4x_NeoDateFormatterGregorian_create_m_with_provider_mv1_result icu4x_NeoDateFormatterGregorian_create_m_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::DateTimeLength_option length, icu4x::capi::DateTimeAlignment_option alignment);
    
    typedef struct icu4x_NeoDateFormatterGregorian_create_ym_mv1_result {union {icu4x::capi::NeoDateFormatterGregorian* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatterGregorian_create_ym_mv1_result;
    icu4x_NeoDateFormatterGregorian_create_ym_mv1_result icu4x_NeoDateFormatterGregorian_create_ym_mv1(const icu4x::capi::Locale* locale, icu4x::capi::DateTimeLength_option length, icu4x::capi::DateTimeAlignment_option alignment, icu4x::capi::YearStyle_option year_style);
    
    typedef struct icu4x_NeoDateFormatterGregorian_create_ym_with_provider_mv1_result {union {icu4x::capi::NeoDateFormatterGregorian* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatterGregorian_create_ym_with_provider_mv1_result;
    icu4x_NeoDateFormatterGregorian_create_ym_with_provider_mv1_result icu4x_NeoDateFormatterGregorian_create_ym_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::DateTimeLength_option length, icu4x::capi::DateTimeAlignment_option alignment, icu4x::capi::YearStyle_option year_style);
    
    typedef struct icu4x_NeoDateFormatterGregorian_create_y_mv1_result {union {icu4x::capi::NeoDateFormatterGregorian* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatterGregorian_create_y_mv1_result;
    icu4x_NeoDateFormatterGregorian_create_y_mv1_result icu4x_NeoDateFormatterGregorian_create_y_mv1(const icu4x::capi::Locale* locale, icu4x::capi::DateTimeLength_option length, icu4x::capi::DateTimeAlignment_option alignment, icu4x::capi::YearStyle_option year_style);
    
    typedef struct icu4x_NeoDateFormatterGregorian_create_y_with_provider_mv1_result {union {icu4x::capi::NeoDateFormatterGregorian* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateFormatterGregorian_create_y_with_provider_mv1_result;
    icu4x_NeoDateFormatterGregorian_create_y_with_provider_mv1_result icu4x_NeoDateFormatterGregorian_create_y_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::DateTimeLength_option length, icu4x::capi::DateTimeAlignment_option alignment, icu4x::capi::YearStyle_option year_style);
    
    void icu4x_NeoDateFormatterGregorian_format_iso_mv1(const icu4x::capi::NeoDateFormatterGregorian* self, const icu4x::capi::IsoDate* date, diplomat::capi::DiplomatWrite* write);
    
    
    void icu4x_NeoDateFormatterGregorian_destroy_mv1(NeoDateFormatterGregorian* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateFormatterGregorian::create_d(const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment) {
  auto result = icu4x::capi::icu4x_NeoDateFormatterGregorian_create_d_mv1(locale.AsFFI(),
    length.has_value() ? (icu4x::capi::DateTimeLength_option{ { length.value().AsFFI() }, true }) : (icu4x::capi::DateTimeLength_option{ {}, false }),
    alignment.has_value() ? (icu4x::capi::DateTimeAlignment_option{ { alignment.value().AsFFI() }, true }) : (icu4x::capi::DateTimeAlignment_option{ {}, false }));
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateFormatterGregorian>>(std::unique_ptr<icu4x::NeoDateFormatterGregorian>(icu4x::NeoDateFormatterGregorian::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateFormatterGregorian::create_d_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment) {
  auto result = icu4x::capi::icu4x_NeoDateFormatterGregorian_create_d_with_provider_mv1(provider.AsFFI(),
    locale.AsFFI(),
    length.has_value() ? (icu4x::capi::DateTimeLength_option{ { length.value().AsFFI() }, true }) : (icu4x::capi::DateTimeLength_option{ {}, false }),
    alignment.has_value() ? (icu4x::capi::DateTimeAlignment_option{ { alignment.value().AsFFI() }, true }) : (icu4x::capi::DateTimeAlignment_option{ {}, false }));
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateFormatterGregorian>>(std::unique_ptr<icu4x::NeoDateFormatterGregorian>(icu4x::NeoDateFormatterGregorian::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateFormatterGregorian::create_md(const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment) {
  auto result = icu4x::capi::icu4x_NeoDateFormatterGregorian_create_md_mv1(locale.AsFFI(),
    length.has_value() ? (icu4x::capi::DateTimeLength_option{ { length.value().AsFFI() }, true }) : (icu4x::capi::DateTimeLength_option{ {}, false }),
    alignment.has_value() ? (icu4x::capi::DateTimeAlignment_option{ { alignment.value().AsFFI() }, true }) : (icu4x::capi::DateTimeAlignment_option{ {}, false }));
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateFormatterGregorian>>(std::unique_ptr<icu4x::NeoDateFormatterGregorian>(icu4x::NeoDateFormatterGregorian::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateFormatterGregorian::create_md_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment) {
  auto result = icu4x::capi::icu4x_NeoDateFormatterGregorian_create_md_with_provider_mv1(provider.AsFFI(),
    locale.AsFFI(),
    length.has_value() ? (icu4x::capi::DateTimeLength_option{ { length.value().AsFFI() }, true }) : (icu4x::capi::DateTimeLength_option{ {}, false }),
    alignment.has_value() ? (icu4x::capi::DateTimeAlignment_option{ { alignment.value().AsFFI() }, true }) : (icu4x::capi::DateTimeAlignment_option{ {}, false }));
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateFormatterGregorian>>(std::unique_ptr<icu4x::NeoDateFormatterGregorian>(icu4x::NeoDateFormatterGregorian::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateFormatterGregorian::create_ymd(const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment, std::optional<icu4x::YearStyle> year_style) {
  auto result = icu4x::capi::icu4x_NeoDateFormatterGregorian_create_ymd_mv1(locale.AsFFI(),
    length.has_value() ? (icu4x::capi::DateTimeLength_option{ { length.value().AsFFI() }, true }) : (icu4x::capi::DateTimeLength_option{ {}, false }),
    alignment.has_value() ? (icu4x::capi::DateTimeAlignment_option{ { alignment.value().AsFFI() }, true }) : (icu4x::capi::DateTimeAlignment_option{ {}, false }),
    year_style.has_value() ? (icu4x::capi::YearStyle_option{ { year_style.value().AsFFI() }, true }) : (icu4x::capi::YearStyle_option{ {}, false }));
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateFormatterGregorian>>(std::unique_ptr<icu4x::NeoDateFormatterGregorian>(icu4x::NeoDateFormatterGregorian::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateFormatterGregorian::create_ymd_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment, std::optional<icu4x::YearStyle> year_style) {
  auto result = icu4x::capi::icu4x_NeoDateFormatterGregorian_create_ymd_with_provider_mv1(provider.AsFFI(),
    locale.AsFFI(),
    length.has_value() ? (icu4x::capi::DateTimeLength_option{ { length.value().AsFFI() }, true }) : (icu4x::capi::DateTimeLength_option{ {}, false }),
    alignment.has_value() ? (icu4x::capi::DateTimeAlignment_option{ { alignment.value().AsFFI() }, true }) : (icu4x::capi::DateTimeAlignment_option{ {}, false }),
    year_style.has_value() ? (icu4x::capi::YearStyle_option{ { year_style.value().AsFFI() }, true }) : (icu4x::capi::YearStyle_option{ {}, false }));
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateFormatterGregorian>>(std::unique_ptr<icu4x::NeoDateFormatterGregorian>(icu4x::NeoDateFormatterGregorian::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateFormatterGregorian::create_de(const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment) {
  auto result = icu4x::capi::icu4x_NeoDateFormatterGregorian_create_de_mv1(locale.AsFFI(),
    length.has_value() ? (icu4x::capi::DateTimeLength_option{ { length.value().AsFFI() }, true }) : (icu4x::capi::DateTimeLength_option{ {}, false }),
    alignment.has_value() ? (icu4x::capi::DateTimeAlignment_option{ { alignment.value().AsFFI() }, true }) : (icu4x::capi::DateTimeAlignment_option{ {}, false }));
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateFormatterGregorian>>(std::unique_ptr<icu4x::NeoDateFormatterGregorian>(icu4x::NeoDateFormatterGregorian::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateFormatterGregorian::create_de_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment) {
  auto result = icu4x::capi::icu4x_NeoDateFormatterGregorian_create_de_with_provider_mv1(provider.AsFFI(),
    locale.AsFFI(),
    length.has_value() ? (icu4x::capi::DateTimeLength_option{ { length.value().AsFFI() }, true }) : (icu4x::capi::DateTimeLength_option{ {}, false }),
    alignment.has_value() ? (icu4x::capi::DateTimeAlignment_option{ { alignment.value().AsFFI() }, true }) : (icu4x::capi::DateTimeAlignment_option{ {}, false }));
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateFormatterGregorian>>(std::unique_ptr<icu4x::NeoDateFormatterGregorian>(icu4x::NeoDateFormatterGregorian::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateFormatterGregorian::create_mde(const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment) {
  auto result = icu4x::capi::icu4x_NeoDateFormatterGregorian_create_mde_mv1(locale.AsFFI(),
    length.has_value() ? (icu4x::capi::DateTimeLength_option{ { length.value().AsFFI() }, true }) : (icu4x::capi::DateTimeLength_option{ {}, false }),
    alignment.has_value() ? (icu4x::capi::DateTimeAlignment_option{ { alignment.value().AsFFI() }, true }) : (icu4x::capi::DateTimeAlignment_option{ {}, false }));
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateFormatterGregorian>>(std::unique_ptr<icu4x::NeoDateFormatterGregorian>(icu4x::NeoDateFormatterGregorian::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateFormatterGregorian::create_mde_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment) {
  auto result = icu4x::capi::icu4x_NeoDateFormatterGregorian_create_mde_with_provider_mv1(provider.AsFFI(),
    locale.AsFFI(),
    length.has_value() ? (icu4x::capi::DateTimeLength_option{ { length.value().AsFFI() }, true }) : (icu4x::capi::DateTimeLength_option{ {}, false }),
    alignment.has_value() ? (icu4x::capi::DateTimeAlignment_option{ { alignment.value().AsFFI() }, true }) : (icu4x::capi::DateTimeAlignment_option{ {}, false }));
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateFormatterGregorian>>(std::unique_ptr<icu4x::NeoDateFormatterGregorian>(icu4x::NeoDateFormatterGregorian::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateFormatterGregorian::create_ymde(const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment, std::optional<icu4x::YearStyle> year_style) {
  auto result = icu4x::capi::icu4x_NeoDateFormatterGregorian_create_ymde_mv1(locale.AsFFI(),
    length.has_value() ? (icu4x::capi::DateTimeLength_option{ { length.value().AsFFI() }, true }) : (icu4x::capi::DateTimeLength_option{ {}, false }),
    alignment.has_value() ? (icu4x::capi::DateTimeAlignment_option{ { alignment.value().AsFFI() }, true }) : (icu4x::capi::DateTimeAlignment_option{ {}, false }),
    year_style.has_value() ? (icu4x::capi::YearStyle_option{ { year_style.value().AsFFI() }, true }) : (icu4x::capi::YearStyle_option{ {}, false }));
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateFormatterGregorian>>(std::unique_ptr<icu4x::NeoDateFormatterGregorian>(icu4x::NeoDateFormatterGregorian::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateFormatterGregorian::create_ymde_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment, std::optional<icu4x::YearStyle> year_style) {
  auto result = icu4x::capi::icu4x_NeoDateFormatterGregorian_create_ymde_with_provider_mv1(provider.AsFFI(),
    locale.AsFFI(),
    length.has_value() ? (icu4x::capi::DateTimeLength_option{ { length.value().AsFFI() }, true }) : (icu4x::capi::DateTimeLength_option{ {}, false }),
    alignment.has_value() ? (icu4x::capi::DateTimeAlignment_option{ { alignment.value().AsFFI() }, true }) : (icu4x::capi::DateTimeAlignment_option{ {}, false }),
    year_style.has_value() ? (icu4x::capi::YearStyle_option{ { year_style.value().AsFFI() }, true }) : (icu4x::capi::YearStyle_option{ {}, false }));
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateFormatterGregorian>>(std::unique_ptr<icu4x::NeoDateFormatterGregorian>(icu4x::NeoDateFormatterGregorian::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateFormatterGregorian::create_e(const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length) {
  auto result = icu4x::capi::icu4x_NeoDateFormatterGregorian_create_e_mv1(locale.AsFFI(),
    length.has_value() ? (icu4x::capi::DateTimeLength_option{ { length.value().AsFFI() }, true }) : (icu4x::capi::DateTimeLength_option{ {}, false }));
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateFormatterGregorian>>(std::unique_ptr<icu4x::NeoDateFormatterGregorian>(icu4x::NeoDateFormatterGregorian::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateFormatterGregorian::create_e_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length) {
  auto result = icu4x::capi::icu4x_NeoDateFormatterGregorian_create_e_with_provider_mv1(provider.AsFFI(),
    locale.AsFFI(),
    length.has_value() ? (icu4x::capi::DateTimeLength_option{ { length.value().AsFFI() }, true }) : (icu4x::capi::DateTimeLength_option{ {}, false }));
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateFormatterGregorian>>(std::unique_ptr<icu4x::NeoDateFormatterGregorian>(icu4x::NeoDateFormatterGregorian::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateFormatterGregorian::create_m(const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment) {
  auto result = icu4x::capi::icu4x_NeoDateFormatterGregorian_create_m_mv1(locale.AsFFI(),
    length.has_value() ? (icu4x::capi::DateTimeLength_option{ { length.value().AsFFI() }, true }) : (icu4x::capi::DateTimeLength_option{ {}, false }),
    alignment.has_value() ? (icu4x::capi::DateTimeAlignment_option{ { alignment.value().AsFFI() }, true }) : (icu4x::capi::DateTimeAlignment_option{ {}, false }));
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateFormatterGregorian>>(std::unique_ptr<icu4x::NeoDateFormatterGregorian>(icu4x::NeoDateFormatterGregorian::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateFormatterGregorian::create_m_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment) {
  auto result = icu4x::capi::icu4x_NeoDateFormatterGregorian_create_m_with_provider_mv1(provider.AsFFI(),
    locale.AsFFI(),
    length.has_value() ? (icu4x::capi::DateTimeLength_option{ { length.value().AsFFI() }, true }) : (icu4x::capi::DateTimeLength_option{ {}, false }),
    alignment.has_value() ? (icu4x::capi::DateTimeAlignment_option{ { alignment.value().AsFFI() }, true }) : (icu4x::capi::DateTimeAlignment_option{ {}, false }));
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateFormatterGregorian>>(std::unique_ptr<icu4x::NeoDateFormatterGregorian>(icu4x::NeoDateFormatterGregorian::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateFormatterGregorian::create_ym(const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment, std::optional<icu4x::YearStyle> year_style) {
  auto result = icu4x::capi::icu4x_NeoDateFormatterGregorian_create_ym_mv1(locale.AsFFI(),
    length.has_value() ? (icu4x::capi::DateTimeLength_option{ { length.value().AsFFI() }, true }) : (icu4x::capi::DateTimeLength_option{ {}, false }),
    alignment.has_value() ? (icu4x::capi::DateTimeAlignment_option{ { alignment.value().AsFFI() }, true }) : (icu4x::capi::DateTimeAlignment_option{ {}, false }),
    year_style.has_value() ? (icu4x::capi::YearStyle_option{ { year_style.value().AsFFI() }, true }) : (icu4x::capi::YearStyle_option{ {}, false }));
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateFormatterGregorian>>(std::unique_ptr<icu4x::NeoDateFormatterGregorian>(icu4x::NeoDateFormatterGregorian::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateFormatterGregorian::create_ym_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment, std::optional<icu4x::YearStyle> year_style) {
  auto result = icu4x::capi::icu4x_NeoDateFormatterGregorian_create_ym_with_provider_mv1(provider.AsFFI(),
    locale.AsFFI(),
    length.has_value() ? (icu4x::capi::DateTimeLength_option{ { length.value().AsFFI() }, true }) : (icu4x::capi::DateTimeLength_option{ {}, false }),
    alignment.has_value() ? (icu4x::capi::DateTimeAlignment_option{ { alignment.value().AsFFI() }, true }) : (icu4x::capi::DateTimeAlignment_option{ {}, false }),
    year_style.has_value() ? (icu4x::capi::YearStyle_option{ { year_style.value().AsFFI() }, true }) : (icu4x::capi::YearStyle_option{ {}, false }));
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateFormatterGregorian>>(std::unique_ptr<icu4x::NeoDateFormatterGregorian>(icu4x::NeoDateFormatterGregorian::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateFormatterGregorian::create_y(const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment, std::optional<icu4x::YearStyle> year_style) {
  auto result = icu4x::capi::icu4x_NeoDateFormatterGregorian_create_y_mv1(locale.AsFFI(),
    length.has_value() ? (icu4x::capi::DateTimeLength_option{ { length.value().AsFFI() }, true }) : (icu4x::capi::DateTimeLength_option{ {}, false }),
    alignment.has_value() ? (icu4x::capi::DateTimeAlignment_option{ { alignment.value().AsFFI() }, true }) : (icu4x::capi::DateTimeAlignment_option{ {}, false }),
    year_style.has_value() ? (icu4x::capi::YearStyle_option{ { year_style.value().AsFFI() }, true }) : (icu4x::capi::YearStyle_option{ {}, false }));
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateFormatterGregorian>>(std::unique_ptr<icu4x::NeoDateFormatterGregorian>(icu4x::NeoDateFormatterGregorian::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateFormatterGregorian::create_y_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::optional<icu4x::DateTimeLength> length, std::optional<icu4x::DateTimeAlignment> alignment, std::optional<icu4x::YearStyle> year_style) {
  auto result = icu4x::capi::icu4x_NeoDateFormatterGregorian_create_y_with_provider_mv1(provider.AsFFI(),
    locale.AsFFI(),
    length.has_value() ? (icu4x::capi::DateTimeLength_option{ { length.value().AsFFI() }, true }) : (icu4x::capi::DateTimeLength_option{ {}, false }),
    alignment.has_value() ? (icu4x::capi::DateTimeAlignment_option{ { alignment.value().AsFFI() }, true }) : (icu4x::capi::DateTimeAlignment_option{ {}, false }),
    year_style.has_value() ? (icu4x::capi::YearStyle_option{ { year_style.value().AsFFI() }, true }) : (icu4x::capi::YearStyle_option{ {}, false }));
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateFormatterGregorian>>(std::unique_ptr<icu4x::NeoDateFormatterGregorian>(icu4x::NeoDateFormatterGregorian::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateFormatterGregorian>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline std::string icu4x::NeoDateFormatterGregorian::format_iso(const icu4x::IsoDate& date) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  icu4x::capi::icu4x_NeoDateFormatterGregorian_format_iso_mv1(this->AsFFI(),
    date.AsFFI(),
    &write);
  return output;
}

inline const icu4x::capi::NeoDateFormatterGregorian* icu4x::NeoDateFormatterGregorian::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::NeoDateFormatterGregorian*>(this);
}

inline icu4x::capi::NeoDateFormatterGregorian* icu4x::NeoDateFormatterGregorian::AsFFI() {
  return reinterpret_cast<icu4x::capi::NeoDateFormatterGregorian*>(this);
}

inline const icu4x::NeoDateFormatterGregorian* icu4x::NeoDateFormatterGregorian::FromFFI(const icu4x::capi::NeoDateFormatterGregorian* ptr) {
  return reinterpret_cast<const icu4x::NeoDateFormatterGregorian*>(ptr);
}

inline icu4x::NeoDateFormatterGregorian* icu4x::NeoDateFormatterGregorian::FromFFI(icu4x::capi::NeoDateFormatterGregorian* ptr) {
  return reinterpret_cast<icu4x::NeoDateFormatterGregorian*>(ptr);
}

inline void icu4x::NeoDateFormatterGregorian::operator delete(void* ptr) {
  icu4x::capi::icu4x_NeoDateFormatterGregorian_destroy_mv1(reinterpret_cast<icu4x::capi::NeoDateFormatterGregorian*>(ptr));
}


#endif // icu4x_NeoDateFormatterGregorian_HPP
