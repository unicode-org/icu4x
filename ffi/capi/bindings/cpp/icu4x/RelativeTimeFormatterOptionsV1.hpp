#ifndef ICU4X_RelativeTimeFormatterOptionsV1_HPP
#define ICU4X_RelativeTimeFormatterOptionsV1_HPP

#include "RelativeTimeFormatterOptionsV1.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "DecimalGroupingStrategy.hpp"
#include "RelativeTimeNumeric.hpp"
#include "diplomat_runtime.hpp"


namespace icu4x {
namespace capi {

} // namespace capi
} // namespace


inline icu4x::capi::RelativeTimeFormatterOptionsV1 icu4x::RelativeTimeFormatterOptionsV1::AsFFI() const {
    return icu4x::capi::RelativeTimeFormatterOptionsV1 {
        /* .numeric = */ numeric.has_value() ? (icu4x::capi::RelativeTimeNumeric_option{ { numeric.value().AsFFI() }, true }) : (icu4x::capi::RelativeTimeNumeric_option{ {}, false }),
        /* .grouping_strategy = */ grouping_strategy.has_value() ? (icu4x::capi::DecimalGroupingStrategy_option{ { grouping_strategy.value().AsFFI() }, true }) : (icu4x::capi::DecimalGroupingStrategy_option{ {}, false }),
    };
}

inline icu4x::RelativeTimeFormatterOptionsV1 icu4x::RelativeTimeFormatterOptionsV1::FromFFI(icu4x::capi::RelativeTimeFormatterOptionsV1 c_struct) {
    return icu4x::RelativeTimeFormatterOptionsV1 {
        /* .numeric = */ c_struct.numeric.is_ok ? std::optional(icu4x::RelativeTimeNumeric::FromFFI(c_struct.numeric.ok)) : std::nullopt,
        /* .grouping_strategy = */ c_struct.grouping_strategy.is_ok ? std::optional(icu4x::DecimalGroupingStrategy::FromFFI(c_struct.grouping_strategy.ok)) : std::nullopt,
    };
}


#endif // ICU4X_RelativeTimeFormatterOptionsV1_HPP
