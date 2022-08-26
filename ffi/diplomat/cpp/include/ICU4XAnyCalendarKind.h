#ifndef ICU4XAnyCalendarKind_H
#define ICU4XAnyCalendarKind_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef enum ICU4XAnyCalendarKind {
  ICU4XAnyCalendarKind_Iso = 0,
  ICU4XAnyCalendarKind_Gregorian = 1,
  ICU4XAnyCalendarKind_Buddhist = 2,
  ICU4XAnyCalendarKind_Japanese = 3,
  ICU4XAnyCalendarKind_JapaneseExtended = 4,
  ICU4XAnyCalendarKind_Ethiopian = 5,
  ICU4XAnyCalendarKind_EthiopianAmeteAlem = 6,
  ICU4XAnyCalendarKind_Indian = 7,
  ICU4XAnyCalendarKind_Coptic = 8,
} ICU4XAnyCalendarKind;
#ifdef __cplusplus
} // namespace capi
#endif
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

void ICU4XAnyCalendarKind_destroy(ICU4XAnyCalendarKind* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
