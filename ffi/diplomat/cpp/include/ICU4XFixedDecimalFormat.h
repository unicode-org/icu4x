#ifndef ICU4XFixedDecimalFormat_H
#define ICU4XFixedDecimalFormat_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ICU4XFixedDecimalFormat ICU4XFixedDecimalFormat;
#include "ICU4XLocale.h"
#include "ICU4XDataProvider.h"
#include "ICU4XFixedDecimalFormatOptions.h"
#include "result_box_ICU4XFixedDecimalFormat_void.h"
#include "ICU4XDataStruct.h"
#include "ICU4XFixedDecimal.h"
#include "result_void_void.h"

decimal_ffi_result_box_ICU4XFixedDecimalFormat_void ICU4XFixedDecimalFormat_try_new(const ICU4XLocale* locale, const ICU4XDataProvider* provider, ICU4XFixedDecimalFormatOptions options);

decimal_ffi_result_box_ICU4XFixedDecimalFormat_void ICU4XFixedDecimalFormat_try_new_from_decimal_symbols_v1(ICU4XDataStruct* data_struct, ICU4XFixedDecimalFormatOptions options);

decimal_ffi_result_void_void ICU4XFixedDecimalFormat_format(const ICU4XFixedDecimalFormat* self, const ICU4XFixedDecimal* value, DiplomatWriteable* write);
void ICU4XFixedDecimalFormat_destroy(ICU4XFixedDecimalFormat* self);

#ifdef __cplusplus
}
#endif
#endif
