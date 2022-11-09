#ifndef ICU4XWeekOf_type_H
#define ICU4XWeekOf_type_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XWeekRelativeUnit_type.h"
#ifdef __cplusplus
namespace capi {
#endif // __cplusplus

typedef struct ICU4XWeekOf {
    uint16_t week;
    ICU4XWeekRelativeUnit unit;
} ICU4XWeekOf;
#ifdef __cplusplus
} // namespace capi
#endif // __cplusplus
#endif // ICU4XWeekOf_type_H
