#ifndef PartSpan_D_H
#define PartSpan_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "PartKind.d.h"




typedef struct PartSpan {
  size_t start;
  size_t end;
  PartKind kind;
} PartSpan;

typedef struct PartSpan_option {union { PartSpan ok; }; bool is_ok; } PartSpan_option;
typedef struct DiplomatPartSpanView {
  const PartSpan* data;
  size_t len;
} DiplomatPartSpanView;

typedef struct DiplomatPartSpanViewMut {
  PartSpan* data;
  size_t len;
} DiplomatPartSpanViewMut;




#endif // PartSpan_D_H
