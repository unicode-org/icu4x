#ifndef ICU4XCodePointMapData16_H
#define ICU4XCodePointMapData16_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XCodePointRangeIterator.d.h"
#include "ICU4XCodePointRangeIterator.h"
#include "ICU4XCodePointSetData.d.h"
#include "ICU4XCodePointSetData.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XDataProvider.h"
#include "diplomat_result_box_ICU4XCodePointMapData16_ICU4XDataError.d.h"

#include "ICU4XCodePointMapData16.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


uint16_t ICU4XCodePointMapData16_get(const ICU4XCodePointMapData16* self, char32_t cp);

uint16_t ICU4XCodePointMapData16_get32(const ICU4XCodePointMapData16* self, uint32_t cp);

ICU4XCodePointRangeIterator* ICU4XCodePointMapData16_iter_ranges_for_value(const ICU4XCodePointMapData16* self, uint16_t value);

ICU4XCodePointRangeIterator* ICU4XCodePointMapData16_iter_ranges_for_value_complemented(const ICU4XCodePointMapData16* self, uint16_t value);

ICU4XCodePointSetData* ICU4XCodePointMapData16_get_set_for_value(const ICU4XCodePointMapData16* self, uint16_t value);

diplomat_result_box_ICU4XCodePointMapData16_ICU4XDataError ICU4XCodePointMapData16_load_script(const ICU4XDataProvider* provider);

void ICU4XCodePointMapData16_destroy(ICU4XCodePointMapData16* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XCodePointMapData16_H
