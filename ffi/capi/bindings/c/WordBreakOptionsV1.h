#ifndef WordBreakOptionsV1_H
#define WordBreakOptionsV1_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "Locale.d.h"

#include "WordBreakOptionsV1.d.h"






WordBreakOptionsV1* icu4x_WordBreakOptionsV1_create_mv1(const Locale* locale);


void icu4x_WordBreakOptionsV1_destroy_mv1(WordBreakOptionsV1* self);





#endif // WordBreakOptionsV1_H
