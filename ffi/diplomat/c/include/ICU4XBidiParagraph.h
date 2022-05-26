#ifndef ICU4XBidiParagraph_H
#define ICU4XBidiParagraph_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ICU4XBidiParagraph ICU4XBidiParagraph;
#include "ICU4XBidiDirection.h"
#include "diplomat_result_void_void.h"

ICU4XBidiDirection ICU4XBidiParagraph_direction(const ICU4XBidiParagraph* self);

size_t ICU4XBidiParagraph_size(const ICU4XBidiParagraph* self);

size_t ICU4XBidiParagraph_range_start(const ICU4XBidiParagraph* self);

size_t ICU4XBidiParagraph_range_end(const ICU4XBidiParagraph* self);

diplomat_result_void_void ICU4XBidiParagraph_reorder_line(const ICU4XBidiParagraph* self, size_t range_start, size_t range_end, DiplomatWriteable* out);

uint8_t ICU4XBidiParagraph_level_at(const ICU4XBidiParagraph* self, size_t pos);

bool ICU4XBidiParagraph_level_is_rtl(uint8_t level);

bool ICU4XBidiParagraph_level_is_ltr(uint8_t level);
void ICU4XBidiParagraph_destroy(ICU4XBidiParagraph* self);

#ifdef __cplusplus
}
#endif
#endif
