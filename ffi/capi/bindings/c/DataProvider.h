#ifndef DataProvider_H
#define DataProvider_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "LocaleFallbacker.d.h"

#include "DataProvider.d.h"






DataProvider* ICU4XDataProvider_create_compiled();

typedef struct ICU4XDataProvider_create_fs_result {union {DataProvider* ok; DataError err;}; bool is_ok;} ICU4XDataProvider_create_fs_result;
ICU4XDataProvider_create_fs_result ICU4XDataProvider_create_fs(const char* path_data, size_t path_len);

typedef struct ICU4XDataProvider_create_from_byte_slice_result {union {DataProvider* ok; DataError err;}; bool is_ok;} ICU4XDataProvider_create_from_byte_slice_result;
ICU4XDataProvider_create_from_byte_slice_result ICU4XDataProvider_create_from_byte_slice(const uint8_t* blob_data, size_t blob_len);

DataProvider* ICU4XDataProvider_create_empty();

typedef struct ICU4XDataProvider_fork_by_key_result {union { DataError err;}; bool is_ok;} ICU4XDataProvider_fork_by_key_result;
ICU4XDataProvider_fork_by_key_result ICU4XDataProvider_fork_by_key(DataProvider* self, DataProvider* other);

typedef struct ICU4XDataProvider_fork_by_locale_result {union { DataError err;}; bool is_ok;} ICU4XDataProvider_fork_by_locale_result;
ICU4XDataProvider_fork_by_locale_result ICU4XDataProvider_fork_by_locale(DataProvider* self, DataProvider* other);

typedef struct ICU4XDataProvider_enable_locale_fallback_with_result {union { DataError err;}; bool is_ok;} ICU4XDataProvider_enable_locale_fallback_with_result;
ICU4XDataProvider_enable_locale_fallback_with_result ICU4XDataProvider_enable_locale_fallback_with(DataProvider* self, const LocaleFallbacker* fallbacker);


void ICU4XDataProvider_destroy(DataProvider* self);





#endif // DataProvider_H
