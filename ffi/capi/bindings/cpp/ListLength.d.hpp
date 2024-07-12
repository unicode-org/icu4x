#ifndef ListLength_D_HPP
#define ListLength_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    enum ListLength {
      ListLength_Wide = 0,
      ListLength_Short = 1,
      ListLength_Narrow = 2,
    };
} // namespace capi
} // namespace

class ListLength {
public:
  enum Value {
    Wide = 0,
    Short = 1,
    Narrow = 2,
  };

  ListLength() = default;
  // Implicit conversions between enum and ::Value
  constexpr ListLength(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline diplomat::capi::ListLength AsFFI() const;
  inline static ListLength FromFFI(diplomat::capi::ListLength c_enum);
private:
    Value value;
};


#endif // ListLength_D_HPP
