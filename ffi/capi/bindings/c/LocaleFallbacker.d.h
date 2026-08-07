#ifndef LocaleFallbacker_D_H
#define LocaleFallbacker_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct LocaleFallbacker LocaleFallbacker;


typedef struct DiplomatLocaleFallbackerView {
  const LocaleFallbacker** data;
  size_t len;
} DiplomatLocaleFallbackerView;



#endif // LocaleFallbacker_D_H
