#ifndef ICU4XDisplayNamesOptionsV1_D_HPP
#define ICU4XDisplayNamesOptionsV1_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDisplayNamesFallback.d.hpp"
#include "ICU4XDisplayNamesStyle.d.hpp"
#include "ICU4XLanguageDisplay.d.hpp"

class ICU4XDisplayNamesFallback;
class ICU4XDisplayNamesStyle;
class ICU4XLanguageDisplay;


namespace capi {
    typedef struct ICU4XDisplayNamesOptionsV1 {
      ICU4XDisplayNamesStyle style;
      ICU4XDisplayNamesFallback fallback;
      ICU4XLanguageDisplay language_display;
    } ICU4XDisplayNamesOptionsV1;
}

struct ICU4XDisplayNamesOptionsV1 {
  ICU4XDisplayNamesStyle style;
  ICU4XDisplayNamesFallback fallback;
  ICU4XLanguageDisplay language_display;

  inline capi::ICU4XDisplayNamesOptionsV1 AsFFI() const;
  inline static ICU4XDisplayNamesOptionsV1 FromFFI(capi::ICU4XDisplayNamesOptionsV1 c_struct);
};


#endif // ICU4XDisplayNamesOptionsV1_D_HPP
