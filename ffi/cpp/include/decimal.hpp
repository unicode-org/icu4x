// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#ifndef ICU4X_DECIMAL_HPP
#define ICU4X_DECIMAL_HPP

#include <algorithm>
#include <memory>

#include "../../capi/include/decimal.h"
#include "fixed_decimal.hpp"
#include "locale.hpp"
#include "provider.hpp"
#include "writeable_utils.hpp"

namespace icu4x {
struct ICU4XFixedDecimalFormatDeleter {
  void operator()(ICU4XFixedDecimalFormat* l) const noexcept {
    icu4x_fixed_decimal_format_destroy(l);
  }
};
enum class GroupingStrategy {
  Auto = ICU4XGroupingStrategy_Auto,
  Never = ICU4XGroupingStrategy_Never,
  Always = ICU4XGroupingStrategy_Always,
  Min2 = ICU4XGroupingStrategy_Min2,
};

enum class SignDisplay {
  Auto = ICU4XSignDisplay_Auto,
  Never = ICU4XSignDisplay_Never,
  Always = ICU4XSignDisplay_Always,
  ExceptZero = ICU4XSignDisplay_ExceptZero,
  Negative = ICU4XSignDisplay_Negative,
};

struct FixedDecimalFormatOptions {
  GroupingStrategy grouping_strategy;
  SignDisplay sign_display;
};

class FixedDecimalFormat {
 public:
  static std::optional<FixedDecimalFormat> Create(
      const Locale& locale, const DataProvider& provider,
      FixedDecimalFormatOptions opts) {
    ICU4XDataProvider dp = provider.AsFFI();
    ICU4XFixedDecimalFormatOptions opts_ffi = {
        static_cast<ICU4XGroupingStrategy>(opts.grouping_strategy),
        static_cast<ICU4XSignDisplay>(opts.sign_display)};
    ICU4XCreateFixedDecimalFormatResult result =
        icu4x_fixed_decimal_format_create(locale.AsFFI(), &dp, opts_ffi);
    if (!result.success) {
      return {};
    }
    return FixedDecimalFormat(result.fdf);
  }

  std::optional<std::string> Format(const FixedDecimal& dec) {
    std::string out;
    ICU4XWriteable writer = icu4x::internal::WriteableFromString(out);
    bool success = icu4x_fixed_decimal_format_write(this->inner.get(),
                                                    dec.AsFFI(), &writer);
    if (!success) {
      return {};
    }
    return out;
  }

 private:
  FixedDecimalFormat(ICU4XFixedDecimalFormat* i) : inner(i) {}
  std::unique_ptr<ICU4XFixedDecimalFormat, ICU4XFixedDecimalFormatDeleter>
      inner;
};
}  // namespace icu4x

#endif  // ICU4X_DECIMAL_HPP
