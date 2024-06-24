// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include "ICU4XDataProvider.h"
#include "ICU4XLocale.h"
#include "ICU4XLocaleCanonicalizer.h"
#include "ICU4XLocaleExpander.h"
#include "ICU4XLogger.h"
#include <string.h>
#include <stdio.h>

/**
 * A helper for testing the locale with nice error messages.
 */
bool test_locale(ICU4XLocale* locale, const char* message, const char* expected) {
    char output[40];

    // Test setters
    DiplomatWrite write = diplomat_simple_write(output, 40);
    ICU4XLocale_to_string(locale, &write);
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

ICU4XLocale* get_locale(const char* localeText) {
    ICU4XLocale_create_from_string_result locale_result = ICU4XLocale_create_from_string(localeText, strlen(localeText));
    if (!locale_result.is_ok) {
        printf("Could not create locale from: %s", localeText);
    }
    return locale_result.ok;
}


int main() {
    ICU4XLogger_init_simple_logger();
    char output[40];

    ICU4XLocale_create_from_string_result locale_result;

    // Test creating a locale.
    DiplomatWrite write = diplomat_simple_write(output, 40);
    locale_result = ICU4XLocale_create_from_string("ar", 2);
    if (!locale_result.is_ok) {
        return 1;
    }
    ICU4XLocale* locale = locale_result.ok;
    ICU4XLocale_to_string(locale, &write);
    if (write.grow_failed) {
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
    write = diplomat_simple_write(output, 40);
    locale_result = ICU4XLocale_create_from_string("fr-FR-u-hc-h23", 14);
    if (!locale_result.is_ok) {
        return 1;
    }
    locale = locale_result.ok;
    ICU4XLocale_language(locale, &write);
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
    ICU4XLocale_region(locale, &write);
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
    ICU4XLocale_get_unicode_extension_result e_result = ICU4XLocale_get_unicode_extension(locale, "hc", 2, &write);
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
    e_result = ICU4XLocale_get_unicode_extension(locale, "ca", 2, &write);
    if (e_result.is_ok || write.grow_failed) {
        return 1;
    }

    // Test setting the language
    write = diplomat_simple_write(output, 40);
    const char* str = "fr-FR-u-hc-h23";
    locale_result = ICU4XLocale_create_from_string(str, strlen(str));
    if (!locale_result.is_ok) {
        printf("Could not create the locale.");
        return 1;
    }
    locale = locale_result.ok;
    str = "zh";
    ICU4XLocale_set_language_result l_result = ICU4XLocale_set_language(locale, str, strlen(str));
    if (!l_result.is_ok) {
        printf("Could not set the language tag.");
        return 1;
    }
    if (!test_locale(locale, "setting the language tag", "zh-FR-u-hc-h23")) {
        return 1;
    }
    ICU4XLocale_destroy(locale);

    // Test setting the region
    write = diplomat_simple_write(output, 40);
    str = "es-ES-u-hc-h23";
    locale_result = ICU4XLocale_create_from_string(str, strlen(str));
    if (!locale_result.is_ok) {
        printf("Could not create the locale.");
        return 1;
    }
    locale = locale_result.ok;
    if (!test_locale(locale, "The region starts as es-ES", "es-ES-u-hc-h23")) {
        return 1;
    }
    str = "MX";
    ICU4XLocale_set_region_result r_result = ICU4XLocale_set_region(locale, str, strlen(str));
    if (!r_result.is_ok) {
        printf("Could not set the region.");
        return 1;
    }
    if (!test_locale(locale, "setting the region", "es-MX-u-hc-h23")) {
        return 1;
    }
    ICU4XLocale_destroy(locale);

     // Test setting the script
    write = diplomat_simple_write(output, 40);
    str = "en-US";
    locale_result = ICU4XLocale_create_from_string(str, strlen(str));
    if (!locale_result.is_ok) {
        printf("Could not create the locale.");
        return 1;
    }
    locale = locale_result.ok;
    if (!test_locale(locale, "initial script-less locale", "en-US")) {
        return 1;
    }
    str = "Latn";
    ICU4XLocale_set_script_result s_result = ICU4XLocale_set_script(locale, str, strlen(str));
    if (!s_result.is_ok) {
        printf("Could not set the script.");
        return 1;
    }
    if (!test_locale(locale, "setting the script", "en-Latn-US")) {
        return 1;
    }
    s_result = ICU4XLocale_set_script(locale, "", 0);
    if (!s_result.is_ok) {
        printf("Could not set the script.");
        return 1;
    }
    if (!test_locale(locale, "removing the script", "en-US")) {
        return 1;
    }

    ICU4XLocale_destroy(locale);

    // Create a LocaleCanonicalizer and LocaleExpander.
    ICU4XDataProvider* provider = ICU4XDataProvider_create_compiled();
    ICU4XLocaleCanonicalizer_create_result result2 = ICU4XLocaleCanonicalizer_create(provider);
    if (!result2.is_ok) {
        printf("Could not construct Locale Canonicalizer");
        return 1;
    }
    ICU4XLocaleCanonicalizer* lc = result2.ok;
    ICU4XLocaleExpander_create_result result3 = ICU4XLocaleExpander_create(provider);
    if (!result3.is_ok) {
        printf("Could not construct Locale Canonicalizer");
        return 1;
    }
    ICU4XLocaleExpander* le = result3.ok;

    // Test maximize.
    write = diplomat_simple_write(output, 40);
    locale_result = ICU4XLocale_create_from_string("und", 3);
    if (!locale_result.is_ok) {
        printf("Could not create the locale.");
        return 1;
    }
    locale = locale_result.ok;
    ICU4XLocaleExpander_maximize(le, locale);
    ICU4XLocale_to_string(locale, &write);
    if (write.grow_failed) {
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
    write = diplomat_simple_write(output, 40);
    locale_result = ICU4XLocale_create_from_string("zh-Hant", 7);
    if (!locale_result.is_ok) {
        printf("Could not create the locale.");
        return 1;
    }
    locale = locale_result.ok;
    ICU4XLocaleExpander_minimize(le, locale);
    ICU4XLocale_to_string(locale, &write);
    if (write.grow_failed) {
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
    write = diplomat_simple_write(output, 40);
    locale_result = ICU4XLocale_create_from_string("no-nynorsk", 10);
    if (!locale_result.is_ok) {
        printf("Could not create the locale.");
        return 1;
    }
    locale = locale_result.ok;
    ICU4XLocaleCanonicalizer_canonicalize(lc, locale);
    ICU4XLocale_to_string(locale, &write);
    if (write.grow_failed) {
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
    ICU4XLocaleExpander_destroy(le);

    return 0;
}
