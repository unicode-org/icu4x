#ifndef ICU4XDisplayNamesOptions_H
#define ICU4XDisplayNamesOptions_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XDisplayNamesStyle.h"
#include "ICU4XDisplayNamesFallback.h"
#include "ICU4XLanguageDisplay.h"
#ifdef __cplusplus
namespace capi {
#endif

typedef struct ICU4XDisplayNamesOptions {
    ICU4XDisplayNamesStyle style;
    ICU4XDisplayNamesFallback fallback;
    ICU4XLanguageDisplay language_display;
} ICU4XDisplayNamesOptions;
#ifdef __cplusplus
} // namespace capi
#endif
#include "ICU4XDisplayNamesStyle.h"
#include "ICU4XDisplayNamesFallback.h"
#include "ICU4XLanguageDisplay.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

void ICU4XDisplayNamesOptions_destroy(ICU4XDisplayNamesOptions* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
