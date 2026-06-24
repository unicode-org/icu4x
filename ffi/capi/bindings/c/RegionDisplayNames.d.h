#ifndef RegionDisplayNames_D_H
#define RegionDisplayNames_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct RegionDisplayNames RegionDisplayNames;


typedef struct DiplomatRegionDisplayNamesView {
  const RegionDisplayNames** data;
  size_t len;
} DiplomatRegionDisplayNamesView;



#endif // RegionDisplayNames_D_H
