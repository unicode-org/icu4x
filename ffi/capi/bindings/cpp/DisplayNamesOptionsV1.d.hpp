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


namespace capi {
    typedef struct DisplayNamesOptionsV1 {
      DisplayNamesStyle style;
      DisplayNamesFallback fallback;
      LanguageDisplay language_display;
    } DisplayNamesOptionsV1;
}

struct DisplayNamesOptionsV1 {
  DisplayNamesStyle style;
  DisplayNamesFallback fallback;
  LanguageDisplay language_display;

  inline capi::DisplayNamesOptionsV1 AsFFI() const;
  inline static DisplayNamesOptionsV1 FromFFI(capi::DisplayNamesOptionsV1 c_struct);
};


#endif // DisplayNamesOptionsV1_D_HPP
