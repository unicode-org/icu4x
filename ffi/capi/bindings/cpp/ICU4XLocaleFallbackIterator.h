#ifndef ICU4XLocaleFallbackIterator_H
#define ICU4XLocaleFallbackIterator_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XLocale.d.h"
#include "ICU4XLocale.h"

#include "ICU4XLocaleFallbackIterator.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


ICU4XLocale* ICU4XLocaleFallbackIterator_next(ICU4XLocaleFallbackIterator* self);

void ICU4XLocaleFallbackIterator_destroy(ICU4XLocaleFallbackIterator* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XLocaleFallbackIterator_H
