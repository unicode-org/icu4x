#ifndef ICU4XScriptWithExtensionsBorrowed_H
#define ICU4XScriptWithExtensionsBorrowed_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XCodePointSetData.d.h"
#include "ICU4XScriptExtensionsSet.d.h"

#include "ICU4XScriptWithExtensionsBorrowed.d.h"






uint16_t ICU4XScriptWithExtensionsBorrowed_get_script_val(const ICU4XScriptWithExtensionsBorrowed* self, uint32_t code_point);

ICU4XScriptExtensionsSet* ICU4XScriptWithExtensionsBorrowed_get_script_extensions_val(const ICU4XScriptWithExtensionsBorrowed* self, uint32_t code_point);

bool ICU4XScriptWithExtensionsBorrowed_has_script(const ICU4XScriptWithExtensionsBorrowed* self, uint32_t code_point, uint16_t script);

ICU4XCodePointSetData* ICU4XScriptWithExtensionsBorrowed_get_script_extensions_set(const ICU4XScriptWithExtensionsBorrowed* self, uint16_t script);


void ICU4XScriptWithExtensionsBorrowed_destroy(ICU4XScriptWithExtensionsBorrowed* self);





#endif // ICU4XScriptWithExtensionsBorrowed_H
