#ifndef ICU4XWeekOf_D_H
#define ICU4XWeekOf_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XWeekRelativeUnit.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


typedef struct ICU4XWeekOf {
  uint16_t week;
  ICU4XWeekRelativeUnit unit;
} ICU4XWeekOf;


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XWeekOf_D_H
