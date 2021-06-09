// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#ifndef ICU4X_FIXED_DECIMAL_HPP
#define ICU4X_FIXED_DECIMAL_HPP

#include <algorithm>
#include <memory>

#include "../../capi/include/fixed_decimal.h"

namespace icu4x {
struct ICU4XFixedDecimalDeleter {
  void operator()(ICU4XFixedDecimal* l) const noexcept {
    icu4x_fixed_decimal_destroy(l);
  }
};
class FixedDecimal {
 public:
  FixedDecimal(int64_t number)
      : FixedDecimal(icu4x_fixed_decimal_create(number)) {}
  void MultiplyPow10(int16_t power) {
    icu4x_fixed_decimal_multiply_pow10(this->inner.get(), power);
  }
  void Negate() { icu4x_fixed_decimal_negate(this->inner.get()); }
  inline const ICU4XFixedDecimal* AsFFI() const { return this->inner.get(); }

 private:
  FixedDecimal(ICU4XFixedDecimal* i) : inner(i) {}
  std::unique_ptr<ICU4XFixedDecimal, ICU4XFixedDecimalDeleter> inner;
};
}  // namespace icu4x

#endif  // ICU4X_FIXED_DECIMAL_HPP
