#ifndef ICU4XDataProvider_H
#define ICU4XDataProvider_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct ICU4XDataProvider ICU4XDataProvider;
#ifdef __cplusplus
} // namespace capi
#endif
#include "diplomat_result_box_ICU4XDataProvider_ICU4XError.h"
#include "diplomat_result_void_ICU4XError.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

diplomat_result_box_ICU4XDataProvider_ICU4XError ICU4XDataProvider_create_fs(const char* path_data, size_t path_len);

ICU4XDataProvider* ICU4XDataProvider_create_test();

diplomat_result_box_ICU4XDataProvider_ICU4XError ICU4XDataProvider_create_from_byte_slice(const uint8_t* blob_data, size_t blob_len);

ICU4XDataProvider* ICU4XDataProvider_create_empty();

diplomat_result_void_ICU4XError ICU4XDataProvider_fork_by_key(ICU4XDataProvider* self, ICU4XDataProvider* other);
void ICU4XDataProvider_destroy(ICU4XDataProvider* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
