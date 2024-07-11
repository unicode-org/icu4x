#ifndef TitlecaseOptionsV1_D_HPP
#define TitlecaseOptionsV1_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "LeadingAdjustment.d.hpp"
#include "TrailingCase.d.hpp"

class LeadingAdjustment;
class TrailingCase;


namespace capi {
    typedef struct TitlecaseOptionsV1 {
      LeadingAdjustment leading_adjustment;
      TrailingCase trailing_case;
    } TitlecaseOptionsV1;
}

struct TitlecaseOptionsV1 {
  LeadingAdjustment leading_adjustment;
  TrailingCase trailing_case;

  inline static TitlecaseOptionsV1 default_options();

  inline capi::TitlecaseOptionsV1 AsFFI() const;
  inline static TitlecaseOptionsV1 FromFFI(capi::TitlecaseOptionsV1 c_struct);
};


#endif // TitlecaseOptionsV1_D_HPP
