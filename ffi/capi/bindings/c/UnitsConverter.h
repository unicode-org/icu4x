#ifndef UnitsConverter_H
#define UnitsConverter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "UnitsConverter.d.h"






double ICU4XUnitsConverter_convert_f64(const UnitsConverter* self, double value);

UnitsConverter* ICU4XUnitsConverter_clone(const UnitsConverter* self);


void ICU4XUnitsConverter_destroy(UnitsConverter* self);





#endif // UnitsConverter_H
