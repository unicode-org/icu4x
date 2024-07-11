#ifndef DisplayNamesOptionsV1_D_H
#define DisplayNamesOptionsV1_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DisplayNamesFallback.d.h"
#include "DisplayNamesStyle.d.h"
#include "LanguageDisplay.d.h"




typedef struct DisplayNamesOptionsV1 {
  DisplayNamesStyle style;
  DisplayNamesFallback fallback;
  LanguageDisplay language_display;
} DisplayNamesOptionsV1;





#endif // DisplayNamesOptionsV1_D_H
