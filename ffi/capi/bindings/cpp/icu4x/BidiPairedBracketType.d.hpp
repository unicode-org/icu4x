#ifndef icu4x_BidiPairedBracketType_D_HPP
#define icu4x_BidiPairedBracketType_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    enum BidiPairedBracketType {
      BidiPairedBracketType_Open = 0,
      BidiPairedBracketType_Close = 1,
      BidiPairedBracketType_None = 2,
    };

    typedef struct BidiPairedBracketType_option {union { BidiPairedBracketType ok; }; bool is_ok; } BidiPairedBracketType_option;
} // namespace capi
} // namespace

namespace icu4x {
/**
 * See the [Rust documentation for `BidiPairedBracketType`](https://docs.rs/icu/2.0.0/icu/properties/props/enum.BidiPairedBracketType.html) for more information.
 */
class BidiPairedBracketType {
public:
  enum Value {
    Open = 0,
    Close = 1,
    None = 2,
  };

  BidiPairedBracketType() = default;
  // Implicit conversions between enum and ::Value
  constexpr BidiPairedBracketType(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline icu4x::capi::BidiPairedBracketType AsFFI() const;
  inline static icu4x::BidiPairedBracketType FromFFI(icu4x::capi::BidiPairedBracketType c_enum);
private:
    Value value;
};

} // namespace
#endif // icu4x_BidiPairedBracketType_D_HPP
