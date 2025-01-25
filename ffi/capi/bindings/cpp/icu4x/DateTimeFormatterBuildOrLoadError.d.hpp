#ifndef icu4x_DateTimeFormatterBuildOrLoadError_D_HPP
#define icu4x_DateTimeFormatterBuildOrLoadError_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    enum DateTimeFormatterBuildOrLoadError {
      DateTimeFormatterBuildOrLoadError_Unknown = 0,
      DateTimeFormatterBuildOrLoadError_InvalidFields = 256,
      DateTimeFormatterBuildOrLoadError_InvalidOptions = 257,
      DateTimeFormatterBuildOrLoadError_UnsupportedLength = 2051,
      DateTimeFormatterBuildOrLoadError_DuplicateField = 2057,
      DateTimeFormatterBuildOrLoadError_TypeTooSpecific = 2058,
      DateTimeFormatterBuildOrLoadError_DataMarkerNotFound = 1,
      DateTimeFormatterBuildOrLoadError_DataIdentifierNotFound = 2,
      DateTimeFormatterBuildOrLoadError_DataInvalidRequest = 3,
      DateTimeFormatterBuildOrLoadError_DataInconsistentData = 4,
      DateTimeFormatterBuildOrLoadError_DataDowncast = 5,
      DateTimeFormatterBuildOrLoadError_DataDeserialize = 6,
      DateTimeFormatterBuildOrLoadError_DataCustom = 7,
      DateTimeFormatterBuildOrLoadError_DataIo = 8,
    };
    
    typedef struct DateTimeFormatterBuildOrLoadError_option {union { DateTimeFormatterBuildOrLoadError ok; }; bool is_ok; } DateTimeFormatterBuildOrLoadError_option;
} // namespace capi
} // namespace

namespace icu4x {
class DateTimeFormatterBuildOrLoadError {
public:
  enum Value {
    Unknown = 0,
    InvalidFields = 256,
    InvalidOptions = 257,
    UnsupportedLength = 2051,
    DuplicateField = 2057,
    TypeTooSpecific = 2058,
    DataMarkerNotFound = 1,
    DataIdentifierNotFound = 2,
    DataInvalidRequest = 3,
    DataInconsistentData = 4,
    DataDowncast = 5,
    DataDeserialize = 6,
    DataCustom = 7,
    DataIo = 8,
  };

  DateTimeFormatterBuildOrLoadError() = default;
  // Implicit conversions between enum and ::Value
  constexpr DateTimeFormatterBuildOrLoadError(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline icu4x::capi::DateTimeFormatterBuildOrLoadError AsFFI() const;
  inline static icu4x::DateTimeFormatterBuildOrLoadError FromFFI(icu4x::capi::DateTimeFormatterBuildOrLoadError c_enum);
private:
    Value value;
};

} // namespace
#endif // icu4x_DateTimeFormatterBuildOrLoadError_D_HPP
