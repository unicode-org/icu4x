#ifndef ICU4XBidiParagraph_H
#define ICU4XBidiParagraph_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XBidiDirection.d.h"
#include "ICU4XBidiDirection.h"
#include "diplomat_result_void_void.d.h"

#include "ICU4XBidiParagraph.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


bool ICU4XBidiParagraph_set_paragraph_in_text(ICU4XBidiParagraph* self, size_t n);

ICU4XBidiDirection ICU4XBidiParagraph_direction(const ICU4XBidiParagraph* self);

size_t ICU4XBidiParagraph_size(const ICU4XBidiParagraph* self);

size_t ICU4XBidiParagraph_range_start(const ICU4XBidiParagraph* self);

size_t ICU4XBidiParagraph_range_end(const ICU4XBidiParagraph* self);

diplomat_result_void_void ICU4XBidiParagraph_reorder_line(const ICU4XBidiParagraph* self, size_t range_start, size_t range_end, DiplomatWrite* write);

uint8_t ICU4XBidiParagraph_level_at(const ICU4XBidiParagraph* self, size_t pos);

void ICU4XBidiParagraph_destroy(ICU4XBidiParagraph* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XBidiParagraph_H
