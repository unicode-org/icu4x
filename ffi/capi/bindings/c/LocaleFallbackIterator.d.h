#ifndef LocaleFallbackIterator_D_H
#define LocaleFallbackIterator_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct LocaleFallbackIterator LocaleFallbackIterator;


typedef struct DiplomatLocaleFallbackIteratorView {
  const LocaleFallbackIterator** data;
  size_t len;
} DiplomatLocaleFallbackIteratorView;



#endif // LocaleFallbackIterator_D_H
