#ifndef ICU4XCollatorResolvedOptionsV1_D_HPP
#define ICU4XCollatorResolvedOptionsV1_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XCollatorAlternateHandling.d.hpp"
#include "ICU4XCollatorBackwardSecondLevel.d.hpp"
#include "ICU4XCollatorCaseFirst.d.hpp"
#include "ICU4XCollatorCaseLevel.d.hpp"
#include "ICU4XCollatorMaxVariable.d.hpp"
#include "ICU4XCollatorNumeric.d.hpp"
#include "ICU4XCollatorStrength.d.hpp"

class ICU4XCollatorAlternateHandling;
class ICU4XCollatorBackwardSecondLevel;
class ICU4XCollatorCaseFirst;
class ICU4XCollatorCaseLevel;
class ICU4XCollatorMaxVariable;
class ICU4XCollatorNumeric;
class ICU4XCollatorStrength;


namespace capi {
    typedef struct ICU4XCollatorResolvedOptionsV1 {
      ICU4XCollatorStrength strength;
      ICU4XCollatorAlternateHandling alternate_handling;
      ICU4XCollatorCaseFirst case_first;
      ICU4XCollatorMaxVariable max_variable;
      ICU4XCollatorCaseLevel case_level;
      ICU4XCollatorNumeric numeric;
      ICU4XCollatorBackwardSecondLevel backward_second_level;
    } ICU4XCollatorResolvedOptionsV1;
}

struct ICU4XCollatorResolvedOptionsV1 {
  ICU4XCollatorStrength strength;
  ICU4XCollatorAlternateHandling alternate_handling;
  ICU4XCollatorCaseFirst case_first;
  ICU4XCollatorMaxVariable max_variable;
  ICU4XCollatorCaseLevel case_level;
  ICU4XCollatorNumeric numeric;
  ICU4XCollatorBackwardSecondLevel backward_second_level;

  inline capi::ICU4XCollatorResolvedOptionsV1 AsFFI() const;
  inline static ICU4XCollatorResolvedOptionsV1 FromFFI(capi::ICU4XCollatorResolvedOptionsV1 c_struct);
};


#endif // ICU4XCollatorResolvedOptionsV1_D_HPP
