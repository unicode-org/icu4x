// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include "DataProvider.h"
#include "Locale.h"
#include "Decimal.h"
#include "DecimalFormatter.h"
#include "Logger.h"
#include <string.h>
#include <stdio.h>

int main() {
    icu4x_Logger_init_simple_logger_mv1();

    struct DiplomatStringView locale_str = {
        "bn",
        2
    };
    icu4x_Locale_from_string_mv1_result locale_result = icu4x_Locale_from_string_mv1(locale_str);
    if (!locale_result.is_ok) {
        return 1;
    }
    Locale* locale = locale_result.ok;

    Decimal* decimal = icu4x_Decimal_from_uint64_mv1(1000007);

    DecimalGroupingStrategy_option o = {.ok = DecimalGroupingStrategy_Auto, .is_ok = true};

    icu4x_DecimalFormatter_create_with_grouping_strategy_mv1_result formatter_result =
        icu4x_DecimalFormatter_create_with_grouping_strategy_mv1(locale, o);
    if (!formatter_result.is_ok)  {
        printf("Failed to create DecimalFormatter\n");
        return 1;
    }
    DecimalFormatter* formatter = formatter_result.ok;
    char output[40];

    DiplomatWrite write = diplomat_simple_write(output, 40);

    icu4x_DecimalFormatter_format_mv1(formatter, decimal, &write);
    if (write.grow_failed) {
        printf("format overflowed the string.\n");
        return 1;
    }
    printf("Output is %s\n", output);

    const char* expected = u8"১০,০০,০০৭";
    if (strcmp(output, expected) != 0) {
        printf("Output does not match expected output!\n");
        return 1;
    }

    icu4x_Decimal_multiply_pow10_mv1(decimal, 2);

    icu4x_Decimal_set_sign_mv1(decimal, DecimalSign_Negative);

    write = diplomat_simple_write(output, 40);

    icu4x_DecimalFormatter_format_mv1(formatter, decimal, &write);
    if (write.grow_failed) {
        printf("format overflowed the string.\n");
        return 1;
    }
    printf("Output x100 and negated is %s\n", output);

    expected = u8"-১০,০০,০০,৭০০";
    if (strcmp(output, expected) != 0) {
        printf("Output does not match expected output!\n");
        return 1;
    }

    icu4x_Decimal_destroy_mv1(decimal);

    struct DiplomatStringView decimal_str = {
        "1000007.070",
        11
    };

    icu4x_Decimal_from_string_mv1_result fd_result = icu4x_Decimal_from_string_mv1(decimal_str);
    if (!fd_result.is_ok) {
        printf("Failed to create FixedDecimal from string.\n");
        return 1;
    }
    decimal = fd_result.ok;

    write = diplomat_simple_write(output, 40);

    icu4x_DecimalFormatter_format_mv1(formatter, decimal, &write);
    if (write.grow_failed) {
        printf("format overflowed the string.\n");
        return 1;
    }
    printf("Output is %s\n", output);

    expected = u8"১০,০০,০০৭.০৭০";
    if (strcmp(output, expected) != 0) {
        printf("Output does not match expected output!\n");
        return 1;
    }

    icu4x_Decimal_destroy_mv1(decimal);
    icu4x_DecimalFormatter_destroy_mv1(formatter);
    icu4x_Locale_destroy_mv1(locale);

    return 0;
}
