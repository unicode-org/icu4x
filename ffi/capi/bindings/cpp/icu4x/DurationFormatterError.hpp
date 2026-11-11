#ifndef ICU4X_DurationFormatterError_HPP
#define ICU4X_DurationFormatterError_HPP

#include "DurationFormatterError.d.hpp"

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

} // namespace capi
} // namespace

inline icu4x::capi::DurationFormatterError icu4x::DurationFormatterError::AsFFI() const {
    return static_cast<icu4x::capi::DurationFormatterError>(value);
}

inline icu4x::DurationFormatterError icu4x::DurationFormatterError::FromFFI(icu4x::capi::DurationFormatterError c_enum) {
    switch (c_enum) {
        case icu4x::capi::DurationFormatterError_FractionalDigitsOutOfRange:
        case icu4x::capi::DurationFormatterError_PreviousNumeric:
        case icu4x::capi::DurationFormatterError_DisplayAlwaysFractional:
        case icu4x::capi::DurationFormatterError_PreviousFractional:
        case icu4x::capi::DurationFormatterError_InvalidUnitStyle:
        case icu4x::capi::DurationFormatterError_DataError:
            return static_cast<icu4x::DurationFormatterError::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // ICU4X_DurationFormatterError_HPP
