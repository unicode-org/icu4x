// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include "../../include/locale.h"
#include "../../include/locale_canonicalizer.h"
#include <string.h>
#include <stdio.h>

const char* path = "../../../../provider/testdata/data/json/";
int main() {
    char output[40];

    // Test creating a locale.
    ICU4XWriteable write = icu4x_simple_writeable(output, 40);
    ICU4XLocale* locale = icu4x_locale_create("ar", 2);
    bool success = icu4x_locale_tostring(locale, &write);
    if (!success) {
        return 1;
    }
    const char* expected = u8"ar";
    if (strcmp(output, expected) != 0) {
        printf("Output does not match expected output!\n");
        return 1;
    }
    printf("Output is %s\n", output);
    icu4x_locale_destroy(locale);

    // Create a LocaleCanonicalizer.
    ICU4XCreateDataProviderResult result = icu4x_fs_data_provider_create(path, strlen(path));
    if (!result.success) {
        printf("Failed to create FsDataProvider\n");
        return 1;
    }
    ICU4XDataProvider provider = result.provider;
    ICU4XLocaleCanonicalizer* lc = icu4x_localecanonicalizer_create(&provider);

    // Test maximize.
    write = icu4x_simple_writeable(output, 40);
    locale = icu4x_locale_create("und", 3);
    icu4x_localecanonicalizer_maximize(lc, locale);
    success = icu4x_locale_tostring(locale, &write);
    if (!success) {
        return 1;
    }
    expected = u8"en-Latn-US";
    if (strcmp(output, expected) != 0) {
        printf("Output does not match expected output!\n");
        return 1;
    }
    printf("Output is %s\n", output);
    icu4x_locale_destroy(locale);

    // Test minimize.
    write = icu4x_simple_writeable(output, 40);
    locale = icu4x_locale_create("zh-Hant", 7);
    icu4x_localecanonicalizer_minimize(lc, locale);
    success = icu4x_locale_tostring(locale, &write);
    if (!success) {
        return 1;
    }
    expected = u8"zh-TW";
    if (strcmp(output, expected) != 0) {
        printf("Output does not match expected output!\n");
        return 1;
    }
    printf("Output is %s\n", output);
    icu4x_locale_destroy(locale);

    // Test canonicalize.
    write = icu4x_simple_writeable(output, 40);
    locale = icu4x_locale_create("no-nynorsk", 10);
    icu4x_localecanonicalizer_canonicalize(lc, locale);
    success = icu4x_locale_tostring(locale, &write);
    if (!success) {
        return 1;
    }
    expected = u8"nn";
    if (strcmp(output, expected) != 0) {
        printf("Output does not match expected output!\n");
        return 1;
    }
    printf("Output is %s\n", output);
    icu4x_locale_destroy(locale);

    icu4x_localecanonicalizer_destroy(lc);

    return 0;
}
