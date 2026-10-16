#ifndef Parts_H
#define Parts_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "PartSpan.d.h"

#include "Parts.d.h"






DiplomatPartSpanView icu4x_Parts_as_slice_mv1(const Parts* self);

size_t icu4x_Parts_len_mv1(const Parts* self);

bool icu4x_Parts_is_empty_mv1(const Parts* self);

typedef struct icu4x_Parts_get_mv1_result {union {PartSpan ok; }; bool is_ok;} icu4x_Parts_get_mv1_result;
icu4x_Parts_get_mv1_result icu4x_Parts_get_mv1(const Parts* self, size_t index);

void icu4x_Parts_destroy_mv1(Parts* self);





#endif // Parts_H
