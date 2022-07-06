#ifndef ICU4XGregorianDateFormat_HPP
#define ICU4XGregorianDateFormat_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XGregorianDateFormat.h"
}

class ICU4XLocale;
class ICU4XDataProvider;
#include "ICU4XDateLength.hpp"
class ICU4XGregorianDateFormat;
#include "ICU4XError.hpp"
class ICU4XGregorianDateTime;

/**
 * A destruction policy for using ICU4XGregorianDateFormat with std::unique_ptr.
 */
struct ICU4XGregorianDateFormatDeleter {
  void operator()(capi::ICU4XGregorianDateFormat* l) const noexcept {
    capi::ICU4XGregorianDateFormat_destroy(l);
  }
};

/**
 * An ICU4X DateFormat object capable of formatting a [`ICU4XGregorianDateTime`] as a string,
 * using the Gregorian Calendar.
 * 
 * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.DateFormat.html) for more information.
 */
class ICU4XGregorianDateFormat {
 public:

  /**
   * Creates a new [`ICU4XGregorianDateFormat`] from locale data.
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.DateFormat.html#method.try_new) for more information.
   */
  static diplomat::result<ICU4XGregorianDateFormat, ICU4XError> try_new(const ICU4XLocale& locale, const ICU4XDataProvider& provider, ICU4XDateLength length);

  /**
   * Formats a [`ICU4XGregorianDateTime`] to a string.
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.DateFormat.html#method.format_to_write) for more information.
   */
  template<typename W> diplomat::result<std::monostate, ICU4XError> format_to_write_to_writeable(const ICU4XGregorianDateTime& value, W& write) const;

  /**
   * Formats a [`ICU4XGregorianDateTime`] to a string.
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.DateFormat.html#method.format_to_write) for more information.
   */
  diplomat::result<std::string, ICU4XError> format_to_write(const ICU4XGregorianDateTime& value) const;
  inline const capi::ICU4XGregorianDateFormat* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XGregorianDateFormat* AsFFIMut() { return this->inner.get(); }
  inline ICU4XGregorianDateFormat(capi::ICU4XGregorianDateFormat* i) : inner(i) {}
  ICU4XGregorianDateFormat() = default;
  ICU4XGregorianDateFormat(ICU4XGregorianDateFormat&&) noexcept = default;
  ICU4XGregorianDateFormat& operator=(ICU4XGregorianDateFormat&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XGregorianDateFormat, ICU4XGregorianDateFormatDeleter> inner;
};

#include "ICU4XLocale.hpp"
#include "ICU4XDataProvider.hpp"
#include "ICU4XGregorianDateTime.hpp"

inline diplomat::result<ICU4XGregorianDateFormat, ICU4XError> ICU4XGregorianDateFormat::try_new(const ICU4XLocale& locale, const ICU4XDataProvider& provider, ICU4XDateLength length) {
  auto diplomat_result_raw_out_value = capi::ICU4XGregorianDateFormat_try_new(locale.AsFFI(), provider.AsFFI(), static_cast<capi::ICU4XDateLength>(length));
  diplomat::result<ICU4XGregorianDateFormat, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(ICU4XGregorianDateFormat(diplomat_result_raw_out_value.ok));
  } else {
    diplomat_result_out_value = diplomat::Err(static_cast<ICU4XError>(diplomat_result_raw_out_value.err));
  }
  return diplomat_result_out_value;
}
template<typename W> inline diplomat::result<std::monostate, ICU4XError> ICU4XGregorianDateFormat::format_to_write_to_writeable(const ICU4XGregorianDateTime& value, W& write) const {
  capi::DiplomatWriteable write_writer = diplomat::WriteableTrait<W>::Construct(write);
  auto diplomat_result_raw_out_value = capi::ICU4XGregorianDateFormat_format_to_write(this->inner.get(), value.AsFFI(), &write_writer);
  diplomat::result<std::monostate, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err(static_cast<ICU4XError>(diplomat_result_raw_out_value.err));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<std::string, ICU4XError> ICU4XGregorianDateFormat::format_to_write(const ICU4XGregorianDateTime& value) const {
  std::string diplomat_writeable_string;
  capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);
  auto diplomat_result_raw_out_value = capi::ICU4XGregorianDateFormat_format_to_write(this->inner.get(), value.AsFFI(), &diplomat_writeable_out);
  diplomat::result<std::monostate, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err(static_cast<ICU4XError>(diplomat_result_raw_out_value.err));
  }
  return diplomat_result_out_value.replace_ok(std::move(diplomat_writeable_string));
}
#endif
