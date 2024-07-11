#ifndef CodePointMapData16_H
#define CodePointMapData16_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "CodePointRangeIterator.d.h"
#include "CodePointSetData.d.h"
#include "DataError.d.h"
#include "DataProvider.d.h"

#include "CodePointMapData16.d.h"






uint16_t ICU4XCodePointMapData16_get(const CodePointMapData16* self, char32_t cp);

uint16_t ICU4XCodePointMapData16_get32(const CodePointMapData16* self, uint32_t cp);

CodePointRangeIterator* ICU4XCodePointMapData16_iter_ranges_for_value(const CodePointMapData16* self, uint16_t value);

CodePointRangeIterator* ICU4XCodePointMapData16_iter_ranges_for_value_complemented(const CodePointMapData16* self, uint16_t value);

CodePointSetData* ICU4XCodePointMapData16_get_set_for_value(const CodePointMapData16* self, uint16_t value);

typedef struct ICU4XCodePointMapData16_load_script_result {union {CodePointMapData16* ok; DataError err;}; bool is_ok;} ICU4XCodePointMapData16_load_script_result;
ICU4XCodePointMapData16_load_script_result ICU4XCodePointMapData16_load_script(const DataProvider* provider);


void ICU4XCodePointMapData16_destroy(CodePointMapData16* self);





#endif // CodePointMapData16_H
