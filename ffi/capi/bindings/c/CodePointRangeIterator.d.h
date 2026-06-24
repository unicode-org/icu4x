#ifndef CodePointRangeIterator_D_H
#define CodePointRangeIterator_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct CodePointRangeIterator CodePointRangeIterator;


typedef struct DiplomatCodePointRangeIteratorView {
  const CodePointRangeIterator** data;
  size_t len;
} DiplomatCodePointRangeIteratorView;



#endif // CodePointRangeIterator_D_H
