#ifndef CaseMapper_D_H
#define CaseMapper_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct CaseMapper CaseMapper;


typedef struct DiplomatCaseMapperView {
  const CaseMapper** data;
  size_t len;
} DiplomatCaseMapperView;



#endif // CaseMapper_D_H
