#ifndef ScriptExtensionsSet_H
#define ScriptExtensionsSet_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "ScriptExtensionsSet.d.h"






bool ICU4XScriptExtensionsSet_contains(const ScriptExtensionsSet* self, uint16_t script);

size_t ICU4XScriptExtensionsSet_count(const ScriptExtensionsSet* self);

typedef struct ICU4XScriptExtensionsSet_script_at_result {union {uint16_t ok; }; bool is_ok;} ICU4XScriptExtensionsSet_script_at_result;
ICU4XScriptExtensionsSet_script_at_result ICU4XScriptExtensionsSet_script_at(const ScriptExtensionsSet* self, size_t index);


void ICU4XScriptExtensionsSet_destroy(ScriptExtensionsSet* self);





#endif // ScriptExtensionsSet_H
