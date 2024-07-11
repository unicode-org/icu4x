#ifndef CollatorCaseFirst_D_HPP
#define CollatorCaseFirst_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef enum CollatorCaseFirst {
      CollatorCaseFirst_Auto = 0,
      CollatorCaseFirst_Off = 1,
      CollatorCaseFirst_LowerFirst = 2,
      CollatorCaseFirst_UpperFirst = 3,
    } CollatorCaseFirst;
}

class CollatorCaseFirst {
public:
  enum Value {
    Auto = 0,
    Off = 1,
    LowerFirst = 2,
    UpperFirst = 3,
  };

  CollatorCaseFirst() = default;
  // Implicit conversions between enum and ::Value
  constexpr CollatorCaseFirst(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::CollatorCaseFirst AsFFI() const;
  inline static CollatorCaseFirst FromFFI(capi::CollatorCaseFirst c_enum);
private:
    Value value;
};


#endif // CollatorCaseFirst_D_HPP
