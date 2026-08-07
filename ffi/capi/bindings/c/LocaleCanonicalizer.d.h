#ifndef LocaleCanonicalizer_D_H
#define LocaleCanonicalizer_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct LocaleCanonicalizer LocaleCanonicalizer;


typedef struct DiplomatLocaleCanonicalizerView {
  const LocaleCanonicalizer** data;
  size_t len;
} DiplomatLocaleCanonicalizerView;



#endif // LocaleCanonicalizer_D_H
