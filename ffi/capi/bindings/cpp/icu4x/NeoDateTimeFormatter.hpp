#ifndef icu4x_NeoDateTimeFormatter_HPP
#define icu4x_NeoDateTimeFormatter_HPP

#include "NeoDateTimeFormatter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"
#include "DateTimeAlignment.hpp"
#include "DateTimeFieldSetBuilder.hpp"
#include "DateTimeFormatterBuildOrLoadError.hpp"
#include "DateTimeFormatterLoadError.hpp"
#include "IsoDate.hpp"
#include "Locale.hpp"
#include "NeoDateTimeLength.hpp"
#include "Time.hpp"
#include "TimePrecision.hpp"
#include "YearStyle.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_NeoDateTimeFormatter_create_from_builder_mv1_result {union {icu4x::capi::NeoDateTimeFormatter* ok; icu4x::capi::DateTimeFormatterBuildOrLoadError err;}; bool is_ok;} icu4x_NeoDateTimeFormatter_create_from_builder_mv1_result;
    icu4x_NeoDateTimeFormatter_create_from_builder_mv1_result icu4x_NeoDateTimeFormatter_create_from_builder_mv1(const icu4x::capi::Locale* locale, icu4x::capi::DateTimeFieldSetBuilder builder);
    
    typedef struct icu4x_NeoDateTimeFormatter_create_dt_mv1_result {union {icu4x::capi::NeoDateTimeFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateTimeFormatter_create_dt_mv1_result;
    icu4x_NeoDateTimeFormatter_create_dt_mv1_result icu4x_NeoDateTimeFormatter_create_dt_mv1(const icu4x::capi::Locale* locale, icu4x::capi::NeoDateTimeLength length, icu4x::capi::TimePrecision time_precision, icu4x::capi::DateTimeAlignment alignment);
    
    typedef struct icu4x_NeoDateTimeFormatter_create_mdt_mv1_result {union {icu4x::capi::NeoDateTimeFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateTimeFormatter_create_mdt_mv1_result;
    icu4x_NeoDateTimeFormatter_create_mdt_mv1_result icu4x_NeoDateTimeFormatter_create_mdt_mv1(const icu4x::capi::Locale* locale, icu4x::capi::NeoDateTimeLength length, icu4x::capi::TimePrecision time_precision, icu4x::capi::DateTimeAlignment alignment);
    
    typedef struct icu4x_NeoDateTimeFormatter_create_ymdt_mv1_result {union {icu4x::capi::NeoDateTimeFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateTimeFormatter_create_ymdt_mv1_result;
    icu4x_NeoDateTimeFormatter_create_ymdt_mv1_result icu4x_NeoDateTimeFormatter_create_ymdt_mv1(const icu4x::capi::Locale* locale, icu4x::capi::NeoDateTimeLength length, icu4x::capi::TimePrecision time_precision, icu4x::capi::DateTimeAlignment alignment, icu4x::capi::YearStyle year_style);
    
    typedef struct icu4x_NeoDateTimeFormatter_create_det_mv1_result {union {icu4x::capi::NeoDateTimeFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateTimeFormatter_create_det_mv1_result;
    icu4x_NeoDateTimeFormatter_create_det_mv1_result icu4x_NeoDateTimeFormatter_create_det_mv1(const icu4x::capi::Locale* locale, icu4x::capi::NeoDateTimeLength length, icu4x::capi::TimePrecision time_precision, icu4x::capi::DateTimeAlignment alignment);
    
    typedef struct icu4x_NeoDateTimeFormatter_create_mdet_mv1_result {union {icu4x::capi::NeoDateTimeFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateTimeFormatter_create_mdet_mv1_result;
    icu4x_NeoDateTimeFormatter_create_mdet_mv1_result icu4x_NeoDateTimeFormatter_create_mdet_mv1(const icu4x::capi::Locale* locale, icu4x::capi::NeoDateTimeLength length, icu4x::capi::TimePrecision time_precision, icu4x::capi::DateTimeAlignment alignment);
    
    typedef struct icu4x_NeoDateTimeFormatter_create_ymdet_mv1_result {union {icu4x::capi::NeoDateTimeFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateTimeFormatter_create_ymdet_mv1_result;
    icu4x_NeoDateTimeFormatter_create_ymdet_mv1_result icu4x_NeoDateTimeFormatter_create_ymdet_mv1(const icu4x::capi::Locale* locale, icu4x::capi::NeoDateTimeLength length, icu4x::capi::TimePrecision time_precision, icu4x::capi::DateTimeAlignment alignment, icu4x::capi::YearStyle year_style);
    
    typedef struct icu4x_NeoDateTimeFormatter_create_et_mv1_result {union {icu4x::capi::NeoDateTimeFormatter* ok; icu4x::capi::DateTimeFormatterLoadError err;}; bool is_ok;} icu4x_NeoDateTimeFormatter_create_et_mv1_result;
    icu4x_NeoDateTimeFormatter_create_et_mv1_result icu4x_NeoDateTimeFormatter_create_et_mv1(const icu4x::capi::Locale* locale, icu4x::capi::NeoDateTimeLength length, icu4x::capi::TimePrecision time_precision, icu4x::capi::DateTimeAlignment alignment);
    
    void icu4x_NeoDateTimeFormatter_format_iso_mv1(const icu4x::capi::NeoDateTimeFormatter* self, const icu4x::capi::IsoDate* date, const icu4x::capi::Time* time, diplomat::capi::DiplomatWrite* write);
    
    
    void icu4x_NeoDateTimeFormatter_destroy_mv1(NeoDateTimeFormatter* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<icu4x::NeoDateTimeFormatter>, icu4x::DateTimeFormatterBuildOrLoadError> icu4x::NeoDateTimeFormatter::create_from_builder(const icu4x::Locale& locale, icu4x::DateTimeFieldSetBuilder builder) {
  auto result = icu4x::capi::icu4x_NeoDateTimeFormatter_create_from_builder_mv1(locale.AsFFI(),
    builder.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateTimeFormatter>, icu4x::DateTimeFormatterBuildOrLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateTimeFormatter>>(std::unique_ptr<icu4x::NeoDateTimeFormatter>(icu4x::NeoDateTimeFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateTimeFormatter>, icu4x::DateTimeFormatterBuildOrLoadError>(diplomat::Err<icu4x::DateTimeFormatterBuildOrLoadError>(icu4x::DateTimeFormatterBuildOrLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoDateTimeFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateTimeFormatter::create_dt(const icu4x::Locale& locale, icu4x::NeoDateTimeLength length, icu4x::TimePrecision time_precision, icu4x::DateTimeAlignment alignment) {
  auto result = icu4x::capi::icu4x_NeoDateTimeFormatter_create_dt_mv1(locale.AsFFI(),
    length.AsFFI(),
    time_precision.AsFFI(),
    alignment.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateTimeFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateTimeFormatter>>(std::unique_ptr<icu4x::NeoDateTimeFormatter>(icu4x::NeoDateTimeFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateTimeFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoDateTimeFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateTimeFormatter::create_mdt(const icu4x::Locale& locale, icu4x::NeoDateTimeLength length, icu4x::TimePrecision time_precision, icu4x::DateTimeAlignment alignment) {
  auto result = icu4x::capi::icu4x_NeoDateTimeFormatter_create_mdt_mv1(locale.AsFFI(),
    length.AsFFI(),
    time_precision.AsFFI(),
    alignment.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateTimeFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateTimeFormatter>>(std::unique_ptr<icu4x::NeoDateTimeFormatter>(icu4x::NeoDateTimeFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateTimeFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoDateTimeFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateTimeFormatter::create_ymdt(const icu4x::Locale& locale, icu4x::NeoDateTimeLength length, icu4x::TimePrecision time_precision, icu4x::DateTimeAlignment alignment, icu4x::YearStyle year_style) {
  auto result = icu4x::capi::icu4x_NeoDateTimeFormatter_create_ymdt_mv1(locale.AsFFI(),
    length.AsFFI(),
    time_precision.AsFFI(),
    alignment.AsFFI(),
    year_style.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateTimeFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateTimeFormatter>>(std::unique_ptr<icu4x::NeoDateTimeFormatter>(icu4x::NeoDateTimeFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateTimeFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoDateTimeFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateTimeFormatter::create_det(const icu4x::Locale& locale, icu4x::NeoDateTimeLength length, icu4x::TimePrecision time_precision, icu4x::DateTimeAlignment alignment) {
  auto result = icu4x::capi::icu4x_NeoDateTimeFormatter_create_det_mv1(locale.AsFFI(),
    length.AsFFI(),
    time_precision.AsFFI(),
    alignment.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateTimeFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateTimeFormatter>>(std::unique_ptr<icu4x::NeoDateTimeFormatter>(icu4x::NeoDateTimeFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateTimeFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoDateTimeFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateTimeFormatter::create_mdet(const icu4x::Locale& locale, icu4x::NeoDateTimeLength length, icu4x::TimePrecision time_precision, icu4x::DateTimeAlignment alignment) {
  auto result = icu4x::capi::icu4x_NeoDateTimeFormatter_create_mdet_mv1(locale.AsFFI(),
    length.AsFFI(),
    time_precision.AsFFI(),
    alignment.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateTimeFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateTimeFormatter>>(std::unique_ptr<icu4x::NeoDateTimeFormatter>(icu4x::NeoDateTimeFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateTimeFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoDateTimeFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateTimeFormatter::create_ymdet(const icu4x::Locale& locale, icu4x::NeoDateTimeLength length, icu4x::TimePrecision time_precision, icu4x::DateTimeAlignment alignment, icu4x::YearStyle year_style) {
  auto result = icu4x::capi::icu4x_NeoDateTimeFormatter_create_ymdet_mv1(locale.AsFFI(),
    length.AsFFI(),
    time_precision.AsFFI(),
    alignment.AsFFI(),
    year_style.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateTimeFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateTimeFormatter>>(std::unique_ptr<icu4x::NeoDateTimeFormatter>(icu4x::NeoDateTimeFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateTimeFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::NeoDateTimeFormatter>, icu4x::DateTimeFormatterLoadError> icu4x::NeoDateTimeFormatter::create_et(const icu4x::Locale& locale, icu4x::NeoDateTimeLength length, icu4x::TimePrecision time_precision, icu4x::DateTimeAlignment alignment) {
  auto result = icu4x::capi::icu4x_NeoDateTimeFormatter_create_et_mv1(locale.AsFFI(),
    length.AsFFI(),
    time_precision.AsFFI(),
    alignment.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::NeoDateTimeFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Ok<std::unique_ptr<icu4x::NeoDateTimeFormatter>>(std::unique_ptr<icu4x::NeoDateTimeFormatter>(icu4x::NeoDateTimeFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::NeoDateTimeFormatter>, icu4x::DateTimeFormatterLoadError>(diplomat::Err<icu4x::DateTimeFormatterLoadError>(icu4x::DateTimeFormatterLoadError::FromFFI(result.err)));
}

inline std::string icu4x::NeoDateTimeFormatter::format_iso(const icu4x::IsoDate& date, const icu4x::Time& time) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  icu4x::capi::icu4x_NeoDateTimeFormatter_format_iso_mv1(this->AsFFI(),
    date.AsFFI(),
    time.AsFFI(),
    &write);
  return output;
}

inline const icu4x::capi::NeoDateTimeFormatter* icu4x::NeoDateTimeFormatter::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::NeoDateTimeFormatter*>(this);
}

inline icu4x::capi::NeoDateTimeFormatter* icu4x::NeoDateTimeFormatter::AsFFI() {
  return reinterpret_cast<icu4x::capi::NeoDateTimeFormatter*>(this);
}

inline const icu4x::NeoDateTimeFormatter* icu4x::NeoDateTimeFormatter::FromFFI(const icu4x::capi::NeoDateTimeFormatter* ptr) {
  return reinterpret_cast<const icu4x::NeoDateTimeFormatter*>(ptr);
}

inline icu4x::NeoDateTimeFormatter* icu4x::NeoDateTimeFormatter::FromFFI(icu4x::capi::NeoDateTimeFormatter* ptr) {
  return reinterpret_cast<icu4x::NeoDateTimeFormatter*>(ptr);
}

inline void icu4x::NeoDateTimeFormatter::operator delete(void* ptr) {
  icu4x::capi::icu4x_NeoDateTimeFormatter_destroy_mv1(reinterpret_cast<icu4x::capi::NeoDateTimeFormatter*>(ptr));
}


#endif // icu4x_NeoDateTimeFormatter_HPP
