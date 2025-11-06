// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include <icu4x/Collator.hpp>
#include <icu4x/Locale.hpp>
#include <icu4x/Logger.hpp>

#include <iostream>
#include <string_view>

using namespace icu4x;

int main() {
  Logger::init_simple_logger();

  // test 01 - basic collation example, default CollatorOptions

  std::string_view manna{ "manna" };
  std::string_view manana{ "mañana" };

  std::unique_ptr<Locale> locale = Locale::from_string("en").ok().value();
  CollatorOptionsV1 options = CollatorOptionsV1();
  auto foo = CollatorStrength();
  std::unique_ptr<Collator> collator = Collator::create_v1(*locale.get(), options).ok().value();
  auto actual = collator->compare(manna, manana);

  if (actual != 1) {
    std::cout << "Expected manna > mañana for locale " << locale->to_string() << std::endl;
    return 1;
  }

  locale = Locale::from_string("es").ok().value();
  collator = Collator::create_v1(*locale.get(), options).ok().value();
  actual = collator->compare(manna, manana);

  if (actual != -1) {
    std::cout << "Expected manna < mañana for locale " << locale->to_string() << std::endl;
    return 1;
  }

  // test 02 - collation strength example, requires non-default CollatorOptions

  std::string_view as{ "as" };
  std::string_view graveAs{ "às" };

  locale = Locale::from_string("en").ok().value();
  options.strength = CollatorStrength::Primary;
  collator = Collator::create_v1(*locale.get(), options).ok().value();
  actual = collator->compare(as, graveAs);

  if (actual != 0) {
    std::cout << "Expected as = às for primary strength, locale " << locale->to_string() << std::endl;
    return 1;
  }

  options.strength = CollatorStrength::Secondary;
  collator = Collator::create_v1(*locale.get(), options).ok().value();
  actual = collator->compare(as, graveAs);

  if (actual != -1) {
    std::cout << "Expected as < às for secondary strength, locale " << locale->to_string() << std::endl;
    return 1;
  }

  return 0;
}
