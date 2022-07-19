#ifndef DIPLOMAT_RUNTIME_C_H
#define DIPLOMAT_RUNTIME_C_H

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

// uchar.h doesn't always exist, but char32_t is always available
// in C++ anyway
#ifndef __cplusplus
#ifdef __APPLE__
#include <stdint.h>
typedef uint16_t char16_t;
typedef uint32_t char32_t;
#else
#include <uchar.h>
#endif
#endif


#ifdef __cplusplus
namespace capi {
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

typedef struct DiplomatStringView {
    const char* data;
    size_t len;
} DiplomatStringView;
#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif

#endif
