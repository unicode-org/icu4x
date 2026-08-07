#ifndef ScriptWithExtensions_D_H
#define ScriptWithExtensions_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct ScriptWithExtensions ScriptWithExtensions;


typedef struct DiplomatScriptWithExtensionsView {
  const ScriptWithExtensions** data;
  size_t len;
} DiplomatScriptWithExtensionsView;



#endif // ScriptWithExtensions_D_H
