#ifndef ICU4XIsoTimeZoneOptions_HPP
#define ICU4XIsoTimeZoneOptions_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XIsoTimeZoneOptions.h"

#include "ICU4XIsoTimeZoneFormat.hpp"
#include "ICU4XIsoTimeZoneMinuteDisplay.hpp"
#include "ICU4XIsoTimeZoneSecondDisplay.hpp"

/**
 * A destruction policy for using ICU4XIsoTimeZoneOptions with std::unique_ptr.
 */
struct ICU4XIsoTimeZoneOptionsDeleter {
  void operator()(capi::ICU4XIsoTimeZoneOptions* l) const noexcept {
    capi::ICU4XIsoTimeZoneOptions_destroy(l);
  }
};
struct ICU4XIsoTimeZoneOptions {
 public:
  ICU4XIsoTimeZoneFormat format;
  ICU4XIsoTimeZoneMinuteDisplay minutes;
  ICU4XIsoTimeZoneSecondDisplay seconds;
};


#endif
