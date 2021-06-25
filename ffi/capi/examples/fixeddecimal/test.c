// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include "../../include/decimal.h"
#include <string.h>
#include <stdio.h>

const char* path = "../../../../provider/testdata/data/json/";
int main() {
    ICU4XLocale* locale = icu4x_locale_create("bn", 2);
    ICU4XCreateDataProviderResult result = icu4x_fs_data_provider_create(path, strlen(path));
    if (!result.success) {
        printf("Failed to create FsDataProvider\n");
        return 1;
    }
    ICU4XDataProvider provider = result.provider;
    ICU4XFixedDecimal* decimal = icu4x_fixed_decimal_create(1000007);

    ICU4XFixedDecimalFormatOptions opts = {ICU4XGroupingStrategy_Auto, ICU4XSignDisplay_Auto};

    ICU4XCreateFixedDecimalFormatResult fdf_result = icu4x_fixed_decimal_format_create(locale, &provider, opts);
    if (!fdf_result.success)  {
        printf("Failed to create FixedDecimalFormat\n");
        return 1;
    }
    ICU4XFixedDecimalFormat* fdf = fdf_result.fdf;
    char output[40];

    ICU4XWriteable write = icu4x_simple_writeable(output, 40);

    bool success = icu4x_fixed_decimal_format_write(fdf, decimal, &write);
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

    success = icu4x_fixed_decimal_multiply_pow10(decimal, 2);
    if (!success) {
        printf("Failed to multiply FixedDecimal\n");
        return 1;
    }

    icu4x_fixed_decimal_negate(decimal);

    write = icu4x_simple_writeable(output, 40);

    success = icu4x_fixed_decimal_format_write(fdf, decimal, &write);
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

    icu4x_fixed_decimal_destroy(decimal);

    ICU4XCreateFixedDecimalResult fd_result = icu4x_fixed_decimal_create_fromstr("1000007.070", 11);
    if (!fd_result.success) {
        printf("Failed to create FixedDecimal from string.\n");
        return 1;
    }
    decimal = fd_result.fd;

    write = icu4x_simple_writeable(output, 40);

    success = icu4x_fixed_decimal_format_write(fdf, decimal, &write);
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

    icu4x_fixed_decimal_destroy(decimal);
    icu4x_fixed_decimal_format_destroy(fdf);

    return 0;
}
