#ifndef ICU4XCanonicalCombiningClassMap_D_HPP
#define ICU4XCanonicalCombiningClassMap_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataError.d.hpp"

class ICU4XDataProvider;
class ICU4XDataError;


namespace capi {
    typedef struct ICU4XCanonicalCombiningClassMap ICU4XCanonicalCombiningClassMap;
}

class ICU4XCanonicalCombiningClassMap {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XCanonicalCombiningClassMap>, ICU4XDataError> create(const ICU4XDataProvider& provider);

  inline uint8_t get(char32_t ch) const;

  inline uint8_t get32(uint32_t ch) const;

  inline const capi::ICU4XCanonicalCombiningClassMap* AsFFI() const;
  inline capi::ICU4XCanonicalCombiningClassMap* AsFFI();
  inline static const ICU4XCanonicalCombiningClassMap* FromFFI(const capi::ICU4XCanonicalCombiningClassMap* ptr);
  inline static ICU4XCanonicalCombiningClassMap* FromFFI(capi::ICU4XCanonicalCombiningClassMap* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XCanonicalCombiningClassMap() = delete;
  ICU4XCanonicalCombiningClassMap(const ICU4XCanonicalCombiningClassMap&) = delete;
  ICU4XCanonicalCombiningClassMap(ICU4XCanonicalCombiningClassMap&&) noexcept = delete;
  ICU4XCanonicalCombiningClassMap operator=(const ICU4XCanonicalCombiningClassMap&) = delete;
  ICU4XCanonicalCombiningClassMap operator=(ICU4XCanonicalCombiningClassMap&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XCanonicalCombiningClassMap_D_HPP
