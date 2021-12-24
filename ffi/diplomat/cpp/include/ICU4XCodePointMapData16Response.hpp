#ifndef ICU4XCodePointMapData16Response_HPP
#define ICU4XCodePointMapData16Response_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XCodePointMapData16Response.h"
}

class ICU4XCodePointMapData16;

/**
 * A destruction policy for using ICU4XCodePointMapData16Response with std::unique_ptr.
 */
struct ICU4XCodePointMapData16ResponseDeleter {
  void operator()(capi::ICU4XCodePointMapData16Response* l) const noexcept {
    capi::ICU4XCodePointMapData16Response_destroy(l);
  }
};
struct ICU4XCodePointMapData16Response {
 public:

  /**
   * The [`ICU4XCodePointMapData16`], if creation was successful.
   */
  std::optional<ICU4XCodePointMapData16> data;

  /**
   * Whether creating the [`ICU4XCodePointMapData16`] was successful.
   */
  bool success;
};


#endif
