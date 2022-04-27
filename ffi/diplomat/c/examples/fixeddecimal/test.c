// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include "../../include/ICU4XFixedDecimalFormat.h"
#include <string.h>
#include <stdio.h>

int main() {
    ICU4XLocale* locale = ICU4XLocale_create("bn", 2);
    ICU4XCreateDataProviderResult result = ICU4XDataProvider_create_test();
    if (!result.success) {
        printf("Failed to create test data provider\n");
        return 1;
    }
    ICU4XDataProvider* provider = result.provider;
    ICU4XFixedDecimal* decimal = ICU4XFixedDecimal_create(1000007);

    ICU4XFixedDecimalFormatOptions opts = {ICU4XFixedDecimalGroupingStrategy_Auto, ICU4XFixedDecimalSignDisplay_Auto};

    decimal_ffi_result_box_ICU4XFixedDecimalFormat_void fdf_result = ICU4XFixedDecimalFormat_try_new(locale, provider, opts);
    if (!fdf_result.is_ok)  {
        printf("Failed to create FixedDecimalFormat\n");
        return 1;
    }
    ICU4XFixedDecimalFormat* fdf = fdf_result.ok;
    char output[40];

    DiplomatWriteable write = diplomat_simple_writeable(output, 40);

    bool success = ICU4XFixedDecimalFormat_format(fdf, decimal, &write).is_ok;
    if (!success) {
        printf("Failed to write result of FixedDecimalFormat::format to string.\n");
        return 1;
    }
    printf("Output is %s\n", output);

    const char* expected = u8"১০,০০,০০৭";
    if (strcmp(output, expected) != 0) {
        printf("Output does not match expected output!\n");
        return 1;
    }

    ICU4XFixedDecimal_multiply_pow10(decimal, 2);
    if (!success) {
        printf("Failed to multiply FixedDecimal\n");
        return 1;
    }

    ICU4XFixedDecimal_negate(decimal);

    write = diplomat_simple_writeable(output, 40);

    success = ICU4XFixedDecimalFormat_format(fdf, decimal, &write).is_ok;
    if (!success) {
        printf("Failed to write result of FixedDecimalFormat::format to string.\n");
        return 1;
    }
    printf("Output x100 and negated is %s\n", output);

    expected = u8"-১০,০০,০০,৭০০";
    if (strcmp(output, expected) != 0) {
        printf("Output does not match expected output!\n");
        return 1;
    }

    ICU4XFixedDecimal_destroy(decimal);

    ICU4XCreateFixedDecimalResult fd_result = ICU4XFixedDecimal_create_fromstr("1000007.070", 11);
    if (!fd_result.success) {
        printf("Failed to create FixedDecimal from string.\n");
        return 1;
    }
    decimal = fd_result.fd;

    write = diplomat_simple_writeable(output, 40);

    success = ICU4XFixedDecimalFormat_format(fdf, decimal, &write).is_ok;
    if (!success) {
        printf("Failed to write result of FixedDecimalFormat::format to string.\n");
        return 1;
    }
    printf("Output is %s\n", output);

    expected = u8"১০,০০,০০৭.০৭০";
    if (strcmp(output, expected) != 0) {
        printf("Output does not match expected output!\n");
        return 1;
    }

    ICU4XFixedDecimal_destroy(decimal);
    ICU4XFixedDecimalFormat_destroy(fdf);
    ICU4XLocale_destroy(locale);
    ICU4XDataProvider_destroy(provider);

    return 0;
}
