#ifndef ICU4X_DurationFormatterOptions_D_HPP
#define ICU4X_DurationFormatterOptions_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "DurationBaseStyle.d.hpp"
#include "DurationFieldDisplay.d.hpp"
#include "DurationUnitStyle.d.hpp"
#include "diplomat_runtime.hpp"
namespace icu4x {
class DurationBaseStyle;
class DurationFieldDisplay;
class DurationUnitStyle;
} // namespace icu4x



namespace icu4x {
namespace capi {
    struct DurationFormatterOptions {
      icu4x::capi::DurationBaseStyle base;
      icu4x::capi::DurationUnitStyle year;
      icu4x::capi::DurationFieldDisplay year_visibility;
      icu4x::capi::DurationUnitStyle month;
      icu4x::capi::DurationFieldDisplay month_visibility;
      icu4x::capi::DurationUnitStyle week;
      icu4x::capi::DurationFieldDisplay week_visibility;
      icu4x::capi::DurationUnitStyle day;
      icu4x::capi::DurationFieldDisplay day_visibility;
      icu4x::capi::DurationUnitStyle hour;
      icu4x::capi::DurationFieldDisplay hour_visibility;
      icu4x::capi::DurationUnitStyle minute;
      icu4x::capi::DurationFieldDisplay minute_visibility;
      icu4x::capi::DurationUnitStyle second;
      icu4x::capi::DurationFieldDisplay second_visibility;
      icu4x::capi::DurationUnitStyle millisecond;
      icu4x::capi::DurationFieldDisplay millisecond_visibility;
      icu4x::capi::DurationUnitStyle microsecond;
      icu4x::capi::DurationFieldDisplay microsecond_visibility;
      icu4x::capi::DurationUnitStyle nanosecond;
      icu4x::capi::DurationFieldDisplay nanosecond_visibility;
      int8_t fractional_digits;
    };

    typedef struct DurationFormatterOptions_option {union { DurationFormatterOptions ok; }; bool is_ok; } DurationFormatterOptions_option;
} // namespace capi
} // namespace


namespace icu4x {
/**
 * See the [Rust documentation for `DurationFormatterOptions`](https://docs.rs/icu/2.4.0/icu/experimental/duration/options/struct.DurationFormatterOptions.html) for more information.
 *
 * 🚧 This API is unstable and may experience breaking changes outside major releases.
 */
struct DurationFormatterOptions {
    icu4x::DurationBaseStyle base;
    icu4x::DurationUnitStyle year;
    icu4x::DurationFieldDisplay year_visibility;
    icu4x::DurationUnitStyle month;
    icu4x::DurationFieldDisplay month_visibility;
    icu4x::DurationUnitStyle week;
    icu4x::DurationFieldDisplay week_visibility;
    icu4x::DurationUnitStyle day;
    icu4x::DurationFieldDisplay day_visibility;
    icu4x::DurationUnitStyle hour;
    icu4x::DurationFieldDisplay hour_visibility;
    icu4x::DurationUnitStyle minute;
    icu4x::DurationFieldDisplay minute_visibility;
    icu4x::DurationUnitStyle second;
    icu4x::DurationFieldDisplay second_visibility;
    icu4x::DurationUnitStyle millisecond;
    icu4x::DurationFieldDisplay millisecond_visibility;
    icu4x::DurationUnitStyle microsecond;
    icu4x::DurationFieldDisplay microsecond_visibility;
    icu4x::DurationUnitStyle nanosecond;
    icu4x::DurationFieldDisplay nanosecond_visibility;
    int8_t fractional_digits;

    inline icu4x::capi::DurationFormatterOptions AsFFI() const;
    inline static icu4x::DurationFormatterOptions FromFFI(icu4x::capi::DurationFormatterOptions c_struct);
};

} // namespace
#endif // ICU4X_DurationFormatterOptions_D_HPP
