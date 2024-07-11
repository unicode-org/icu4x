#ifndef ScriptWithExtensions_H
#define ScriptWithExtensions_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "CodePointRangeIterator.d.h"
#include "DataError.d.h"
#include "DataProvider.d.h"
#include "ScriptWithExtensionsBorrowed.d.h"

#include "ScriptWithExtensions.d.h"






typedef struct ICU4XScriptWithExtensions_create_result {union {ScriptWithExtensions* ok; DataError err;}; bool is_ok;} ICU4XScriptWithExtensions_create_result;
ICU4XScriptWithExtensions_create_result ICU4XScriptWithExtensions_create(const DataProvider* provider);

uint16_t ICU4XScriptWithExtensions_get_script_val(const ScriptWithExtensions* self, uint32_t code_point);

bool ICU4XScriptWithExtensions_has_script(const ScriptWithExtensions* self, uint32_t code_point, uint16_t script);

ScriptWithExtensionsBorrowed* ICU4XScriptWithExtensions_as_borrowed(const ScriptWithExtensions* self);

CodePointRangeIterator* ICU4XScriptWithExtensions_iter_ranges_for_script(const ScriptWithExtensions* self, uint16_t script);


void ICU4XScriptWithExtensions_destroy(ScriptWithExtensions* self);





#endif // ScriptWithExtensions_H
