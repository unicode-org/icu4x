#ifndef ICU4XFixedDecimal_HPP
#define ICU4XFixedDecimal_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XFixedDecimal.h"

class ICU4XFixedDecimal;
#include "ICU4XError.hpp"
#include "ICU4XFixedDecimalSign.hpp"

/**
 * A destruction policy for using ICU4XFixedDecimal with std::unique_ptr.
 */
struct ICU4XFixedDecimalDeleter {
  void operator()(capi::ICU4XFixedDecimal* l) const noexcept {
    capi::ICU4XFixedDecimal_destroy(l);
  }
};

/**
 * 
 * 
 * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html) for more information.
 */
class ICU4XFixedDecimal {
 public:

  /**
   * Construct an [`ICU4XFixedDecimal`] from an integer.
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html) for more information.
   */
  static ICU4XFixedDecimal create(int32_t v);

  /**
   * Construct an [`ICU4XFixedDecimal`] from an float, with enough digits to recover
   * the original floating point in IEEE 754 without needing trailing zeros
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.try_from_f64) for more information.
   */
  static diplomat::result<ICU4XFixedDecimal, ICU4XError> create_from_f64_with_max_precision(double f);

  /**
   * Construct an [`ICU4XFixedDecimal`] from an float, with a given power of 10 for the lower magnitude
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.try_from_f64) for more information.
   */
  static diplomat::result<ICU4XFixedDecimal, ICU4XError> create_from_f64_with_lower_magnitude(double f, int16_t precision);

  /**
   * Construct an [`ICU4XFixedDecimal`] from an float, for a given number of significant digits
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.try_from_f64) for more information.
   */
  static diplomat::result<ICU4XFixedDecimal, ICU4XError> create_from_f64_with_significant_digits(double f, uint8_t digits);

  /**
   * Construct an [`ICU4XFixedDecimal`] from a string.
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html) for more information.
   */
  static diplomat::result<ICU4XFixedDecimal, ICU4XError> create_fromstr(const std::string_view v);

  /**
   * Multiply the [`ICU4XFixedDecimal`] by a given power of ten.
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.multiply_pow10) for more information.
   */
  bool multiply_pow10(int16_t power);

  /**
   * Set the sign of the [`ICU4XFixedDecimal`].
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.set_sign) for more information.
   */
  void set_sign(ICU4XFixedDecimalSign sign);

  /**
   * Zero-pad the [`ICU4XFixedDecimal`] on the left to a particular position
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.pad_start) for more information.
   */
  void pad_start(int16_t position);

  /**
   * Truncate the [`ICU4XFixedDecimal`] on the left to a particular position, deleting digits if necessary. This is useful for, e.g. abbreviating years
   * ("2022" -> "22")
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.set_max_position) for more information.
   */
  void set_max_position(int16_t position);

  /**
   * Zero-pad the [`ICU4XFixedDecimal`] on the right to a particular position
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.pad_end) for more information.
   */
  void pad_end(int16_t position);

  /**
   * Format the [`ICU4XFixedDecimal`] as a string.
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.write_to) for more information.
   */
  template<typename W> void to_string_to_writeable(W& to) const;

  /**
   * Format the [`ICU4XFixedDecimal`] as a string.
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.write_to) for more information.
   */
  std::string to_string() const;
  inline const capi::ICU4XFixedDecimal* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XFixedDecimal* AsFFIMut() { return this->inner.get(); }
  inline ICU4XFixedDecimal(capi::ICU4XFixedDecimal* i) : inner(i) {}
  ICU4XFixedDecimal() = default;
  ICU4XFixedDecimal(ICU4XFixedDecimal&&) noexcept = default;
  ICU4XFixedDecimal& operator=(ICU4XFixedDecimal&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XFixedDecimal, ICU4XFixedDecimalDeleter> inner;
};


inline ICU4XFixedDecimal ICU4XFixedDecimal::create(int32_t v) {
  return ICU4XFixedDecimal(capi::ICU4XFixedDecimal_create(v));
}
inline diplomat::result<ICU4XFixedDecimal, ICU4XError> ICU4XFixedDecimal::create_from_f64_with_max_precision(double f) {
  auto diplomat_result_raw_out_value = capi::ICU4XFixedDecimal_create_from_f64_with_max_precision(f);
  diplomat::result<ICU4XFixedDecimal, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XFixedDecimal>(std::move(ICU4XFixedDecimal(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XFixedDecimal, ICU4XError> ICU4XFixedDecimal::create_from_f64_with_lower_magnitude(double f, int16_t precision) {
  auto diplomat_result_raw_out_value = capi::ICU4XFixedDecimal_create_from_f64_with_lower_magnitude(f, precision);
  diplomat::result<ICU4XFixedDecimal, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XFixedDecimal>(std::move(ICU4XFixedDecimal(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XFixedDecimal, ICU4XError> ICU4XFixedDecimal::create_from_f64_with_significant_digits(double f, uint8_t digits) {
  auto diplomat_result_raw_out_value = capi::ICU4XFixedDecimal_create_from_f64_with_significant_digits(f, digits);
  diplomat::result<ICU4XFixedDecimal, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XFixedDecimal>(std::move(ICU4XFixedDecimal(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XFixedDecimal, ICU4XError> ICU4XFixedDecimal::create_fromstr(const std::string_view v) {
  auto diplomat_result_raw_out_value = capi::ICU4XFixedDecimal_create_fromstr(v.data(), v.size());
  diplomat::result<ICU4XFixedDecimal, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XFixedDecimal>(std::move(ICU4XFixedDecimal(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline bool ICU4XFixedDecimal::multiply_pow10(int16_t power) {
  return capi::ICU4XFixedDecimal_multiply_pow10(this->inner.get(), power);
}
inline void ICU4XFixedDecimal::set_sign(ICU4XFixedDecimalSign sign) {
  capi::ICU4XFixedDecimal_set_sign(this->inner.get(), static_cast<capi::ICU4XFixedDecimalSign>(sign));
}
inline void ICU4XFixedDecimal::pad_start(int16_t position) {
  capi::ICU4XFixedDecimal_pad_start(this->inner.get(), position);
}
inline void ICU4XFixedDecimal::set_max_position(int16_t position) {
  capi::ICU4XFixedDecimal_set_max_position(this->inner.get(), position);
}
inline void ICU4XFixedDecimal::pad_end(int16_t position) {
  capi::ICU4XFixedDecimal_pad_end(this->inner.get(), position);
}
template<typename W> inline void ICU4XFixedDecimal::to_string_to_writeable(W& to) const {
  capi::DiplomatWriteable to_writer = diplomat::WriteableTrait<W>::Construct(to);
  capi::ICU4XFixedDecimal_to_string(this->inner.get(), &to_writer);
}
inline std::string ICU4XFixedDecimal::to_string() const {
  std::string diplomat_writeable_string;
  capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);
  capi::ICU4XFixedDecimal_to_string(this->inner.get(), &diplomat_writeable_out);
  return diplomat_writeable_string;
}
#endif
