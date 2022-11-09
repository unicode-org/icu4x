#ifndef ICU4XBidiInfo_H
#define ICU4XBidiInfo_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XBidiInfo_type.h"
#include "ICU4XBidiParagraph_type.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

size_t ICU4XBidiInfo_paragraph_count(const ICU4XBidiInfo* self);

ICU4XBidiParagraph* ICU4XBidiInfo_paragraph_at(const ICU4XBidiInfo* self, size_t n);

size_t ICU4XBidiInfo_size(const ICU4XBidiInfo* self);

uint8_t ICU4XBidiInfo_level_at(const ICU4XBidiInfo* self, size_t pos);
void ICU4XBidiInfo_destroy(ICU4XBidiInfo* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus
#endif // ICU4XBidiInfo_H
