#ifndef CollatorCaseLevel_D_HPP
#define CollatorCaseLevel_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    enum CollatorCaseLevel {
      CollatorCaseLevel_Auto = 0,
      CollatorCaseLevel_Off = 1,
      CollatorCaseLevel_On = 2,
    };
} // namespace capi
} // namespace

class CollatorCaseLevel {
public:
  enum Value {
    Auto = 0,
    Off = 1,
    On = 2,
  };

  CollatorCaseLevel() = default;
  // Implicit conversions between enum and ::Value
  constexpr CollatorCaseLevel(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline diplomat::capi::CollatorCaseLevel AsFFI() const;
  inline static CollatorCaseLevel FromFFI(diplomat::capi::CollatorCaseLevel c_enum);
private:
    Value value;
};


#endif // CollatorCaseLevel_D_HPP
