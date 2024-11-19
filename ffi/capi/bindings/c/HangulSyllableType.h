#ifndef HangulSyllableType_H
#define HangulSyllableType_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "HangulSyllableType.d.h"






uint8_t icu4x_HangulSyllableType_to_integer_mv1(HangulSyllableType self);

typedef struct icu4x_HangulSyllableType_from_integer_mv1_result {union {HangulSyllableType ok; }; bool is_ok;} icu4x_HangulSyllableType_from_integer_mv1_result;
icu4x_HangulSyllableType_from_integer_mv1_result icu4x_HangulSyllableType_from_integer_mv1(uint8_t other);






#endif // HangulSyllableType_H
