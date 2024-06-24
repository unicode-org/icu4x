#ifndef ICU4XLocaleFallbackerWithConfig_H
#define ICU4XLocaleFallbackerWithConfig_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XLocale.d.h"
#include "ICU4XLocaleFallbackIterator.d.h"

#include "ICU4XLocaleFallbackerWithConfig.d.h"






ICU4XLocaleFallbackIterator* ICU4XLocaleFallbackerWithConfig_fallback_for_locale(const ICU4XLocaleFallbackerWithConfig* self, const ICU4XLocale* locale);


void ICU4XLocaleFallbackerWithConfig_destroy(ICU4XLocaleFallbackerWithConfig* self);





#endif // ICU4XLocaleFallbackerWithConfig_H
