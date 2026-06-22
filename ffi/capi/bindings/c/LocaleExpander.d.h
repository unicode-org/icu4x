#ifndef LocaleExpander_D_H
#define LocaleExpander_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct LocaleExpander LocaleExpander;


typedef struct DiplomatLocaleExpanderView {
  const LocaleExpander** data;
  size_t len;
} DiplomatLocaleExpanderView;



#endif // LocaleExpander_D_H
