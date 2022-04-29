// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include "../../include/ICU4XLocaleCanonicalizer.h"
#include <string.h>
#include <stdio.h>

/**
 * A helper for testing the locale with nice error messages.
 */
bool test_locale(ICU4XLocale* locale, const char* message, const char* expected) {
    char output[40];

    // Test setters
    DiplomatWriteable write = diplomat_simple_writeable(output, 40);
    locale_ffi_result_void_ICU4XLocaleError result = ICU4XLocale_tostring(locale, &write);
    if (!result.is_ok) {
        return 1;
    }
    printf("Output for %s is \"%s\"\n", message, output);
    if (strcmp(output, expected) != 0) {
        printf("Output does not match expected output!\n");
        return false;
    }
    return true;
}

ICU4XLocale* get_locale(const char* localeText) {
    ICU4XLocale* locale = ICU4XLocale_create(localeText, strlen(localeText));
    if (!locale) {
        printf("Could not create locale from: %s", localeText);
    }
    return locale;
}


int main() {
    char output[40];

    // Test creating a locale.
    DiplomatWriteable write = diplomat_simple_writeable(output, 40);
    ICU4XLocale* locale = ICU4XLocale_create("ar", 2);
    locale_ffi_result_void_ICU4XLocaleError result = ICU4XLocale_tostring(locale, &write);
    if (!result.is_ok) {
        return 1;
    }
    printf("Output for a basic locale is %s\n", output);
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
    if (!result.is_ok) {
        return 1;
    }
    printf("Output for the language is %s\n", output);
    expected = u8"fr";
    if (strcmp(output, expected) != 0) {
        printf("Output does not match expected output!\n");
        return 1;
    }

    write = diplomat_simple_writeable(output, 40);
    result = ICU4XLocale_region(locale, &write);
    if (!result.is_ok) {
        return 1;
    }
    printf("Output for the region is %s\n", output);
    expected = u8"FR";
    if (strcmp(output, expected) != 0) {
        printf("Output does not match expected output!\n");
        return 1;
    }

    write = diplomat_simple_writeable(output, 40);
    result = ICU4XLocale_get_unicode_extension(locale, "hc", 2, &write);
    if (!result.is_ok) {
        return 1;
    }
    printf("Output for the extension is %s\n", output);
    expected = u8"h23";
    if (strcmp(output, expected) != 0) {
        printf("Output does not match expected output!\n");
        return 1;
    }

    result = ICU4XLocale_get_unicode_extension(locale, "ca", 2, &write);
    if (!(!result.is_ok && result.err == ICU4XLocaleError_Undefined)) {
        return 1;
    }

    // Test setting the language
    write = diplomat_simple_writeable(output, 40);
    const char* str = "fr-FR-u-hc-h23";
    locale = ICU4XLocale_create(str, strlen(str));
    if (!locale) {
        printf("Could not create the locale.");
        return 1;
    }
    str = "zh";
    result = ICU4XLocale_set_language(locale, str, strlen(str));
    if (!result.is_ok) {
        printf("Could not set the language tag.");
        return 1;
    }
    if (!test_locale(locale, "setting the language tag", "zh-FR-u-hc-h23")) {
        return 1;
    }
    ICU4XLocale_destroy(locale);

    // Test setting the region
    write = diplomat_simple_writeable(output, 40);
    str = "es-ES-u-hc-h23";
    locale = ICU4XLocale_create(str, strlen(str));
    if (!locale) {
        printf("Could not create the locale.");
        return 1;
    }
    if (!test_locale(locale, "The region starts as es-ES", "es-ES-u-hc-h23")) {
        return 1;
    }
    str = "MX";
    result = ICU4XLocale_set_region(locale, str, strlen(str));
    if (!result.is_ok) {
        printf("Could not set the region.");
        return 1;
    }
    if (!test_locale(locale, "setting the region", "es-MX-u-hc-h23")) {
        return 1;
    }
    ICU4XLocale_destroy(locale);

     // Test setting the script
    write = diplomat_simple_writeable(output, 40);
    str = "en-US";
    locale = ICU4XLocale_create(str, strlen(str));
    if (!locale) {
        printf("Could not create the locale.");
        return 1;
    }
    if (!test_locale(locale, "initial script-less locale", "en-US")) {
        return 1;
    }
    str = "Latn";
    result = ICU4XLocale_set_script(locale, str, strlen(str));
    if (!result.is_ok) {
        printf("Could not set the script.");
        return 1;
    }
    if (!test_locale(locale, "setting the script", "en-Latn-US")) {
        return 1;
    }
    result = ICU4XLocale_set_script(locale, "", 0);
    if (!result.is_ok) {
        printf("Could not set the script.");
        return 1;
    }
    if (!test_locale(locale, "removing the script", "en-US")) {
        return 1;
    }

    ICU4XLocale_destroy(locale);

    // Create a LocaleCanonicalizer.
    ICU4XCreateDataProviderResult provider_result = ICU4XDataProvider_create_test();
    if (!provider_result.success) {
        printf("Failed to create test data provider\n");
        return 1;
    }
    ICU4XDataProvider* provider = provider_result.provider;
    ICU4XLocaleCanonicalizer* lc = ICU4XLocaleCanonicalizer_create(provider);

    // Test maximize.
    write = diplomat_simple_writeable(output, 40);
    locale = ICU4XLocale_create("und", 3);
    ICU4XLocaleCanonicalizer_maximize(lc, locale);
    result = ICU4XLocale_tostring(locale, &write);
    if (!result.is_ok) {
        return 1;
    }
    printf("Output for maximizing is %s\n", output);
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
    if (!result.is_ok) {
        return 1;
    }
    printf("Output for minimizing is %s\n", output);
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
    if (!result.is_ok) {
        return 1;
    }
    printf("Output for canonicalizing is %s\n", output);
    expected = u8"nn";
    if (strcmp(output, expected) != 0) {
        printf("Output does not match expected output!\n");
        return 1;
    }
    ICU4XLocale_destroy(locale);

    ICU4XLocaleCanonicalizer_destroy(lc);

    return 0;
}
