#ifndef ICU4XMeasureUnitParser_H
#define ICU4XMeasureUnitParser_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XMeasureUnit.d.h"

#include "ICU4XMeasureUnitParser.d.h"






ICU4XMeasureUnit* ICU4XMeasureUnitParser_parse(const ICU4XMeasureUnitParser* self, const char* unit_id_data, size_t unit_id_len);


void ICU4XMeasureUnitParser_destroy(ICU4XMeasureUnitParser* self);





#endif // ICU4XMeasureUnitParser_H
