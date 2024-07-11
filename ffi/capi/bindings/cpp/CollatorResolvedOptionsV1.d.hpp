#ifndef CollatorResolvedOptionsV1_D_HPP
#define CollatorResolvedOptionsV1_D_HPP

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
    typedef struct CollatorResolvedOptionsV1 {
      CollatorStrength strength;
      CollatorAlternateHandling alternate_handling;
      CollatorCaseFirst case_first;
      CollatorMaxVariable max_variable;
      CollatorCaseLevel case_level;
      CollatorNumeric numeric;
      CollatorBackwardSecondLevel backward_second_level;
    } CollatorResolvedOptionsV1;
}

struct CollatorResolvedOptionsV1 {
  CollatorStrength strength;
  CollatorAlternateHandling alternate_handling;
  CollatorCaseFirst case_first;
  CollatorMaxVariable max_variable;
  CollatorCaseLevel case_level;
  CollatorNumeric numeric;
  CollatorBackwardSecondLevel backward_second_level;

  inline capi::CollatorResolvedOptionsV1 AsFFI() const;
  inline static CollatorResolvedOptionsV1 FromFFI(capi::CollatorResolvedOptionsV1 c_struct);
};


#endif // CollatorResolvedOptionsV1_D_HPP
