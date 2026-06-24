#ifndef CanonicalCombiningClassMap_D_H
#define CanonicalCombiningClassMap_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct CanonicalCombiningClassMap CanonicalCombiningClassMap;


typedef struct DiplomatCanonicalCombiningClassMapView {
  const CanonicalCombiningClassMap** data;
  size_t len;
} DiplomatCanonicalCombiningClassMapView;



#endif // CanonicalCombiningClassMap_D_H
