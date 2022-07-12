#ifndef ICU4XGregorianDateTimeFormat_HPP
#define ICU4XGregorianDateTimeFormat_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XGregorianDateTimeFormat.h"
}

class ICU4XLocale;
class ICU4XDataProvider;
#include "ICU4XDateLength.hpp"
#include "ICU4XTimeLength.hpp"
#include "ICU4XHourCyclePreference.hpp"
class ICU4XGregorianDateTimeFormat;
#include "ICU4XError.hpp"
class ICU4XGregorianDateTime;

/**
 * A destruction policy for using ICU4XGregorianDateTimeFormat with std::unique_ptr.
 */
struct ICU4XGregorianDateTimeFormatDeleter {
  void operator()(capi::ICU4XGregorianDateTimeFormat* l) const noexcept {
    capi::ICU4XGregorianDateTimeFormat_destroy(l);
  }
};

/**
 * An ICU4X DateFormat object capable of formatting a [`ICU4XGregorianDateTime`] as a string,
 * using the Gregorian Calendar.
 * 
 * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.DateTimeFormat.html) for more information.
 */
class ICU4XGregorianDateTimeFormat {
 public:

  /**
   * Creates a new [`ICU4XGregorianDateFormat`] from locale data.
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.DateTimeFormat.html#method.try_new) for more information.
   */
  static diplomat::result<ICU4XGregorianDateTimeFormat, ICU4XError> try_new(const ICU4XLocale& locale, const ICU4XDataProvider& provider, ICU4XDateLength date_length, ICU4XTimeLength time_length, ICU4XHourCyclePreference time_preferences);

  /**
   * Formats a [`ICU4XGregorianDateTime`] to a string.
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.DateTimeFormat.html#method.format_to_write) for more information.
   */
  template<typename W> diplomat::result<std::monostate, ICU4XError> format_datetime_to_writeable(const ICU4XGregorianDateTime& value, W& write) const;

  /**
   * Formats a [`ICU4XGregorianDateTime`] to a string.
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.DateTimeFormat.html#method.format_to_write) for more information.
   */
  diplomat::result<std::string, ICU4XError> format_datetime(const ICU4XGregorianDateTime& value) const;
  inline const capi::ICU4XGregorianDateTimeFormat* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XGregorianDateTimeFormat* AsFFIMut() { return this->inner.get(); }
  inline ICU4XGregorianDateTimeFormat(capi::ICU4XGregorianDateTimeFormat* i) : inner(i) {}
  ICU4XGregorianDateTimeFormat() = default;
  ICU4XGregorianDateTimeFormat(ICU4XGregorianDateTimeFormat&&) noexcept = default;
  ICU4XGregorianDateTimeFormat& operator=(ICU4XGregorianDateTimeFormat&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XGregorianDateTimeFormat, ICU4XGregorianDateTimeFormatDeleter> inner;
};

#include "ICU4XLocale.hpp"
#include "ICU4XDataProvider.hpp"
#include "ICU4XGregorianDateTime.hpp"

inline diplomat::result<ICU4XGregorianDateTimeFormat, ICU4XError> ICU4XGregorianDateTimeFormat::try_new(const ICU4XLocale& locale, const ICU4XDataProvider& provider, ICU4XDateLength date_length, ICU4XTimeLength time_length, ICU4XHourCyclePreference time_preferences) {
  auto diplomat_result_raw_out_value = capi::ICU4XGregorianDateTimeFormat_try_new(locale.AsFFI(), provider.AsFFI(), static_cast<capi::ICU4XDateLength>(date_length), static_cast<capi::ICU4XTimeLength>(time_length), static_cast<capi::ICU4XHourCyclePreference>(time_preferences));
  diplomat::result<ICU4XGregorianDateTimeFormat, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(ICU4XGregorianDateTimeFormat(diplomat_result_raw_out_value.ok));
  } else {
    diplomat_result_out_value = diplomat::Err(static_cast<ICU4XError>(diplomat_result_raw_out_value.err));
  }
  return diplomat_result_out_value;
}
template<typename W> inline diplomat::result<std::monostate, ICU4XError> ICU4XGregorianDateTimeFormat::format_datetime_to_writeable(const ICU4XGregorianDateTime& value, W& write) const {
  capi::DiplomatWriteable write_writer = diplomat::WriteableTrait<W>::Construct(write);
  auto diplomat_result_raw_out_value = capi::ICU4XGregorianDateTimeFormat_format_datetime(this->inner.get(), value.AsFFI(), &write_writer);
  diplomat::result<std::monostate, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err(static_cast<ICU4XError>(diplomat_result_raw_out_value.err));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<std::string, ICU4XError> ICU4XGregorianDateTimeFormat::format_datetime(const ICU4XGregorianDateTime& value) const {
  std::string diplomat_writeable_string;
  capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);
  auto diplomat_result_raw_out_value = capi::ICU4XGregorianDateTimeFormat_format_datetime(this->inner.get(), value.AsFFI(), &diplomat_writeable_out);
  diplomat::result<std::monostate, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err(static_cast<ICU4XError>(diplomat_result_raw_out_value.err));
  }
  return diplomat_result_out_value.replace_ok(std::move(diplomat_writeable_string));
}
#endif
