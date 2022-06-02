#ifndef ICU4XDataProvider_H
#define ICU4XDataProvider_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ICU4XDataProvider ICU4XDataProvider;
#include "ICU4XCreateDataProviderResult.h"

ICU4XCreateDataProviderResult ICU4XDataProvider_create_fs(const char* path_data, size_t path_len);

ICU4XCreateDataProviderResult ICU4XDataProvider_create_test();

ICU4XCreateDataProviderResult ICU4XDataProvider_create_from_byte_slice(const uint8_t* blob_data, size_t blob_len);

ICU4XCreateDataProviderResult ICU4XDataProvider_create_empty();
void ICU4XDataProvider_destroy(ICU4XDataProvider* self);

#ifdef __cplusplus
}
#endif
#endif
