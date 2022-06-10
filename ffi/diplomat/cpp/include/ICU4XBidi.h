#ifndef ICU4XBidi_H
#define ICU4XBidi_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ICU4XBidi ICU4XBidi;
#include "ICU4XDataProvider.h"
#include "diplomat_result_box_ICU4XBidi_void.h"
#include "ICU4XBidiInfo.h"

diplomat_result_box_ICU4XBidi_void ICU4XBidi_try_new(const ICU4XDataProvider* provider);

ICU4XBidiInfo* ICU4XBidi_for_text(const ICU4XBidi* self, const char* text_data, size_t text_len, uint8_t default_level);

bool ICU4XBidi_level_is_rtl(uint8_t level);

bool ICU4XBidi_level_is_ltr(uint8_t level);

uint8_t ICU4XBidi_level_rtl();

uint8_t ICU4XBidi_level_ltr();
void ICU4XBidi_destroy(ICU4XBidi* self);

#ifdef __cplusplus
}
#endif
#endif
