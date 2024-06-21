#ifndef ICU4XTitlecaseOptionsV1_D_HPP
#define ICU4XTitlecaseOptionsV1_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XLeadingAdjustment.d.hpp"
#include "ICU4XTrailingCase.d.hpp"

class ICU4XLeadingAdjustment;
class ICU4XTrailingCase;


namespace capi {
    typedef struct ICU4XTitlecaseOptionsV1 {
      ICU4XLeadingAdjustment leading_adjustment;
      ICU4XTrailingCase trailing_case;
    } ICU4XTitlecaseOptionsV1;
}

struct ICU4XTitlecaseOptionsV1 {
  ICU4XLeadingAdjustment leading_adjustment;
  ICU4XTrailingCase trailing_case;

  inline static ICU4XTitlecaseOptionsV1 default_options();

  inline capi::ICU4XTitlecaseOptionsV1 AsFFI() const;
  inline static ICU4XTitlecaseOptionsV1 FromFFI(capi::ICU4XTitlecaseOptionsV1 c_struct);
};


#endif // ICU4XTitlecaseOptionsV1_D_HPP
