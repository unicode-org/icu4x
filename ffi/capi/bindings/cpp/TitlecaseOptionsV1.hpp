#ifndef TitlecaseOptionsV1_HPP
#define TitlecaseOptionsV1_HPP

#include "TitlecaseOptionsV1.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "LeadingAdjustment.hpp"
#include "TrailingCase.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    diplomat::capi::TitlecaseOptionsV1 icu4x_TitlecaseOptionsV1_default_mv1(void);
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline TitlecaseOptionsV1 TitlecaseOptionsV1::default_options() {
  auto result = diplomat::capi::icu4x_TitlecaseOptionsV1_default_mv1();
  return TitlecaseOptionsV1::FromFFI(result);
}


inline diplomat::capi::TitlecaseOptionsV1 TitlecaseOptionsV1::AsFFI() const {
  return diplomat::capi::TitlecaseOptionsV1 {
    /* .leading_adjustment = */ leading_adjustment.AsFFI(),
    /* .trailing_case = */ trailing_case.AsFFI(),
  };
}

inline TitlecaseOptionsV1 TitlecaseOptionsV1::FromFFI(diplomat::capi::TitlecaseOptionsV1 c_struct) {
  return TitlecaseOptionsV1 {
    /* .leading_adjustment = */ LeadingAdjustment::FromFFI(c_struct.leading_adjustment),
    /* .trailing_case = */ TrailingCase::FromFFI(c_struct.trailing_case),
  };
}


#endif // TitlecaseOptionsV1_HPP
