#ifndef LocaleDisplayNamesFormatter_D_H
#define LocaleDisplayNamesFormatter_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct LocaleDisplayNamesFormatter LocaleDisplayNamesFormatter;


typedef struct DiplomatLocaleDisplayNamesFormatterView {
  const LocaleDisplayNamesFormatter** data;
  size_t len;
} DiplomatLocaleDisplayNamesFormatterView;



#endif // LocaleDisplayNamesFormatter_D_H
