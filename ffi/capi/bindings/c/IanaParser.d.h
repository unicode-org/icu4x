#ifndef IanaParser_D_H
#define IanaParser_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct IanaParser IanaParser;


typedef struct DiplomatIanaParserView {
  const IanaParser** data;
  size_t len;
} DiplomatIanaParserView;



#endif // IanaParser_D_H
