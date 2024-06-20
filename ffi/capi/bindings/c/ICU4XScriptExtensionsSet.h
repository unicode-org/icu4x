#ifndef ICU4XScriptExtensionsSet_H
#define ICU4XScriptExtensionsSet_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "ICU4XScriptExtensionsSet.d.h"






bool ICU4XScriptExtensionsSet_contains(const ICU4XScriptExtensionsSet* self, uint16_t script);

size_t ICU4XScriptExtensionsSet_count(const ICU4XScriptExtensionsSet* self);

struct ICU4XScriptExtensionsSet_script_at_result {union {uint16_t ok; }; bool is_ok;};
struct ICU4XScriptExtensionsSet_script_at_result ICU4XScriptExtensionsSet_script_at(const ICU4XScriptExtensionsSet* self, size_t index);


void ICU4XScriptExtensionsSet_destroy(ICU4XScriptExtensionsSet* self);





#endif // ICU4XScriptExtensionsSet_H
