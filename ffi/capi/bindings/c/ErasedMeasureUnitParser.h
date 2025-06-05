#ifndef ErasedMeasureUnitParser_H
#define ErasedMeasureUnitParser_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"
#include "ErasedMeasureUnit.d.h"

#include "ErasedMeasureUnitParser.d.h"






ErasedMeasureUnitParser* icu4x_ErasedMeasureUnitParser_create_mv1(void);

typedef struct icu4x_ErasedMeasureUnitParser_create_with_provider_mv1_result {union {ErasedMeasureUnitParser* ok; DataError err;}; bool is_ok;} icu4x_ErasedMeasureUnitParser_create_with_provider_mv1_result;
icu4x_ErasedMeasureUnitParser_create_with_provider_mv1_result icu4x_ErasedMeasureUnitParser_create_with_provider_mv1(const DataProvider* provider);

ErasedMeasureUnit* icu4x_ErasedMeasureUnitParser_parse_mv1(const ErasedMeasureUnitParser* self, DiplomatStringView unit_id);

void icu4x_ErasedMeasureUnitParser_destroy_mv1(ErasedMeasureUnitParser* self);





#endif // ErasedMeasureUnitParser_H
