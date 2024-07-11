#ifndef MeasureUnitParser_H
#define MeasureUnitParser_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "MeasureUnit.d.h"

#include "MeasureUnitParser.d.h"






MeasureUnit* ICU4XMeasureUnitParser_parse(const MeasureUnitParser* self, const char* unit_id_data, size_t unit_id_len);


void ICU4XMeasureUnitParser_destroy(MeasureUnitParser* self);





#endif // MeasureUnitParser_H
