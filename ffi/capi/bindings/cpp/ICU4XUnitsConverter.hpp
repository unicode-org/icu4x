#ifndef ICU4XUnitsConverter_HPP
#define ICU4XUnitsConverter_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XUnitsConverter.h"


/**
 * A destruction policy for using ICU4XUnitsConverter with std::unique_ptr.
 */
struct ICU4XUnitsConverterDeleter {
  void operator()(capi::ICU4XUnitsConverter* l) const noexcept {
    capi::ICU4XUnitsConverter_destroy(l);
  }
};

/**
 * An ICU4X Units Converter object, capable of converting between two [`ICU4XMeasureUnit`]s.
 * 
 * You can create an instance of this object using [`ICU4XUnitsConverterFactory`] by calling the `converter` method.
 * 
 * See the [Rust documentation for `UnitsConverter`](https://docs.rs/icu/latest/icu/experimental/units/converter/struct.UnitsConverter.html) for more information.
 */
class ICU4XUnitsConverter {
 public:

  /**
   * Converts the input value in float from the input unit to the output unit (that have been used to create this converter).
   * NOTE:
   * The conversion using floating-point operations is not as accurate as the conversion using ratios.
   * 
   * See the [Rust documentation for `convert_f64`](https://docs.rs/icu/latest/icu/experimental/units/converter/struct.UnitsConverter.html#method.convert_f64) for more information.
   */
  double convert_f64(double value) const;
  inline const capi::ICU4XUnitsConverter* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XUnitsConverter* AsFFIMut() { return this->inner.get(); }
  inline explicit ICU4XUnitsConverter(capi::ICU4XUnitsConverter* i) : inner(i) {}
  ICU4XUnitsConverter() = default;
  ICU4XUnitsConverter(ICU4XUnitsConverter&&) noexcept = default;
  ICU4XUnitsConverter& operator=(ICU4XUnitsConverter&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XUnitsConverter, ICU4XUnitsConverterDeleter> inner;
};


inline double ICU4XUnitsConverter::convert_f64(double value) const {
  return capi::ICU4XUnitsConverter_convert_f64(this->inner.get(), value);
}
#endif
