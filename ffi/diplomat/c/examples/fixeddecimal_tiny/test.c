// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include "../../include/ICU4XFixedDecimalFormatter.h"
#include <string.h>
#include <stdio.h>

int main() {
    ICU4XLocale* locale = ICU4XLocale_create_bn();
    ICU4XDataProvider* provider = ICU4XDataProvider_create_test();
    ICU4XFixedDecimal* decimal = ICU4XFixedDecimal_create(1000007);

    diplomat_result_box_ICU4XFixedDecimalFormatter_ICU4XError fdf_result =
        ICU4XFixedDecimalFormatter_try_new(provider, locale, ICU4XFixedDecimalGroupingStrategy_Auto);
    if (!fdf_result.is_ok)  {
        printf("Failed to create FixedDecimalFormatter\n");
        return 1;
    }
    ICU4XFixedDecimalFormatter* fdf = fdf_result.ok;
    char output[40];

    DiplomatWriteable write = diplomat_simple_writeable(output, 40);

    bool success = ICU4XFixedDecimalFormatter_format(fdf, decimal, &write).is_ok;
    if (!success) {
        printf("Failed to write result of FixedDecimalFormatter::format to string.\n");
        return 1;
    }
    printf("Output is %s\n", output);

    const char* expected = u8"১০,০০,০০৭";
    if (strcmp(output, expected) != 0) {
        printf("Output does not match expected output!\n");
        return 1;
    }

    ICU4XFixedDecimal_destroy(decimal);
    ICU4XFixedDecimalFormatter_destroy(fdf);
    ICU4XLocale_destroy(locale);
    ICU4XDataProvider_destroy(provider);

    return 0;
}
