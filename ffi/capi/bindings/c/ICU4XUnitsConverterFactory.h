#ifndef ICU4XUnitsConverterFactory_H
#define ICU4XUnitsConverterFactory_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XDataError.d.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XMeasureUnit.d.h"
#include "ICU4XMeasureUnitParser.d.h"
#include "ICU4XUnitsConverter.d.h"

#include "ICU4XUnitsConverterFactory.d.h"






typedef struct ICU4XUnitsConverterFactory_create_result {union {ICU4XUnitsConverterFactory* ok; ICU4XDataError err;}; bool is_ok;} ICU4XUnitsConverterFactory_create_result;
ICU4XUnitsConverterFactory_create_result ICU4XUnitsConverterFactory_create(const ICU4XDataProvider* provider);

ICU4XUnitsConverter* ICU4XUnitsConverterFactory_converter(const ICU4XUnitsConverterFactory* self, const ICU4XMeasureUnit* from, const ICU4XMeasureUnit* to);

ICU4XMeasureUnitParser* ICU4XUnitsConverterFactory_parser(const ICU4XUnitsConverterFactory* self);


void ICU4XUnitsConverterFactory_destroy(ICU4XUnitsConverterFactory* self);





#endif // ICU4XUnitsConverterFactory_H
