#ifndef TitlecaseOptionsV1_HPP
#define TitlecaseOptionsV1_HPP

#include "TitlecaseOptionsV1.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "LeadingAdjustment.hpp"
#include "TrailingCase.hpp"


namespace capi {
    extern "C" {
    
    TitlecaseOptionsV1 ICU4XTitlecaseOptionsV1_default_options();
    
    
    } // extern "C"
}

inline TitlecaseOptionsV1 TitlecaseOptionsV1::default_options() {
  auto result = capi::ICU4XTitlecaseOptionsV1_default_options();
  return TitlecaseOptionsV1::FromFFI(result);
}


inline capi::TitlecaseOptionsV1 TitlecaseOptionsV1::AsFFI() const {
  return capi::TitlecaseOptionsV1 {
    .leading_adjustment = leading_adjustment.AsFFI(),
    .trailing_case = trailing_case.AsFFI(),
  };
}

inline TitlecaseOptionsV1 TitlecaseOptionsV1::FromFFI(capi::TitlecaseOptionsV1 c_struct) {
  return TitlecaseOptionsV1 {
    .leading_adjustment = LeadingAdjustment::FromFFI(c_struct.leading_adjustment),
    .trailing_case = TrailingCase::FromFFI(c_struct.trailing_case),
  };
}


#endif // TitlecaseOptionsV1_HPP
