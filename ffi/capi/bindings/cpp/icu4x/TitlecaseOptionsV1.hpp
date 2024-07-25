#ifndef icu4x_TitlecaseOptionsV1_HPP
#define icu4x_TitlecaseOptionsV1_HPP

#include "TitlecaseOptionsV1.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"
#include "LeadingAdjustment.hpp"
#include "TrailingCase.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    icu4x::capi::TitlecaseOptionsV1 icu4x_TitlecaseOptionsV1_default_mv1(void);
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline icu4x::TitlecaseOptionsV1 icu4x::TitlecaseOptionsV1::default_options() {
  auto result = icu4x::capi::icu4x_TitlecaseOptionsV1_default_mv1();
  return icu4x::TitlecaseOptionsV1::FromFFI(result);
}


inline icu4x::capi::TitlecaseOptionsV1 icu4x::TitlecaseOptionsV1::AsFFI() const {
  return icu4x::capi::TitlecaseOptionsV1 {
    /* .leading_adjustment = */ leading_adjustment.AsFFI(),
    /* .trailing_case = */ trailing_case.AsFFI(),
  };
}

inline icu4x::TitlecaseOptionsV1 icu4x::TitlecaseOptionsV1::FromFFI(icu4x::capi::TitlecaseOptionsV1 c_struct) {
  return icu4x::TitlecaseOptionsV1 {
    /* .leading_adjustment = */ icu4x::LeadingAdjustment::FromFFI(c_struct.leading_adjustment),
    /* .trailing_case = */ icu4x::TrailingCase::FromFFI(c_struct.trailing_case),
  };
}


#endif // icu4x_TitlecaseOptionsV1_HPP
