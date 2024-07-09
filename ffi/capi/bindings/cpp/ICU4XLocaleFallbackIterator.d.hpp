#ifndef ICU4XLocaleFallbackIterator_D_HPP
#define ICU4XLocaleFallbackIterator_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

class ICU4XLocale;


namespace capi {
    typedef struct ICU4XLocaleFallbackIterator ICU4XLocaleFallbackIterator;
}

class ICU4XLocaleFallbackIterator {
public:

  inline std::unique_ptr<ICU4XLocale> next();

  inline const capi::ICU4XLocaleFallbackIterator* AsFFI() const;
  inline capi::ICU4XLocaleFallbackIterator* AsFFI();
  inline static const ICU4XLocaleFallbackIterator* FromFFI(const capi::ICU4XLocaleFallbackIterator* ptr);
  inline static ICU4XLocaleFallbackIterator* FromFFI(capi::ICU4XLocaleFallbackIterator* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XLocaleFallbackIterator() = delete;
  ICU4XLocaleFallbackIterator(const ICU4XLocaleFallbackIterator&) = delete;
  ICU4XLocaleFallbackIterator(ICU4XLocaleFallbackIterator&&) noexcept = delete;
  ICU4XLocaleFallbackIterator operator=(const ICU4XLocaleFallbackIterator&) = delete;
  ICU4XLocaleFallbackIterator operator=(ICU4XLocaleFallbackIterator&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XLocaleFallbackIterator_D_HPP
