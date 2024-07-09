#ifndef ICU4XCollatorResolvedOptionsV1_HPP
#define ICU4XCollatorResolvedOptionsV1_HPP

#include "ICU4XCollatorResolvedOptionsV1.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XCollatorAlternateHandling.hpp"
#include "ICU4XCollatorBackwardSecondLevel.hpp"
#include "ICU4XCollatorCaseFirst.hpp"
#include "ICU4XCollatorCaseLevel.hpp"
#include "ICU4XCollatorMaxVariable.hpp"
#include "ICU4XCollatorNumeric.hpp"
#include "ICU4XCollatorStrength.hpp"


namespace capi {
    extern "C" {
    
    
    } // extern "C"
}


inline capi::ICU4XCollatorResolvedOptionsV1 ICU4XCollatorResolvedOptionsV1::AsFFI() const {
  return capi::ICU4XCollatorResolvedOptionsV1 {
    .strength = strength.AsFFI(),
    .alternate_handling = alternate_handling.AsFFI(),
    .case_first = case_first.AsFFI(),
    .max_variable = max_variable.AsFFI(),
    .case_level = case_level.AsFFI(),
    .numeric = numeric.AsFFI(),
    .backward_second_level = backward_second_level.AsFFI(),
  };
}

inline ICU4XCollatorResolvedOptionsV1 ICU4XCollatorResolvedOptionsV1::FromFFI(capi::ICU4XCollatorResolvedOptionsV1 c_struct) {
  return ICU4XCollatorResolvedOptionsV1 {
    .strength = ICU4XCollatorStrength::FromFFI(c_struct.strength),
    .alternate_handling = ICU4XCollatorAlternateHandling::FromFFI(c_struct.alternate_handling),
    .case_first = ICU4XCollatorCaseFirst::FromFFI(c_struct.case_first),
    .max_variable = ICU4XCollatorMaxVariable::FromFFI(c_struct.max_variable),
    .case_level = ICU4XCollatorCaseLevel::FromFFI(c_struct.case_level),
    .numeric = ICU4XCollatorNumeric::FromFFI(c_struct.numeric),
    .backward_second_level = ICU4XCollatorBackwardSecondLevel::FromFFI(c_struct.backward_second_level),
  };
}


#endif // ICU4XCollatorResolvedOptionsV1_HPP
