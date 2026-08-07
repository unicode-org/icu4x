#ifndef BidiInfo_D_H
#define BidiInfo_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct BidiInfo BidiInfo;


typedef struct DiplomatBidiInfoView {
  const BidiInfo** data;
  size_t len;
} DiplomatBidiInfoView;



#endif // BidiInfo_D_H
