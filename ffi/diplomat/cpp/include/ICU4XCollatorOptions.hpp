#ifndef ICU4XCollatorOptions_HPP
#define ICU4XCollatorOptions_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XCollatorOptions.h"

#include "ICU4XCollatorStrength.hpp"
#include "ICU4XCollatorAlternateHandling.hpp"
#include "ICU4XCollatorCaseFirst.hpp"
#include "ICU4XCollatorMaxVariable.hpp"
#include "ICU4XCollatorCaseLevel.hpp"
#include "ICU4XCollatorNumeric.hpp"
#include "ICU4XCollatorBackwardSecondLevel.hpp"

/**
 * A destruction policy for using ICU4XCollatorOptions with std::unique_ptr.
 */
struct ICU4XCollatorOptionsDeleter {
  void operator()(capi::ICU4XCollatorOptions* l) const noexcept {
    capi::ICU4XCollatorOptions_destroy(l);
  }
};

/**
 * 
 * 
 * See the [Rust documentation for `CollatorOptions`](https://unicode-org.github.io/icu4x-docs/doc/icu/collator/struct.CollatorOptions.html) for more information.
 */
struct ICU4XCollatorOptions {
 public:
  ICU4XCollatorStrength strength;
  ICU4XCollatorAlternateHandling alternate_handling;
  ICU4XCollatorCaseFirst case_first;
  ICU4XCollatorMaxVariable max_variable;
  ICU4XCollatorCaseLevel case_level;
  ICU4XCollatorNumeric numeric;
  ICU4XCollatorBackwardSecondLevel backward_second_level;
};


#endif
