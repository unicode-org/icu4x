#ifndef ICU4XBcp47ToIanaMapper_H
#define ICU4XBcp47ToIanaMapper_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XDataProvider.h"
#include "diplomat_result_box_ICU4XBcp47ToIanaMapper_ICU4XError.d.h"
#include "diplomat_result_void_ICU4XError.d.h"

#include "ICU4XBcp47ToIanaMapper.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


diplomat_result_box_ICU4XBcp47ToIanaMapper_ICU4XError ICU4XBcp47ToIanaMapper_create(const ICU4XDataProvider* provider);

diplomat_result_void_ICU4XError ICU4XBcp47ToIanaMapper_get(const ICU4XBcp47ToIanaMapper* self, const char* value_data, size_t value_len, DiplomatWrite* write);

void ICU4XBcp47ToIanaMapper_destroy(ICU4XBcp47ToIanaMapper* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XBcp47ToIanaMapper_H
