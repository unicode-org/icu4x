#ifndef LeadingAdjustment_D_HPP
#define LeadingAdjustment_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    typedef enum LeadingAdjustment {
      LeadingAdjustment_Auto = 0,
      LeadingAdjustment_None = 1,
      LeadingAdjustment_ToCased = 2,
    } LeadingAdjustment;
} // namespace capi
} // namespace

class LeadingAdjustment {
public:
  enum Value {
    Auto = 0,
    None = 1,
    ToCased = 2,
  };

  LeadingAdjustment() = default;
  // Implicit conversions between enum and ::Value
  constexpr LeadingAdjustment(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline diplomat::capi::LeadingAdjustment AsFFI() const;
  inline static LeadingAdjustment FromFFI(diplomat::capi::LeadingAdjustment c_enum);
private:
    Value value;
};


#endif // LeadingAdjustment_D_HPP
