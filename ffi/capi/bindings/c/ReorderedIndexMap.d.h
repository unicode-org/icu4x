#ifndef ReorderedIndexMap_D_H
#define ReorderedIndexMap_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct ReorderedIndexMap ReorderedIndexMap;


typedef struct DiplomatReorderedIndexMapView {
  const ReorderedIndexMap** data;
  size_t len;
} DiplomatReorderedIndexMapView;



#endif // ReorderedIndexMap_D_H
