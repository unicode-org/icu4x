#ifndef ICU4XLogger_H
#define ICU4XLogger_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "ICU4XLogger.d.h"






bool ICU4XLogger_init_simple_logger();


void ICU4XLogger_destroy(ICU4XLogger* self);





#endif // ICU4XLogger_H
