// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include "DataProvider.h"
#include "Locale.h"
#include "LocaleCanonicalizer.h"
#include "LocaleExpander.h"
#include "Logger.h"
#include <string.h>
#include <stdio.h>

/**
 * A helper for testing the locale with nice error messages.
 */
bool test_locale(Locale* locale, const char* message, const char* expected) {
    char output[40];

    // Test setters
    DiplomatWrite write = diplomat_simple_write(output, 40);
    icu4x_Locale_to_string_mv1(locale, &write);
    if (write.grow_failed) {
        return 1;
    }
    printf("Output for %s is \"%s\"\n", message, output);
    if (strcmp(output, expected) != 0) {
        printf("Output does not match expected output!\n");
        return false;
    }
    return true;
}

Locale* get_locale(const char* localeText) {
    icu4x_Locale_from_string_mv1_result locale_result = icu4x_Locale_from_string_mv1(localeText, strlen(localeText));
    if (!locale_result.is_ok) {
        printf("Could not create locale from: %s", localeText);
    }
    return locale_result.ok;
}


int main() {
    icu4x_Logger_init_simple_logger_mv1();
    char output[40];

    icu4x_Locale_from_string_mv1_result locale_result;

    // Test creating a locale.
    DiplomatWrite write = diplomat_simple_write(output, 40);
    locale_result = icu4x_Locale_from_string_mv1("ar", 2);
    if (!locale_result.is_ok) {
        return 1;
    }
    Locale* locale = locale_result.ok;
    icu4x_Locale_to_string_mv1(locale, &write);
    if (write.grow_failed) {
        return 1;
    }
    printf("Output for a basic locale is %s\n", output);
    const char* expected = u8"ar";
    if (strcmp(output, expected) != 0) {
        printf("Output does not match expected output!\n");
        return 1;
    }
    icu4x_Locale_destroy_mv1(locale);

    // Test some accessors.
    write = diplomat_simple_write(output, 40);
    locale_result = icu4x_Locale_from_string_mv1("fr-FR-u-hc-h23", 14);
    if (!locale_result.is_ok) {
        return 1;
    }
    locale = locale_result.ok;
    icu4x_Locale_language_mv1(locale, &write);
    if (write.grow_failed) {
        return 1;
    }
    printf("Output for the language is %s\n", output);
    expected = u8"fr";
    if (strcmp(output, expected) != 0) {
        printf("Output does not match expected output!\n");
        return 1;
    }

    write = diplomat_simple_write(output, 40);
    icu4x_Locale_region_mv1(locale, &write);
    if (write.grow_failed) {
        return 1;
    }
    printf("Output for the region is %s\n", output);
    expected = u8"FR";
    if (strcmp(output, expected) != 0) {
        printf("Output does not match expected output!\n");
        return 1;
    }

    write = diplomat_simple_write(output, 40);
    icu4x_Locale_get_unicode_extension_mv1_result e_result = icu4x_Locale_get_unicode_extension_mv1(locale, "hc", 2, &write);
    if (!e_result.is_ok || write.grow_failed) {
        return 1;
    }
    printf("Output for the extension is %s\n", output);
    expected = u8"h23";
    if (strcmp(output, expected) != 0) {
        printf("Output does not match expected output!\n");
        return 1;
    }

    write = diplomat_simple_write(output, 40);
    e_result = icu4x_Locale_get_unicode_extension_mv1(locale, "ca", 2, &write);
    if (e_result.is_ok || write.grow_failed) {
        return 1;
    }

    // Test setting the language
    write = diplomat_simple_write(output, 40);
    const char* str = "fr-FR-u-hc-h23";
    locale_result = icu4x_Locale_from_string_mv1(str, strlen(str));
    if (!locale_result.is_ok) {
        printf("Could not create the locale.");
        return 1;
    }
    locale = locale_result.ok;
    str = "zh";
    icu4x_Locale_set_language_mv1_result l_result = icu4x_Locale_set_language_mv1(locale, str, strlen(str));
    if (!l_result.is_ok) {
        printf("Could not set the language tag.");
        return 1;
    }
    if (!test_locale(locale, "setting the language tag", "zh-FR-u-hc-h23")) {
        return 1;
    }
    icu4x_Locale_destroy_mv1(locale);

    // Test setting the region
    write = diplomat_simple_write(output, 40);
    str = "es-ES-u-hc-h23";
    locale_result = icu4x_Locale_from_string_mv1(str, strlen(str));
    if (!locale_result.is_ok) {
        printf("Could not create the locale.");
        return 1;
    }
    locale = locale_result.ok;
    if (!test_locale(locale, "The region starts as es-ES", "es-ES-u-hc-h23")) {
        return 1;
    }
    str = "MX";
    icu4x_Locale_set_region_mv1_result r_result = icu4x_Locale_set_region_mv1(locale, str, strlen(str));
    if (!r_result.is_ok) {
        printf("Could not set the region.");
        return 1;
    }
    if (!test_locale(locale, "setting the region", "es-MX-u-hc-h23")) {
        return 1;
    }
    icu4x_Locale_destroy_mv1(locale);

     // Test setting the script
    write = diplomat_simple_write(output, 40);
    str = "en-US";
    locale_result = icu4x_Locale_from_string_mv1(str, strlen(str));
    if (!locale_result.is_ok) {
        printf("Could not create the locale.");
        return 1;
    }
    locale = locale_result.ok;
    if (!test_locale(locale, "initial script-less locale", "en-US")) {
        return 1;
    }
    str = "Latn";
    icu4x_Locale_set_script_mv1_result s_result = icu4x_Locale_set_script_mv1(locale, str, strlen(str));
    if (!s_result.is_ok) {
        printf("Could not set the script.");
        return 1;
    }
    if (!test_locale(locale, "setting the script", "en-Latn-US")) {
        return 1;
    }
    s_result = icu4x_Locale_set_script_mv1(locale, "", 0);
    if (!s_result.is_ok) {
        printf("Could not set the script.");
        return 1;
    }
    if (!test_locale(locale, "removing the script", "en-US")) {
        return 1;
    }

    icu4x_Locale_destroy_mv1(locale);

    // Create a LocaleCanonicalizer and LocaleExpander.
    DataProvider* provider = icu4x_DataProvider_compiled_mv1();
    icu4x_LocaleCanonicalizer_create_mv1_result result2 = icu4x_LocaleCanonicalizer_create_mv1(provider);
    if (!result2.is_ok) {
        printf("Could not construct Locale Canonicalizer");
        return 1;
    }
    LocaleCanonicalizer* lc = result2.ok;
    icu4x_LocaleExpander_create_mv1_result result3 = icu4x_LocaleExpander_create_mv1(provider);
    if (!result3.is_ok) {
        printf("Could not construct Locale Canonicalizer");
        return 1;
    }
    LocaleExpander* le = result3.ok;

    // Test maximize.
    write = diplomat_simple_write(output, 40);
    locale_result = icu4x_Locale_from_string_mv1("und", 3);
    if (!locale_result.is_ok) {
        printf("Could not create the locale.");
        return 1;
    }
    locale = locale_result.ok;
    icu4x_LocaleExpander_maximize_mv1(le, locale);
    icu4x_Locale_to_string_mv1(locale, &write);
    if (write.grow_failed) {
        return 1;
    }
    printf("Output for maximizing is %s\n", output);
    expected = u8"en-Latn-US";
    if (strcmp(output, expected) != 0) {
        printf("Output does not match expected output!\n");
        return 1;
    }
    icu4x_Locale_destroy_mv1(locale);

    // Test minimize.
    write = diplomat_simple_write(output, 40);
    locale_result = icu4x_Locale_from_string_mv1("zh-Hant", 7);
    if (!locale_result.is_ok) {
        printf("Could not create the locale.");
        return 1;
    }
    locale = locale_result.ok;
    icu4x_LocaleExpander_minimize_mv1(le, locale);
    icu4x_Locale_to_string_mv1(locale, &write);
    if (write.grow_failed) {
        return 1;
    }
    printf("Output for minimizing is %s\n", output);
    expected = u8"zh-TW";
    if (strcmp(output, expected) != 0) {
        printf("Output does not match expected output!\n");
        return 1;
    }
    icu4x_Locale_destroy_mv1(locale);

    // Test canonicalize.
    write = diplomat_simple_write(output, 40);
    locale_result = icu4x_Locale_from_string_mv1("no-nynorsk", 10);
    if (!locale_result.is_ok) {
        printf("Could not create the locale.");
        return 1;
    }
    locale = locale_result.ok;
    icu4x_LocaleCanonicalizer_canonicalize_mv1(lc, locale);
    icu4x_Locale_to_string_mv1(locale, &write);
    if (write.grow_failed) {
        return 1;
    }
    printf("Output for canonicalizing is %s\n", output);
    expected = u8"nn";
    if (strcmp(output, expected) != 0) {
        printf("Output does not match expected output!\n");
        return 1;
    }
    icu4x_Locale_destroy_mv1(locale);

    icu4x_LocaleCanonicalizer_destroy_mv1(lc);
    icu4x_LocaleExpander_destroy_mv1(le);

    return 0;
}
