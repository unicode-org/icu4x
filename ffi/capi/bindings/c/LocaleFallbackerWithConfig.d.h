#ifndef LocaleFallbackerWithConfig_D_H
#define LocaleFallbackerWithConfig_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct LocaleFallbackerWithConfig LocaleFallbackerWithConfig;


typedef struct DiplomatLocaleFallbackerWithConfigView {
  const LocaleFallbackerWithConfig** data;
  size_t len;
} DiplomatLocaleFallbackerWithConfigView;



#endif // LocaleFallbackerWithConfig_D_H
