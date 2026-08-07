#ifndef CanonicalComposition_D_H
#define CanonicalComposition_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct CanonicalComposition CanonicalComposition;


typedef struct DiplomatCanonicalCompositionView {
  const CanonicalComposition** data;
  size_t len;
} DiplomatCanonicalCompositionView;



#endif // CanonicalComposition_D_H
