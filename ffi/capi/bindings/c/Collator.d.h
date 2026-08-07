#ifndef Collator_D_H
#define Collator_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct Collator Collator;


typedef struct DiplomatCollatorView {
  const Collator** data;
  size_t len;
} DiplomatCollatorView;



#endif // Collator_D_H
