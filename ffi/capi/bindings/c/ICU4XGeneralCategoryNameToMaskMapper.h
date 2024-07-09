#ifndef ICU4XGeneralCategoryNameToMaskMapper_H
#define ICU4XGeneralCategoryNameToMaskMapper_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XDataError.d.h"
#include "ICU4XDataProvider.d.h"

#include "ICU4XGeneralCategoryNameToMaskMapper.d.h"






uint32_t ICU4XGeneralCategoryNameToMaskMapper_get_strict(const ICU4XGeneralCategoryNameToMaskMapper* self, const char* name_data, size_t name_len);

uint32_t ICU4XGeneralCategoryNameToMaskMapper_get_loose(const ICU4XGeneralCategoryNameToMaskMapper* self, const char* name_data, size_t name_len);

typedef struct ICU4XGeneralCategoryNameToMaskMapper_load_result {union {ICU4XGeneralCategoryNameToMaskMapper* ok; ICU4XDataError err;}; bool is_ok;} ICU4XGeneralCategoryNameToMaskMapper_load_result;
ICU4XGeneralCategoryNameToMaskMapper_load_result ICU4XGeneralCategoryNameToMaskMapper_load(const ICU4XDataProvider* provider);


void ICU4XGeneralCategoryNameToMaskMapper_destroy(ICU4XGeneralCategoryNameToMaskMapper* self);





#endif // ICU4XGeneralCategoryNameToMaskMapper_H
