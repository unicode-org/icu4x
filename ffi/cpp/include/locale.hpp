// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#ifndef ICU4X_LOCALE_HPP
#define ICU4X_LOCALE_HPP

#include <memory>
#include <string_view>
#include <utility>

#include "../../capi/include/locale.h"
#include "writeable_utils.hpp"

namespace icu4x {
struct ICU4XLocaleDeleter {
  void operator()(ICU4XLocale* l) const noexcept { icu4x_locale_destroy(l); }
};
class Locale {
 public:
  Locale(const std::string_view& value)
      : inner(icu4x_locale_create(value.data(), value.size())) {}
  std::optional<std::string> ToString() const {
    std::string out;
    ICU4XWriteable writer = icu4x::internal::WriteableFromString(out);
    bool success = icu4x_locale_tostring(this->inner.get(), &writer);
    if (!success) {
      return {};
    }
    return out;
  }
  inline const ICU4XLocale* AsFFI() const { return this->inner.get(); }

 private:
  std::unique_ptr<ICU4XLocale, ICU4XLocaleDeleter> inner;
};
}  // namespace icu4x

#endif  // ICU4X_LOCALE_HPP
