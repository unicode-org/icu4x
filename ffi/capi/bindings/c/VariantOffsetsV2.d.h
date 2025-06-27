#ifndef VariantOffsetsV2_D_H
#define VariantOffsetsV2_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "UtcOffset.d.h"




typedef struct VariantOffsetsV2 {
  UtcOffset* standard;
  UtcOffset* daylight;
  UtcOffset* sundown;
} VariantOffsetsV2;

typedef struct VariantOffsetsV2_option {union { VariantOffsetsV2 ok; }; bool is_ok; } VariantOffsetsV2_option;



#endif // VariantOffsetsV2_D_H
