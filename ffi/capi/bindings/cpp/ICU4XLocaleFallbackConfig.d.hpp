#ifndef ICU4XLocaleFallbackConfig_D_HPP
#define ICU4XLocaleFallbackConfig_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XLocaleFallbackPriority.d.hpp"
#include "ICU4XLocaleFallbackSupplement.d.hpp"

class ICU4XLocaleFallbackPriority;
class ICU4XLocaleFallbackSupplement;


namespace capi {
    typedef struct ICU4XLocaleFallbackConfig {
      ICU4XLocaleFallbackPriority priority;
      DiplomatStringView extension_key;
      ICU4XLocaleFallbackSupplement fallback_supplement;
    } ICU4XLocaleFallbackConfig;
}

struct ICU4XLocaleFallbackConfig {
  ICU4XLocaleFallbackPriority priority;
  std::string_view extension_key;
  ICU4XLocaleFallbackSupplement fallback_supplement;

  inline capi::ICU4XLocaleFallbackConfig AsFFI() const;
  inline static ICU4XLocaleFallbackConfig FromFFI(capi::ICU4XLocaleFallbackConfig c_struct);
};


#endif // ICU4XLocaleFallbackConfig_D_HPP
