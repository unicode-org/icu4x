#ifndef ICU4XLocaleFallbacker_H
#define ICU4XLocaleFallbacker_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XDataProvider.h"
#include "ICU4XLocaleFallbackConfig.d.h"
#include "ICU4XLocaleFallbackConfig.h"
#include "diplomat_result_box_ICU4XLocaleFallbackerWithConfig_ICU4XLocaleParseError.d.h"
#include "diplomat_result_box_ICU4XLocaleFallbacker_ICU4XDataError.d.h"

#include "ICU4XLocaleFallbacker.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


diplomat_result_box_ICU4XLocaleFallbacker_ICU4XDataError ICU4XLocaleFallbacker_create(const ICU4XDataProvider* provider);

ICU4XLocaleFallbacker* ICU4XLocaleFallbacker_create_without_data();

diplomat_result_box_ICU4XLocaleFallbackerWithConfig_ICU4XLocaleParseError ICU4XLocaleFallbacker_for_config(const ICU4XLocaleFallbacker* self, ICU4XLocaleFallbackConfig config);

void ICU4XLocaleFallbacker_destroy(ICU4XLocaleFallbacker* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XLocaleFallbacker_H
