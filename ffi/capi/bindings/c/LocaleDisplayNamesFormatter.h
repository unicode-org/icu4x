#ifndef LocaleDisplayNamesFormatter_H
#define LocaleDisplayNamesFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"
#include "DisplayNamesOptionsV1.d.h"
#include "Locale.d.h"

#include "LocaleDisplayNamesFormatter.d.h"






typedef struct ICU4XLocaleDisplayNamesFormatter_create_result {union {LocaleDisplayNamesFormatter* ok; DataError err;}; bool is_ok;} ICU4XLocaleDisplayNamesFormatter_create_result;
ICU4XLocaleDisplayNamesFormatter_create_result ICU4XLocaleDisplayNamesFormatter_create(const DataProvider* provider, const Locale* locale, DisplayNamesOptionsV1 options);

void ICU4XLocaleDisplayNamesFormatter_of(const LocaleDisplayNamesFormatter* self, const Locale* locale, DiplomatWrite* write);


void ICU4XLocaleDisplayNamesFormatter_destroy(LocaleDisplayNamesFormatter* self);





#endif // LocaleDisplayNamesFormatter_H
