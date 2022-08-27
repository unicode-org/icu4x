#ifndef ICU4XIsoDate_HPP
#define ICU4XIsoDate_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XIsoDate.h"

class ICU4XIsoDate;
#include "ICU4XError.hpp"
class ICU4XDate;

/**
 * A destruction policy for using ICU4XIsoDate with std::unique_ptr.
 */
struct ICU4XIsoDateDeleter {
  void operator()(capi::ICU4XIsoDate* l) const noexcept {
    capi::ICU4XIsoDate_destroy(l);
  }
};

/**
 * An ICU4X Date object capable of containing a ISO-8601 date
 * 
 * See the [Rust documentation for `Date`](https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html) for more information.
 */
class ICU4XIsoDate {
 public:

  /**
   * Creates a new [`ICU4XIsoDate`] from the specified date and time.
   * 
   * See the [Rust documentation for `new_gregorian_date`](https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.new_gregorian_date) for more information.
   */
  static diplomat::result<ICU4XIsoDate, ICU4XError> try_new(int32_t year, uint8_t month, uint8_t day);

  /**
   * 
   * 
   * See the [Rust documentation for `to_any`](https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.to_any) for more information.
   */
  ICU4XDate to_any() const;
  inline const capi::ICU4XIsoDate* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XIsoDate* AsFFIMut() { return this->inner.get(); }
  inline ICU4XIsoDate(capi::ICU4XIsoDate* i) : inner(i) {}
  ICU4XIsoDate() = default;
  ICU4XIsoDate(ICU4XIsoDate&&) noexcept = default;
  ICU4XIsoDate& operator=(ICU4XIsoDate&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XIsoDate, ICU4XIsoDateDeleter> inner;
};

#include "ICU4XDate.hpp"

inline diplomat::result<ICU4XIsoDate, ICU4XError> ICU4XIsoDate::try_new(int32_t year, uint8_t month, uint8_t day) {
  auto diplomat_result_raw_out_value = capi::ICU4XIsoDate_try_new(year, month, day);
  diplomat::result<ICU4XIsoDate, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XIsoDate>(std::move(ICU4XIsoDate(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline ICU4XDate ICU4XIsoDate::to_any() const {
  return ICU4XDate(capi::ICU4XIsoDate_to_any(this->inner.get()));
}
#endif
