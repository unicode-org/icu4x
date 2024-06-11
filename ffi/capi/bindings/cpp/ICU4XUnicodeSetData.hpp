#ifndef ICU4XUnicodeSetData_HPP
#define ICU4XUnicodeSetData_HPP

#include "ICU4XUnicodeSetData.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataError.hpp"
#include "ICU4XDataProvider.hpp"
#include "ICU4XLocale.hpp"
#include "ICU4XUnicodeSetData.h"


inline bool ICU4XUnicodeSetData::contains(std::string_view s) const {
  auto result = capi::ICU4XUnicodeSetData_contains(this->AsFFI(),
    s.data(),
    s.size());
  return result;
}

inline bool ICU4XUnicodeSetData::contains_char(char32_t cp) const {
  auto result = capi::ICU4XUnicodeSetData_contains_char(this->AsFFI(),
    cp);
  return result;
}

inline bool ICU4XUnicodeSetData::contains32(uint32_t cp) const {
  auto result = capi::ICU4XUnicodeSetData_contains32(this->AsFFI(),
    cp);
  return result;
}

inline diplomat::result<std::unique_ptr<ICU4XUnicodeSetData>, ICU4XDataError> ICU4XUnicodeSetData::load_basic_emoji(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XUnicodeSetData_load_basic_emoji(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XUnicodeSetData>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XUnicodeSetData>>(std::unique_ptr<ICU4XUnicodeSetData>(ICU4XUnicodeSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XUnicodeSetData>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XUnicodeSetData>, ICU4XDataError> ICU4XUnicodeSetData::load_exemplars_main(const ICU4XDataProvider& provider, const ICU4XLocale& locale) {
  auto result = capi::ICU4XUnicodeSetData_load_exemplars_main(provider.AsFFI(),
    locale.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XUnicodeSetData>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XUnicodeSetData>>(std::unique_ptr<ICU4XUnicodeSetData>(ICU4XUnicodeSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XUnicodeSetData>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XUnicodeSetData>, ICU4XDataError> ICU4XUnicodeSetData::load_exemplars_auxiliary(const ICU4XDataProvider& provider, const ICU4XLocale& locale) {
  auto result = capi::ICU4XUnicodeSetData_load_exemplars_auxiliary(provider.AsFFI(),
    locale.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XUnicodeSetData>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XUnicodeSetData>>(std::unique_ptr<ICU4XUnicodeSetData>(ICU4XUnicodeSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XUnicodeSetData>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XUnicodeSetData>, ICU4XDataError> ICU4XUnicodeSetData::load_exemplars_punctuation(const ICU4XDataProvider& provider, const ICU4XLocale& locale) {
  auto result = capi::ICU4XUnicodeSetData_load_exemplars_punctuation(provider.AsFFI(),
    locale.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XUnicodeSetData>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XUnicodeSetData>>(std::unique_ptr<ICU4XUnicodeSetData>(ICU4XUnicodeSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XUnicodeSetData>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XUnicodeSetData>, ICU4XDataError> ICU4XUnicodeSetData::load_exemplars_numbers(const ICU4XDataProvider& provider, const ICU4XLocale& locale) {
  auto result = capi::ICU4XUnicodeSetData_load_exemplars_numbers(provider.AsFFI(),
    locale.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XUnicodeSetData>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XUnicodeSetData>>(std::unique_ptr<ICU4XUnicodeSetData>(ICU4XUnicodeSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XUnicodeSetData>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XUnicodeSetData>, ICU4XDataError> ICU4XUnicodeSetData::load_exemplars_index(const ICU4XDataProvider& provider, const ICU4XLocale& locale) {
  auto result = capi::ICU4XUnicodeSetData_load_exemplars_index(provider.AsFFI(),
    locale.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XUnicodeSetData>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XUnicodeSetData>>(std::unique_ptr<ICU4XUnicodeSetData>(ICU4XUnicodeSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XUnicodeSetData>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline const capi::ICU4XUnicodeSetData* ICU4XUnicodeSetData::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XUnicodeSetData*>(this);
}

inline capi::ICU4XUnicodeSetData* ICU4XUnicodeSetData::AsFFI() {
  return reinterpret_cast<capi::ICU4XUnicodeSetData*>(this);
}

inline const ICU4XUnicodeSetData* ICU4XUnicodeSetData::FromFFI(const capi::ICU4XUnicodeSetData* ptr) {
  return reinterpret_cast<const ICU4XUnicodeSetData*>(ptr);
}

inline ICU4XUnicodeSetData* ICU4XUnicodeSetData::FromFFI(capi::ICU4XUnicodeSetData* ptr) {
  return reinterpret_cast<ICU4XUnicodeSetData*>(ptr);
}

inline void ICU4XUnicodeSetData::operator delete(void* ptr) {
  capi::ICU4XUnicodeSetData_destroy(reinterpret_cast<capi::ICU4XUnicodeSetData*>(ptr));
}


#endif // ICU4XUnicodeSetData_HPP
