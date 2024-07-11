#ifndef LocaleFallbackConfig_D_HPP
#define LocaleFallbackConfig_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "LocaleFallbackPriority.d.hpp"
#include "LocaleFallbackSupplement.d.hpp"

class LocaleFallbackPriority;
class LocaleFallbackSupplement;


namespace diplomat {
namespace capi {
    typedef struct LocaleFallbackConfig {
      diplomat::capi::LocaleFallbackPriority priority;
      DiplomatStringView extension_key;
      diplomat::capi::LocaleFallbackSupplement fallback_supplement;
    } LocaleFallbackConfig;
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
