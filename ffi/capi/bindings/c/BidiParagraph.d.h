#ifndef BidiParagraph_D_H
#define BidiParagraph_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct BidiParagraph BidiParagraph;


typedef struct DiplomatBidiParagraphView {
  const BidiParagraph** data;
  size_t len;
} DiplomatBidiParagraphView;



#endif // BidiParagraph_D_H
