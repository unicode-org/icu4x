#ifndef ICU4XLocaleFallbackerWithConfig_H
#define ICU4XLocaleFallbackerWithConfig_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XLocaleFallbackerWithConfig_type.h"
#include "ICU4XLocale_type.h"
#include "ICU4XLocaleFallbackIterator_type.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

ICU4XLocaleFallbackIterator* ICU4XLocaleFallbackerWithConfig_fallback_for_locale(const ICU4XLocaleFallbackerWithConfig* self, const ICU4XLocale* locale);
void ICU4XLocaleFallbackerWithConfig_destroy(ICU4XLocaleFallbackerWithConfig* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus
#endif // ICU4XLocaleFallbackerWithConfig_H
