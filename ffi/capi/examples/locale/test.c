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
    ICU4XLocaleResult result = icu4x_locale_tostring(locale, &write);
    if (result != ICU4XLocaleResult_Ok) {
        return 1;
    }
    printf("Output is %s\n", output);
    const char* expected = u8"ar";
    if (strcmp(output, expected) != 0) {
        printf("Output does not match expected output!\n");
        return 1;
    }
    icu4x_locale_destroy(locale);

    // Test some accessors.
    write = icu4x_simple_writeable(output, 40);
    locale = icu4x_locale_create("fr-FR-u-hc-h23", 14);
    result = icu4x_locale_language(locale, &write);
    if (result != ICU4XLocaleResult_Ok) {
        return 1;
    }
    printf("Output is %s\n", output);
    expected = u8"fr";
    if (strcmp(output, expected) != 0) {
        printf("Output does not match expected output!\n");
        return 1;
    }

    write = icu4x_simple_writeable(output, 40);
    result = icu4x_locale_region(locale, &write);
    if (result != ICU4XLocaleResult_Ok) {
        return 1;
    }
    printf("Output is %s\n", output);
    expected = u8"FR";
    if (strcmp(output, expected) != 0) {
        printf("Output does not match expected output!\n");
        return 1;
    }

    write = icu4x_simple_writeable(output, 40);
    result = icu4x_locale_get_unicode_extension(locale, "hc", 2, &write);
    if (result != ICU4XLocaleResult_Ok) {
        return 1;
    }
    printf("Output is %s\n", output);
    expected = u8"h23";
    if (strcmp(output, expected) != 0) {
        printf("Output does not match expected output!\n");
        return 1;
    }

    result = icu4x_locale_get_unicode_extension(locale, "ca", 2, &write);
    if (result != ICU4XLocaleResult_Undefined) {
        return 1;
    }

    icu4x_locale_destroy(locale);

    // Create a LocaleCanonicalizer.
    ICU4XCreateDataProviderResult provider_result = icu4x_fs_data_provider_create(path, strlen(path));
    if (!provider_result.success) {
        printf("Failed to create FsDataProvider\n");
        return 1;
    }
    ICU4XDataProvider provider = provider_result.provider;
    ICU4XLocaleCanonicalizer* lc = icu4x_localecanonicalizer_create(&provider);

    // Test maximize.
    write = icu4x_simple_writeable(output, 40);
    locale = icu4x_locale_create("und", 3);
    icu4x_localecanonicalizer_maximize(lc, locale);
    result = icu4x_locale_tostring(locale, &write);
    if (result != ICU4XLocaleResult_Ok) {
        return 1;
    }
    printf("Output is %s\n", output);
    expected = u8"en-Latn-US";
    if (strcmp(output, expected) != 0) {
        printf("Output does not match expected output!\n");
        return 1;
    }
    icu4x_locale_destroy(locale);

    // Test minimize.
    write = icu4x_simple_writeable(output, 40);
    locale = icu4x_locale_create("zh-Hant", 7);
    icu4x_localecanonicalizer_minimize(lc, locale);
    result = icu4x_locale_tostring(locale, &write);
    if (result != ICU4XLocaleResult_Ok) {
        return 1;
    }
    printf("Output is %s\n", output);
    expected = u8"zh-TW";
    if (strcmp(output, expected) != 0) {
        printf("Output does not match expected output!\n");
        return 1;
    }
    icu4x_locale_destroy(locale);

    // Test canonicalize.
    write = icu4x_simple_writeable(output, 40);
    locale = icu4x_locale_create("no-nynorsk", 10);
    icu4x_localecanonicalizer_canonicalize(lc, locale);
    result = icu4x_locale_tostring(locale, &write);
    if (result != ICU4XLocaleResult_Ok) {
        return 1;
    }
    printf("Output is %s\n", output);
    expected = u8"nn";
    if (strcmp(output, expected) != 0) {
        printf("Output does not match expected output!\n");
        return 1;
    }
    icu4x_locale_destroy(locale);

    icu4x_localecanonicalizer_destroy(lc);

    return 0;
}
