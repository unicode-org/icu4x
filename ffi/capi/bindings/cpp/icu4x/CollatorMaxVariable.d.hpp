#ifndef icu4x_CollatorMaxVariable_D_HPP
#define icu4x_CollatorMaxVariable_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    enum CollatorMaxVariable {
      CollatorMaxVariable_Auto = 0,
      CollatorMaxVariable_Space = 1,
      CollatorMaxVariable_Punctuation = 2,
      CollatorMaxVariable_Symbol = 3,
      CollatorMaxVariable_Currency = 4,
    };
    
    typedef struct CollatorMaxVariable_option {union { CollatorMaxVariable ok; }; bool is_ok; } CollatorMaxVariable_option;
} // namespace capi
} // namespace

namespace icu4x {
class CollatorMaxVariable {
public:
  enum Value {
    Auto = 0,
    Space = 1,
    Punctuation = 2,
    Symbol = 3,
    Currency = 4,
  };

  CollatorMaxVariable() = default;
  // Implicit conversions between enum and ::Value
  constexpr CollatorMaxVariable(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline icu4x::capi::CollatorMaxVariable AsFFI() const;
  inline static icu4x::CollatorMaxVariable FromFFI(icu4x::capi::CollatorMaxVariable c_enum);
private:
    Value value;
};

} // namespace
#endif // icu4x_CollatorMaxVariable_D_HPP
