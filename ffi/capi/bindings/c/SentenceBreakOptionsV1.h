#ifndef SentenceBreakOptionsV1_H
#define SentenceBreakOptionsV1_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "Locale.d.h"

#include "SentenceBreakOptionsV1.d.h"






SentenceBreakOptionsV1* icu4x_SentenceBreakOptionsV1_create_mv1(const Locale* locale);


void icu4x_SentenceBreakOptionsV1_destroy_mv1(SentenceBreakOptionsV1* self);





#endif // SentenceBreakOptionsV1_H
