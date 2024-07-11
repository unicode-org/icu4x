#ifndef LocaleFallbackerWithConfig_H
#define LocaleFallbackerWithConfig_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "Locale.d.h"
#include "LocaleFallbackIterator.d.h"

#include "LocaleFallbackerWithConfig.d.h"






LocaleFallbackIterator* ICU4XLocaleFallbackerWithConfig_fallback_for_locale(const LocaleFallbackerWithConfig* self, const Locale* locale);


void ICU4XLocaleFallbackerWithConfig_destroy(LocaleFallbackerWithConfig* self);





#endif // LocaleFallbackerWithConfig_H
