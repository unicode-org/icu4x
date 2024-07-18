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
    
    typedef struct icu4x_PluralOperands_create_from_string_mv1_result {union {diplomat::capi::PluralOperands* ok; diplomat::capi::FixedDecimalParseError err;}; bool is_ok;} icu4x_PluralOperands_create_from_string_mv1_result;
    icu4x_PluralOperands_create_from_string_mv1_result icu4x_PluralOperands_create_from_string_mv1(const char* s_data, size_t s_len);
    
    diplomat::capi::PluralOperands* icu4x_PluralOperands_create_from_fixed_decimal_mv1(const diplomat::capi::FixedDecimal* x);
    
    
    void icu4x_PluralOperands_destroy_mv1(PluralOperands* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<PluralOperands>, FixedDecimalParseError> PluralOperands::create_from_string(std::string_view s) {
  auto result = diplomat::capi::icu4x_PluralOperands_create_from_string_mv1(s.data(),
    s.size());
  return result.is_ok ? diplomat::result<std::unique_ptr<PluralOperands>, FixedDecimalParseError>(diplomat::Ok<std::unique_ptr<PluralOperands>>(std::unique_ptr<PluralOperands>(PluralOperands::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<PluralOperands>, FixedDecimalParseError>(diplomat::Err<FixedDecimalParseError>(FixedDecimalParseError::FromFFI(result.err)));
}

inline std::unique_ptr<PluralOperands> PluralOperands::create_from_fixed_decimal(const FixedDecimal& x) {
  auto result = diplomat::capi::icu4x_PluralOperands_create_from_fixed_decimal_mv1(x.AsFFI());
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
  diplomat::capi::icu4x_PluralOperands_destroy_mv1(reinterpret_cast<diplomat::capi::PluralOperands*>(ptr));
}


#endif // PluralOperands_HPP
