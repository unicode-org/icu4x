#ifndef CollatorOptionsV1_D_HPP
#define CollatorOptionsV1_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "CollatorAlternateHandling.d.hpp"
#include "CollatorBackwardSecondLevel.d.hpp"
#include "CollatorCaseFirst.d.hpp"
#include "CollatorCaseLevel.d.hpp"
#include "CollatorMaxVariable.d.hpp"
#include "CollatorNumeric.d.hpp"
#include "CollatorStrength.d.hpp"

class CollatorAlternateHandling;
class CollatorBackwardSecondLevel;
class CollatorCaseFirst;
class CollatorCaseLevel;
class CollatorMaxVariable;
class CollatorNumeric;
class CollatorStrength;


namespace capi {
    typedef struct CollatorOptionsV1 {
      CollatorStrength strength;
      CollatorAlternateHandling alternate_handling;
      CollatorCaseFirst case_first;
      CollatorMaxVariable max_variable;
      CollatorCaseLevel case_level;
      CollatorNumeric numeric;
      CollatorBackwardSecondLevel backward_second_level;
    } CollatorOptionsV1;
}

struct CollatorOptionsV1 {
  CollatorStrength strength;
  CollatorAlternateHandling alternate_handling;
  CollatorCaseFirst case_first;
  CollatorMaxVariable max_variable;
  CollatorCaseLevel case_level;
  CollatorNumeric numeric;
  CollatorBackwardSecondLevel backward_second_level;

  inline capi::CollatorOptionsV1 AsFFI() const;
  inline static CollatorOptionsV1 FromFFI(capi::CollatorOptionsV1 c_struct);
};


#endif // CollatorOptionsV1_D_HPP
