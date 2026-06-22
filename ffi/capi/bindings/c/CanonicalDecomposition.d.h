#ifndef CanonicalDecomposition_D_H
#define CanonicalDecomposition_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct CanonicalDecomposition CanonicalDecomposition;


typedef struct DiplomatCanonicalDecompositionView {
  const CanonicalDecomposition** data;
  size_t len;
} DiplomatCanonicalDecompositionView;



#endif // CanonicalDecomposition_D_H
