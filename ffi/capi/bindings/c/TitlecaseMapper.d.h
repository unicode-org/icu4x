#ifndef TitlecaseMapper_D_H
#define TitlecaseMapper_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct TitlecaseMapper TitlecaseMapper;


typedef struct DiplomatTitlecaseMapperView {
  const TitlecaseMapper** data;
  size_t len;
} DiplomatTitlecaseMapperView;



#endif // TitlecaseMapper_D_H
