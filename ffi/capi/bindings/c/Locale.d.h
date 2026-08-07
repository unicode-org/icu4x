#ifndef Locale_D_H
#define Locale_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct Locale Locale;


typedef struct DiplomatLocaleView {
  const Locale** data;
  size_t len;
} DiplomatLocaleView;



#endif // Locale_D_H
