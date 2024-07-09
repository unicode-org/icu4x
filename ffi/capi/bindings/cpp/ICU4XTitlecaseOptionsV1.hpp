#ifndef ICU4XTitlecaseOptionsV1_HPP
#define ICU4XTitlecaseOptionsV1_HPP

#include "ICU4XTitlecaseOptionsV1.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XLeadingAdjustment.hpp"
#include "ICU4XTrailingCase.hpp"


namespace capi {
    extern "C" {
    
    ICU4XTitlecaseOptionsV1 ICU4XTitlecaseOptionsV1_default_options();
    
    
    } // extern "C"
}

inline ICU4XTitlecaseOptionsV1 ICU4XTitlecaseOptionsV1::default_options() {
  auto result = capi::ICU4XTitlecaseOptionsV1_default_options();
  return ICU4XTitlecaseOptionsV1::FromFFI(result);
}


inline capi::ICU4XTitlecaseOptionsV1 ICU4XTitlecaseOptionsV1::AsFFI() const {
  return capi::ICU4XTitlecaseOptionsV1 {
    .leading_adjustment = leading_adjustment.AsFFI(),
    .trailing_case = trailing_case.AsFFI(),
  };
}

inline ICU4XTitlecaseOptionsV1 ICU4XTitlecaseOptionsV1::FromFFI(capi::ICU4XTitlecaseOptionsV1 c_struct) {
  return ICU4XTitlecaseOptionsV1 {
    .leading_adjustment = ICU4XLeadingAdjustment::FromFFI(c_struct.leading_adjustment),
    .trailing_case = ICU4XTrailingCase::FromFFI(c_struct.trailing_case),
  };
}


#endif // ICU4XTitlecaseOptionsV1_HPP
