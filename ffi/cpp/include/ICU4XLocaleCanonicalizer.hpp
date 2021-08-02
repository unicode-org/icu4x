#ifndef ICU4XLocaleCanonicalizer_HPP
#define ICU4XLocaleCanonicalizer_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <optional>
#include <variant>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XLocaleCanonicalizer.h"
}

class ICU4XDataProvider;
class ICU4XLocaleCanonicalizer;
class ICU4XLocale;
enum struct ICU4XCanonicalizationResult;

struct ICU4XLocaleCanonicalizerDeleter {
  void operator()(capi::ICU4XLocaleCanonicalizer* l) const noexcept {
    capi::ICU4XLocaleCanonicalizer_destroy(l);
  }
};
class ICU4XLocaleCanonicalizer {
 public:
  static std::optional<ICU4XLocaleCanonicalizer> create(const ICU4XDataProvider& provider);
  ICU4XCanonicalizationResult canonicalize(ICU4XLocale& locale);
  ICU4XCanonicalizationResult maximize(ICU4XLocale& locale);
  ICU4XCanonicalizationResult minimize(ICU4XLocale& locale);
  inline const capi::ICU4XLocaleCanonicalizer* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XLocaleCanonicalizer* AsFFIMut() { return this->inner.get(); }
  ICU4XLocaleCanonicalizer(capi::ICU4XLocaleCanonicalizer* i) : inner(i) {}
 private:
  std::unique_ptr<capi::ICU4XLocaleCanonicalizer, ICU4XLocaleCanonicalizerDeleter> inner;
};

#include "ICU4XDataProvider.hpp"
#include "ICU4XLocale.hpp"
#include "ICU4XCanonicalizationResult.hpp"

std::optional<ICU4XLocaleCanonicalizer> ICU4XLocaleCanonicalizer::create(const ICU4XDataProvider& provider) {
  auto diplomat_optional_raw_out_value = capi::ICU4XLocaleCanonicalizer_create(provider.AsFFI());
  std::optional<ICU4XLocaleCanonicalizer> diplomat_optional_out_value;
  if (diplomat_optional_raw_out_value != nullptr) {
    diplomat_optional_out_value = ICU4XLocaleCanonicalizer(diplomat_optional_raw_out_value);
  } else {
    diplomat_optional_out_value = std::nullopt;
  }
  return diplomat_optional_out_value;
}
ICU4XCanonicalizationResult ICU4XLocaleCanonicalizer::canonicalize(ICU4XLocale& locale) {
  return ICU4XCanonicalizationResult{ capi::ICU4XLocaleCanonicalizer_canonicalize(this->inner.get(), locale.AsFFIMut()) };
}
ICU4XCanonicalizationResult ICU4XLocaleCanonicalizer::maximize(ICU4XLocale& locale) {
  return ICU4XCanonicalizationResult{ capi::ICU4XLocaleCanonicalizer_maximize(this->inner.get(), locale.AsFFIMut()) };
}
ICU4XCanonicalizationResult ICU4XLocaleCanonicalizer::minimize(ICU4XLocale& locale) {
  return ICU4XCanonicalizationResult{ capi::ICU4XLocaleCanonicalizer_minimize(this->inner.get(), locale.AsFFIMut()) };
}
#endif
