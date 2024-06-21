#ifndef ICU4XDataProvider_H
#define ICU4XDataProvider_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XDataError.d.h"
#include "ICU4XLocaleFallbacker.d.h"

#include "ICU4XDataProvider.d.h"






ICU4XDataProvider* ICU4XDataProvider_create_compiled();

typedef struct ICU4XDataProvider_create_fs_result {union {ICU4XDataProvider* ok; ICU4XDataError err;}; bool is_ok;} ICU4XDataProvider_create_fs_result;
ICU4XDataProvider_create_fs_result ICU4XDataProvider_create_fs(const char* path_data, size_t path_len);

typedef struct ICU4XDataProvider_create_from_byte_slice_result {union {ICU4XDataProvider* ok; ICU4XDataError err;}; bool is_ok;} ICU4XDataProvider_create_from_byte_slice_result;
ICU4XDataProvider_create_from_byte_slice_result ICU4XDataProvider_create_from_byte_slice(const uint8_t* blob_data, size_t blob_len);

ICU4XDataProvider* ICU4XDataProvider_create_empty();

typedef struct ICU4XDataProvider_fork_by_key_result {union { ICU4XDataError err;}; bool is_ok;} ICU4XDataProvider_fork_by_key_result;
ICU4XDataProvider_fork_by_key_result ICU4XDataProvider_fork_by_key(ICU4XDataProvider* self, ICU4XDataProvider* other);

typedef struct ICU4XDataProvider_fork_by_locale_result {union { ICU4XDataError err;}; bool is_ok;} ICU4XDataProvider_fork_by_locale_result;
ICU4XDataProvider_fork_by_locale_result ICU4XDataProvider_fork_by_locale(ICU4XDataProvider* self, ICU4XDataProvider* other);

typedef struct ICU4XDataProvider_enable_locale_fallback_result {union { ICU4XDataError err;}; bool is_ok;} ICU4XDataProvider_enable_locale_fallback_result;
ICU4XDataProvider_enable_locale_fallback_result ICU4XDataProvider_enable_locale_fallback(ICU4XDataProvider* self);

typedef struct ICU4XDataProvider_enable_locale_fallback_with_result {union { ICU4XDataError err;}; bool is_ok;} ICU4XDataProvider_enable_locale_fallback_with_result;
ICU4XDataProvider_enable_locale_fallback_with_result ICU4XDataProvider_enable_locale_fallback_with(ICU4XDataProvider* self, const ICU4XLocaleFallbacker* fallbacker);


void ICU4XDataProvider_destroy(ICU4XDataProvider* self);





#endif // ICU4XDataProvider_H
