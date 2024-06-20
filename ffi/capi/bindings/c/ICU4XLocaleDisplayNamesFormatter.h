#ifndef ICU4XLocaleDisplayNamesFormatter_H
#define ICU4XLocaleDisplayNamesFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XDataProvider.h"
#include "ICU4XDisplayNamesOptionsV1.d.h"
#include "ICU4XDisplayNamesOptionsV1.h"
#include "ICU4XLocale.d.h"
#include "ICU4XLocale.h"
#include "diplomat_result_box_ICU4XLocaleDisplayNamesFormatter_ICU4XDataError.d.h"

#include "ICU4XLocaleDisplayNamesFormatter.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


diplomat_result_box_ICU4XLocaleDisplayNamesFormatter_ICU4XDataError ICU4XLocaleDisplayNamesFormatter_create(const ICU4XDataProvider* provider, const ICU4XLocale* locale, ICU4XDisplayNamesOptionsV1 options);

void ICU4XLocaleDisplayNamesFormatter_of(const ICU4XLocaleDisplayNamesFormatter* self, const ICU4XLocale* locale, DiplomatWrite* write);

void ICU4XLocaleDisplayNamesFormatter_destroy(ICU4XLocaleDisplayNamesFormatter* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XLocaleDisplayNamesFormatter_H
