#ifndef ICU4XLocaleCanonicalizer_HPP
#define ICU4XLocaleCanonicalizer_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XLocaleCanonicalizer.h"
}

class ICU4XDataProvider;
class ICU4XLocaleCanonicalizer;
class ICU4XLocale;
#include "ICU4XCanonicalizationResult.hpp"

/**
 * A destruction policy for using ICU4XLocaleCanonicalizer with std::unique_ptr.
 */
struct ICU4XLocaleCanonicalizerDeleter {
  void operator()(capi::ICU4XLocaleCanonicalizer* l) const noexcept {
    capi::ICU4XLocaleCanonicalizer_destroy(l);
  }
};
class ICU4XLocaleCanonicalizer {
 public:

  /**
   * Create a new [`ICU4XLocaleCanonicalizer`].
   * See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu/locale_canonicalizer/struct.LocaleCanonicalizer.html#method.new) for more details.
   */
  static std::optional<ICU4XLocaleCanonicalizer> create(const ICU4XDataProvider& provider);

  /**
   * FFI version of `LocaleCanonicalizer::canonicalize()`.
   * See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu/locale_canonicalizer/struct.LocaleCanonicalizer.html#method.canonicalize) for more details.
   */
  ICU4XCanonicalizationResult canonicalize(ICU4XLocale& locale) const;

  /**
   * FFI version of `LocaleCanonicalizer::maximize()`.
   * See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu/locale_canonicalizer/struct.LocaleCanonicalizer.html#method.maximize) for more details.
   */
  ICU4XCanonicalizationResult maximize(ICU4XLocale& locale) const;

  /**
   * FFI version of `LocaleCanonicalizer::minimize()`.
   * See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu/locale_canonicalizer/struct.LocaleCanonicalizer.html#method.minimize) for more details.
   */
  ICU4XCanonicalizationResult minimize(ICU4XLocale& locale) const;
  inline const capi::ICU4XLocaleCanonicalizer* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XLocaleCanonicalizer* AsFFIMut() { return this->inner.get(); }
  inline ICU4XLocaleCanonicalizer(capi::ICU4XLocaleCanonicalizer* i) : inner(i) {}
  ICU4XLocaleCanonicalizer() = default;
  ICU4XLocaleCanonicalizer(ICU4XLocaleCanonicalizer&&) noexcept = default;
  ICU4XLocaleCanonicalizer& operator=(ICU4XLocaleCanonicalizer&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XLocaleCanonicalizer, ICU4XLocaleCanonicalizerDeleter> inner;
};

#include "ICU4XDataProvider.hpp"
#include "ICU4XLocale.hpp"

inline std::optional<ICU4XLocaleCanonicalizer> ICU4XLocaleCanonicalizer::create(const ICU4XDataProvider& provider) {
  auto diplomat_optional_raw_out_value = capi::ICU4XLocaleCanonicalizer_create(provider.AsFFI());
  std::optional<ICU4XLocaleCanonicalizer> diplomat_optional_out_value;
  if (diplomat_optional_raw_out_value != nullptr) {
    diplomat_optional_out_value = ICU4XLocaleCanonicalizer(diplomat_optional_raw_out_value);
  } else {
    diplomat_optional_out_value = std::nullopt;
  }
  return diplomat_optional_out_value;
}
inline ICU4XCanonicalizationResult ICU4XLocaleCanonicalizer::canonicalize(ICU4XLocale& locale) const {
  return static_cast<ICU4XCanonicalizationResult>(capi::ICU4XLocaleCanonicalizer_canonicalize(this->inner.get(), locale.AsFFIMut()));
}
inline ICU4XCanonicalizationResult ICU4XLocaleCanonicalizer::maximize(ICU4XLocale& locale) const {
  return static_cast<ICU4XCanonicalizationResult>(capi::ICU4XLocaleCanonicalizer_maximize(this->inner.get(), locale.AsFFIMut()));
}
inline ICU4XCanonicalizationResult ICU4XLocaleCanonicalizer::minimize(ICU4XLocale& locale) const {
  return static_cast<ICU4XCanonicalizationResult>(capi::ICU4XLocaleCanonicalizer_minimize(this->inner.get(), locale.AsFFIMut()));
}
#endif
