#ifndef ICU4X_PartSpan_D_HPP
#define ICU4X_PartSpan_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "PartKind.d.hpp"
#include "diplomat_runtime.hpp"
namespace icu4x {
class PartKind;
} // namespace icu4x



namespace icu4x {
namespace capi {
    struct PartSpan {
      size_t start;
      size_t end;
      icu4x::capi::PartKind kind;
    };

    typedef struct PartSpan_option {union { PartSpan ok; }; bool is_ok; } PartSpan_option;
} // namespace capi
} // namespace


namespace icu4x {
/**
 * A span within a formatted string annotated with a {@link PartKind}.
 *
 * `start` and `end` are code-unit offsets into the string written by the corresponding
 * `*_to_parts` method (UTF-8 byte offsets for UTF-8 methods, or UTF-16 code-unit offsets
 * for UTF-16 methods).
 * Spans are recorded in pre-order: ordered by `start` ascending, then `end` descending,
 * with outer spans preceding inner nested spans.
 *
 * See the [Rust documentation for `Part`](https://docs.rs/writeable/0.6.4/writeable/struct.Part.html) for more information.
 *
 * 🚧 This API is unstable and may experience breaking changes outside major releases.
 */
struct PartSpan {
    size_t start;
    size_t end;
    icu4x::PartKind kind;

    inline icu4x::capi::PartSpan AsFFI() const;
    inline static icu4x::PartSpan FromFFI(icu4x::capi::PartSpan c_struct);
};

} // namespace
#endif // ICU4X_PartSpan_D_HPP
