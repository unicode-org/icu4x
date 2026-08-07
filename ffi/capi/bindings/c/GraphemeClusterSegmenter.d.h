#ifndef GraphemeClusterSegmenter_D_H
#define GraphemeClusterSegmenter_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct GraphemeClusterSegmenter GraphemeClusterSegmenter;


typedef struct DiplomatGraphemeClusterSegmenterView {
  const GraphemeClusterSegmenter** data;
  size_t len;
} DiplomatGraphemeClusterSegmenterView;



#endif // GraphemeClusterSegmenter_D_H
