#ifndef DisplayNamesOptionsV1_D_HPP
#define DisplayNamesOptionsV1_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "DisplayNamesFallback.d.hpp"
#include "DisplayNamesStyle.d.hpp"
#include "LanguageDisplay.d.hpp"

class DisplayNamesFallback;
class DisplayNamesStyle;
class LanguageDisplay;


namespace diplomat {
namespace capi {
    struct DisplayNamesOptionsV1 {
      diplomat::capi::DisplayNamesStyle style;
      diplomat::capi::DisplayNamesFallback fallback;
      diplomat::capi::LanguageDisplay language_display;
    };
} // namespace capi
} // namespace


struct DisplayNamesOptionsV1 {
  DisplayNamesStyle style;
  DisplayNamesFallback fallback;
  LanguageDisplay language_display;

  inline diplomat::capi::DisplayNamesOptionsV1 AsFFI() const;
  inline static DisplayNamesOptionsV1 FromFFI(diplomat::capi::DisplayNamesOptionsV1 c_struct);
};


#endif // DisplayNamesOptionsV1_D_HPP
