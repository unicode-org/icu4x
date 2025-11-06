// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include <icu4x/Locale.hpp>
#include <icu4x/Logger.hpp>

#include <iostream>

using namespace icu4x;

static bool test_locale(Locale &locale, std::string_view expectedString,
                        const char *message) {
  std::string actualString = locale.to_string();
  std::cout << message << ": \"" << actualString << "\"" << std::endl;
  if (actualString != expectedString) {
    std::cout << "Locale did not match expected: \"" << expectedString << "\""
              << std::endl;
    return false;
  }
  return true;
}

static bool test_string(std::string_view actualString,
                        std::string_view expectedString, const char *message) {
  std::cout << message << ": \"" << actualString << "\"" << std::endl;
  if (actualString != expectedString) {
    std::cout << "String did not match expected: \"" << expectedString << "\""
              << std::endl;
    return false;
  }
  return true;
}

int main() {
  Logger::init_simple_logger();
  std::unique_ptr<Locale> locale = Locale::from_string("es-ES").ok().value();
  if (!test_locale(*locale.get(), "es-ES", "Created a locale")) {
    return 1;
  }

  locale->set_language("en").ok();
  if (!test_locale(*locale.get(), "en-ES", "The language can be updated")) {
    return 1;
  }

  locale->set_region("US").ok();
  if (!test_locale(*locale.get(), "en-US", "The region can be updated")) {
    return 1;
  }

  locale->set_script("Latn").ok();
  if (!test_locale(*locale.get(), "en-Latn-US", "The script can be updated")) {
    return 1;
  }

  if (!test_string(locale->language(), "en",
                   "The language can be accessed")) {
    return 1;
  }
  if (!test_string(locale->region().value(), "US",
                   "The region can be accessed")) {
    return 1;
  }
  if (!test_string(locale->script().value(), "Latn",
                   "The script can be accessed")) {
    return 1;
  }

  locale->set_language("").ok();
  if (!test_locale(*locale.get(), "und-Latn-US", "Removed the language")) {
    return 1;
  }

  locale->set_region("").ok();
  if (locale->region().has_value()) {
    std::cout << "Expected region to be an err" << std::endl;
    return 1;
  }
  if (!test_locale(*locale.get(), "und-Latn", "Removed the region")) {
    return 1;
  }

  locale->set_script("").ok();
  if (locale->script().has_value()) {
    std::cout << "Expected script to be an err" << std::endl;
    return 1;
  }
  if (!test_locale(*locale.get(), "und", "Removed the script")) {
    return 1;
  }

  locale = Locale::from_string("en-US-u-hc-h12").ok().value();
  if (!test_string(locale->get_unicode_extension("hc").value(), "h12",
                   "The unicode extension can be accessed")) {
    return 1;
  }
  if (!test_string(locale->basename(), "en-US",
                   "The basename can be accessed")) {
    return 1;
  }

  locale = Locale::unknown();
  if (!test_locale(*locale.get(), "und", "Created an undefined locale")) {
    return 1;
  }

  return 0;
}
