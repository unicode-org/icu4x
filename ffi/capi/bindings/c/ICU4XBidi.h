#ifndef ICU4XBidi_H
#define ICU4XBidi_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XBidiInfo.d.h"
#include "ICU4XDataError.d.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XReorderedIndexMap.d.h"

#include "ICU4XBidi.d.h"






typedef struct ICU4XBidi_create_result {union {ICU4XBidi* ok; ICU4XDataError err;}; bool is_ok;} ICU4XBidi_create_result;
ICU4XBidi_create_result ICU4XBidi_create(const ICU4XDataProvider* provider);

ICU4XBidiInfo* ICU4XBidi_for_text(const ICU4XBidi* self, const char* text_data, size_t text_len, uint8_t default_level);

ICU4XReorderedIndexMap* ICU4XBidi_reorder_visual(const ICU4XBidi* self, const uint8_t* levels_data, size_t levels_len);

bool ICU4XBidi_level_is_rtl(uint8_t level);

bool ICU4XBidi_level_is_ltr(uint8_t level);

uint8_t ICU4XBidi_level_rtl();

uint8_t ICU4XBidi_level_ltr();


void ICU4XBidi_destroy(ICU4XBidi* self);





#endif // ICU4XBidi_H
