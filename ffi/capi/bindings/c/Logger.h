#ifndef Logger_H
#define Logger_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "Logger.d.h"






bool ICU4XLogger_init_simple_logger();


void ICU4XLogger_destroy(Logger* self);





#endif // Logger_H
