#ifndef LocaleFallbacker_H
#define LocaleFallbacker_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"
#include "LocaleFallbackConfig.d.h"
#include "LocaleFallbackerWithConfig.d.h"
#include "LocaleParseError.d.h"

#include "LocaleFallbacker.d.h"






typedef struct ICU4XLocaleFallbacker_create_result {union {LocaleFallbacker* ok; DataError err;}; bool is_ok;} ICU4XLocaleFallbacker_create_result;
ICU4XLocaleFallbacker_create_result ICU4XLocaleFallbacker_create(const DataProvider* provider);

LocaleFallbacker* ICU4XLocaleFallbacker_create_without_data();

typedef struct ICU4XLocaleFallbacker_for_config_result {union {LocaleFallbackerWithConfig* ok; LocaleParseError err;}; bool is_ok;} ICU4XLocaleFallbacker_for_config_result;
ICU4XLocaleFallbacker_for_config_result ICU4XLocaleFallbacker_for_config(const LocaleFallbacker* self, LocaleFallbackConfig config);


void ICU4XLocaleFallbacker_destroy(LocaleFallbacker* self);





#endif // LocaleFallbacker_H
