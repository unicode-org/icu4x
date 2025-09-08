// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include "Locale.h"
#include "Decimal.h"
#include "DecimalFormatter.h"
#include <string.h>
#include <stdio.h>

int main(int argc, char *argv[]) {
    if (argc != 2) {
        printf("Usage: %s <language>\n", argv[0]);
        return 1;
    }

    Locale* locale = icu4x_Locale_unknown_mv1();
    struct DiplomatStringView arg_str = {
        argv[1],
        strlen(argv[1])
    };
    if (!icu4x_Locale_set_language_mv1(locale, arg_str).is_ok) {
        printf("Invalid language tag \"%s\"\n", argv[1]);
        return 1;
    }

    Decimal* decimal = icu4x_Decimal_from_uint64_mv1(1000007);
    icu4x_Decimal_round_mv1(decimal, 0);

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

    icu4x_Decimal_destroy_mv1(decimal);
    icu4x_DecimalFormatter_destroy_mv1(formatter);
    icu4x_Locale_destroy_mv1(locale);

    return 0;
}
