#ifndef LocaleDirectionality_D_H
#define LocaleDirectionality_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct LocaleDirectionality LocaleDirectionality;


typedef struct DiplomatLocaleDirectionalityView {
  const LocaleDirectionality** data;
  size_t len;
} DiplomatLocaleDirectionalityView;



#endif // LocaleDirectionality_D_H
