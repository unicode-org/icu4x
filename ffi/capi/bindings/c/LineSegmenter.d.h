#ifndef LineSegmenter_D_H
#define LineSegmenter_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct LineSegmenter LineSegmenter;


typedef struct DiplomatLineSegmenterView {
  const LineSegmenter** data;
  size_t len;
} DiplomatLineSegmenterView;



#endif // LineSegmenter_D_H
