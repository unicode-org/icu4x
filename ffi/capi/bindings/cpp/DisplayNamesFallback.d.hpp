#ifndef DisplayNamesFallback_D_HPP
#define DisplayNamesFallback_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    typedef enum DisplayNamesFallback {
      DisplayNamesFallback_Code = 0,
      DisplayNamesFallback_None = 1,
    } DisplayNamesFallback;
} // namespace capi
} // namespace

class DisplayNamesFallback {
public:
  enum Value {
    Code = 0,
    None = 1,
  };

  DisplayNamesFallback() = default;
  // Implicit conversions between enum and ::Value
  constexpr DisplayNamesFallback(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline diplomat::capi::DisplayNamesFallback AsFFI() const;
  inline static DisplayNamesFallback FromFFI(diplomat::capi::DisplayNamesFallback c_enum);
private:
    Value value;
};


#endif // DisplayNamesFallback_D_HPP
