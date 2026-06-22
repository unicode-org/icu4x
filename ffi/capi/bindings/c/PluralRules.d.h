#ifndef PluralRules_D_H
#define PluralRules_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct PluralRules PluralRules;


typedef struct DiplomatPluralRulesView {
  const PluralRules** data;
  size_t len;
} DiplomatPluralRulesView;



#endif // PluralRules_D_H
