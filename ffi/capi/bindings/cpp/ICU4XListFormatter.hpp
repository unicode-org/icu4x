#ifndef ICU4XListFormatter_HPP
#define ICU4XListFormatter_HPP

#include "ICU4XListFormatter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataError.hpp"
#include "ICU4XDataProvider.hpp"
#include "ICU4XListFormatter.h"
#include "ICU4XListLength.hpp"
#include "ICU4XLocale.hpp"


inline diplomat::result<std::unique_ptr<ICU4XListFormatter>, ICU4XDataError> ICU4XListFormatter::create_and_with_length(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XListLength length) {
  auto result = capi::ICU4XListFormatter_create_and_with_length(provider.AsFFI(),
    locale.AsFFI(),
    length.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XListFormatter>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XListFormatter>>(std::unique_ptr<ICU4XListFormatter>(ICU4XListFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XListFormatter>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XListFormatter>, ICU4XDataError> ICU4XListFormatter::create_or_with_length(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XListLength length) {
  auto result = capi::ICU4XListFormatter_create_or_with_length(provider.AsFFI(),
    locale.AsFFI(),
    length.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XListFormatter>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XListFormatter>>(std::unique_ptr<ICU4XListFormatter>(ICU4XListFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XListFormatter>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XListFormatter>, ICU4XDataError> ICU4XListFormatter::create_unit_with_length(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XListLength length) {
  auto result = capi::ICU4XListFormatter_create_unit_with_length(provider.AsFFI(),
    locale.AsFFI(),
    length.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XListFormatter>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XListFormatter>>(std::unique_ptr<ICU4XListFormatter>(ICU4XListFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XListFormatter>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline std::string ICU4XListFormatter::format_valid_utf8(diplomat::span<const std::string_view> list) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XListFormatter_format_valid_utf8(this->AsFFI(),
    list.data(),
    list.size(),
    &write);
  return output;
}

inline std::string ICU4XListFormatter::format_utf8(diplomat::span<const std::string_view> list) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XListFormatter_format_utf8(this->AsFFI(),
    list.data(),
    list.size(),
    &write);
  return output;
}

inline std::string ICU4XListFormatter::format_utf16(diplomat::span<const std::u16string_view> list) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XListFormatter_format_utf16(this->AsFFI(),
    list.data(),
    list.size(),
    &write);
  return output;
}

inline const capi::ICU4XListFormatter* ICU4XListFormatter::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XListFormatter*>(this);
}

inline capi::ICU4XListFormatter* ICU4XListFormatter::AsFFI() {
  return reinterpret_cast<capi::ICU4XListFormatter*>(this);
}

inline const ICU4XListFormatter* ICU4XListFormatter::FromFFI(const capi::ICU4XListFormatter* ptr) {
  return reinterpret_cast<const ICU4XListFormatter*>(ptr);
}

inline ICU4XListFormatter* ICU4XListFormatter::FromFFI(capi::ICU4XListFormatter* ptr) {
  return reinterpret_cast<ICU4XListFormatter*>(ptr);
}

inline void ICU4XListFormatter::operator delete(void* ptr) {
  capi::ICU4XListFormatter_destroy(reinterpret_cast<capi::ICU4XListFormatter*>(ptr));
}


#endif // ICU4XListFormatter_HPP
