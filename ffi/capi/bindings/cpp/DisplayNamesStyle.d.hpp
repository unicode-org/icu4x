#ifndef DisplayNamesStyle_D_HPP
#define DisplayNamesStyle_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    enum DisplayNamesStyle {
      DisplayNamesStyle_Auto = 0,
      DisplayNamesStyle_Narrow = 1,
      DisplayNamesStyle_Short = 2,
      DisplayNamesStyle_Long = 3,
      DisplayNamesStyle_Menu = 4,
    };
} // namespace capi
} // namespace

class DisplayNamesStyle {
public:
  enum Value {
    Auto = 0,
    Narrow = 1,
    Short = 2,
    Long = 3,
    Menu = 4,
  };

  DisplayNamesStyle() = default;
  // Implicit conversions between enum and ::Value
  constexpr DisplayNamesStyle(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline diplomat::capi::DisplayNamesStyle AsFFI() const;
  inline static DisplayNamesStyle FromFFI(diplomat::capi::DisplayNamesStyle c_enum);
private:
    Value value;
};


#endif // DisplayNamesStyle_D_HPP
