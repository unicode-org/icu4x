#ifndef ICU4XLocaleFallbacker_H
#define ICU4XLocaleFallbacker_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XDataError.d.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XLocaleFallbackConfig.d.h"
#include "ICU4XLocaleFallbackerWithConfig.d.h"
#include "ICU4XLocaleParseError.d.h"

#include "ICU4XLocaleFallbacker.d.h"






typedef struct ICU4XLocaleFallbacker_create_result {union {ICU4XLocaleFallbacker* ok; ICU4XDataError err;}; bool is_ok;} ICU4XLocaleFallbacker_create_result;
ICU4XLocaleFallbacker_create_result ICU4XLocaleFallbacker_create(const ICU4XDataProvider* provider);

ICU4XLocaleFallbacker* ICU4XLocaleFallbacker_create_without_data();

typedef struct ICU4XLocaleFallbacker_for_config_result {union {ICU4XLocaleFallbackerWithConfig* ok; ICU4XLocaleParseError err;}; bool is_ok;} ICU4XLocaleFallbacker_for_config_result;
ICU4XLocaleFallbacker_for_config_result ICU4XLocaleFallbacker_for_config(const ICU4XLocaleFallbacker* self, ICU4XLocaleFallbackConfig config);


void ICU4XLocaleFallbacker_destroy(ICU4XLocaleFallbacker* self);





#endif // ICU4XLocaleFallbacker_H
