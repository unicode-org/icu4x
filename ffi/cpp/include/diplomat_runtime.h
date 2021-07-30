#ifndef DIPLOMAT_RUNTIME_C_H
#define DIPLOMAT_RUNTIME_C_H

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

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
