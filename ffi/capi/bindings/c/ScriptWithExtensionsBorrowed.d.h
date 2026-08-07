#ifndef ScriptWithExtensionsBorrowed_D_H
#define ScriptWithExtensionsBorrowed_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct ScriptWithExtensionsBorrowed ScriptWithExtensionsBorrowed;


typedef struct DiplomatScriptWithExtensionsBorrowedView {
  const ScriptWithExtensionsBorrowed** data;
  size_t len;
} DiplomatScriptWithExtensionsBorrowedView;



#endif // ScriptWithExtensionsBorrowed_D_H
