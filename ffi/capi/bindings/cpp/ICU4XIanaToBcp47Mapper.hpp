#ifndef ICU4XIanaToBcp47Mapper_HPP
#define ICU4XIanaToBcp47Mapper_HPP

#include "ICU4XIanaToBcp47Mapper.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataProvider.hpp"
#include "ICU4XError.hpp"
#include "ICU4XIanaToBcp47Mapper.h"


inline diplomat::result<std::unique_ptr<ICU4XIanaToBcp47Mapper>, ICU4XError> ICU4XIanaToBcp47Mapper::create(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XIanaToBcp47Mapper_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XIanaToBcp47Mapper>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XIanaToBcp47Mapper>>(std::unique_ptr<ICU4XIanaToBcp47Mapper>(ICU4XIanaToBcp47Mapper::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XIanaToBcp47Mapper>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::string, ICU4XError> ICU4XIanaToBcp47Mapper::get(std::string_view value) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XIanaToBcp47Mapper_get(this->AsFFI(),
    value.data(),
    value.size(),
    &write);
  return result.is_ok ? diplomat::result<std::string, ICU4XError>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline const capi::ICU4XIanaToBcp47Mapper* ICU4XIanaToBcp47Mapper::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XIanaToBcp47Mapper*>(this);
}

inline capi::ICU4XIanaToBcp47Mapper* ICU4XIanaToBcp47Mapper::AsFFI() {
  return reinterpret_cast<capi::ICU4XIanaToBcp47Mapper*>(this);
}

inline const ICU4XIanaToBcp47Mapper* ICU4XIanaToBcp47Mapper::FromFFI(const capi::ICU4XIanaToBcp47Mapper* ptr) {
  return reinterpret_cast<const ICU4XIanaToBcp47Mapper*>(ptr);
}

inline ICU4XIanaToBcp47Mapper* ICU4XIanaToBcp47Mapper::FromFFI(capi::ICU4XIanaToBcp47Mapper* ptr) {
  return reinterpret_cast<ICU4XIanaToBcp47Mapper*>(ptr);
}

inline void ICU4XIanaToBcp47Mapper::operator delete(void* ptr) {
  capi::ICU4XIanaToBcp47Mapper_destroy(reinterpret_cast<capi::ICU4XIanaToBcp47Mapper*>(ptr));
}


#endif // ICU4XIanaToBcp47Mapper_HPP
