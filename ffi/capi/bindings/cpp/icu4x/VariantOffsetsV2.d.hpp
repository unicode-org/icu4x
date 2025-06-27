#ifndef icu4x_VariantOffsetsV2_D_HPP
#define icu4x_VariantOffsetsV2_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"

namespace icu4x {
namespace capi { struct UtcOffset; }
class UtcOffset;
}


namespace icu4x {
namespace capi {
    struct VariantOffsetsV2 {
      icu4x::capi::UtcOffset* standard;
      icu4x::capi::UtcOffset* daylight;
      icu4x::capi::UtcOffset* sundown;
    };

    typedef struct VariantOffsetsV2_option {union { VariantOffsetsV2 ok; }; bool is_ok; } VariantOffsetsV2_option;
} // namespace capi
} // namespace


namespace icu4x {
/**
 * See the [Rust documentation for `VariantOffsets`](https://docs.rs/icu/2.0.0/icu/time/zone/struct.VariantOffsets.html) for more information.
 */
struct VariantOffsetsV2 {
  std::unique_ptr<icu4x::UtcOffset> standard;
  std::unique_ptr<icu4x::UtcOffset> daylight;
  std::unique_ptr<icu4x::UtcOffset> sundown;

  inline icu4x::capi::VariantOffsetsV2 AsFFI() const;
  inline static icu4x::VariantOffsetsV2 FromFFI(icu4x::capi::VariantOffsetsV2 c_struct);
};

} // namespace
#endif // icu4x_VariantOffsetsV2_D_HPP
