#ifndef BidiParagraph_H
#define BidiParagraph_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BidiDirection.d.h"

#include "BidiParagraph.d.h"






bool ICU4XBidiParagraph_set_paragraph_in_text(BidiParagraph* self, size_t n);

BidiDirection ICU4XBidiParagraph_direction(const BidiParagraph* self);

size_t ICU4XBidiParagraph_size(const BidiParagraph* self);

size_t ICU4XBidiParagraph_range_start(const BidiParagraph* self);

size_t ICU4XBidiParagraph_range_end(const BidiParagraph* self);

typedef struct ICU4XBidiParagraph_reorder_line_result { bool is_ok;} ICU4XBidiParagraph_reorder_line_result;
ICU4XBidiParagraph_reorder_line_result ICU4XBidiParagraph_reorder_line(const BidiParagraph* self, size_t range_start, size_t range_end, DiplomatWrite* write);

uint8_t ICU4XBidiParagraph_level_at(const BidiParagraph* self, size_t pos);


void ICU4XBidiParagraph_destroy(BidiParagraph* self);





#endif // BidiParagraph_H
