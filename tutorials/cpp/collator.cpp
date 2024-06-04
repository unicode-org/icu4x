// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include "ICU4XCollator.hpp"
#include "ICU4XDataProvider.hpp"
#include "ICU4XLocale.hpp"
#include "ICU4XLogger.hpp"

#include <iostream>
#include <string_view>

int main() {
  ICU4XLogger::init_simple_logger();
  std::unique_ptr<ICU4XDataProvider> dp = ICU4XDataProvider::create_compiled();

  // test 01 - basic collation example, default CollatorOptions

  std::string_view manna{ "manna" };
  std::string_view manana{ "mañana" };

  std::unique_ptr<ICU4XLocale> locale = ICU4XLocale::create_from_string("en").ok().value();
  ICU4XCollatorOptionsV1 options = ICU4XCollatorOptionsV1();
  auto foo = ICU4XCollatorStrength();
  std::unique_ptr<ICU4XCollator> collator = ICU4XCollator::create_v1(*dp.get(), *locale.get(), options).ok().value();
  auto actual = collator->compare(manna, manana);

  if (actual != 1) {
    std::cout << "Expected manna > mañana for locale " << locale->to_string() << std::endl;
    return 1;
  }

  locale = ICU4XLocale::create_from_string("es").ok().value();
  collator = ICU4XCollator::create_v1(*dp.get(), *locale.get(), options).ok().value();
  actual = collator->compare(manna, manana);

  if (actual != -1) {
    std::cout << "Expected manna < mañana for locale " << locale->to_string() << std::endl;
    return 1;
  }

  // test 02 - collation strength example, requires non-default CollatorOptions

  std::string_view as{ "as" };
  std::string_view graveAs{ "às" };

  locale = ICU4XLocale::create_from_string("en").ok().value();
  options.strength = ICU4XCollatorStrength::Primary;
  collator = ICU4XCollator::create_v1(*dp.get(), *locale.get(), options).ok().value();
  actual = collator->compare(as, graveAs);

  if (actual != 0) {
    std::cout << "Expected as = às for primary strength, locale " << locale->to_string() << std::endl;
    return 1;
  }

  options.strength = ICU4XCollatorStrength::Secondary;
  collator = ICU4XCollator::create_v1(*dp.get(), *locale.get(), options).ok().value();
  actual = collator->compare(as, graveAs);

  if (actual != -1) {
    std::cout << "Expected as < às for secondary strength, locale " << locale->to_string() << std::endl;
    return 1;
  }

  return 0;
}
