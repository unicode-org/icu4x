// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#ifndef ICU4X_CUSTOM_WRITEABLE_H
#define ICU4X_CUSTOM_WRITEABLE_H

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ICU4XWriteable {
    void* context;
    char* buf;
    size_t len;
    size_t cap;
    void (*flush)(struct ICU4XWriteable*);
    bool (*grow)(struct ICU4XWriteable*, size_t);
} ICU4XWriteable;

ICU4XWriteable icu4x_simple_writeable(char* buf, size_t buf_size);

#ifdef __cplusplus
}
#endif

#endif // ICU4X_CUSTOM_WRITEABLE_H