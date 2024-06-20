#ifndef DIPLOMAT_RUNTIME_C_H
#define DIPLOMAT_RUNTIME_C_H

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <sys/types.h>

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





typedef struct DiplomatWrite {
    void* context;
    char* buf;
    size_t len;
    size_t cap;
    bool grow_failed;
    void (*flush)(struct DiplomatWrite*);
    bool (*grow)(struct DiplomatWrite*, size_t);
} DiplomatWrite;


DiplomatWrite diplomat_simple_write(char* buf, size_t buf_size);

DiplomatWrite* diplomat_buffer_write_create(size_t cap);
char* diplomat_buffer_write_get_bytes(DiplomatWrite* t);
size_t diplomat_buffer_write_len(DiplomatWrite* t);
void diplomat_buffer_write_destroy(DiplomatWrite* t);

bool diplomat_is_str(const char* buf, size_t len);



#define MAKE_SLICES(name, c_ty) \
    typedef struct Diplomat##name##View { \
        const c_ty* data; \
        size_t len; \
    } Diplomat##name##View; \
    typedef struct Diplomat##name##Array { \
        const c_ty* data; \
        size_t len; \
    } Diplomat##name##Array;

MAKE_SLICES(I8, int8_t)
MAKE_SLICES(U8, uint8_t)
MAKE_SLICES(I16, int16_t)
MAKE_SLICES(U16, uint16_t)
MAKE_SLICES(I32, int32_t)
MAKE_SLICES(U32, uint32_t)
MAKE_SLICES(I64, int64_t)
MAKE_SLICES(U64, uint64_t)
MAKE_SLICES(Isize, intptr_t)
MAKE_SLICES(Usize, size_t)
MAKE_SLICES(F32, float)
MAKE_SLICES(F64, double)
MAKE_SLICES(Bool, bool)
MAKE_SLICES(Char, char32_t)
MAKE_SLICES(String, char)
MAKE_SLICES(String16, char16_t)
MAKE_SLICES(Strings, DiplomatStringView)
MAKE_SLICES(Strings16, DiplomatString16View)



#endif
