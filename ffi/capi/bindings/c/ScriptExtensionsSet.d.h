#ifndef ScriptExtensionsSet_D_H
#define ScriptExtensionsSet_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct ScriptExtensionsSet ScriptExtensionsSet;


typedef struct DiplomatScriptExtensionsSetView {
  const ScriptExtensionsSet** data;
  size_t len;
} DiplomatScriptExtensionsSetView;



#endif // ScriptExtensionsSet_D_H
