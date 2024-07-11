#ifndef LocaleParseError_D_HPP
#define LocaleParseError_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    typedef enum LocaleParseError {
      LocaleParseError_Unknown = 0,
      LocaleParseError_Language = 1,
      LocaleParseError_Subtag = 2,
      LocaleParseError_Extension = 3,
    } LocaleParseError;
} // namespace capi
} // namespace

class LocaleParseError {
public:
  enum Value {
    Unknown = 0,
    Language = 1,
    Subtag = 2,
    Extension = 3,
  };

  LocaleParseError() = default;
  // Implicit conversions between enum and ::Value
  constexpr LocaleParseError(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline diplomat::capi::LocaleParseError AsFFI() const;
  inline static LocaleParseError FromFFI(diplomat::capi::LocaleParseError c_enum);
private:
    Value value;
};


#endif // LocaleParseError_D_HPP
