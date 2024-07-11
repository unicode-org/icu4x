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


namespace capi {
    extern "C" {
    
    typedef struct ICU4XLocaleFallbacker_create_result {union {LocaleFallbacker* ok; DataError err;}; bool is_ok;} ICU4XLocaleFallbacker_create_result;
    ICU4XLocaleFallbacker_create_result ICU4XLocaleFallbacker_create(const DataProvider* provider);
    
    LocaleFallbacker* ICU4XLocaleFallbacker_create_without_data();
    
    typedef struct ICU4XLocaleFallbacker_for_config_result {union {LocaleFallbackerWithConfig* ok; LocaleParseError err;}; bool is_ok;} ICU4XLocaleFallbacker_for_config_result;
    ICU4XLocaleFallbacker_for_config_result ICU4XLocaleFallbacker_for_config(const LocaleFallbacker* self, LocaleFallbackConfig config);
    
    
    void ICU4XLocaleFallbacker_destroy(LocaleFallbacker* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<LocaleFallbacker>, DataError> LocaleFallbacker::create(const DataProvider& provider) {
  auto result = capi::ICU4XLocaleFallbacker_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<LocaleFallbacker>, DataError>(diplomat::Ok<std::unique_ptr<LocaleFallbacker>>(std::unique_ptr<LocaleFallbacker>(LocaleFallbacker::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<LocaleFallbacker>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline std::unique_ptr<LocaleFallbacker> LocaleFallbacker::create_without_data() {
  auto result = capi::ICU4XLocaleFallbacker_create_without_data();
  return std::unique_ptr<LocaleFallbacker>(LocaleFallbacker::FromFFI(result));
}

inline diplomat::result<std::unique_ptr<LocaleFallbackerWithConfig>, LocaleParseError> LocaleFallbacker::for_config(LocaleFallbackConfig config) const {
  auto result = capi::ICU4XLocaleFallbacker_for_config(this->AsFFI(),
    config.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<LocaleFallbackerWithConfig>, LocaleParseError>(diplomat::Ok<std::unique_ptr<LocaleFallbackerWithConfig>>(std::unique_ptr<LocaleFallbackerWithConfig>(LocaleFallbackerWithConfig::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<LocaleFallbackerWithConfig>, LocaleParseError>(diplomat::Err<LocaleParseError>(LocaleParseError::FromFFI(result.err)));
}

inline const capi::LocaleFallbacker* LocaleFallbacker::AsFFI() const {
  return reinterpret_cast<const capi::LocaleFallbacker*>(this);
}

inline capi::LocaleFallbacker* LocaleFallbacker::AsFFI() {
  return reinterpret_cast<capi::LocaleFallbacker*>(this);
}

inline const LocaleFallbacker* LocaleFallbacker::FromFFI(const capi::LocaleFallbacker* ptr) {
  return reinterpret_cast<const LocaleFallbacker*>(ptr);
}

inline LocaleFallbacker* LocaleFallbacker::FromFFI(capi::LocaleFallbacker* ptr) {
  return reinterpret_cast<LocaleFallbacker*>(ptr);
}

inline void LocaleFallbacker::operator delete(void* ptr) {
  capi::ICU4XLocaleFallbacker_destroy(reinterpret_cast<capi::LocaleFallbacker*>(ptr));
}


#endif // LocaleFallbacker_HPP
