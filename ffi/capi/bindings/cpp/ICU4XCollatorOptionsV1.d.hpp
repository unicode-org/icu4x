#ifndef ICU4XCollatorOptionsV1_D_HPP
#define ICU4XCollatorOptionsV1_D_HPP

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
#include "ICU4XCollatorOptionsV1.d.h"
#include "ICU4XCollatorStrength.d.hpp"

class ICU4XCollatorAlternateHandling;
class ICU4XCollatorBackwardSecondLevel;
class ICU4XCollatorCaseFirst;
class ICU4XCollatorCaseLevel;
class ICU4XCollatorMaxVariable;
class ICU4XCollatorNumeric;
class ICU4XCollatorStrength;


struct ICU4XCollatorOptionsV1 {
  ICU4XCollatorStrength strength;
  ICU4XCollatorAlternateHandling alternate_handling;
  ICU4XCollatorCaseFirst case_first;
  ICU4XCollatorMaxVariable max_variable;
  ICU4XCollatorCaseLevel case_level;
  ICU4XCollatorNumeric numeric;
  ICU4XCollatorBackwardSecondLevel backward_second_level;

  inline capi::ICU4XCollatorOptionsV1 AsFFI() const;
  inline static ICU4XCollatorOptionsV1 FromFFI(capi::ICU4XCollatorOptionsV1 c_struct);
};


#endif // ICU4XCollatorOptionsV1_D_HPP
