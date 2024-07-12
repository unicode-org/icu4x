#ifndef LocaleFallbackPriority_D_HPP
#define LocaleFallbackPriority_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    enum LocaleFallbackPriority {
      LocaleFallbackPriority_Language = 0,
      LocaleFallbackPriority_Region = 1,
      LocaleFallbackPriority_Collation = 2,
    };
} // namespace capi
} // namespace

class LocaleFallbackPriority {
public:
  enum Value {
    Language = 0,
    Region = 1,
    Collation = 2,
  };

  LocaleFallbackPriority() = default;
  // Implicit conversions between enum and ::Value
  constexpr LocaleFallbackPriority(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline diplomat::capi::LocaleFallbackPriority AsFFI() const;
  inline static LocaleFallbackPriority FromFFI(diplomat::capi::LocaleFallbackPriority c_enum);
private:
    Value value;
};


#endif // LocaleFallbackPriority_D_HPP
