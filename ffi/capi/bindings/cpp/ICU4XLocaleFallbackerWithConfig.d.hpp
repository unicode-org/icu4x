#ifndef ICU4XLocaleFallbackerWithConfig_D_HPP
#define ICU4XLocaleFallbackerWithConfig_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XLocaleFallbackerWithConfig.d.h"

class ICU4XLocale;
class ICU4XLocaleFallbackIterator;


class ICU4XLocaleFallbackerWithConfig {
public:

  inline std::unique_ptr<ICU4XLocaleFallbackIterator> fallback_for_locale(const ICU4XLocale& locale) const;

  inline const capi::ICU4XLocaleFallbackerWithConfig* AsFFI() const;
  inline capi::ICU4XLocaleFallbackerWithConfig* AsFFI();
  inline static const ICU4XLocaleFallbackerWithConfig* FromFFI(const capi::ICU4XLocaleFallbackerWithConfig* ptr);
  inline static ICU4XLocaleFallbackerWithConfig* FromFFI(capi::ICU4XLocaleFallbackerWithConfig* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XLocaleFallbackerWithConfig() = delete;
  ICU4XLocaleFallbackerWithConfig(const ICU4XLocaleFallbackerWithConfig&) = delete;
  ICU4XLocaleFallbackerWithConfig(ICU4XLocaleFallbackerWithConfig&&) noexcept = delete;
  ICU4XLocaleFallbackerWithConfig operator=(const ICU4XLocaleFallbackerWithConfig&) = delete;
  ICU4XLocaleFallbackerWithConfig operator=(ICU4XLocaleFallbackerWithConfig&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XLocaleFallbackerWithConfig_D_HPP
