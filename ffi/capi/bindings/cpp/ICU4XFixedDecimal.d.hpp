#ifndef ICU4XFixedDecimal_D_HPP
#define ICU4XFixedDecimal_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XFixedDecimalLimitError.d.hpp"
#include "ICU4XFixedDecimalParseError.d.hpp"
#include "ICU4XFixedDecimalRoundingIncrement.d.hpp"
#include "ICU4XFixedDecimalSign.d.hpp"
#include "ICU4XFixedDecimalSignDisplay.d.hpp"

class ICU4XFixedDecimalLimitError;
class ICU4XFixedDecimalParseError;
class ICU4XFixedDecimalRoundingIncrement;
class ICU4XFixedDecimalSign;
class ICU4XFixedDecimalSignDisplay;


namespace capi {
    typedef struct ICU4XFixedDecimal ICU4XFixedDecimal;
}

class ICU4XFixedDecimal {
public:

  inline static std::unique_ptr<ICU4XFixedDecimal> create_from_i32(int32_t v);

  inline static std::unique_ptr<ICU4XFixedDecimal> create_from_u32(uint32_t v);

  inline static std::unique_ptr<ICU4XFixedDecimal> create_from_i64(int64_t v);

  inline static std::unique_ptr<ICU4XFixedDecimal> create_from_u64(uint64_t v);

  inline static diplomat::result<std::unique_ptr<ICU4XFixedDecimal>, ICU4XFixedDecimalLimitError> create_from_f64_with_integer_precision(double f);

  inline static diplomat::result<std::unique_ptr<ICU4XFixedDecimal>, ICU4XFixedDecimalLimitError> create_from_f64_with_lower_magnitude(double f, int16_t magnitude);

  inline static diplomat::result<std::unique_ptr<ICU4XFixedDecimal>, ICU4XFixedDecimalLimitError> create_from_f64_with_significant_digits(double f, uint8_t digits);

  inline static diplomat::result<std::unique_ptr<ICU4XFixedDecimal>, ICU4XFixedDecimalLimitError> create_from_f64_with_floating_precision(double f);

  inline static diplomat::result<std::unique_ptr<ICU4XFixedDecimal>, ICU4XFixedDecimalParseError> create_from_string(std::string_view v);

  inline uint8_t digit_at(int16_t magnitude) const;

  inline int16_t magnitude_start() const;

  inline int16_t magnitude_end() const;

  inline int16_t nonzero_magnitude_start() const;

  inline int16_t nonzero_magnitude_end() const;

  inline bool is_zero() const;

  inline void multiply_pow10(int16_t power);

  inline ICU4XFixedDecimalSign sign() const;

  inline void set_sign(ICU4XFixedDecimalSign sign);

  inline void apply_sign_display(ICU4XFixedDecimalSignDisplay sign_display);

  inline void trim_start();

  inline void trim_end();

  inline void pad_start(int16_t position);

  inline void pad_end(int16_t position);

  inline void set_max_position(int16_t position);

  inline void trunc(int16_t position);

  inline void trunc_to_increment(int16_t position, ICU4XFixedDecimalRoundingIncrement increment);

  inline void half_trunc(int16_t position);

  inline void half_trunc_to_increment(int16_t position, ICU4XFixedDecimalRoundingIncrement increment);

  inline void expand(int16_t position);

  inline void expand_to_increment(int16_t position, ICU4XFixedDecimalRoundingIncrement increment);

  inline void half_expand(int16_t position);

  inline void half_expand_to_increment(int16_t position, ICU4XFixedDecimalRoundingIncrement increment);

  inline void ceil(int16_t position);

  inline void ceil_to_increment(int16_t position, ICU4XFixedDecimalRoundingIncrement increment);

  inline void half_ceil(int16_t position);

  inline void half_ceil_to_increment(int16_t position, ICU4XFixedDecimalRoundingIncrement increment);

  inline void floor(int16_t position);

  inline void floor_to_increment(int16_t position, ICU4XFixedDecimalRoundingIncrement increment);

  inline void half_floor(int16_t position);

  inline void half_floor_to_increment(int16_t position, ICU4XFixedDecimalRoundingIncrement increment);

  inline void half_even(int16_t position);

  inline void half_even_to_increment(int16_t position, ICU4XFixedDecimalRoundingIncrement increment);

  inline diplomat::result<std::monostate, std::monostate> concatenate_end(ICU4XFixedDecimal& other);

  inline std::string to_string() const;

  inline const capi::ICU4XFixedDecimal* AsFFI() const;
  inline capi::ICU4XFixedDecimal* AsFFI();
  inline static const ICU4XFixedDecimal* FromFFI(const capi::ICU4XFixedDecimal* ptr);
  inline static ICU4XFixedDecimal* FromFFI(capi::ICU4XFixedDecimal* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XFixedDecimal() = delete;
  ICU4XFixedDecimal(const ICU4XFixedDecimal&) = delete;
  ICU4XFixedDecimal(ICU4XFixedDecimal&&) noexcept = delete;
  ICU4XFixedDecimal operator=(const ICU4XFixedDecimal&) = delete;
  ICU4XFixedDecimal operator=(ICU4XFixedDecimal&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XFixedDecimal_D_HPP
