#ifndef ICU4XLanguageDisplayNames_HPP
#define ICU4XLanguageDisplayNames_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XLanguageDisplayNames.h"

class ICU4XDataProvider;
class ICU4XLocale;
class ICU4XLanguageDisplayNames;
#include "ICU4XError.hpp"

/**
 * A destruction policy for using ICU4XLanguageDisplayNames with std::unique_ptr.
 */
struct ICU4XLanguageDisplayNamesDeleter {
  void operator()(capi::ICU4XLanguageDisplayNames* l) const noexcept {
    capi::ICU4XLanguageDisplayNames_destroy(l);
  }
};

/**
 * 
 * 
 * See the [Rust documentation for `LanguageDisplayNames`](https://unicode-org.github.io/icu4x-docs/doc/icu/displaynames/struct.LanguageDisplayNames.html) for more information.
 */
class ICU4XLanguageDisplayNames {
 public:

  /**
   * Creates a new `LanguageDisplayNames` from locale data and an options bag.
   * 
   * See the [Rust documentation for `try_new_unstable`](https://unicode-org.github.io/icu4x-docs/doc/icu/displaynames/struct.LanguageDisplayNames.html#method.try_new_unstable) for more information.
   */
  static diplomat::result<ICU4XLanguageDisplayNames, ICU4XError> try_new_unstable(const ICU4XDataProvider& provider, const ICU4XLocale& locale);
  inline const capi::ICU4XLanguageDisplayNames* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XLanguageDisplayNames* AsFFIMut() { return this->inner.get(); }
  inline ICU4XLanguageDisplayNames(capi::ICU4XLanguageDisplayNames* i) : inner(i) {}
  ICU4XLanguageDisplayNames() = default;
  ICU4XLanguageDisplayNames(ICU4XLanguageDisplayNames&&) noexcept = default;
  ICU4XLanguageDisplayNames& operator=(ICU4XLanguageDisplayNames&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XLanguageDisplayNames, ICU4XLanguageDisplayNamesDeleter> inner;
};

#include "ICU4XDataProvider.hpp"
#include "ICU4XLocale.hpp"

inline diplomat::result<ICU4XLanguageDisplayNames, ICU4XError> ICU4XLanguageDisplayNames::try_new_unstable(const ICU4XDataProvider& provider, const ICU4XLocale& locale) {
  auto diplomat_result_raw_out_value = capi::ICU4XLanguageDisplayNames_try_new_unstable(provider.AsFFI(), locale.AsFFI());
  diplomat::result<ICU4XLanguageDisplayNames, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XLanguageDisplayNames>(std::move(ICU4XLanguageDisplayNames(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
#endif
