#ifndef LocaleFallbackSupplement_D_HPP
#define LocaleFallbackSupplement_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    enum LocaleFallbackSupplement {
      LocaleFallbackSupplement_None = 0,
      LocaleFallbackSupplement_Collation = 1,
    };
} // namespace capi
} // namespace

class LocaleFallbackSupplement {
public:
  enum Value {
    None = 0,
    Collation = 1,
  };

  LocaleFallbackSupplement() = default;
  // Implicit conversions between enum and ::Value
  constexpr LocaleFallbackSupplement(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline diplomat::capi::LocaleFallbackSupplement AsFFI() const;
  inline static LocaleFallbackSupplement FromFFI(diplomat::capi::LocaleFallbackSupplement c_enum);
private:
    Value value;
};


#endif // LocaleFallbackSupplement_D_HPP
