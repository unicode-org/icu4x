#ifndef ICU4XCodePointSetDataResult_HPP
#define ICU4XCodePointSetDataResult_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XCodePointSetDataResult.h"
}

class ICU4XCodePointSetData;

/**
 * A destruction policy for using ICU4XCodePointSetDataResult with std::unique_ptr.
 */
struct ICU4XCodePointSetDataResultDeleter {
  void operator()(capi::ICU4XCodePointSetDataResult* l) const noexcept {
    capi::ICU4XCodePointSetDataResult_destroy(l);
  }
};
struct ICU4XCodePointSetDataResult {
 public:

  /**
   * The [`ICU4XCodePointSetData`], if creation was successful.
   */
  std::optional<ICU4XCodePointSetData> data;

  /**
   * Whether creating the [`ICU4XCodePointSetData`] was successful.
   */
  bool success;
};


#endif
