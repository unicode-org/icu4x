#ifndef Bidi_D_H
#define Bidi_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct Bidi Bidi;


typedef struct DiplomatBidiView {
  const Bidi** data;
  size_t len;
} DiplomatBidiView;



#endif // Bidi_D_H
