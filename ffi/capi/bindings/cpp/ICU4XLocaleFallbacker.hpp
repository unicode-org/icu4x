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
#include "ICU4XDataError.hpp"
#include "ICU4XDataProvider.hpp"
#include "ICU4XLocaleFallbackConfig.hpp"
#include "ICU4XLocaleFallbackerWithConfig.hpp"
#include "ICU4XLocaleParseError.hpp"


namespace capi {
    extern "C" {
    
    typedef struct ICU4XLocaleFallbacker_create_result {union {ICU4XLocaleFallbacker* ok; ICU4XDataError err;}; bool is_ok;} ICU4XLocaleFallbacker_create_result;
    ICU4XLocaleFallbacker_create_result ICU4XLocaleFallbacker_create(const ICU4XDataProvider* provider);
    
    ICU4XLocaleFallbacker* ICU4XLocaleFallbacker_create_without_data();
    
    typedef struct ICU4XLocaleFallbacker_for_config_result {union {ICU4XLocaleFallbackerWithConfig* ok; ICU4XLocaleParseError err;}; bool is_ok;} ICU4XLocaleFallbacker_for_config_result;
    ICU4XLocaleFallbacker_for_config_result ICU4XLocaleFallbacker_for_config(const ICU4XLocaleFallbacker* self, ICU4XLocaleFallbackConfig config);
    
    
    void ICU4XLocaleFallbacker_destroy(ICU4XLocaleFallbacker* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<ICU4XLocaleFallbacker>, ICU4XDataError> ICU4XLocaleFallbacker::create(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XLocaleFallbacker_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XLocaleFallbacker>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XLocaleFallbacker>>(std::unique_ptr<ICU4XLocaleFallbacker>(ICU4XLocaleFallbacker::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XLocaleFallbacker>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline std::unique_ptr<ICU4XLocaleFallbacker> ICU4XLocaleFallbacker::create_without_data() {
  auto result = capi::ICU4XLocaleFallbacker_create_without_data();
  return std::unique_ptr<ICU4XLocaleFallbacker>(ICU4XLocaleFallbacker::FromFFI(result));
}

inline diplomat::result<std::unique_ptr<ICU4XLocaleFallbackerWithConfig>, ICU4XLocaleParseError> ICU4XLocaleFallbacker::for_config(ICU4XLocaleFallbackConfig config) const {
  auto result = capi::ICU4XLocaleFallbacker_for_config(this->AsFFI(),
    config.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XLocaleFallbackerWithConfig>, ICU4XLocaleParseError>(diplomat::Ok<std::unique_ptr<ICU4XLocaleFallbackerWithConfig>>(std::unique_ptr<ICU4XLocaleFallbackerWithConfig>(ICU4XLocaleFallbackerWithConfig::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XLocaleFallbackerWithConfig>, ICU4XLocaleParseError>(diplomat::Err<ICU4XLocaleParseError>(ICU4XLocaleParseError::FromFFI(result.err)));
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
