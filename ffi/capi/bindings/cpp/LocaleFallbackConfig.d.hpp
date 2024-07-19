#ifndef LocaleFallbackConfig_D_HPP
#define LocaleFallbackConfig_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "LocaleFallbackPriority.d.hpp"
#include "LocaleFallbackSupplement.d.hpp"
#include "diplomat_runtime.hpp"

class LocaleFallbackPriority;
class LocaleFallbackSupplement;


namespace diplomat {
namespace capi {
    struct LocaleFallbackConfig {
      diplomat::capi::LocaleFallbackPriority priority;
      diplomat::capi::DiplomatStringView extension_key;
      diplomat::capi::LocaleFallbackSupplement fallback_supplement;
    };
} // namespace capi
} // namespace


struct LocaleFallbackConfig {
  LocaleFallbackPriority priority;
  std::string_view extension_key;
  LocaleFallbackSupplement fallback_supplement;

  inline diplomat::capi::LocaleFallbackConfig AsFFI() const;
  inline static LocaleFallbackConfig FromFFI(diplomat::capi::LocaleFallbackConfig c_struct);
};


#endif // LocaleFallbackConfig_D_HPP
