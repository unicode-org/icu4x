#ifndef BidiDirection_D_HPP
#define BidiDirection_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    typedef enum BidiDirection {
      BidiDirection_Ltr = 0,
      BidiDirection_Rtl = 1,
      BidiDirection_Mixed = 2,
    } BidiDirection;
} // namespace capi
} // namespace

class BidiDirection {
public:
  enum Value {
    Ltr = 0,
    Rtl = 1,
    Mixed = 2,
  };

  BidiDirection() = default;
  // Implicit conversions between enum and ::Value
  constexpr BidiDirection(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline diplomat::capi::BidiDirection AsFFI() const;
  inline static BidiDirection FromFFI(diplomat::capi::BidiDirection c_enum);
private:
    Value value;
};


#endif // BidiDirection_D_HPP
