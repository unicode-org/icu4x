#ifndef CollatorAlternateHandling_D_HPP
#define CollatorAlternateHandling_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    enum CollatorAlternateHandling {
      CollatorAlternateHandling_Auto = 0,
      CollatorAlternateHandling_NonIgnorable = 1,
      CollatorAlternateHandling_Shifted = 2,
    };
} // namespace capi
} // namespace

class CollatorAlternateHandling {
public:
  enum Value {
    Auto = 0,
    NonIgnorable = 1,
    Shifted = 2,
  };

  CollatorAlternateHandling() = default;
  // Implicit conversions between enum and ::Value
  constexpr CollatorAlternateHandling(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline diplomat::capi::CollatorAlternateHandling AsFFI() const;
  inline static CollatorAlternateHandling FromFFI(diplomat::capi::CollatorAlternateHandling c_enum);
private:
    Value value;
};


#endif // CollatorAlternateHandling_D_HPP
