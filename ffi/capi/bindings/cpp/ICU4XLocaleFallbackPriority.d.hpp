#ifndef ICU4XLocaleFallbackPriority_D_HPP
#define ICU4XLocaleFallbackPriority_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef enum ICU4XLocaleFallbackPriority {
      ICU4XLocaleFallbackPriority_Language = 0,
      ICU4XLocaleFallbackPriority_Region = 1,
      ICU4XLocaleFallbackPriority_Collation = 2,
    } ICU4XLocaleFallbackPriority;
}

class ICU4XLocaleFallbackPriority {
public:
  enum Value {
    Language = 0,
    Region = 1,
    Collation = 2,
  };

  ICU4XLocaleFallbackPriority() = default;
  // Implicit conversions between enum and ::Value
  constexpr ICU4XLocaleFallbackPriority(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::ICU4XLocaleFallbackPriority AsFFI() const;
  inline static ICU4XLocaleFallbackPriority FromFFI(capi::ICU4XLocaleFallbackPriority c_enum);
private:
    Value value;
};


#endif // ICU4XLocaleFallbackPriority_D_HPP
