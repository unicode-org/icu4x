#ifndef SentenceSegmenter_D_H
#define SentenceSegmenter_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct SentenceSegmenter SentenceSegmenter;


typedef struct DiplomatSentenceSegmenterView {
  const SentenceSegmenter** data;
  size_t len;
} DiplomatSentenceSegmenterView;



#endif // SentenceSegmenter_D_H
