#ifndef ScriptWithExtensionsBorrowed_H
#define ScriptWithExtensionsBorrowed_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "CodePointSetData.d.h"
#include "ScriptExtensionsSet.d.h"

#include "ScriptWithExtensionsBorrowed.d.h"






uint16_t ICU4XScriptWithExtensionsBorrowed_get_script_val(const ScriptWithExtensionsBorrowed* self, uint32_t code_point);

ScriptExtensionsSet* ICU4XScriptWithExtensionsBorrowed_get_script_extensions_val(const ScriptWithExtensionsBorrowed* self, uint32_t code_point);

bool ICU4XScriptWithExtensionsBorrowed_has_script(const ScriptWithExtensionsBorrowed* self, uint32_t code_point, uint16_t script);

CodePointSetData* ICU4XScriptWithExtensionsBorrowed_get_script_extensions_set(const ScriptWithExtensionsBorrowed* self, uint16_t script);


void ICU4XScriptWithExtensionsBorrowed_destroy(ScriptWithExtensionsBorrowed* self);





#endif // ScriptWithExtensionsBorrowed_H
