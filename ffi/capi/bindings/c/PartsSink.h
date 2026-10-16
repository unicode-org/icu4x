#ifndef PartsSink_H
#define PartsSink_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "Parts.d.h"

#include "PartsSink.d.h"






PartsSink* icu4x_PartsSink_create_mv1(void);

PartsSink* icu4x_PartsSink_create_with_capacity_mv1(size_t capacity);

Parts* icu4x_PartsSink_drain_mv1(PartsSink* self);

void icu4x_PartsSink_destroy_mv1(PartsSink* self);





#endif // PartsSink_H
