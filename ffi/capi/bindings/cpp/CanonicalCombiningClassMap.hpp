#ifndef CanonicalCombiningClassMap_HPP
#define CanonicalCombiningClassMap_HPP

#include "CanonicalCombiningClassMap.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"


namespace capi {
    extern "C" {
    
    typedef struct ICU4XCanonicalCombiningClassMap_create_result {union {CanonicalCombiningClassMap* ok; DataError err;}; bool is_ok;} ICU4XCanonicalCombiningClassMap_create_result;
    ICU4XCanonicalCombiningClassMap_create_result ICU4XCanonicalCombiningClassMap_create(const DataProvider* provider);
    
    uint8_t ICU4XCanonicalCombiningClassMap_get(const CanonicalCombiningClassMap* self, char32_t ch);
    
    uint8_t ICU4XCanonicalCombiningClassMap_get32(const CanonicalCombiningClassMap* self, uint32_t ch);
    
    
    void ICU4XCanonicalCombiningClassMap_destroy(CanonicalCombiningClassMap* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<CanonicalCombiningClassMap>, DataError> CanonicalCombiningClassMap::create(const DataProvider& provider) {
  auto result = capi::ICU4XCanonicalCombiningClassMap_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CanonicalCombiningClassMap>, DataError>(diplomat::Ok<std::unique_ptr<CanonicalCombiningClassMap>>(std::unique_ptr<CanonicalCombiningClassMap>(CanonicalCombiningClassMap::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CanonicalCombiningClassMap>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline uint8_t CanonicalCombiningClassMap::get(char32_t ch) const {
  auto result = capi::ICU4XCanonicalCombiningClassMap_get(this->AsFFI(),
    ch);
  return result;
}

inline uint8_t CanonicalCombiningClassMap::get32(uint32_t ch) const {
  auto result = capi::ICU4XCanonicalCombiningClassMap_get32(this->AsFFI(),
    ch);
  return result;
}

inline const capi::CanonicalCombiningClassMap* CanonicalCombiningClassMap::AsFFI() const {
  return reinterpret_cast<const capi::CanonicalCombiningClassMap*>(this);
}

inline capi::CanonicalCombiningClassMap* CanonicalCombiningClassMap::AsFFI() {
  return reinterpret_cast<capi::CanonicalCombiningClassMap*>(this);
}

inline const CanonicalCombiningClassMap* CanonicalCombiningClassMap::FromFFI(const capi::CanonicalCombiningClassMap* ptr) {
  return reinterpret_cast<const CanonicalCombiningClassMap*>(ptr);
}

inline CanonicalCombiningClassMap* CanonicalCombiningClassMap::FromFFI(capi::CanonicalCombiningClassMap* ptr) {
  return reinterpret_cast<CanonicalCombiningClassMap*>(ptr);
}

inline void CanonicalCombiningClassMap::operator delete(void* ptr) {
  capi::ICU4XCanonicalCombiningClassMap_destroy(reinterpret_cast<capi::CanonicalCombiningClassMap*>(ptr));
}


#endif // CanonicalCombiningClassMap_HPP
