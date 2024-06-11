#ifndef ICU4XDataProvider_H
#define ICU4XDataProvider_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XLocaleFallbacker.d.h"
#include "ICU4XLocaleFallbacker.h"
#include "diplomat_result_box_ICU4XDataProvider_ICU4XDataError.d.h"
#include "diplomat_result_void_ICU4XDataError.d.h"

#include "ICU4XDataProvider.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


ICU4XDataProvider* ICU4XDataProvider_create_compiled();

diplomat_result_box_ICU4XDataProvider_ICU4XDataError ICU4XDataProvider_create_fs(const char* path_data, size_t path_len);

diplomat_result_box_ICU4XDataProvider_ICU4XDataError ICU4XDataProvider_create_from_byte_slice(const uint8_t* blob_data, size_t blob_len);

ICU4XDataProvider* ICU4XDataProvider_create_empty();

diplomat_result_void_ICU4XDataError ICU4XDataProvider_fork_by_key(ICU4XDataProvider* self, ICU4XDataProvider* other);

diplomat_result_void_ICU4XDataError ICU4XDataProvider_fork_by_locale(ICU4XDataProvider* self, ICU4XDataProvider* other);

diplomat_result_void_ICU4XDataError ICU4XDataProvider_enable_locale_fallback(ICU4XDataProvider* self);

diplomat_result_void_ICU4XDataError ICU4XDataProvider_enable_locale_fallback_with(ICU4XDataProvider* self, const ICU4XLocaleFallbacker* fallbacker);

void ICU4XDataProvider_destroy(ICU4XDataProvider* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XDataProvider_H
