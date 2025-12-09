#ifndef ICU4X_IndicConjunctBreak_D_HPP
#define ICU4X_IndicConjunctBreak_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"
namespace icu4x {
class IndicConjunctBreak;
} // namespace icu4x



namespace icu4x {
namespace capi {
    enum IndicConjunctBreak {
      IndicConjunctBreak_None = 0,
      IndicConjunctBreak_Consonant = 1,
      IndicConjunctBreak_Extend = 2,
      IndicConjunctBreak_Linker = 3,
    };

    typedef struct IndicConjunctBreak_option {union { IndicConjunctBreak ok; }; bool is_ok; } IndicConjunctBreak_option;
} // namespace capi
} // namespace

namespace icu4x {
/**
 * See the [Rust documentation for `IndicConjunctBreak`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.IndicConjunctBreak.html) for more information.
 */
class IndicConjunctBreak {
public:
    enum Value {
        None = 0,
        Consonant = 1,
        Extend = 2,
        Linker = 3,
    };

    IndicConjunctBreak(): value(Value::None) {}

    // Implicit conversions between enum and ::Value
    constexpr IndicConjunctBreak(Value v) : value(v) {}
    constexpr operator Value() const { return value; }
    // Prevent usage as boolean value
    explicit operator bool() const = delete;

  /**
   * See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.EnumeratedProperty.html#tymethod.for_char) for more information.
   */
  inline static icu4x::IndicConjunctBreak for_char(char32_t ch);

  /**
   * Convert to an integer value usable with ICU4C and CodePointMapData
   *
   * See the [Rust documentation for `to_icu4c_value`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.IndicConjunctBreak.html#method.to_icu4c_value) for more information.
   */
  inline uint8_t to_integer_value() const;

  /**
   * Convert from an integer value from ICU4C or CodePointMapData
   *
   * See the [Rust documentation for `from_icu4c_value`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.IndicConjunctBreak.html#method.from_icu4c_value) for more information.
   */
  inline static std::optional<icu4x::IndicConjunctBreak> from_integer_value(uint8_t other);

    inline icu4x::capi::IndicConjunctBreak AsFFI() const;
    inline static icu4x::IndicConjunctBreak FromFFI(icu4x::capi::IndicConjunctBreak c_enum);
private:
    Value value;
};

} // namespace
#endif // ICU4X_IndicConjunctBreak_D_HPP
