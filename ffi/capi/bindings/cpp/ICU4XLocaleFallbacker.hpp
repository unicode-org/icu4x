#ifndef ICU4XLocaleFallbacker_HPP
#define ICU4XLocaleFallbacker_HPP

#include "ICU4XLocaleFallbacker.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataProvider.hpp"
#include "ICU4XError.hpp"
#include "ICU4XLocaleFallbackConfig.hpp"
#include "ICU4XLocaleFallbacker.h"
#include "ICU4XLocaleFallbackerWithConfig.hpp"


inline diplomat::result<std::unique_ptr<ICU4XLocaleFallbacker>, ICU4XError> ICU4XLocaleFallbacker::create(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XLocaleFallbacker_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XLocaleFallbacker>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XLocaleFallbacker>>(std::unique_ptr<ICU4XLocaleFallbacker>(ICU4XLocaleFallbacker::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XLocaleFallbacker>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline std::unique_ptr<ICU4XLocaleFallbacker> ICU4XLocaleFallbacker::create_without_data() {
  auto result = capi::ICU4XLocaleFallbacker_create_without_data();
  return std::unique_ptr<ICU4XLocaleFallbacker>(ICU4XLocaleFallbacker::FromFFI(result));
}

inline diplomat::result<std::unique_ptr<ICU4XLocaleFallbackerWithConfig>, ICU4XError> ICU4XLocaleFallbacker::for_config(ICU4XLocaleFallbackConfig config) const {
  auto result = capi::ICU4XLocaleFallbacker_for_config(this->AsFFI(),
    config.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XLocaleFallbackerWithConfig>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XLocaleFallbackerWithConfig>>(std::unique_ptr<ICU4XLocaleFallbackerWithConfig>(ICU4XLocaleFallbackerWithConfig::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XLocaleFallbackerWithConfig>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline const capi::ICU4XLocaleFallbacker* ICU4XLocaleFallbacker::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XLocaleFallbacker*>(this);
}

inline capi::ICU4XLocaleFallbacker* ICU4XLocaleFallbacker::AsFFI() {
  return reinterpret_cast<capi::ICU4XLocaleFallbacker*>(this);
}

inline const ICU4XLocaleFallbacker* ICU4XLocaleFallbacker::FromFFI(const capi::ICU4XLocaleFallbacker* ptr) {
  return reinterpret_cast<const ICU4XLocaleFallbacker*>(ptr);
}

inline ICU4XLocaleFallbacker* ICU4XLocaleFallbacker::FromFFI(capi::ICU4XLocaleFallbacker* ptr) {
  return reinterpret_cast<ICU4XLocaleFallbacker*>(ptr);
}

inline void ICU4XLocaleFallbacker::operator delete(void* ptr) {
  capi::ICU4XLocaleFallbacker_destroy(reinterpret_cast<capi::ICU4XLocaleFallbacker*>(ptr));
}


#endif // ICU4XLocaleFallbacker_HPP
