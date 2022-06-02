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

struct ICU4XCreatePluralOperandsResult;

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
  static ICU4XCreatePluralOperandsResult create(const std::string_view s);
};

#include "ICU4XCreatePluralOperandsResult.hpp"

inline ICU4XCreatePluralOperandsResult ICU4XPluralOperands::create(const std::string_view s) {
  capi::ICU4XCreatePluralOperandsResult diplomat_raw_struct_out_value = capi::ICU4XPluralOperands_create(s.data(), s.size());
  capi::ICU4XPluralOperands diplomat_raw_struct_out_value_operands = diplomat_raw_struct_out_value.operands;
  return ICU4XCreatePluralOperandsResult{ .operands = std::move(ICU4XPluralOperands{ .i = std::move(diplomat_raw_struct_out_value_operands.i), .v = std::move(diplomat_raw_struct_out_value_operands.v), .w = std::move(diplomat_raw_struct_out_value_operands.w), .f = std::move(diplomat_raw_struct_out_value_operands.f), .t = std::move(diplomat_raw_struct_out_value_operands.t), .c = std::move(diplomat_raw_struct_out_value_operands.c) }), .success = std::move(diplomat_raw_struct_out_value.success) };
}
#endif
