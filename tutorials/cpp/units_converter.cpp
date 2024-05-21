// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include "ICU4XDataProvider.hpp"
#include "ICU4XMeasureUnit.hpp"
#include "ICU4XMeasureUnitParser.hpp"
#include "ICU4XUnitsConverter.hpp"
#include "ICU4XUnitsConverterFactory.hpp"

#include <iostream>

int main() {
  ICU4XDataProvider dp = ICU4XDataProvider::create_compiled();
  auto converter_factory = ICU4XUnitsConverterFactory::create(dp).ok().value();
  auto parser = converter_factory.parser();
  auto from = parser.parse_valid_utf8_unit_identifier("meter").ok().value();
  auto to = parser.parse_valid_utf8_unit_identifier("foot").ok().value();
  auto converter = converter_factory.converter(from, to).value();
  auto result = converter.convert_f64(1.0);

  auto converter_cloned = converter.clone();
  auto result_cloned = converter_cloned.convert_f64(1.0);

  if (std::abs(result - 3.28084) > 0.00001 &&
      std::abs(result_cloned - 3.28084) > 0.00001) {
    std::cout << "Conversion failed" << std::endl;
    return 1;
  } else {
    std::cout << "Conversion succeeded" << std::endl;
    return 0;
  }
}