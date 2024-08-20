#ifndef icu4x_CollatorAlternateHandling_D_HPP
#define icu4x_CollatorAlternateHandling_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    enum CollatorAlternateHandling {
      CollatorAlternateHandling_Auto = 0,
      CollatorAlternateHandling_NonIgnorable = 1,
      CollatorAlternateHandling_Shifted = 2,
    };
    
    typedef struct CollatorAlternateHandling_option {union { CollatorAlternateHandling ok; }; bool is_ok; } CollatorAlternateHandling_option;
} // namespace capi
} // namespace

namespace icu4x {
class CollatorAlternateHandling {
public:
  enum Value {
    Auto = 0,
    NonIgnorable = 1,
    Shifted = 2,
  };

  CollatorAlternateHandling() = default;
  // Implicit conversions between enum and ::Value
  constexpr CollatorAlternateHandling(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline icu4x::capi::CollatorAlternateHandling AsFFI() const;
  inline static icu4x::CollatorAlternateHandling FromFFI(icu4x::capi::CollatorAlternateHandling c_enum);
private:
    Value value;
};

} // namespace
#endif // icu4x_CollatorAlternateHandling_D_HPP
