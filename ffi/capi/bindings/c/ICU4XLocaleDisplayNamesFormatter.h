#ifndef ICU4XLocaleDisplayNamesFormatter_H
#define ICU4XLocaleDisplayNamesFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XDataError.d.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XDisplayNamesOptionsV1.d.h"
#include "ICU4XLocale.d.h"

#include "ICU4XLocaleDisplayNamesFormatter.d.h"






struct ICU4XLocaleDisplayNamesFormatter_create_result {union {ICU4XLocaleDisplayNamesFormatter* ok; ICU4XDataError err;}; bool is_ok;};
struct ICU4XLocaleDisplayNamesFormatter_create_result ICU4XLocaleDisplayNamesFormatter_create(const ICU4XDataProvider* provider, const ICU4XLocale* locale, ICU4XDisplayNamesOptionsV1 options);

void ICU4XLocaleDisplayNamesFormatter_of(const ICU4XLocaleDisplayNamesFormatter* self, const ICU4XLocale* locale, DiplomatWrite* write);


void ICU4XLocaleDisplayNamesFormatter_destroy(ICU4XLocaleDisplayNamesFormatter* self);





#endif // ICU4XLocaleDisplayNamesFormatter_H
