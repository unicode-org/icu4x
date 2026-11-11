#ifndef ICU4X_DurationFormatterError_D_HPP
#define ICU4X_DurationFormatterError_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    enum DurationFormatterError {
      DurationFormatterError_Unknown = 0,
      DurationFormatterError_FractionalDigitsOutOfRange = 2049,
      DurationFormatterError_PreviousNumeric = 2050,
      DurationFormatterError_DisplayAlwaysFractional = 2051,
      DurationFormatterError_PreviousFractional = 2052,
      DurationFormatterError_InvalidUnitStyle = 2053,
      DurationFormatterError_DataMarkerNotFound = 1,
      DurationFormatterError_DataIdentifierNotFound = 2,
      DurationFormatterError_DataInvalidRequest = 3,
      DurationFormatterError_DataInconsistentData = 4,
      DurationFormatterError_DataDowncast = 5,
      DurationFormatterError_DataDeserialize = 6,
      DurationFormatterError_DataCustom = 7,
      DurationFormatterError_DataIo = 8,
    };

    typedef struct DurationFormatterError_option {union { DurationFormatterError ok; }; bool is_ok; } DurationFormatterError_option;
} // namespace capi
} // namespace

namespace icu4x {
/**
 * See the [Rust documentation for `DurationFormatterOptionsError`](https://docs.rs/icu/2.4.0/icu/experimental/duration/enum.DurationFormatterOptionsError.html) for more information.
 *
 * Additional information: [1](https://docs.rs/icu_provider/2.4.0/icu_provider/struct.DataError.html), [2](https://docs.rs/icu_provider/2.4.0/icu_provider/enum.DataErrorKind.html)
 *
 * 🚧 This API is unstable and may experience breaking changes outside major releases.
 */
class DurationFormatterError {
public:
    enum Value {
        Unknown = 0,
        FractionalDigitsOutOfRange = 2049,
        PreviousNumeric = 2050,
        DisplayAlwaysFractional = 2051,
        PreviousFractional = 2052,
        InvalidUnitStyle = 2053,
        DataMarkerNotFound = 1,
        DataIdentifierNotFound = 2,
        DataInvalidRequest = 3,
        DataInconsistentData = 4,
        DataDowncast = 5,
        DataDeserialize = 6,
        DataCustom = 7,
        DataIo = 8,
    };

    DurationFormatterError(): value(Value::Unknown) {}

    // Implicit conversions between enum and ::Value
    constexpr DurationFormatterError(Value v) : value(v) {}
    constexpr operator Value() const { return value; }
    // Prevent usage as boolean value
    explicit operator bool() const = delete;

    inline icu4x::capi::DurationFormatterError AsFFI() const;
    inline static icu4x::DurationFormatterError FromFFI(icu4x::capi::DurationFormatterError c_enum);
private:
    Value value;
};

} // namespace
#endif // ICU4X_DurationFormatterError_D_HPP
