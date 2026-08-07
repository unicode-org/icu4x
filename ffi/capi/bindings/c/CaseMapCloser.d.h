#ifndef CaseMapCloser_D_H
#define CaseMapCloser_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct CaseMapCloser CaseMapCloser;


typedef struct DiplomatCaseMapCloserView {
  const CaseMapCloser** data;
  size_t len;
} DiplomatCaseMapCloserView;



#endif // CaseMapCloser_D_H
