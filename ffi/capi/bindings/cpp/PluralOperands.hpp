#ifndef PluralOperands_HPP
#define PluralOperands_HPP

#include "PluralOperands.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "FixedDecimal.hpp"
#include "FixedDecimalParseError.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct ICU4XPluralOperands_create_from_string_result {union {diplomat::capi::PluralOperands* ok; diplomat::capi::FixedDecimalParseError err;}; bool is_ok;} ICU4XPluralOperands_create_from_string_result;
    ICU4XPluralOperands_create_from_string_result ICU4XPluralOperands_create_from_string(const char* s_data, size_t s_len);
    
    diplomat::capi::PluralOperands* ICU4XPluralOperands_create_from_fixed_decimal(const diplomat::capi::FixedDecimal* x);
    
    
    void ICU4XPluralOperands_destroy(PluralOperands* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<PluralOperands>, FixedDecimalParseError> PluralOperands::create_from_string(std::string_view s) {
  auto result = diplomat::capi::ICU4XPluralOperands_create_from_string(s.data(),
    s.size());
  return result.is_ok ? diplomat::result<std::unique_ptr<PluralOperands>, FixedDecimalParseError>(diplomat::Ok<std::unique_ptr<PluralOperands>>(std::unique_ptr<PluralOperands>(PluralOperands::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<PluralOperands>, FixedDecimalParseError>(diplomat::Err<FixedDecimalParseError>(FixedDecimalParseError::FromFFI(result.err)));
}

inline std::unique_ptr<PluralOperands> PluralOperands::create_from_fixed_decimal(const FixedDecimal& x) {
  auto result = diplomat::capi::ICU4XPluralOperands_create_from_fixed_decimal(x.AsFFI());
  return std::unique_ptr<PluralOperands>(PluralOperands::FromFFI(result));
}

inline const diplomat::capi::PluralOperands* PluralOperands::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::PluralOperands*>(this);
}

inline diplomat::capi::PluralOperands* PluralOperands::AsFFI() {
  return reinterpret_cast<diplomat::capi::PluralOperands*>(this);
}

inline const PluralOperands* PluralOperands::FromFFI(const diplomat::capi::PluralOperands* ptr) {
  return reinterpret_cast<const PluralOperands*>(ptr);
}

inline PluralOperands* PluralOperands::FromFFI(diplomat::capi::PluralOperands* ptr) {
  return reinterpret_cast<PluralOperands*>(ptr);
}

inline void PluralOperands::operator delete(void* ptr) {
  diplomat::capi::ICU4XPluralOperands_destroy(reinterpret_cast<diplomat::capi::PluralOperands*>(ptr));
}


#endif // PluralOperands_HPP
