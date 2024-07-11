#ifndef TrailingCase_D_HPP
#define TrailingCase_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef enum TrailingCase {
      TrailingCase_Lower = 0,
      TrailingCase_Unchanged = 1,
    } TrailingCase;
}

class TrailingCase {
public:
  enum Value {
    Lower = 0,
    Unchanged = 1,
  };

  TrailingCase() = default;
  // Implicit conversions between enum and ::Value
  constexpr TrailingCase(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::TrailingCase AsFFI() const;
  inline static TrailingCase FromFFI(capi::TrailingCase c_enum);
private:
    Value value;
};


#endif // TrailingCase_D_HPP
