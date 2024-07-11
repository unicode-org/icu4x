#ifndef LocaleFallbackerWithConfig_D_HPP
#define LocaleFallbackerWithConfig_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

class Locale;
class LocaleFallbackIterator;


namespace capi {
    typedef struct LocaleFallbackerWithConfig LocaleFallbackerWithConfig;
}

class LocaleFallbackerWithConfig {
public:

  inline std::unique_ptr<LocaleFallbackIterator> fallback_for_locale(const Locale& locale) const;

  inline const capi::LocaleFallbackerWithConfig* AsFFI() const;
  inline capi::LocaleFallbackerWithConfig* AsFFI();
  inline static const LocaleFallbackerWithConfig* FromFFI(const capi::LocaleFallbackerWithConfig* ptr);
  inline static LocaleFallbackerWithConfig* FromFFI(capi::LocaleFallbackerWithConfig* ptr);
  inline static void operator delete(void* ptr);
private:
  LocaleFallbackerWithConfig() = delete;
  LocaleFallbackerWithConfig(const LocaleFallbackerWithConfig&) = delete;
  LocaleFallbackerWithConfig(LocaleFallbackerWithConfig&&) noexcept = delete;
  LocaleFallbackerWithConfig operator=(const LocaleFallbackerWithConfig&) = delete;
  LocaleFallbackerWithConfig operator=(LocaleFallbackerWithConfig&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // LocaleFallbackerWithConfig_D_HPP
