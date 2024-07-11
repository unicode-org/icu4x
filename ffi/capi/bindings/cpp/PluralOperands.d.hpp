#ifndef PluralOperands_D_HPP
#define PluralOperands_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "FixedDecimalParseError.d.hpp"

class FixedDecimal;
class FixedDecimalParseError;


namespace capi {
    typedef struct PluralOperands PluralOperands;
}

class PluralOperands {
public:

  inline static diplomat::result<std::unique_ptr<PluralOperands>, FixedDecimalParseError> create_from_string(std::string_view s);

  inline static std::unique_ptr<PluralOperands> create_from_fixed_decimal(const FixedDecimal& x);

  inline const capi::PluralOperands* AsFFI() const;
  inline capi::PluralOperands* AsFFI();
  inline static const PluralOperands* FromFFI(const capi::PluralOperands* ptr);
  inline static PluralOperands* FromFFI(capi::PluralOperands* ptr);
  inline static void operator delete(void* ptr);
private:
  PluralOperands() = delete;
  PluralOperands(const PluralOperands&) = delete;
  PluralOperands(PluralOperands&&) noexcept = delete;
  PluralOperands operator=(const PluralOperands&) = delete;
  PluralOperands operator=(PluralOperands&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // PluralOperands_D_HPP
