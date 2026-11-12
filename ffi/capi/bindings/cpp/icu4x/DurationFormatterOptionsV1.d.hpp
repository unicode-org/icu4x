#ifndef ICU4X_DurationFormatterOptionsV1_D_HPP
#define ICU4X_DurationFormatterOptionsV1_D_HPP

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
    struct DurationFormatterOptionsV1 {
      icu4x::capi::DurationBaseStyle base;
      icu4x::capi::DurationUnitStyle_option year;
      icu4x::capi::DurationFieldDisplay_option year_visibility;
      icu4x::capi::DurationUnitStyle_option month;
      icu4x::capi::DurationFieldDisplay_option month_visibility;
      icu4x::capi::DurationUnitStyle_option week;
      icu4x::capi::DurationFieldDisplay_option week_visibility;
      icu4x::capi::DurationUnitStyle_option day;
      icu4x::capi::DurationFieldDisplay_option day_visibility;
      icu4x::capi::DurationUnitStyle_option hour;
      icu4x::capi::DurationFieldDisplay_option hour_visibility;
      icu4x::capi::DurationUnitStyle_option minute;
      icu4x::capi::DurationFieldDisplay_option minute_visibility;
      icu4x::capi::DurationUnitStyle_option second;
      icu4x::capi::DurationFieldDisplay_option second_visibility;
      icu4x::capi::DurationUnitStyle_option millisecond;
      icu4x::capi::DurationFieldDisplay_option millisecond_visibility;
      icu4x::capi::DurationUnitStyle_option microsecond;
      icu4x::capi::DurationFieldDisplay_option microsecond_visibility;
      icu4x::capi::DurationUnitStyle_option nanosecond;
      icu4x::capi::DurationFieldDisplay_option nanosecond_visibility;
      icu4x::diplomat::capi::OptionU8 fractional_digits;
    };

    typedef struct DurationFormatterOptionsV1_option {union { DurationFormatterOptionsV1 ok; }; bool is_ok; } DurationFormatterOptionsV1_option;
} // namespace capi
} // namespace


namespace icu4x {
/**
 * See the [Rust documentation for `DurationFormatterOptions`](https://docs.rs/icu/2.4.0/icu/experimental/duration/options/struct.DurationFormatterOptions.html) for more information.
 *
 * 🚧 This API is unstable and may experience breaking changes outside major releases.
 */
struct DurationFormatterOptionsV1 {
    icu4x::DurationBaseStyle base;
    std::optional<icu4x::DurationUnitStyle> year;
    std::optional<icu4x::DurationFieldDisplay> year_visibility;
    std::optional<icu4x::DurationUnitStyle> month;
    std::optional<icu4x::DurationFieldDisplay> month_visibility;
    std::optional<icu4x::DurationUnitStyle> week;
    std::optional<icu4x::DurationFieldDisplay> week_visibility;
    std::optional<icu4x::DurationUnitStyle> day;
    std::optional<icu4x::DurationFieldDisplay> day_visibility;
    std::optional<icu4x::DurationUnitStyle> hour;
    std::optional<icu4x::DurationFieldDisplay> hour_visibility;
    std::optional<icu4x::DurationUnitStyle> minute;
    std::optional<icu4x::DurationFieldDisplay> minute_visibility;
    std::optional<icu4x::DurationUnitStyle> second;
    std::optional<icu4x::DurationFieldDisplay> second_visibility;
    std::optional<icu4x::DurationUnitStyle> millisecond;
    std::optional<icu4x::DurationFieldDisplay> millisecond_visibility;
    std::optional<icu4x::DurationUnitStyle> microsecond;
    std::optional<icu4x::DurationFieldDisplay> microsecond_visibility;
    std::optional<icu4x::DurationUnitStyle> nanosecond;
    std::optional<icu4x::DurationFieldDisplay> nanosecond_visibility;
    std::optional<uint8_t> fractional_digits;

    inline icu4x::capi::DurationFormatterOptionsV1 AsFFI() const;
    inline static icu4x::DurationFormatterOptionsV1 FromFFI(icu4x::capi::DurationFormatterOptionsV1 c_struct);
};

} // namespace
#endif // ICU4X_DurationFormatterOptionsV1_D_HPP
