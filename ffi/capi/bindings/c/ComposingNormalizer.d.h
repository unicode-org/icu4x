#ifndef ComposingNormalizer_D_H
#define ComposingNormalizer_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct ComposingNormalizer ComposingNormalizer;


typedef struct DiplomatComposingNormalizerView {
  const ComposingNormalizer** data;
  size_t len;
} DiplomatComposingNormalizerView;



#endif // ComposingNormalizer_D_H
