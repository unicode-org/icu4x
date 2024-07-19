#ifndef TitlecaseOptionsV1_D_HPP
#define TitlecaseOptionsV1_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "LeadingAdjustment.d.hpp"
#include "TrailingCase.d.hpp"
#include "diplomat_runtime.hpp"

class LeadingAdjustment;
class TrailingCase;


namespace diplomat {
namespace capi {
    struct TitlecaseOptionsV1 {
      diplomat::capi::LeadingAdjustment leading_adjustment;
      diplomat::capi::TrailingCase trailing_case;
    };
} // namespace capi
} // namespace


struct TitlecaseOptionsV1 {
  LeadingAdjustment leading_adjustment;
  TrailingCase trailing_case;

  inline static TitlecaseOptionsV1 default_options();

  inline diplomat::capi::TitlecaseOptionsV1 AsFFI() const;
  inline static TitlecaseOptionsV1 FromFFI(diplomat::capi::TitlecaseOptionsV1 c_struct);
};


#endif // TitlecaseOptionsV1_D_HPP
