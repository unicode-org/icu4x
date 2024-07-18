// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include "DataProvider.hpp"
#include "MeasureUnit.hpp"
#include "MeasureUnitParser.hpp"
#include "UnitsConverter.hpp"
#include "UnitsConverterFactory.hpp"

#include <iostream>

int main() {
  auto dp = DataProvider::compiled();
  auto converter_factory = UnitsConverterFactory::create(*dp.get()).ok().value();
  auto parser = converter_factory->parser();
  auto from = parser->parse("meter");
  auto to = parser->parse("foot");
  auto converter = converter_factory->converter(*from.get(), *to.get());
  auto result = converter->convert_double(1.0);

  auto converter_cloned = converter->clone();
  auto result_cloned = converter_cloned->convert_double(1.0);

  if (std::abs(result - 3.28084) > 0.00001 &&
      std::abs(result_cloned - 3.28084) > 0.00001) {
    std::cout << "Conversion failed" << std::endl;
    return 1;
  } else {
    std::cout << "Conversion succeeded" << std::endl;
    return 0;
  }
}