#ifndef CodePointSetData_D_H
#define CodePointSetData_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct CodePointSetData CodePointSetData;


typedef struct DiplomatCodePointSetDataView {
  const CodePointSetData** data;
  size_t len;
} DiplomatCodePointSetDataView;



#endif // CodePointSetData_D_H
