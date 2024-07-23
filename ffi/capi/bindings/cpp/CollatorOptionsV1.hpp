#ifndef CollatorOptionsV1_HPP
#define CollatorOptionsV1_HPP

#include "CollatorOptionsV1.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "CollatorAlternateHandling.hpp"
#include "CollatorBackwardSecondLevel.hpp"
#include "CollatorCaseFirst.hpp"
#include "CollatorCaseLevel.hpp"
#include "CollatorMaxVariable.hpp"
#include "CollatorNumeric.hpp"
#include "CollatorStrength.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace


inline diplomat::capi::CollatorOptionsV1 CollatorOptionsV1::AsFFI() const {
  return diplomat::capi::CollatorOptionsV1 {
    /* .strength = */ strength.AsFFI(),
    /* .alternate_handling = */ alternate_handling.AsFFI(),
    /* .case_first = */ case_first.AsFFI(),
    /* .max_variable = */ max_variable.AsFFI(),
    /* .case_level = */ case_level.AsFFI(),
    /* .numeric = */ numeric.AsFFI(),
    /* .backward_second_level = */ backward_second_level.AsFFI(),
  };
}

inline CollatorOptionsV1 CollatorOptionsV1::FromFFI(diplomat::capi::CollatorOptionsV1 c_struct) {
  return CollatorOptionsV1 {
    /* .strength = */ CollatorStrength::FromFFI(c_struct.strength),
    /* .alternate_handling = */ CollatorAlternateHandling::FromFFI(c_struct.alternate_handling),
    /* .case_first = */ CollatorCaseFirst::FromFFI(c_struct.case_first),
    /* .max_variable = */ CollatorMaxVariable::FromFFI(c_struct.max_variable),
    /* .case_level = */ CollatorCaseLevel::FromFFI(c_struct.case_level),
    /* .numeric = */ CollatorNumeric::FromFFI(c_struct.numeric),
    /* .backward_second_level = */ CollatorBackwardSecondLevel::FromFFI(c_struct.backward_second_level),
  };
}


#endif // CollatorOptionsV1_HPP
