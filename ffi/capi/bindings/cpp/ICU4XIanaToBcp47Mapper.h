#ifndef ICU4XIanaToBcp47Mapper_H
#define ICU4XIanaToBcp47Mapper_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XDataProvider.h"
#include "diplomat_result_box_ICU4XIanaToBcp47Mapper_ICU4XError.d.h"
#include "diplomat_result_void_ICU4XError.d.h"

#include "ICU4XIanaToBcp47Mapper.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


diplomat_result_box_ICU4XIanaToBcp47Mapper_ICU4XError ICU4XIanaToBcp47Mapper_create(const ICU4XDataProvider* provider);

diplomat_result_void_ICU4XError ICU4XIanaToBcp47Mapper_get(const ICU4XIanaToBcp47Mapper* self, const char* value_data, size_t value_len, DiplomatWrite* write);

void ICU4XIanaToBcp47Mapper_destroy(ICU4XIanaToBcp47Mapper* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XIanaToBcp47Mapper_H
