#ifndef VariantOffsetsCalculator_D_H
#define VariantOffsetsCalculator_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct VariantOffsetsCalculator VariantOffsetsCalculator;


typedef struct DiplomatVariantOffsetsCalculatorView {
  const VariantOffsetsCalculator** data;
  size_t len;
} DiplomatVariantOffsetsCalculatorView;



#endif // VariantOffsetsCalculator_D_H
