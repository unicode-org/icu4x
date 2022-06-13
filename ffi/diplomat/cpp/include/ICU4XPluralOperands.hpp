#ifndef ICU4XPluralOperands_HPP
#define ICU4XPluralOperands_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XPluralOperands.h"
}

struct ICU4XPluralOperands;
#include "ICU4XError.hpp"

/**
 * A destruction policy for using ICU4XPluralOperands with std::unique_ptr.
 */
struct ICU4XPluralOperandsDeleter {
  void operator()(capi::ICU4XPluralOperands* l) const noexcept {
    capi::ICU4XPluralOperands_destroy(l);
  }
};

/**
 * FFI version of `PluralOperands`.
 * 
 * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralOperands.html) for more information.
 */
struct ICU4XPluralOperands {
 public:
  uint64_t i;
  size_t v;
  size_t w;
  uint64_t f;
  uint64_t t;
  size_t c;

  /**
   * FFI version of `PluralOperands::from_str()`.
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralOperands.html#method.from_str) for more information.
   */
  static diplomat::result<ICU4XPluralOperands, ICU4XError> create(const std::string_view s);
};


inline diplomat::result<ICU4XPluralOperands, ICU4XError> ICU4XPluralOperands::create(const std::string_view s) {
  auto diplomat_result_raw_out_value = capi::ICU4XPluralOperands_create(s.data(), s.size());
  diplomat::result<ICU4XPluralOperands, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
  capi::ICU4XPluralOperands diplomat_raw_struct_out_value = diplomat_result_raw_out_value.ok;
    diplomat_result_out_value = diplomat::Ok(ICU4XPluralOperands{ .i = std::move(diplomat_raw_struct_out_value.i), .v = std::move(diplomat_raw_struct_out_value.v), .w = std::move(diplomat_raw_struct_out_value.w), .f = std::move(diplomat_raw_struct_out_value.f), .t = std::move(diplomat_raw_struct_out_value.t), .c = std::move(diplomat_raw_struct_out_value.c) });
  } else {
    diplomat_result_out_value = diplomat::Err(static_cast<ICU4XError>(diplomat_result_raw_out_value.err));
  }
  return diplomat_result_out_value;
}
#endif
