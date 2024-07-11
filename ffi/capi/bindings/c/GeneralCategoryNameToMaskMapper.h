#ifndef GeneralCategoryNameToMaskMapper_H
#define GeneralCategoryNameToMaskMapper_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"

#include "GeneralCategoryNameToMaskMapper.d.h"






uint32_t ICU4XGeneralCategoryNameToMaskMapper_get_strict(const GeneralCategoryNameToMaskMapper* self, const char* name_data, size_t name_len);

uint32_t ICU4XGeneralCategoryNameToMaskMapper_get_loose(const GeneralCategoryNameToMaskMapper* self, const char* name_data, size_t name_len);

typedef struct ICU4XGeneralCategoryNameToMaskMapper_load_result {union {GeneralCategoryNameToMaskMapper* ok; DataError err;}; bool is_ok;} ICU4XGeneralCategoryNameToMaskMapper_load_result;
ICU4XGeneralCategoryNameToMaskMapper_load_result ICU4XGeneralCategoryNameToMaskMapper_load(const DataProvider* provider);


void ICU4XGeneralCategoryNameToMaskMapper_destroy(GeneralCategoryNameToMaskMapper* self);





#endif // GeneralCategoryNameToMaskMapper_H
