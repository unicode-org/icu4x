#ifndef icu4x_NeoDateTimeLength_D_HPP
#define icu4x_NeoDateTimeLength_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    enum NeoDateTimeLength {
      NeoDateTimeLength_Long = 0,
      NeoDateTimeLength_Medium = 1,
      NeoDateTimeLength_Short = 2,
    };
    
    typedef struct NeoDateTimeLength_option {union { NeoDateTimeLength ok; }; bool is_ok; } NeoDateTimeLength_option;
} // namespace capi
} // namespace

namespace icu4x {
class NeoDateTimeLength {
public:
  enum Value {
    Long = 0,
    Medium = 1,
    Short = 2,
  };

  NeoDateTimeLength() = default;
  // Implicit conversions between enum and ::Value
  constexpr NeoDateTimeLength(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline icu4x::capi::NeoDateTimeLength AsFFI() const;
  inline static icu4x::NeoDateTimeLength FromFFI(icu4x::capi::NeoDateTimeLength c_enum);
private:
    Value value;
};

} // namespace
#endif // icu4x_NeoDateTimeLength_D_HPP
