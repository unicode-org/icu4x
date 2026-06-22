#ifndef WindowsParser_D_H
#define WindowsParser_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct WindowsParser WindowsParser;


typedef struct DiplomatWindowsParserView {
  const WindowsParser** data;
  size_t len;
} DiplomatWindowsParserView;



#endif // WindowsParser_D_H
