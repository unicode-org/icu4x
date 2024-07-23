#ifndef CanonicalCombiningClassMap_HPP
#define CanonicalCombiningClassMap_HPP

#include "CanonicalCombiningClassMap.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_CanonicalCombiningClassMap_create_mv1_result {union {diplomat::capi::CanonicalCombiningClassMap* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CanonicalCombiningClassMap_create_mv1_result;
    icu4x_CanonicalCombiningClassMap_create_mv1_result icu4x_CanonicalCombiningClassMap_create_mv1(const diplomat::capi::DataProvider* provider);
    
    uint8_t icu4x_CanonicalCombiningClassMap_get_mv1(const diplomat::capi::CanonicalCombiningClassMap* self, char32_t ch);
    
    
    void icu4x_CanonicalCombiningClassMap_destroy_mv1(CanonicalCombiningClassMap* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<CanonicalCombiningClassMap>, DataError> CanonicalCombiningClassMap::create(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CanonicalCombiningClassMap_create_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CanonicalCombiningClassMap>, DataError>(diplomat::Ok<std::unique_ptr<CanonicalCombiningClassMap>>(std::unique_ptr<CanonicalCombiningClassMap>(CanonicalCombiningClassMap::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CanonicalCombiningClassMap>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline uint8_t CanonicalCombiningClassMap::get(char32_t ch) const {
  auto result = diplomat::capi::icu4x_CanonicalCombiningClassMap_get_mv1(this->AsFFI(),
    ch);
  return result;
}

inline const diplomat::capi::CanonicalCombiningClassMap* CanonicalCombiningClassMap::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::CanonicalCombiningClassMap*>(this);
}

inline diplomat::capi::CanonicalCombiningClassMap* CanonicalCombiningClassMap::AsFFI() {
  return reinterpret_cast<diplomat::capi::CanonicalCombiningClassMap*>(this);
}

inline const CanonicalCombiningClassMap* CanonicalCombiningClassMap::FromFFI(const diplomat::capi::CanonicalCombiningClassMap* ptr) {
  return reinterpret_cast<const CanonicalCombiningClassMap*>(ptr);
}

inline CanonicalCombiningClassMap* CanonicalCombiningClassMap::FromFFI(diplomat::capi::CanonicalCombiningClassMap* ptr) {
  return reinterpret_cast<CanonicalCombiningClassMap*>(ptr);
}

inline void CanonicalCombiningClassMap::operator delete(void* ptr) {
  diplomat::capi::icu4x_CanonicalCombiningClassMap_destroy_mv1(reinterpret_cast<diplomat::capi::CanonicalCombiningClassMap*>(ptr));
}


#endif // CanonicalCombiningClassMap_HPP
