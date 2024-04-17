#ifndef ICU4XUnitsConverterFactory_H
#define ICU4XUnitsConverterFactory_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct ICU4XUnitsConverterFactory ICU4XUnitsConverterFactory;
#ifdef __cplusplus
} // namespace capi
#endif
#include "ICU4XDataProvider.h"
#include "diplomat_result_box_ICU4XUnitsConverterFactory_ICU4XError.h"
#include "ICU4XMeasureUnit.h"
#include "ICU4XUnitsConverter.h"
#include "diplomat_result_box_ICU4XMeasureUnit_ICU4XError.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

diplomat_result_box_ICU4XUnitsConverterFactory_ICU4XError ICU4XUnitsConverterFactory_create(const ICU4XDataProvider* provider);

ICU4XUnitsConverter* ICU4XUnitsConverterFactory_converter(const ICU4XUnitsConverterFactory* self, const ICU4XMeasureUnit* input_unit, const ICU4XMeasureUnit* output_unit);

diplomat_result_box_ICU4XMeasureUnit_ICU4XError ICU4XUnitsConverterFactory_parse(const ICU4XUnitsConverterFactory* self, const char* unit_id_data, size_t unit_id_len);
void ICU4XUnitsConverterFactory_destroy(ICU4XUnitsConverterFactory* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
