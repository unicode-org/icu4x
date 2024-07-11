#ifndef CodePointRangeIterator_H
#define CodePointRangeIterator_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "CodePointRangeIteratorResult.d.h"

#include "CodePointRangeIterator.d.h"






CodePointRangeIteratorResult ICU4XCodePointRangeIterator_next(CodePointRangeIterator* self);


void ICU4XCodePointRangeIterator_destroy(CodePointRangeIterator* self);





#endif // CodePointRangeIterator_H
