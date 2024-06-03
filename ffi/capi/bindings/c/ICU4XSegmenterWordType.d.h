#ifndef ICU4XSegmenterWordType_D_H
#define ICU4XSegmenterWordType_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


typedef enum ICU4XSegmenterWordType {
  ICU4XSegmenterWordType_None = 0,
  ICU4XSegmenterWordType_Number = 1,
  ICU4XSegmenterWordType_Letter = 2,
} ICU4XSegmenterWordType;


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XSegmenterWordType_D_H
