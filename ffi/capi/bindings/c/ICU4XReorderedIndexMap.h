#ifndef ICU4XReorderedIndexMap_H
#define ICU4XReorderedIndexMap_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XReorderedIndexMap.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


DiplomatUsizeView ICU4XReorderedIndexMap_as_slice(const ICU4XReorderedIndexMap* self);

size_t ICU4XReorderedIndexMap_len(const ICU4XReorderedIndexMap* self);

bool ICU4XReorderedIndexMap_is_empty(const ICU4XReorderedIndexMap* self);

size_t ICU4XReorderedIndexMap_get(const ICU4XReorderedIndexMap* self, size_t index);

void ICU4XReorderedIndexMap_destroy(ICU4XReorderedIndexMap* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XReorderedIndexMap_H
