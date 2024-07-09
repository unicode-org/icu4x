#ifndef ICU4XDisplayNamesOptionsV1_D_H
#define ICU4XDisplayNamesOptionsV1_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XDisplayNamesFallback.d.h"
#include "ICU4XDisplayNamesStyle.d.h"
#include "ICU4XLanguageDisplay.d.h"




typedef struct ICU4XDisplayNamesOptionsV1 {
  ICU4XDisplayNamesStyle style;
  ICU4XDisplayNamesFallback fallback;
  ICU4XLanguageDisplay language_display;
} ICU4XDisplayNamesOptionsV1;





#endif // ICU4XDisplayNamesOptionsV1_D_H
