// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#ifndef ICU4X_CUSTOM_WRITEABLE_H
#define ICU4X_CUSTOM_WRITEABLE_H

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

typedef struct {
    void* data;
    char* buf;
    size_t len;
    size_t cap;
    void (*flush)(void*, size_t);
    char (*grow)(void*, size_t*);
} ICU4XCustomWriteable;

ICU4XCustomWriteable icu4x_simple_writeable(char* buf, size_t len);

#endif // ICU4X_CUSTOM_WRITEABLE_H