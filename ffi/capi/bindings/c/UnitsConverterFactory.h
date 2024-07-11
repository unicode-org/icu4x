#ifndef UnitsConverterFactory_H
#define UnitsConverterFactory_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"
#include "MeasureUnit.d.h"
#include "MeasureUnitParser.d.h"
#include "UnitsConverter.d.h"

#include "UnitsConverterFactory.d.h"






typedef struct ICU4XUnitsConverterFactory_create_result {union {UnitsConverterFactory* ok; DataError err;}; bool is_ok;} ICU4XUnitsConverterFactory_create_result;
ICU4XUnitsConverterFactory_create_result ICU4XUnitsConverterFactory_create(const DataProvider* provider);

UnitsConverter* ICU4XUnitsConverterFactory_converter(const UnitsConverterFactory* self, const MeasureUnit* from, const MeasureUnit* to);

MeasureUnitParser* ICU4XUnitsConverterFactory_parser(const UnitsConverterFactory* self);


void ICU4XUnitsConverterFactory_destroy(UnitsConverterFactory* self);





#endif // UnitsConverterFactory_H
