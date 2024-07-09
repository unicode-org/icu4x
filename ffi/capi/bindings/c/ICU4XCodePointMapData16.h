#ifndef ICU4XCodePointMapData16_H
#define ICU4XCodePointMapData16_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XCodePointRangeIterator.d.h"
#include "ICU4XCodePointSetData.d.h"
#include "ICU4XDataError.d.h"
#include "ICU4XDataProvider.d.h"

#include "ICU4XCodePointMapData16.d.h"






uint16_t ICU4XCodePointMapData16_get(const ICU4XCodePointMapData16* self, char32_t cp);

uint16_t ICU4XCodePointMapData16_get32(const ICU4XCodePointMapData16* self, uint32_t cp);

ICU4XCodePointRangeIterator* ICU4XCodePointMapData16_iter_ranges_for_value(const ICU4XCodePointMapData16* self, uint16_t value);

ICU4XCodePointRangeIterator* ICU4XCodePointMapData16_iter_ranges_for_value_complemented(const ICU4XCodePointMapData16* self, uint16_t value);

ICU4XCodePointSetData* ICU4XCodePointMapData16_get_set_for_value(const ICU4XCodePointMapData16* self, uint16_t value);

typedef struct ICU4XCodePointMapData16_load_script_result {union {ICU4XCodePointMapData16* ok; ICU4XDataError err;}; bool is_ok;} ICU4XCodePointMapData16_load_script_result;
ICU4XCodePointMapData16_load_script_result ICU4XCodePointMapData16_load_script(const ICU4XDataProvider* provider);


void ICU4XCodePointMapData16_destroy(ICU4XCodePointMapData16* self);





#endif // ICU4XCodePointMapData16_H
