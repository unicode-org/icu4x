#ifndef ExemplarCharacters_D_H
#define ExemplarCharacters_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct ExemplarCharacters ExemplarCharacters;


typedef struct DiplomatExemplarCharactersView {
  const ExemplarCharacters** data;
  size_t len;
} DiplomatExemplarCharactersView;



#endif // ExemplarCharacters_D_H
