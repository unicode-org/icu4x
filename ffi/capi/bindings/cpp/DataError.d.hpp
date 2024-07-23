#ifndef DataError_D_HPP
#define DataError_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    enum DataError {
      DataError_Unknown = 0,
      DataError_MarkerNotFound = 1,
      DataError_IdentifierNotFound = 2,
      DataError_InvalidRequest = 3,
      DataError_InconsistentData = 4,
      DataError_Downcast = 5,
      DataError_Deserialize = 6,
      DataError_Custom = 7,
      DataError_Io = 8,
    };
} // namespace capi
} // namespace

class DataError {
public:
  enum Value {
    Unknown = 0,
    MarkerNotFound = 1,
    IdentifierNotFound = 2,
    InvalidRequest = 3,
    InconsistentData = 4,
    Downcast = 5,
    Deserialize = 6,
    Custom = 7,
    Io = 8,
  };

  DataError() = default;
  // Implicit conversions between enum and ::Value
  constexpr DataError(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline diplomat::capi::DataError AsFFI() const;
  inline static DataError FromFFI(diplomat::capi::DataError c_enum);
private:
    Value value;
};


#endif // DataError_D_HPP
