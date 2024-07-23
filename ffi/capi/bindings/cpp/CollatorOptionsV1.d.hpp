#ifndef CollatorOptionsV1_D_HPP
#define CollatorOptionsV1_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "CollatorAlternateHandling.d.hpp"
#include "CollatorBackwardSecondLevel.d.hpp"
#include "CollatorCaseFirst.d.hpp"
#include "CollatorCaseLevel.d.hpp"
#include "CollatorMaxVariable.d.hpp"
#include "CollatorNumeric.d.hpp"
#include "CollatorStrength.d.hpp"
#include "diplomat_runtime.hpp"

class CollatorAlternateHandling;
class CollatorBackwardSecondLevel;
class CollatorCaseFirst;
class CollatorCaseLevel;
class CollatorMaxVariable;
class CollatorNumeric;
class CollatorStrength;


namespace diplomat {
namespace capi {
    struct CollatorOptionsV1 {
      diplomat::capi::CollatorStrength strength;
      diplomat::capi::CollatorAlternateHandling alternate_handling;
      diplomat::capi::CollatorCaseFirst case_first;
      diplomat::capi::CollatorMaxVariable max_variable;
      diplomat::capi::CollatorCaseLevel case_level;
      diplomat::capi::CollatorNumeric numeric;
      diplomat::capi::CollatorBackwardSecondLevel backward_second_level;
    };
} // namespace capi
} // namespace


struct CollatorOptionsV1 {
  CollatorStrength strength;
  CollatorAlternateHandling alternate_handling;
  CollatorCaseFirst case_first;
  CollatorMaxVariable max_variable;
  CollatorCaseLevel case_level;
  CollatorNumeric numeric;
  CollatorBackwardSecondLevel backward_second_level;

  inline diplomat::capi::CollatorOptionsV1 AsFFI() const;
  inline static CollatorOptionsV1 FromFFI(diplomat::capi::CollatorOptionsV1 c_struct);
};


#endif // CollatorOptionsV1_D_HPP
