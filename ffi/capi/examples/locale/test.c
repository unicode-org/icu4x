// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include "../../include/api.h"
#include <string.h>
#include <stdio.h>

const char* path = "../../../../provider/testdata/data/json/";
int main() {
    char output[40];

    // Test creating a locale.
    DiplomatWriteable write = diplomat_simple_writeable(output, 40);
    ICU4XLocale* locale = ICU4XLocale_create("ar", 2);
    ICU4XLocaleResult result = ICU4XLocale_tostring(locale, &write);
    if (result != ICU4XLocaleResult_Ok) {
        return 1;
    }
    printf("Output is %s\n", output);
    const char* expected = u8"ar";
    if (strcmp(output, expected) != 0) {
        printf("Output does not match expected output!\n");
        return 1;
    }
    ICU4XLocale_destroy(locale);

    // Test some accessors.
    write = diplomat_simple_writeable(output, 40);
    locale = ICU4XLocale_create("fr-FR-u-hc-h23", 14);
    result = ICU4XLocale_language(locale, &write);
    if (result != ICU4XLocaleResult_Ok) {
        return 1;
    }
    printf("Output is %s\n", output);
    expected = u8"fr";
    if (strcmp(output, expected) != 0) {
        printf("Output does not match expected output!\n");
        return 1;
    }

    write = diplomat_simple_writeable(output, 40);
    result = ICU4XLocale_region(locale, &write);
    if (result != ICU4XLocaleResult_Ok) {
        return 1;
    }
    printf("Output is %s\n", output);
    expected = u8"FR";
    if (strcmp(output, expected) != 0) {
        printf("Output does not match expected output!\n");
        return 1;
    }

    write = diplomat_simple_writeable(output, 40);
    result = ICU4XLocale_get_unicode_extension(locale, "hc", 2, &write);
    if (result != ICU4XLocaleResult_Ok) {
        return 1;
    }
    printf("Output is %s\n", output);
    expected = u8"h23";
    if (strcmp(output, expected) != 0) {
        printf("Output does not match expected output!\n");
        return 1;
    }

    result = ICU4XLocale_get_unicode_extension(locale, "ca", 2, &write);
    if (result != ICU4XLocaleResult_Undefined) {
        return 1;
    }

    ICU4XLocale_destroy(locale);

    // Create a LocaleCanonicalizer.
    ICU4XCreateDataProviderResult provider_result = ICU4XDataProvider_create_fs(path, strlen(path));
    if (!provider_result.success) {
        printf("Failed to create FsDataProvider\n");
        return 1;
    }
    ICU4XDataProvider* provider = provider_result.provider;
    ICU4XLocaleCanonicalizer* lc = ICU4XLocaleCanonicalizer_create(provider);

    // Test maximize.
    write = diplomat_simple_writeable(output, 40);
    locale = ICU4XLocale_create("und", 3);
    ICU4XLocaleCanonicalizer_maximize(lc, locale);
    result = ICU4XLocale_tostring(locale, &write);
    if (result != ICU4XLocaleResult_Ok) {
        return 1;
    }
    printf("Output is %s\n", output);
    expected = u8"en-Latn-US";
    if (strcmp(output, expected) != 0) {
        printf("Output does not match expected output!\n");
        return 1;
    }
    ICU4XLocale_destroy(locale);

    // Test minimize.
    write = diplomat_simple_writeable(output, 40);
    locale = ICU4XLocale_create("zh-Hant", 7);
    ICU4XLocaleCanonicalizer_minimize(lc, locale);
    result = ICU4XLocale_tostring(locale, &write);
    if (result != ICU4XLocaleResult_Ok) {
        return 1;
    }
    printf("Output is %s\n", output);
    expected = u8"zh-TW";
    if (strcmp(output, expected) != 0) {
        printf("Output does not match expected output!\n");
        return 1;
    }
    ICU4XLocale_destroy(locale);

    // Test canonicalize.
    write = diplomat_simple_writeable(output, 40);
    locale = ICU4XLocale_create("no-nynorsk", 10);
    ICU4XLocaleCanonicalizer_canonicalize(lc, locale);
    result = ICU4XLocale_tostring(locale, &write);
    if (result != ICU4XLocaleResult_Ok) {
        return 1;
    }
    printf("Output is %s\n", output);
    expected = u8"nn";
    if (strcmp(output, expected) != 0) {
        printf("Output does not match expected output!\n");
        return 1;
    }
    ICU4XLocale_destroy(locale);

    ICU4XLocaleCanonicalizer_destroy(lc);

    return 0;
}
