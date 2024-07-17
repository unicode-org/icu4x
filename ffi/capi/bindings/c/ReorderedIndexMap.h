#ifndef ReorderedIndexMap_H
#define ReorderedIndexMap_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "ReorderedIndexMap.d.h"






DiplomatUsizeView ICU4XReorderedIndexMap_as_slice(const ReorderedIndexMap* self);

size_t ICU4XReorderedIndexMap_len(const ReorderedIndexMap* self);

bool ICU4XReorderedIndexMap_is_empty(const ReorderedIndexMap* self);

size_t ICU4XReorderedIndexMap_get(const ReorderedIndexMap* self, size_t index);


void ICU4XReorderedIndexMap_destroy(ReorderedIndexMap* self);





#endif // ReorderedIndexMap_H
