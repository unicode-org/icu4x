#ifndef ICU4XDisplayNamesOptionsV1_D_H
#define ICU4XDisplayNamesOptionsV1_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XDisplayNamesFallback.d.h"
#include "ICU4XDisplayNamesStyle.d.h"
#include "ICU4XLanguageDisplay.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


typedef struct ICU4XDisplayNamesOptionsV1 {
  ICU4XDisplayNamesStyle style;
  ICU4XDisplayNamesFallback fallback;
  ICU4XLanguageDisplay language_display;
} ICU4XDisplayNamesOptionsV1;


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XDisplayNamesOptionsV1_D_H
