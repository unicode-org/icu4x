#ifndef BidiInfo_H
#define BidiInfo_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BidiParagraph.d.h"

#include "BidiInfo.d.h"






size_t ICU4XBidiInfo_paragraph_count(const BidiInfo* self);

BidiParagraph* ICU4XBidiInfo_paragraph_at(const BidiInfo* self, size_t n);

size_t ICU4XBidiInfo_size(const BidiInfo* self);

uint8_t ICU4XBidiInfo_level_at(const BidiInfo* self, size_t pos);


void ICU4XBidiInfo_destroy(BidiInfo* self);





#endif // BidiInfo_H
