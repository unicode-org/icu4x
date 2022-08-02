#ifndef ICU4XTimeFormatter_HPP
#define ICU4XTimeFormatter_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XTimeFormatter.h"

class ICU4XDataProvider;
class ICU4XLocale;
#include "ICU4XTimeLength.hpp"
#include "ICU4XHourCyclePreference.hpp"
class ICU4XTimeFormatter;
#include "ICU4XError.hpp"
class ICU4XGregorianDateTime;

/**
 * A destruction policy for using ICU4XTimeFormatter with std::unique_ptr.
 */
struct ICU4XTimeFormatterDeleter {
  void operator()(capi::ICU4XTimeFormatter* l) const noexcept {
    capi::ICU4XTimeFormatter_destroy(l);
  }
};

/**
 * An ICU4X TimeFormatter object capable of formatting a [`ICU4XGregorianDateTime`] as a string
 * 
 * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeFormatter.html) for more information.
 */
class ICU4XTimeFormatter {
 public:

  /**
   * Creates a new [`ICU4XTimeFormatter`] from locale data.
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.TypedDateFormatter.html#method.try_new) for more information.
   */
  static diplomat::result<ICU4XTimeFormatter, ICU4XError> try_new(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XTimeLength length, ICU4XHourCyclePreference preferences);

  /**
   * Formats a [`ICU4XGregorianDateTime`] to a string.
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeFormatter.html#method.format_to_write) for more information.
   */
  template<typename W> diplomat::result<std::monostate, ICU4XError> format_gregorian_datetime_to_writeable(const ICU4XGregorianDateTime& value, W& write) const;

  /**
   * Formats a [`ICU4XGregorianDateTime`] to a string.
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeFormatter.html#method.format_to_write) for more information.
   */
  diplomat::result<std::string, ICU4XError> format_gregorian_datetime(const ICU4XGregorianDateTime& value) const;
  inline const capi::ICU4XTimeFormatter* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XTimeFormatter* AsFFIMut() { return this->inner.get(); }
  inline ICU4XTimeFormatter(capi::ICU4XTimeFormatter* i) : inner(i) {}
  ICU4XTimeFormatter() = default;
  ICU4XTimeFormatter(ICU4XTimeFormatter&&) noexcept = default;
  ICU4XTimeFormatter& operator=(ICU4XTimeFormatter&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XTimeFormatter, ICU4XTimeFormatterDeleter> inner;
};

#include "ICU4XDataProvider.hpp"
#include "ICU4XLocale.hpp"
#include "ICU4XGregorianDateTime.hpp"

inline diplomat::result<ICU4XTimeFormatter, ICU4XError> ICU4XTimeFormatter::try_new(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XTimeLength length, ICU4XHourCyclePreference preferences) {
  auto diplomat_result_raw_out_value = capi::ICU4XTimeFormatter_try_new(provider.AsFFI(), locale.AsFFI(), static_cast<capi::ICU4XTimeLength>(length), static_cast<capi::ICU4XHourCyclePreference>(preferences));
  diplomat::result<ICU4XTimeFormatter, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XTimeFormatter>(std::move(ICU4XTimeFormatter(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
template<typename W> inline diplomat::result<std::monostate, ICU4XError> ICU4XTimeFormatter::format_gregorian_datetime_to_writeable(const ICU4XGregorianDateTime& value, W& write) const {
  capi::DiplomatWriteable write_writer = diplomat::WriteableTrait<W>::Construct(write);
  auto diplomat_result_raw_out_value = capi::ICU4XTimeFormatter_format_gregorian_datetime(this->inner.get(), value.AsFFI(), &write_writer);
  diplomat::result<std::monostate, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<std::string, ICU4XError> ICU4XTimeFormatter::format_gregorian_datetime(const ICU4XGregorianDateTime& value) const {
  std::string diplomat_writeable_string;
  capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);
  auto diplomat_result_raw_out_value = capi::ICU4XTimeFormatter_format_gregorian_datetime(this->inner.get(), value.AsFFI(), &diplomat_writeable_out);
  diplomat::result<std::monostate, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value.replace_ok(std::move(diplomat_writeable_string));
}
#endif
