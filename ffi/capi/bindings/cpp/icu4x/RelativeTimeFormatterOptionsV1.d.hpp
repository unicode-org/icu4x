#ifndef ICU4X_RelativeTimeFormatterOptionsV1_D_HPP
#define ICU4X_RelativeTimeFormatterOptionsV1_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "DecimalGroupingStrategy.d.hpp"
#include "RelativeTimeNumeric.d.hpp"
#include "diplomat_runtime.hpp"
namespace icu4x {
class DecimalGroupingStrategy;
class RelativeTimeNumeric;
} // namespace icu4x



namespace icu4x {
namespace capi {
    struct RelativeTimeFormatterOptionsV1 {
      icu4x::capi::RelativeTimeNumeric_option numeric;
      icu4x::capi::DecimalGroupingStrategy_option grouping_strategy;
    };

    typedef struct RelativeTimeFormatterOptionsV1_option {union { RelativeTimeFormatterOptionsV1 ok; }; bool is_ok; } RelativeTimeFormatterOptionsV1_option;
} // namespace capi
} // namespace


namespace icu4x {
/**
 * A bag of options for defining how to format time using {@link RelativeTimeFormatter}.
 *
 * See the [Rust documentation for `RelativeTimeFormatterOptions`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatterOptions.html) for more information.
 *
 * 🚧 This API is unstable and may experience breaking changes outside major releases.
 */
struct RelativeTimeFormatterOptionsV1 {
    std::optional<icu4x::RelativeTimeNumeric> numeric;
    std::optional<icu4x::DecimalGroupingStrategy> grouping_strategy;

    inline icu4x::capi::RelativeTimeFormatterOptionsV1 AsFFI() const;
    inline static icu4x::RelativeTimeFormatterOptionsV1 FromFFI(icu4x::capi::RelativeTimeFormatterOptionsV1 c_struct);
};

} // namespace
#endif // ICU4X_RelativeTimeFormatterOptionsV1_D_HPP
