#ifndef icu4x_PatternLoadError_D_HPP
#define icu4x_PatternLoadError_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    enum PatternLoadError {
      PatternLoadError_Unknown = 0,
      PatternLoadError_UnsupportedField = 2051,
      PatternLoadError_DuplicateField = 2057,
      PatternLoadError_TypeTooSpecific = 2058,
      PatternLoadError_DataMarkerNotFound = 1,
      PatternLoadError_DataIdentifierNotFound = 2,
      PatternLoadError_DataInvalidRequest = 3,
      PatternLoadError_DataInconsistentData = 4,
      PatternLoadError_DataDowncast = 5,
      PatternLoadError_DataDeserialize = 6,
      PatternLoadError_DataCustom = 7,
      PatternLoadError_DataIo = 8,
    };
    
    typedef struct PatternLoadError_option {union { PatternLoadError ok; }; bool is_ok; } PatternLoadError_option;
} // namespace capi
} // namespace

namespace icu4x {
class PatternLoadError {
public:
  enum Value {
    Unknown = 0,
    UnsupportedField = 2051,
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

  PatternLoadError() = default;
  // Implicit conversions between enum and ::Value
  constexpr PatternLoadError(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline icu4x::capi::PatternLoadError AsFFI() const;
  inline static icu4x::PatternLoadError FromFFI(icu4x::capi::PatternLoadError c_enum);
private:
    Value value;
};

} // namespace
#endif // icu4x_PatternLoadError_D_HPP
