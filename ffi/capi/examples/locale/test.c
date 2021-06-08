// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include "../../include/locale.h"
#include <string.h>
#include <stdio.h>

const char* path = "../../../../provider/testdata/data/json/";
int main() {
    char output[40];
    ICU4XWriteable write = icu4x_simple_writeable(output, 40);

    ICU4XLocale* locale = icu4x_locale_create("ar", 2);
    bool success = icu4x_locale_tostring(locale, &write);
    if (!success) {
        return 1;
    }

    // expect "ar"
    printf("Output is %s\n", output);

    icu4x_locale_destroy(locale);
    return 0;
}
