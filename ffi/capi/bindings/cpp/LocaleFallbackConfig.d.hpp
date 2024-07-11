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


namespace capi {
    typedef struct LocaleFallbackConfig {
      LocaleFallbackPriority priority;
      DiplomatStringView extension_key;
      LocaleFallbackSupplement fallback_supplement;
    } LocaleFallbackConfig;
}

struct LocaleFallbackConfig {
  LocaleFallbackPriority priority;
  std::string_view extension_key;
  LocaleFallbackSupplement fallback_supplement;

  inline capi::LocaleFallbackConfig AsFFI() const;
  inline static LocaleFallbackConfig FromFFI(capi::LocaleFallbackConfig c_struct);
};


#endif // LocaleFallbackConfig_D_HPP
