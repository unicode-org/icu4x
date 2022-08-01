#ifndef ICU4XGregorianDateFormatter_HPP
#define ICU4XGregorianDateFormatter_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XGregorianDateFormatter.h"

class ICU4XDataProvider;
class ICU4XLocale;
#include "ICU4XDateLength.hpp"
class ICU4XGregorianDateFormatter;
#include "ICU4XError.hpp"
class ICU4XGregorianDateTime;

/**
 * A destruction policy for using ICU4XGregorianDateFormatter with std::unique_ptr.
 */
struct ICU4XGregorianDateFormatterDeleter {
  void operator()(capi::ICU4XGregorianDateFormatter* l) const noexcept {
    capi::ICU4XGregorianDateFormatter_destroy(l);
  }
};

/**
 * An ICU4X TypedDateFormatter object capable of formatting a [`ICU4XGregorianDateTime`] as a string,
 * using the Gregorian Calendar.
 * 
 * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TypedDateFormatter.html) for more information.
 */
class ICU4XGregorianDateFormatter {
 public:

  /**
   * Creates a new [`ICU4XGregorianDateFormatter`] from locale data.
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.TypedDateFormatter.html#method.try_new) for more information.
   */
  static diplomat::result<ICU4XGregorianDateFormatter, ICU4XError> try_new(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XDateLength length);

  /**
   * Formats a [`ICU4XGregorianDateTime`] to a string.
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TypedDateFormatter.html#method.format_to_write) for more information.
   */
  template<typename W> diplomat::result<std::monostate, ICU4XError> format_datetime_to_writeable(const ICU4XGregorianDateTime& value, W& write) const;

  /**
   * Formats a [`ICU4XGregorianDateTime`] to a string.
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TypedDateFormatter.html#method.format_to_write) for more information.
   */
  diplomat::result<std::string, ICU4XError> format_datetime(const ICU4XGregorianDateTime& value) const;
  inline const capi::ICU4XGregorianDateFormatter* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XGregorianDateFormatter* AsFFIMut() { return this->inner.get(); }
  inline ICU4XGregorianDateFormatter(capi::ICU4XGregorianDateFormatter* i) : inner(i) {}
  ICU4XGregorianDateFormatter() = default;
  ICU4XGregorianDateFormatter(ICU4XGregorianDateFormatter&&) noexcept = default;
  ICU4XGregorianDateFormatter& operator=(ICU4XGregorianDateFormatter&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XGregorianDateFormatter, ICU4XGregorianDateFormatterDeleter> inner;
};

#include "ICU4XDataProvider.hpp"
#include "ICU4XLocale.hpp"
#include "ICU4XGregorianDateTime.hpp"

inline diplomat::result<ICU4XGregorianDateFormatter, ICU4XError> ICU4XGregorianDateFormatter::try_new(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XDateLength length) {
  auto diplomat_result_raw_out_value = capi::ICU4XGregorianDateFormatter_try_new(provider.AsFFI(), locale.AsFFI(), static_cast<capi::ICU4XDateLength>(length));
  diplomat::result<ICU4XGregorianDateFormatter, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XGregorianDateFormatter>(std::move(ICU4XGregorianDateFormatter(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
template<typename W> inline diplomat::result<std::monostate, ICU4XError> ICU4XGregorianDateFormatter::format_datetime_to_writeable(const ICU4XGregorianDateTime& value, W& write) const {
  capi::DiplomatWriteable write_writer = diplomat::WriteableTrait<W>::Construct(write);
  auto diplomat_result_raw_out_value = capi::ICU4XGregorianDateFormatter_format_datetime(this->inner.get(), value.AsFFI(), &write_writer);
  diplomat::result<std::monostate, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<std::string, ICU4XError> ICU4XGregorianDateFormatter::format_datetime(const ICU4XGregorianDateTime& value) const {
  std::string diplomat_writeable_string;
  capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);
  auto diplomat_result_raw_out_value = capi::ICU4XGregorianDateFormatter_format_datetime(this->inner.get(), value.AsFFI(), &diplomat_writeable_out);
  diplomat::result<std::monostate, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value.replace_ok(std::move(diplomat_writeable_string));
}
#endif
