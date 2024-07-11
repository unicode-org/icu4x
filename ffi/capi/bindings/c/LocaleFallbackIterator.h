#ifndef LocaleFallbackIterator_H
#define LocaleFallbackIterator_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "Locale.d.h"

#include "LocaleFallbackIterator.d.h"






Locale* ICU4XLocaleFallbackIterator_next(LocaleFallbackIterator* self);


void ICU4XLocaleFallbackIterator_destroy(LocaleFallbackIterator* self);





#endif // LocaleFallbackIterator_H
