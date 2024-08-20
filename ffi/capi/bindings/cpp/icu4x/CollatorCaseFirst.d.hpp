#ifndef icu4x_CollatorCaseFirst_D_HPP
#define icu4x_CollatorCaseFirst_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    enum CollatorCaseFirst {
      CollatorCaseFirst_Auto = 0,
      CollatorCaseFirst_Off = 1,
      CollatorCaseFirst_LowerFirst = 2,
      CollatorCaseFirst_UpperFirst = 3,
    };
    
    typedef struct CollatorCaseFirst_option {union { CollatorCaseFirst ok; }; bool is_ok; } CollatorCaseFirst_option;
} // namespace capi
} // namespace

namespace icu4x {
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

  inline icu4x::capi::CollatorCaseFirst AsFFI() const;
  inline static icu4x::CollatorCaseFirst FromFFI(icu4x::capi::CollatorCaseFirst c_enum);
private:
    Value value;
};

} // namespace
#endif // icu4x_CollatorCaseFirst_D_HPP
