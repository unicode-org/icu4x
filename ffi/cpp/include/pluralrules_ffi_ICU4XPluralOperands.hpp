#ifndef pluralrules_ffi_ICU4XPluralOperands_HPP
#define pluralrules_ffi_ICU4XPluralOperands_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <optional>
#include <variant>
#include "diplomat_runtime.hpp"

namespace capi {
#include "pluralrules_ffi_ICU4XPluralOperands.h"
}

struct ICU4XCreatePluralOperandsResult;

struct ICU4XPluralOperandsDeleter {
  void operator()(capi::ICU4XPluralOperands* l) const noexcept {
    capi::ICU4XPluralOperands_destroy(l);
  }
};
struct ICU4XPluralOperands {
 public:
  uint64_t i;
  size_t v;
  size_t w;
  uint64_t f;
  uint64_t t;
  size_t c;
  static ICU4XCreatePluralOperandsResult create(const std::string_view s);
};

#include "pluralrules_ffi_ICU4XCreatePluralOperandsResult.hpp"

ICU4XCreatePluralOperandsResult ICU4XPluralOperands::create(const std::string_view s) {
  capi::ICU4XCreatePluralOperandsResult diplomat_raw_struct_out_value = capi::ICU4XPluralOperands_create(s.data(), s.length());
  capi::ICU4XPluralOperands diplomat_raw_struct_out_value_operands = diplomat_raw_struct_out_value.operands;
  return ICU4XCreatePluralOperandsResult{ .operands = std::move(ICU4XPluralOperands{ .i = std::move(diplomat_raw_struct_out_value_operands.i), .v = std::move(diplomat_raw_struct_out_value_operands.v), .w = std::move(diplomat_raw_struct_out_value_operands.w), .f = std::move(diplomat_raw_struct_out_value_operands.f), .t = std::move(diplomat_raw_struct_out_value_operands.t), .c = std::move(diplomat_raw_struct_out_value_operands.c) }), .success = std::move(diplomat_raw_struct_out_value.success) };
}
#endif
