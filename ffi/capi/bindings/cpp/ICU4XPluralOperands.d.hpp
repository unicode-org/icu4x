#ifndef ICU4XPluralOperands_D_HPP
#define ICU4XPluralOperands_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XPluralOperands.d.h"
#include "ICU4XPluralsParseError.d.hpp"

class ICU4XFixedDecimal;
class ICU4XPluralsParseError;


class ICU4XPluralOperands {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XPluralOperands>, ICU4XPluralsParseError> create_from_string(std::string_view s);

  inline static std::unique_ptr<ICU4XPluralOperands> create_from_fixed_decimal(const ICU4XFixedDecimal& x);

  inline const capi::ICU4XPluralOperands* AsFFI() const;
  inline capi::ICU4XPluralOperands* AsFFI();
  inline static const ICU4XPluralOperands* FromFFI(const capi::ICU4XPluralOperands* ptr);
  inline static ICU4XPluralOperands* FromFFI(capi::ICU4XPluralOperands* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XPluralOperands() = delete;
  ICU4XPluralOperands(const ICU4XPluralOperands&) = delete;
  ICU4XPluralOperands(ICU4XPluralOperands&&) noexcept = delete;
  ICU4XPluralOperands operator=(const ICU4XPluralOperands&) = delete;
  ICU4XPluralOperands operator=(ICU4XPluralOperands&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XPluralOperands_D_HPP
