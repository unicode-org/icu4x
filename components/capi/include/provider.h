// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#pragma once
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

typedef struct {
    uintptr_t _field1;
    uintptr_t _field2;
} ICU4XDataProvider;

typedef struct {
    ICU4XDataProvider provider;
    bool success;
} ICU4XCreateDataProviderResult;

void icu4x_erased_data_provider_destroy(ICU4XDataProvider d);

ICU4XCreateDataProviderResult icu4x_fs_data_provider_create(const char* path, size_t len);
