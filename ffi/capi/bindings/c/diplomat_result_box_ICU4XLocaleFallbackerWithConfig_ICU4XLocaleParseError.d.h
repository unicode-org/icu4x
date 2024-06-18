#ifndef diplomat_result_box_ICU4XLocaleFallbackerWithConfig_ICU4XLocaleParseError_D_H
#define diplomat_result_box_ICU4XLocaleFallbackerWithConfig_ICU4XLocaleParseError_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XLocaleFallbackerWithConfig.d.h"
#include "ICU4XLocaleParseError.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


typedef struct diplomat_result_box_ICU4XLocaleFallbackerWithConfig_ICU4XLocaleParseError {
  union {
    ICU4XLocaleFallbackerWithConfig* ok;
    ICU4XLocaleParseError err;
  };
  bool is_ok;
} diplomat_result_box_ICU4XLocaleFallbackerWithConfig_ICU4XLocaleParseError;

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // diplomat_result_box_ICU4XLocaleFallbackerWithConfig_ICU4XLocaleParseError_D_H
