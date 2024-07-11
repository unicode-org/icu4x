#ifndef LineBreakWordOption_D_HPP
#define LineBreakWordOption_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    typedef enum LineBreakWordOption {
      LineBreakWordOption_Normal = 0,
      LineBreakWordOption_BreakAll = 1,
      LineBreakWordOption_KeepAll = 2,
    } LineBreakWordOption;
} // namespace capi
} // namespace

class LineBreakWordOption {
public:
  enum Value {
    Normal = 0,
    BreakAll = 1,
    KeepAll = 2,
  };

  LineBreakWordOption() = default;
  // Implicit conversions between enum and ::Value
  constexpr LineBreakWordOption(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline diplomat::capi::LineBreakWordOption AsFFI() const;
  inline static LineBreakWordOption FromFFI(diplomat::capi::LineBreakWordOption c_enum);
private:
    Value value;
};


#endif // LineBreakWordOption_D_HPP
