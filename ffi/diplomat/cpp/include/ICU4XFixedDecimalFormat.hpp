#ifndef ICU4XFixedDecimalFormat_HPP
#define ICU4XFixedDecimalFormat_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XFixedDecimalFormat.h"
}

class ICU4XLocale;
class ICU4XDataProvider;
struct ICU4XFixedDecimalFormatOptions;
class ICU4XFixedDecimalFormat;
#include "ICU4XError.hpp"
class ICU4XDataStruct;
class ICU4XFixedDecimal;

/**
 * A destruction policy for using ICU4XFixedDecimalFormat with std::unique_ptr.
 */
struct ICU4XFixedDecimalFormatDeleter {
  void operator()(capi::ICU4XFixedDecimalFormat* l) const noexcept {
    capi::ICU4XFixedDecimalFormat_destroy(l);
  }
};

/**
 * An ICU4X Fixed Decimal Format object, capable of formatting a [`ICU4XFixedDecimal`] as a string.
 * 
 * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormat.html) for more information.
 */
class ICU4XFixedDecimalFormat {
 public:

  /**
   * Creates a new [`ICU4XFixedDecimalFormat`] from locale data.
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormat.html#method.try_new) for more information.
   */
  static diplomat::result<ICU4XFixedDecimalFormat, ICU4XError> try_new(const ICU4XLocale& locale, const ICU4XDataProvider& provider, ICU4XFixedDecimalFormatOptions options);

  /**
   * Creates a new [`ICU4XFixedDecimalFormat`] from preconstructed locale data in the form of an [`ICU4XDataStruct`]
   * constructed from `ICU4XDataStruct::create_decimal_symbols()`.
   * 
   * The contents of the data struct will be consumed: if you wish to use the struct again it will have to be reconstructed.
   * Passing a consumed struct to this method will return an error.
   */
  static diplomat::result<ICU4XFixedDecimalFormat, ICU4XError> try_new_from_decimal_symbols_v1(const ICU4XDataStruct& data_struct, ICU4XFixedDecimalFormatOptions options);

  /**
   * Formats a [`ICU4XFixedDecimal`] to a string.
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormat.html#method.format) for more information.
   */
  template<typename W> diplomat::result<std::monostate, ICU4XError> format_to_writeable(const ICU4XFixedDecimal& value, W& write) const;

  /**
   * Formats a [`ICU4XFixedDecimal`] to a string.
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormat.html#method.format) for more information.
   */
  diplomat::result<std::string, ICU4XError> format(const ICU4XFixedDecimal& value) const;
  inline const capi::ICU4XFixedDecimalFormat* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XFixedDecimalFormat* AsFFIMut() { return this->inner.get(); }
  inline ICU4XFixedDecimalFormat(capi::ICU4XFixedDecimalFormat* i) : inner(i) {}
  ICU4XFixedDecimalFormat() = default;
  ICU4XFixedDecimalFormat(ICU4XFixedDecimalFormat&&) noexcept = default;
  ICU4XFixedDecimalFormat& operator=(ICU4XFixedDecimalFormat&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XFixedDecimalFormat, ICU4XFixedDecimalFormatDeleter> inner;
};

#include "ICU4XLocale.hpp"
#include "ICU4XDataProvider.hpp"
#include "ICU4XFixedDecimalFormatOptions.hpp"
#include "ICU4XDataStruct.hpp"
#include "ICU4XFixedDecimal.hpp"

inline diplomat::result<ICU4XFixedDecimalFormat, ICU4XError> ICU4XFixedDecimalFormat::try_new(const ICU4XLocale& locale, const ICU4XDataProvider& provider, ICU4XFixedDecimalFormatOptions options) {
  ICU4XFixedDecimalFormatOptions diplomat_wrapped_struct_options = options;
  auto diplomat_result_raw_out_value = capi::ICU4XFixedDecimalFormat_try_new(locale.AsFFI(), provider.AsFFI(), capi::ICU4XFixedDecimalFormatOptions{ .grouping_strategy = static_cast<capi::ICU4XFixedDecimalGroupingStrategy>(diplomat_wrapped_struct_options.grouping_strategy), .sign_display = static_cast<capi::ICU4XFixedDecimalSignDisplay>(diplomat_wrapped_struct_options.sign_display) });
  diplomat::result<ICU4XFixedDecimalFormat, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(ICU4XFixedDecimalFormat(diplomat_result_raw_out_value.ok));
  } else {
    diplomat_result_out_value = diplomat::Err(static_cast<ICU4XError>(diplomat_result_raw_out_value.err));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XFixedDecimalFormat, ICU4XError> ICU4XFixedDecimalFormat::try_new_from_decimal_symbols_v1(const ICU4XDataStruct& data_struct, ICU4XFixedDecimalFormatOptions options) {
  ICU4XFixedDecimalFormatOptions diplomat_wrapped_struct_options = options;
  auto diplomat_result_raw_out_value = capi::ICU4XFixedDecimalFormat_try_new_from_decimal_symbols_v1(data_struct.AsFFI(), capi::ICU4XFixedDecimalFormatOptions{ .grouping_strategy = static_cast<capi::ICU4XFixedDecimalGroupingStrategy>(diplomat_wrapped_struct_options.grouping_strategy), .sign_display = static_cast<capi::ICU4XFixedDecimalSignDisplay>(diplomat_wrapped_struct_options.sign_display) });
  diplomat::result<ICU4XFixedDecimalFormat, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(ICU4XFixedDecimalFormat(diplomat_result_raw_out_value.ok));
  } else {
    diplomat_result_out_value = diplomat::Err(static_cast<ICU4XError>(diplomat_result_raw_out_value.err));
  }
  return diplomat_result_out_value;
}
template<typename W> inline diplomat::result<std::monostate, ICU4XError> ICU4XFixedDecimalFormat::format_to_writeable(const ICU4XFixedDecimal& value, W& write) const {
  capi::DiplomatWriteable write_writer = diplomat::WriteableTrait<W>::Construct(write);
  auto diplomat_result_raw_out_value = capi::ICU4XFixedDecimalFormat_format(this->inner.get(), value.AsFFI(), &write_writer);
  diplomat::result<std::monostate, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err(static_cast<ICU4XError>(diplomat_result_raw_out_value.err));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<std::string, ICU4XError> ICU4XFixedDecimalFormat::format(const ICU4XFixedDecimal& value) const {
  std::string diplomat_writeable_string;
  capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);
  auto diplomat_result_raw_out_value = capi::ICU4XFixedDecimalFormat_format(this->inner.get(), value.AsFFI(), &diplomat_writeable_out);
  diplomat::result<std::monostate, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err(static_cast<ICU4XError>(diplomat_result_raw_out_value.err));
  }
  return diplomat_result_out_value.replace_ok(std::move(diplomat_writeable_string));
}
#endif
