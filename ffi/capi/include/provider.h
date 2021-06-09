// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#ifndef ICU4X_PROVIDER_H
#define ICU4X_PROVIDER_H

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

// Rust fat pointers are two words large,
// and neither word is null. We represent this
// as two opaque uintptr_ts
typedef struct {
    uintptr_t _field1;
    uintptr_t _field2;
} ICU4XDataProvider;

typedef struct {
    ICU4XDataProvider provider;
    bool success;
} ICU4XCreateDataProviderResult;



void icu4x_data_provider_destroy(ICU4XDataProvider d);

ICU4XCreateDataProviderResult icu4x_fs_data_provider_create(const char* path, size_t len);

#ifdef __cplusplus
}
#endif

#endif // ICU4X_PROVIDER_H