#ifndef ICU4XCanonicalCombiningClassMap_HPP
#define ICU4XCanonicalCombiningClassMap_HPP

#include "ICU4XCanonicalCombiningClassMap.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XCanonicalCombiningClassMap.h"
#include "ICU4XDataProvider.hpp"
#include "ICU4XError.hpp"


inline diplomat::result<std::unique_ptr<ICU4XCanonicalCombiningClassMap>, ICU4XError> ICU4XCanonicalCombiningClassMap::create(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCanonicalCombiningClassMap_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCanonicalCombiningClassMap>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCanonicalCombiningClassMap>>(std::unique_ptr<ICU4XCanonicalCombiningClassMap>(ICU4XCanonicalCombiningClassMap::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCanonicalCombiningClassMap>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline uint8_t ICU4XCanonicalCombiningClassMap::get(char32_t ch) const {
  auto result = capi::ICU4XCanonicalCombiningClassMap_get(this->AsFFI(),
    ch);
  return result;
}

inline uint8_t ICU4XCanonicalCombiningClassMap::get32(uint32_t ch) const {
  auto result = capi::ICU4XCanonicalCombiningClassMap_get32(this->AsFFI(),
    ch);
  return result;
}

inline const capi::ICU4XCanonicalCombiningClassMap* ICU4XCanonicalCombiningClassMap::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XCanonicalCombiningClassMap*>(this);
}

inline capi::ICU4XCanonicalCombiningClassMap* ICU4XCanonicalCombiningClassMap::AsFFI() {
  return reinterpret_cast<capi::ICU4XCanonicalCombiningClassMap*>(this);
}

inline const ICU4XCanonicalCombiningClassMap* ICU4XCanonicalCombiningClassMap::FromFFI(const capi::ICU4XCanonicalCombiningClassMap* ptr) {
  return reinterpret_cast<const ICU4XCanonicalCombiningClassMap*>(ptr);
}

inline ICU4XCanonicalCombiningClassMap* ICU4XCanonicalCombiningClassMap::FromFFI(capi::ICU4XCanonicalCombiningClassMap* ptr) {
  return reinterpret_cast<ICU4XCanonicalCombiningClassMap*>(ptr);
}

inline void ICU4XCanonicalCombiningClassMap::operator delete(void* ptr) {
  capi::ICU4XCanonicalCombiningClassMap_destroy(reinterpret_cast<capi::ICU4XCanonicalCombiningClassMap*>(ptr));
}


#endif // ICU4XCanonicalCombiningClassMap_HPP
