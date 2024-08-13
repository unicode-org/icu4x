#ifndef icu4x_CollatorOptionsV1_D_HPP
#define icu4x_CollatorOptionsV1_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"
#include "CollatorAlternateHandling.d.hpp"
#include "CollatorBackwardSecondLevel.d.hpp"
#include "CollatorCaseFirst.d.hpp"
#include "CollatorCaseLevel.d.hpp"
#include "CollatorMaxVariable.d.hpp"
#include "CollatorNumeric.d.hpp"
#include "CollatorStrength.d.hpp"

namespace icu4x {
class CollatorAlternateHandling;
class CollatorBackwardSecondLevel;
class CollatorCaseFirst;
class CollatorCaseLevel;
class CollatorMaxVariable;
class CollatorNumeric;
class CollatorStrength;
}


namespace icu4x {
namespace capi {
    struct CollatorOptionsV1 {
      icu4x::capi::CollatorStrength strength;
      icu4x::capi::CollatorAlternateHandling alternate_handling;
      icu4x::capi::CollatorCaseFirst case_first;
      icu4x::capi::CollatorMaxVariable max_variable;
      icu4x::capi::CollatorCaseLevel case_level;
      icu4x::capi::CollatorNumeric numeric;
      icu4x::capi::CollatorBackwardSecondLevel backward_second_level;
    };
} // namespace capi
} // namespace


namespace icu4x {
struct CollatorOptionsV1 {
  icu4x::CollatorStrength strength;
  icu4x::CollatorAlternateHandling alternate_handling;
  icu4x::CollatorCaseFirst case_first;
  icu4x::CollatorMaxVariable max_variable;
  icu4x::CollatorCaseLevel case_level;
  icu4x::CollatorNumeric numeric;
  icu4x::CollatorBackwardSecondLevel backward_second_level;

  inline icu4x::capi::CollatorOptionsV1 AsFFI() const;
  inline static icu4x::CollatorOptionsV1 FromFFI(icu4x::capi::CollatorOptionsV1 c_struct);
};

} // namespace
#endif // icu4x_CollatorOptionsV1_D_HPP
