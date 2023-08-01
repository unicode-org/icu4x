#ifndef ICU4XCaseMapCloser_HPP
#define ICU4XCaseMapCloser_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XCaseMapCloser.h"

class ICU4XDataProvider;
class ICU4XCaseMapCloser;
#include "ICU4XError.hpp"
class ICU4XCodePointSetBuilder;

/**
 * A destruction policy for using ICU4XCaseMapCloser with std::unique_ptr.
 */
struct ICU4XCaseMapCloserDeleter {
  void operator()(capi::ICU4XCaseMapCloser* l) const noexcept {
    capi::ICU4XCaseMapCloser_destroy(l);
  }
};

/**
 * 
 * 
 * See the [Rust documentation for `CaseMapCloser`](https://docs.rs/icu/latest/icu/casemap/struct.CaseMapCloser.html) for more information.
 */
class ICU4XCaseMapCloser {
 public:

  /**
   * Construct a new ICU4XCaseMapper instance for NFC
   * 
   * See the [Rust documentation for `new`](https://docs.rs/icu/latest/icu/casemap/struct.CaseMapCloser.html#method.new) for more information.
   */
  static diplomat::result<ICU4XCaseMapCloser, ICU4XError> create(const ICU4XDataProvider& provider);

  /**
   * Adds all simple case mappings and the full case folding for `c` to `builder`.
   * Also adds special case closure mappings.
   * 
   * In other words, this adds all characters that this casemaps to, as
   * well as all characters that may casemap to this one.
   * 
   * Note that since ICU4XCodePointSetBuilder does not contain strings, this will
   * ignore string mappings
   * 
   * Identical to the similarly named method on `ICU4XCaseMapCloser`, use that if you
   * do not plan on using string case closure mappings to limit the amount of data loaded.
   * 
   * See the [Rust documentation for `add_case_closure`](https://docs.rs/icu/latest/icu/casemap/struct.CaseMapCloser.html#method.add_case_closure) for more information.
   */
  void add_case_closure(char32_t c, ICU4XCodePointSetBuilder& builder) const;

  /**
   * Maps the string to single code points and adds the associated case closure
   * mappings, if they exist.
   * 
   * The string is mapped to code points if it is their full case folding string.
   * In other words, this performs a reverse full case folding and then
   * adds the case closure items of the resulting code points.
   * If the string is found and its closure applied, then
   * the string itself is added as well as part of its code points' closure.
   * 
   * Returns true if the string was found
   * 
   * See the [Rust documentation for `add_string_case_closure`](https://docs.rs/icu/latest/icu/casemap/struct.CaseMapCloser.html#method.add_string_case_closure) for more information.
   */
  bool add_string_case_closure(const std::string_view s, ICU4XCodePointSetBuilder& builder) const;
  inline const capi::ICU4XCaseMapCloser* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XCaseMapCloser* AsFFIMut() { return this->inner.get(); }
  inline ICU4XCaseMapCloser(capi::ICU4XCaseMapCloser* i) : inner(i) {}
  ICU4XCaseMapCloser() = default;
  ICU4XCaseMapCloser(ICU4XCaseMapCloser&&) noexcept = default;
  ICU4XCaseMapCloser& operator=(ICU4XCaseMapCloser&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XCaseMapCloser, ICU4XCaseMapCloserDeleter> inner;
};

#include "ICU4XDataProvider.hpp"
#include "ICU4XCodePointSetBuilder.hpp"

inline diplomat::result<ICU4XCaseMapCloser, ICU4XError> ICU4XCaseMapCloser::create(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCaseMapCloser_create(provider.AsFFI());
  diplomat::result<ICU4XCaseMapCloser, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCaseMapCloser>(std::move(ICU4XCaseMapCloser(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline void ICU4XCaseMapCloser::add_case_closure(char32_t c, ICU4XCodePointSetBuilder& builder) const {
  capi::ICU4XCaseMapCloser_add_case_closure(this->inner.get(), c, builder.AsFFIMut());
}
inline bool ICU4XCaseMapCloser::add_string_case_closure(const std::string_view s, ICU4XCodePointSetBuilder& builder) const {
  return capi::ICU4XCaseMapCloser_add_string_case_closure(this->inner.get(), s.data(), s.size(), builder.AsFFIMut());
}
#endif
