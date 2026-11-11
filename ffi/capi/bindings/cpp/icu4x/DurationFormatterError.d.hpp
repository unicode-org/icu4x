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
      DurationFormatterError_FractionalDigitsOutOfRange = 0,
      DurationFormatterError_PreviousNumeric = 1,
      DurationFormatterError_DisplayAlwaysFractional = 2,
      DurationFormatterError_PreviousFractional = 3,
      DurationFormatterError_InvalidUnitStyle = 4,
      DurationFormatterError_DataError = 5,
    };

    typedef struct DurationFormatterError_option {union { DurationFormatterError ok; }; bool is_ok; } DurationFormatterError_option;
} // namespace capi
} // namespace

namespace icu4x {
/**
 * See the [Rust documentation for `DurationFormatterOptionsError`](https://docs.rs/icu/2.4.0/icu/experimental/duration/enum.DurationFormatterOptionsError.html) for more information.
 *
 * 🚧 This API is unstable and may experience breaking changes outside major releases.
 */
class DurationFormatterError {
public:
    enum Value {
        FractionalDigitsOutOfRange = 0,
        PreviousNumeric = 1,
        DisplayAlwaysFractional = 2,
        PreviousFractional = 3,
        InvalidUnitStyle = 4,
        DataError = 5,
    };

    DurationFormatterError(): value(Value::FractionalDigitsOutOfRange) {}

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
