#ifndef ICU4X_PartCategory_D_HPP
#define ICU4X_PartCategory_D_HPP

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
    enum PartCategory {
      PartCategory_Unknown = 0,
      PartCategory_Error = 1,
      PartCategory_List = 2,
      PartCategory_Decimal = 3,
      PartCategory_DateTime = 4,
      PartCategory_RelativeTime = 5,
      PartCategory_Duration = 6,
      PartCategory_Unit = 7,
    };

    typedef struct PartCategory_option {union { PartCategory ok; }; bool is_ok; } PartCategory_option;
} // namespace capi
} // namespace

namespace icu4x {
/**
 * The category of a {@link PartKind}, corresponding to {@link writeable::Part::category}.
 *
 * See the [Rust documentation for `category`](https://docs.rs/writeable/0.6.4/writeable/struct.Part.html#structfield.category) for more information.
 *
 * 🚧 This API is unstable and may experience breaking changes outside major releases.
 */
class PartCategory {
public:
    enum Value {
        Unknown = 0,
        /**
         * The `"writeable"` category (used for {@link PartKind::Error}).
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        Error = 1,
        /**
         * The `"list"` category (`icu_list::parts`).
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        List = 2,
        /**
         * The `"decimal"` category (`icu_decimal::parts`).
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        Decimal = 3,
        /**
         * The `"datetime"` category (`icu_datetime::parts`).
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        DateTime = 4,
        /**
         * The `"relativetime"` category (`icu_experimental::relativetime::parts`).
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        RelativeTime = 5,
        /**
         * The `"duration"` category (`icu_experimental::duration::parts`).
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        Duration = 6,
        /**
         * The `"unit"` category (`icu_experimental::duration::parts`).
         *
         * 🚧 This API is unstable and may experience breaking changes outside major releases.
         */
        Unit = 7,
    };

    PartCategory(): value(Value::Unknown) {}

    // Implicit conversions between enum and ::Value
    constexpr PartCategory(Value v) : value(v) {}
    constexpr operator Value() const { return value; }
    // Prevent usage as boolean value
    explicit operator bool() const = delete;

    inline icu4x::capi::PartCategory AsFFI() const;
    inline static icu4x::PartCategory FromFFI(icu4x::capi::PartCategory c_enum);
private:
    Value value;
};

} // namespace
#endif // ICU4X_PartCategory_D_HPP
