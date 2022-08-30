#ifndef ICU4XTimeZoneFormatter_HPP
#define ICU4XTimeZoneFormatter_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XTimeZoneFormatter.h"

class ICU4XDataProvider;
class ICU4XLocale;
class ICU4XTimeZoneFormatter;
#include "ICU4XError.hpp"
#include "ICU4XIsoTimeZoneFormat.hpp"
#include "ICU4XIsoTimeZoneMinuteDisplay.hpp"
#include "ICU4XIsoTimeZoneSecondDisplay.hpp"
class ICU4XCustomTimeZone;

/**
 * A destruction policy for using ICU4XTimeZoneFormatter with std::unique_ptr.
 */
struct ICU4XTimeZoneFormatterDeleter {
  void operator()(capi::ICU4XTimeZoneFormatter* l) const noexcept {
    capi::ICU4XTimeZoneFormatter_destroy(l);
  }
};

/**
 * An ICU4X TimeZoneFormatter object capable of formatting an [`ICU4XCustomTimeZone`] type (and others) as a string
 * 
 * See the [Rust documentation for `TimeZoneFormatter`](https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeZoneFormatter.html) for more information.
 */
class ICU4XTimeZoneFormatter {
 public:

  /**
   * Creates a new [`ICU4XTimeZoneFormatter`] from locale data.
   * 
   * Uses localized GMT as the fallback format.
   * 
   * See the [Rust documentation for `try_new_unstable`](https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeZoneFormatter.html#method.try_new_unstable) for more information.
   * 
   *  Additional information: [1](https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/time_zone/enum.FallbackFormat.html)
   */
  static diplomat::result<ICU4XTimeZoneFormatter, ICU4XError> try_new_with_localized_gmt_fallback(const ICU4XDataProvider& provider, const ICU4XLocale& locale);

  /**
   * Creates a new [`ICU4XTimeZoneFormatter`] from locale data.
   * 
   * Uses ISO-8601 as the fallback format.
   * 
   * See the [Rust documentation for `try_new_unstable`](https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeZoneFormatter.html#method.try_new_unstable) for more information.
   * 
   *  Additional information: [1](https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/time_zone/enum.FallbackFormat.html)
   */
  static diplomat::result<ICU4XTimeZoneFormatter, ICU4XError> try_new_with_iso_8601_fallback(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XIsoTimeZoneFormat format, ICU4XIsoTimeZoneMinuteDisplay minutes, ICU4XIsoTimeZoneSecondDisplay seconds);

  /**
   * Formats a [`ICU4XCustomTimeZone`] to a string.
   * 
   * See the [Rust documentation for `format_to_write`](https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeZoneFormatter.html#method.format_to_write) for more information.
   */
  template<typename W> diplomat::result<std::monostate, ICU4XError> format_custom_time_zone_to_writeable(const ICU4XCustomTimeZone& value, W& write) const;

  /**
   * Formats a [`ICU4XCustomTimeZone`] to a string.
   * 
   * See the [Rust documentation for `format_to_write`](https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeZoneFormatter.html#method.format_to_write) for more information.
   */
  diplomat::result<std::string, ICU4XError> format_custom_time_zone(const ICU4XCustomTimeZone& value) const;
  inline const capi::ICU4XTimeZoneFormatter* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XTimeZoneFormatter* AsFFIMut() { return this->inner.get(); }
  inline ICU4XTimeZoneFormatter(capi::ICU4XTimeZoneFormatter* i) : inner(i) {}
  ICU4XTimeZoneFormatter() = default;
  ICU4XTimeZoneFormatter(ICU4XTimeZoneFormatter&&) noexcept = default;
  ICU4XTimeZoneFormatter& operator=(ICU4XTimeZoneFormatter&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XTimeZoneFormatter, ICU4XTimeZoneFormatterDeleter> inner;
};

#include "ICU4XDataProvider.hpp"
#include "ICU4XLocale.hpp"
#include "ICU4XCustomTimeZone.hpp"

inline diplomat::result<ICU4XTimeZoneFormatter, ICU4XError> ICU4XTimeZoneFormatter::try_new_with_localized_gmt_fallback(const ICU4XDataProvider& provider, const ICU4XLocale& locale) {
  auto diplomat_result_raw_out_value = capi::ICU4XTimeZoneFormatter_try_new_with_localized_gmt_fallback(provider.AsFFI(), locale.AsFFI());
  diplomat::result<ICU4XTimeZoneFormatter, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XTimeZoneFormatter>(std::move(ICU4XTimeZoneFormatter(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XTimeZoneFormatter, ICU4XError> ICU4XTimeZoneFormatter::try_new_with_iso_8601_fallback(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XIsoTimeZoneFormat format, ICU4XIsoTimeZoneMinuteDisplay minutes, ICU4XIsoTimeZoneSecondDisplay seconds) {
  auto diplomat_result_raw_out_value = capi::ICU4XTimeZoneFormatter_try_new_with_iso_8601_fallback(provider.AsFFI(), locale.AsFFI(), static_cast<capi::ICU4XIsoTimeZoneFormat>(format), static_cast<capi::ICU4XIsoTimeZoneMinuteDisplay>(minutes), static_cast<capi::ICU4XIsoTimeZoneSecondDisplay>(seconds));
  diplomat::result<ICU4XTimeZoneFormatter, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XTimeZoneFormatter>(std::move(ICU4XTimeZoneFormatter(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
template<typename W> inline diplomat::result<std::monostate, ICU4XError> ICU4XTimeZoneFormatter::format_custom_time_zone_to_writeable(const ICU4XCustomTimeZone& value, W& write) const {
  capi::DiplomatWriteable write_writer = diplomat::WriteableTrait<W>::Construct(write);
  auto diplomat_result_raw_out_value = capi::ICU4XTimeZoneFormatter_format_custom_time_zone(this->inner.get(), value.AsFFI(), &write_writer);
  diplomat::result<std::monostate, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<std::string, ICU4XError> ICU4XTimeZoneFormatter::format_custom_time_zone(const ICU4XCustomTimeZone& value) const {
  std::string diplomat_writeable_string;
  capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);
  auto diplomat_result_raw_out_value = capi::ICU4XTimeZoneFormatter_format_custom_time_zone(this->inner.get(), value.AsFFI(), &diplomat_writeable_out);
  diplomat::result<std::monostate, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value.replace_ok(std::move(diplomat_writeable_string));
}
#endif
