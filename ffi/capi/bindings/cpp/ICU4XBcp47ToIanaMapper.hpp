#ifndef ICU4XBcp47ToIanaMapper_HPP
#define ICU4XBcp47ToIanaMapper_HPP

#include "ICU4XBcp47ToIanaMapper.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XBcp47ToIanaMapper.h"
#include "ICU4XDataProvider.hpp"
#include "ICU4XError.hpp"


inline diplomat::result<std::unique_ptr<ICU4XBcp47ToIanaMapper>, ICU4XError> ICU4XBcp47ToIanaMapper::create(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XBcp47ToIanaMapper_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XBcp47ToIanaMapper>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XBcp47ToIanaMapper>>(std::unique_ptr<ICU4XBcp47ToIanaMapper>(ICU4XBcp47ToIanaMapper::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XBcp47ToIanaMapper>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::string, ICU4XError> ICU4XBcp47ToIanaMapper::get(std::string_view value) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XBcp47ToIanaMapper_get(this->AsFFI(),
    value.data(),
    value.size(),
    &write);
  return result.is_ok ? diplomat::result<std::string, ICU4XError>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline const capi::ICU4XBcp47ToIanaMapper* ICU4XBcp47ToIanaMapper::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XBcp47ToIanaMapper*>(this);
}

inline capi::ICU4XBcp47ToIanaMapper* ICU4XBcp47ToIanaMapper::AsFFI() {
  return reinterpret_cast<capi::ICU4XBcp47ToIanaMapper*>(this);
}

inline const ICU4XBcp47ToIanaMapper* ICU4XBcp47ToIanaMapper::FromFFI(const capi::ICU4XBcp47ToIanaMapper* ptr) {
  return reinterpret_cast<const ICU4XBcp47ToIanaMapper*>(ptr);
}

inline ICU4XBcp47ToIanaMapper* ICU4XBcp47ToIanaMapper::FromFFI(capi::ICU4XBcp47ToIanaMapper* ptr) {
  return reinterpret_cast<ICU4XBcp47ToIanaMapper*>(ptr);
}

inline void ICU4XBcp47ToIanaMapper::operator delete(void* ptr) {
  capi::ICU4XBcp47ToIanaMapper_destroy(reinterpret_cast<capi::ICU4XBcp47ToIanaMapper*>(ptr));
}


#endif // ICU4XBcp47ToIanaMapper_HPP
