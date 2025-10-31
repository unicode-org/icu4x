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
bool test_locale(Locale *locale, const char *message, const char *expected)
{
    char output[40];

    // Test setters
    DiplomatWrite write = diplomat_simple_write(output, 40);
    icu4x_Locale_to_string_mv1(locale, &write);
    if (write.grow_failed)
    {
        return 1;
    }
    printf("Output for %s is \"%s\"\n", message, output);
    if (strcmp(output, expected) != 0)
    {
        printf("Output does not match expected output!\n");
        return false;
    }
    return true;
}

Locale *get_locale(const char *localeText)
{
    struct DiplomatStringView locale_text_str = {
        localeText,
        strlen(localeText)};

    icu4x_Locale_from_string_mv1_result locale_result = icu4x_Locale_from_string_mv1(locale_text_str);
    if (!locale_result.is_ok)
    {
        printf("Could not create locale from: %s", localeText);
    }
    return locale_result.ok;
}

int c_main()
{
    icu4x_Logger_init_simple_logger_mv1();
    char output[40];

    icu4x_Locale_from_string_mv1_result locale_result;

    // Test creating a locale.
    DiplomatWrite write = diplomat_simple_write(output, 40);

    struct DiplomatStringView ar_str = {
        "ar",
        2};

    locale_result = icu4x_Locale_from_string_mv1(ar_str);
    if (!locale_result.is_ok)
    {
        return 1;
    }
    Locale *locale = locale_result.ok;
    icu4x_Locale_to_string_mv1(locale, &write);
    if (write.grow_failed)
    {
        return 1;
    }
    printf("Output for a basic locale is %s\n", output);
    const char *expected = u8"ar";
    if (strcmp(output, expected) != 0)
    {
        printf("Output does not match expected output!\n");
        return 1;
    }
    icu4x_Locale_destroy_mv1(locale);

    // Test some accessors.
    write = diplomat_simple_write(output, 40);

    struct DiplomatStringView fr_str = {
        "fr-FR-u-hc-h23",
        14};

    locale_result = icu4x_Locale_from_string_mv1(fr_str);
    if (!locale_result.is_ok)
    {
        return 1;
    }
    locale = locale_result.ok;
    icu4x_Locale_language_mv1(locale, &write);
    if (write.grow_failed)
    {
        return 1;
    }
    printf("Output for the language is %s\n", output);
    expected = u8"fr";
    if (strcmp(output, expected) != 0)
    {
        printf("Output does not match expected output!\n");
        return 1;
    }

    write = diplomat_simple_write(output, 40);
    icu4x_Locale_region_mv1(locale, &write);
    if (write.grow_failed)
    {
        return 1;
    }
    printf("Output for the region is %s\n", output);
    expected = u8"FR";
    if (strcmp(output, expected) != 0)
    {
        printf("Output does not match expected output!\n");
        return 1;
    }

    write = diplomat_simple_write(output, 40);
    struct DiplomatStringView hc_str = {
        "hc",
        2};
    icu4x_Locale_get_unicode_extension_mv1_result e_result = icu4x_Locale_get_unicode_extension_mv1(locale, hc_str, &write);
    if (!e_result.is_ok || write.grow_failed)
    {
        return 1;
    }
    printf("Output for the extension is %s\n", output);
    expected = u8"h23";
    if (strcmp(output, expected) != 0)
    {
        printf("Output does not match expected output!\n");
        return 1;
    }

    write = diplomat_simple_write(output, 40);
    struct DiplomatStringView ca_str = {
        "ca",
        2};
    e_result = icu4x_Locale_get_unicode_extension_mv1(locale, ca_str, &write);
    if (e_result.is_ok || write.grow_failed)
    {
        return 1;
    }

    // Test setting the language
    write = diplomat_simple_write(output, 40);
    const char *str = "fr-FR-u-hc-h23";
    struct DiplomatStringView view_from_str = {
        str,
        strlen(str)};
    locale_result = icu4x_Locale_from_string_mv1(view_from_str);
    if (!locale_result.is_ok)
    {
        printf("Could not create the locale.");
        return 1;
    }
    locale = locale_result.ok;
    str = "zh";
    struct DiplomatStringView str_view = {
        str,
        strlen(str)};
    icu4x_Locale_set_language_mv1_result l_result = icu4x_Locale_set_language_mv1(locale, str_view);
    if (!l_result.is_ok)
    {
        printf("Could not set the language tag.");
        return 1;
    }
    if (!test_locale(locale, "setting the language tag", "zh-FR-u-hc-h23"))
    {
        return 1;
    }
    icu4x_Locale_destroy_mv1(locale);

    // Test setting the region
    write = diplomat_simple_write(output, 40);
    str = "es-ES-u-hc-h23";
    struct DiplomatStringView es_str = {
        str,
        strlen(str)};
    locale_result = icu4x_Locale_from_string_mv1(es_str);
    if (!locale_result.is_ok)
    {
        printf("Could not create the locale.");
        return 1;
    }
    locale = locale_result.ok;
    if (!test_locale(locale, "The region starts as es-ES", "es-ES-u-hc-h23"))
    {
        return 1;
    }
    str = "MX";
    struct DiplomatStringView mx_str = {
        str,
        strlen(str)};
    icu4x_Locale_set_region_mv1_result r_result = icu4x_Locale_set_region_mv1(locale, mx_str);
    if (!r_result.is_ok)
    {
        printf("Could not set the region.");
        return 1;
    }
    if (!test_locale(locale, "setting the region", "es-MX-u-hc-h23"))
    {
        return 1;
    }
    icu4x_Locale_destroy_mv1(locale);

    // Test setting the script
    write = diplomat_simple_write(output, 40);
    str = "en-US";
    struct DiplomatStringView en_str = {
        str,
        strlen(str)};
    locale_result = icu4x_Locale_from_string_mv1(en_str);
    if (!locale_result.is_ok)
    {
        printf("Could not create the locale.");
        return 1;
    }
    locale = locale_result.ok;
    if (!test_locale(locale, "initial script-less locale", "en-US"))
    {
        return 1;
    }
    str = "Latn";
    struct DiplomatStringView latn_str = {
        str,
        strlen(str)};
    icu4x_Locale_set_script_mv1_result s_result = icu4x_Locale_set_script_mv1(locale, latn_str);
    if (!s_result.is_ok)
    {
        printf("Could not set the script.");
        return 1;
    }
    if (!test_locale(locale, "setting the script", "en-Latn-US"))
    {
        return 1;
    }
    struct DiplomatStringView empty_str = {
        "",
        0};
    s_result = icu4x_Locale_set_script_mv1(locale, empty_str);
    if (!s_result.is_ok)
    {
        printf("Could not set the script.");
        return 1;
    }
    if (!test_locale(locale, "removing the script", "en-US"))
    {
        return 1;
    }

    icu4x_Locale_destroy_mv1(locale);

    // Create a LocaleCanonicalizer and LocaleExpander.
    LocaleCanonicalizer *lc = icu4x_LocaleCanonicalizer_create_extended_mv1();

    LocaleExpander *le = icu4x_LocaleExpander_create_extended_mv1();

    // Test maximize.
    write = diplomat_simple_write(output, 40);
    struct DiplomatStringView und_str = {
        "und",
        3};
    locale_result = icu4x_Locale_from_string_mv1(und_str);
    if (!locale_result.is_ok)
    {
        printf("Could not create the locale.");
        return 1;
    }
    locale = locale_result.ok;
    icu4x_LocaleExpander_maximize_mv1(le, locale);
    icu4x_Locale_to_string_mv1(locale, &write);
    if (write.grow_failed)
    {
        return 1;
    }
    printf("Output for maximizing is %s\n", output);
    expected = u8"en-Latn-US";
    if (strcmp(output, expected) != 0)
    {
        printf("Output does not match expected output!\n");
        return 1;
    }
    icu4x_Locale_destroy_mv1(locale);

    // Test minimize.
    write = diplomat_simple_write(output, 40);
    struct DiplomatStringView zh_str = {
        "zh-Hant",
        7};
    locale_result = icu4x_Locale_from_string_mv1(zh_str);
    if (!locale_result.is_ok)
    {
        printf("Could not create the locale.");
        return 1;
    }
    locale = locale_result.ok;
    icu4x_LocaleExpander_minimize_mv1(le, locale);
    icu4x_Locale_to_string_mv1(locale, &write);
    if (write.grow_failed)
    {
        return 1;
    }
    printf("Output for minimizing is %s\n", output);
    expected = u8"zh-TW";
    if (strcmp(output, expected) != 0)
    {
        printf("Output does not match expected output!\n");
        return 1;
    }
    icu4x_Locale_destroy_mv1(locale);

    // Test canonicalize.
    write = diplomat_simple_write(output, 40);
    struct DiplomatStringView no_str = {
        "no-nynorsk",
        10};
    locale_result = icu4x_Locale_from_string_mv1(no_str);
    if (!locale_result.is_ok)
    {
        printf("Could not create the locale.");
        return 1;
    }
    locale = locale_result.ok;
    icu4x_LocaleCanonicalizer_canonicalize_mv1(lc, locale);
    icu4x_Locale_to_string_mv1(locale, &write);
    if (write.grow_failed)
    {
        return 1;
    }
    printf("Output for canonicalizing is %s\n", output);
    expected = u8"nn";
    if (strcmp(output, expected) != 0)
    {
        printf("Output does not match expected output!\n");
        return 1;
    }
    icu4x_Locale_destroy_mv1(locale);

    icu4x_LocaleCanonicalizer_destroy_mv1(lc);
    icu4x_LocaleExpander_destroy_mv1(le);

    return 0;
}
