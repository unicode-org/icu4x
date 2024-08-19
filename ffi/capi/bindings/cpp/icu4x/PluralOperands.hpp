#ifndef icu4x_PluralOperands_HPP
#define icu4x_PluralOperands_HPP

#include "PluralOperands.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"
#include "FixedDecimal.hpp"
#include "FixedDecimalParseError.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_PluralOperands_from_string_mv1_result {union {icu4x::capi::PluralOperands* ok; icu4x::capi::FixedDecimalParseError err;}; bool is_ok;} icu4x_PluralOperands_from_string_mv1_result;
    icu4x_PluralOperands_from_string_mv1_result icu4x_PluralOperands_from_string_mv1(const char* s_data, size_t s_len);
    
    icu4x::capi::PluralOperands* icu4x_PluralOperands_from_fixed_decimal_mv1(const icu4x::capi::FixedDecimal* x);
    
    
    void icu4x_PluralOperands_destroy_mv1(PluralOperands* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<icu4x::PluralOperands>, icu4x::FixedDecimalParseError> icu4x::PluralOperands::from_string(std::string_view s) {
  auto result = icu4x::capi::icu4x_PluralOperands_from_string_mv1(s.data(),
    s.size());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::PluralOperands>, icu4x::FixedDecimalParseError>(diplomat::Ok<std::unique_ptr<icu4x::PluralOperands>>(std::unique_ptr<icu4x::PluralOperands>(icu4x::PluralOperands::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::PluralOperands>, icu4x::FixedDecimalParseError>(diplomat::Err<icu4x::FixedDecimalParseError>(icu4x::FixedDecimalParseError::FromFFI(result.err)));
}

inline std::unique_ptr<icu4x::PluralOperands> icu4x::PluralOperands::from_fixed_decimal(const icu4x::FixedDecimal& x) {
  auto result = icu4x::capi::icu4x_PluralOperands_from_fixed_decimal_mv1(x.AsFFI());
  return std::unique_ptr<icu4x::PluralOperands>(icu4x::PluralOperands::FromFFI(result));
}

inline const icu4x::capi::PluralOperands* icu4x::PluralOperands::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::PluralOperands*>(this);
}

inline icu4x::capi::PluralOperands* icu4x::PluralOperands::AsFFI() {
  return reinterpret_cast<icu4x::capi::PluralOperands*>(this);
}

inline const icu4x::PluralOperands* icu4x::PluralOperands::FromFFI(const icu4x::capi::PluralOperands* ptr) {
  return reinterpret_cast<const icu4x::PluralOperands*>(ptr);
}

inline icu4x::PluralOperands* icu4x::PluralOperands::FromFFI(icu4x::capi::PluralOperands* ptr) {
  return reinterpret_cast<icu4x::PluralOperands*>(ptr);
}

inline void icu4x::PluralOperands::operator delete(void* ptr) {
  icu4x::capi::icu4x_PluralOperands_destroy_mv1(reinterpret_cast<icu4x::capi::PluralOperands*>(ptr));
}


#endif // icu4x_PluralOperands_HPP
