#ifndef Bidi_H
#define Bidi_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BidiInfo.d.h"
#include "DataError.d.h"
#include "DataProvider.d.h"
#include "ReorderedIndexMap.d.h"

#include "Bidi.d.h"






typedef struct ICU4XBidi_create_result {union {Bidi* ok; DataError err;}; bool is_ok;} ICU4XBidi_create_result;
ICU4XBidi_create_result ICU4XBidi_create(const DataProvider* provider);

BidiInfo* ICU4XBidi_for_text(const Bidi* self, const char* text_data, size_t text_len, uint8_t default_level);

ReorderedIndexMap* ICU4XBidi_reorder_visual(const Bidi* self, const uint8_t* levels_data, size_t levels_len);

bool ICU4XBidi_level_is_rtl(uint8_t level);

bool ICU4XBidi_level_is_ltr(uint8_t level);

uint8_t ICU4XBidi_level_rtl();

uint8_t ICU4XBidi_level_ltr();


void ICU4XBidi_destroy(Bidi* self);





#endif // Bidi_H
