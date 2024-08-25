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

typedef struct DisplayNamesOptionsV1_option {union { DisplayNamesOptionsV1 ok; }; bool is_ok; } DisplayNamesOptionsV1_option;



#endif // DisplayNamesOptionsV1_D_H
