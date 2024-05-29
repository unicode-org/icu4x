#ifndef ICU4XMeasureUnitParser_HPP
#define ICU4XMeasureUnitParser_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XMeasureUnitParser.h"

class ICU4XMeasureUnit;

/**
 * A destruction policy for using ICU4XMeasureUnitParser with std::unique_ptr.
 */
struct ICU4XMeasureUnitParserDeleter {
  void operator()(capi::ICU4XMeasureUnitParser* l) const noexcept {
    capi::ICU4XMeasureUnitParser_destroy(l);
  }
};

/**
 * An ICU4X Measurement Unit parser object which is capable of parsing the CLDR unit identifier
 * (e.g. `meter-per-square-second`) and get the [`ICU4XMeasureUnit`].
 * 
 * See the [Rust documentation for `MeasureUnitParser`](https://docs.rs/icu/latest/icu/experimental/units/measureunit/struct.MeasureUnitParser.html) for more information.
 */
class ICU4XMeasureUnitParser {
 public:

  /**
   * Parses the CLDR unit identifier (e.g. `meter-per-square-second`) and returns the corresponding [`ICU4XMeasureUnit`],
   * if the identifier is valid.
   * 
   * See the [Rust documentation for `parse`](https://docs.rs/icu/latest/icu/experimental/units/measureunit/struct.MeasureUnitParser.html#method.parse) for more information.
   */
  std::optional<ICU4XMeasureUnit> parse(const std::string_view unit_id) const;
  inline const capi::ICU4XMeasureUnitParser* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XMeasureUnitParser* AsFFIMut() { return this->inner.get(); }
  inline explicit ICU4XMeasureUnitParser(capi::ICU4XMeasureUnitParser* i) : inner(i) {}
  ICU4XMeasureUnitParser() = default;
  ICU4XMeasureUnitParser(ICU4XMeasureUnitParser&&) noexcept = default;
  ICU4XMeasureUnitParser& operator=(ICU4XMeasureUnitParser&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XMeasureUnitParser, ICU4XMeasureUnitParserDeleter> inner;
};

#include "ICU4XMeasureUnit.hpp"

inline std::optional<ICU4XMeasureUnit> ICU4XMeasureUnitParser::parse(const std::string_view unit_id) const {
  auto diplomat_optional_raw_out_value = capi::ICU4XMeasureUnitParser_parse(this->inner.get(), unit_id.data(), unit_id.size());
  std::optional<ICU4XMeasureUnit> diplomat_optional_out_value;
  if (diplomat_optional_raw_out_value != nullptr) {
    diplomat_optional_out_value = ICU4XMeasureUnit(diplomat_optional_raw_out_value);
  } else {
    diplomat_optional_out_value = std::nullopt;
  }
  return diplomat_optional_out_value;
}
#endif
