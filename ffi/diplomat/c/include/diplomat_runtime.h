#ifndef DIPLOMAT_RUNTIME_C_H
#define DIPLOMAT_RUNTIME_C_H

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

// uchar.h doesn't always exist, but char32_t is always available
// in C++ anyway
#ifndef __cplusplus
#include<uchar.h>
#endif


#ifdef __cplusplus
extern "C" {
#endif

typedef struct DiplomatWriteable {
    void* context;
    char* buf;
    size_t len;
    size_t cap;
    void (*flush)(struct DiplomatWriteable*);
    bool (*grow)(struct DiplomatWriteable*, size_t);
} DiplomatWriteable;

DiplomatWriteable diplomat_simple_writeable(char* buf, size_t buf_size);

#ifdef __cplusplus
}
#endif

#endif
