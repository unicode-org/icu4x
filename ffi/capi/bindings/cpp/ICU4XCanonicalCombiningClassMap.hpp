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
#include "ICU4XDataError.hpp"
#include "ICU4XDataProvider.hpp"


namespace capi {
    extern "C" {
    
    struct ICU4XCanonicalCombiningClassMap_create_result {union {ICU4XCanonicalCombiningClassMap* ok; ICU4XDataError err;}; bool is_ok;};
    struct ICU4XCanonicalCombiningClassMap_create_result ICU4XCanonicalCombiningClassMap_create(const ICU4XDataProvider* provider);
    
    uint8_t ICU4XCanonicalCombiningClassMap_get(const ICU4XCanonicalCombiningClassMap* self, char32_t ch);
    
    uint8_t ICU4XCanonicalCombiningClassMap_get32(const ICU4XCanonicalCombiningClassMap* self, uint32_t ch);
    
    
    void ICU4XCanonicalCombiningClassMap_destroy(ICU4XCanonicalCombiningClassMap* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<ICU4XCanonicalCombiningClassMap>, ICU4XDataError> ICU4XCanonicalCombiningClassMap::create(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCanonicalCombiningClassMap_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCanonicalCombiningClassMap>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XCanonicalCombiningClassMap>>(std::unique_ptr<ICU4XCanonicalCombiningClassMap>(ICU4XCanonicalCombiningClassMap::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCanonicalCombiningClassMap>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
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
