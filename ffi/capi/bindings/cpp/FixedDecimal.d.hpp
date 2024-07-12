#ifndef FixedDecimal_D_HPP
#define FixedDecimal_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

class FixedDecimalLimitError;
class FixedDecimalParseError;
class FixedDecimalRoundingIncrement;
class FixedDecimalRoundingMode;
class FixedDecimalSign;
class FixedDecimalSignDisplay;


namespace diplomat {
namespace capi {
    struct FixedDecimal;
} // namespace capi
} // namespace

class FixedDecimal {
public:

  inline static std::unique_ptr<FixedDecimal> create_from_i32(int32_t v);

  inline static std::unique_ptr<FixedDecimal> create_from_u32(uint32_t v);

  inline static std::unique_ptr<FixedDecimal> create_from_i64(int64_t v);

  inline static std::unique_ptr<FixedDecimal> create_from_u64(uint64_t v);

  inline static diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalLimitError> create_from_f64_with_integer_precision(double f);

  inline static diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalLimitError> create_from_f64_with_lower_magnitude(double f, int16_t magnitude);

  inline static diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalLimitError> create_from_f64_with_significant_digits(double f, uint8_t digits);

  inline static diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalLimitError> create_from_f64_with_floating_precision(double f);

  inline static diplomat::result<std::unique_ptr<FixedDecimal>, FixedDecimalParseError> create_from_string(std::string_view v);

  inline uint8_t digit_at(int16_t magnitude) const;

  inline int16_t magnitude_start() const;

  inline int16_t magnitude_end() const;

  inline int16_t nonzero_magnitude_start() const;

  inline int16_t nonzero_magnitude_end() const;

  inline bool is_zero() const;

  inline void multiply_pow10(int16_t power);

  inline FixedDecimalSign sign() const;

  inline void set_sign(FixedDecimalSign sign);

  inline void apply_sign_display(FixedDecimalSignDisplay sign_display);

  inline void trim_start();

  inline void trim_end();

  inline void pad_start(int16_t position);

  inline void pad_end(int16_t position);

  inline void set_max_position(int16_t position);

  inline void round(int16_t position);

  inline void ceil(int16_t position);

  inline void expand(int16_t position);

  inline void floor(int16_t position);

  inline void trunc(int16_t position);

  inline void round_with_mode(int16_t position, FixedDecimalRoundingMode mode);

  inline void round_with_mode_and_increment(int16_t position, FixedDecimalRoundingMode mode, FixedDecimalRoundingIncrement increment);

  inline diplomat::result<std::monostate, std::monostate> concatenate_end(FixedDecimal& other);

  inline std::string to_string() const;

  inline const diplomat::capi::FixedDecimal* AsFFI() const;
  inline diplomat::capi::FixedDecimal* AsFFI();
  inline static const FixedDecimal* FromFFI(const diplomat::capi::FixedDecimal* ptr);
  inline static FixedDecimal* FromFFI(diplomat::capi::FixedDecimal* ptr);
  inline static void operator delete(void* ptr);
private:
  FixedDecimal() = delete;
  FixedDecimal(const FixedDecimal&) = delete;
  FixedDecimal(FixedDecimal&&) noexcept = delete;
  FixedDecimal operator=(const FixedDecimal&) = delete;
  FixedDecimal operator=(FixedDecimal&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // FixedDecimal_D_HPP
