#ifndef CollationSortKey_H
#define CollationSortKey_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "CollationSortKey.d.h"






DiplomatU8View icu4x_CollationSortKey_as_bytes_mv1(const CollationSortKey* self);

void icu4x_CollationSortKey_destroy_mv1(CollationSortKey* self);





#endif // CollationSortKey_H
