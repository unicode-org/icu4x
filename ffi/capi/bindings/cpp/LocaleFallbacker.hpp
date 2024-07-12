#ifndef LocaleFallbacker_HPP
#define LocaleFallbacker_HPP

#include "LocaleFallbacker.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "LocaleFallbackConfig.hpp"
#include "LocaleFallbackerWithConfig.hpp"
#include "LocaleParseError.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct ICU4XLocaleFallbacker_create_result {union {diplomat::capi::LocaleFallbacker* ok; diplomat::capi::DataError err;}; bool is_ok;} ICU4XLocaleFallbacker_create_result;
    ICU4XLocaleFallbacker_create_result ICU4XLocaleFallbacker_create(const diplomat::capi::DataProvider* provider);
    
    diplomat::capi::LocaleFallbacker* ICU4XLocaleFallbacker_create_without_data();
    
    typedef struct ICU4XLocaleFallbacker_for_config_result {union {diplomat::capi::LocaleFallbackerWithConfig* ok; diplomat::capi::LocaleParseError err;}; bool is_ok;} ICU4XLocaleFallbacker_for_config_result;
    ICU4XLocaleFallbacker_for_config_result ICU4XLocaleFallbacker_for_config(const diplomat::capi::LocaleFallbacker* self, diplomat::capi::LocaleFallbackConfig config);
    
    
    void ICU4XLocaleFallbacker_destroy(LocaleFallbacker* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<LocaleFallbacker>, DataError> LocaleFallbacker::create(const DataProvider& provider) {
  auto result = diplomat::capi::ICU4XLocaleFallbacker_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<LocaleFallbacker>, DataError>(diplomat::Ok<std::unique_ptr<LocaleFallbacker>>(std::unique_ptr<LocaleFallbacker>(LocaleFallbacker::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<LocaleFallbacker>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline std::unique_ptr<LocaleFallbacker> LocaleFallbacker::create_without_data() {
  auto result = diplomat::capi::ICU4XLocaleFallbacker_create_without_data();
  return std::unique_ptr<LocaleFallbacker>(LocaleFallbacker::FromFFI(result));
}

inline diplomat::result<std::unique_ptr<LocaleFallbackerWithConfig>, LocaleParseError> LocaleFallbacker::for_config(LocaleFallbackConfig config) const {
  auto result = diplomat::capi::ICU4XLocaleFallbacker_for_config(this->AsFFI(),
    config.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<LocaleFallbackerWithConfig>, LocaleParseError>(diplomat::Ok<std::unique_ptr<LocaleFallbackerWithConfig>>(std::unique_ptr<LocaleFallbackerWithConfig>(LocaleFallbackerWithConfig::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<LocaleFallbackerWithConfig>, LocaleParseError>(diplomat::Err<LocaleParseError>(LocaleParseError::FromFFI(result.err)));
}

inline const diplomat::capi::LocaleFallbacker* LocaleFallbacker::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::LocaleFallbacker*>(this);
}

inline diplomat::capi::LocaleFallbacker* LocaleFallbacker::AsFFI() {
  return reinterpret_cast<diplomat::capi::LocaleFallbacker*>(this);
}

inline const LocaleFallbacker* LocaleFallbacker::FromFFI(const diplomat::capi::LocaleFallbacker* ptr) {
  return reinterpret_cast<const LocaleFallbacker*>(ptr);
}

inline LocaleFallbacker* LocaleFallbacker::FromFFI(diplomat::capi::LocaleFallbacker* ptr) {
  return reinterpret_cast<LocaleFallbacker*>(ptr);
}

inline void LocaleFallbacker::operator delete(void* ptr) {
  diplomat::capi::ICU4XLocaleFallbacker_destroy(reinterpret_cast<diplomat::capi::LocaleFallbacker*>(ptr));
}


#endif // LocaleFallbacker_HPP
