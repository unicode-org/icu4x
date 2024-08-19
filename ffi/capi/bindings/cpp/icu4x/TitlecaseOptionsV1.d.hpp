#ifndef icu4x_TitlecaseOptionsV1_D_HPP
#define icu4x_TitlecaseOptionsV1_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"
#include "LeadingAdjustment.d.hpp"
#include "TrailingCase.d.hpp"

namespace icu4x {
struct TitlecaseOptionsV1;
class LeadingAdjustment;
class TrailingCase;
}


namespace icu4x {
namespace capi {
    struct TitlecaseOptionsV1 {
      icu4x::capi::LeadingAdjustment leading_adjustment;
      icu4x::capi::TrailingCase trailing_case;
    };
} // namespace capi
} // namespace


namespace icu4x {
struct TitlecaseOptionsV1 {
  icu4x::LeadingAdjustment leading_adjustment;
  icu4x::TrailingCase trailing_case;

  inline static icu4x::TitlecaseOptionsV1 default_options();

  inline icu4x::capi::TitlecaseOptionsV1 AsFFI() const;
  inline static icu4x::TitlecaseOptionsV1 FromFFI(icu4x::capi::TitlecaseOptionsV1 c_struct);
};

} // namespace
#endif // icu4x_TitlecaseOptionsV1_D_HPP
