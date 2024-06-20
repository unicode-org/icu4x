#ifndef ICU4XPluralOperands_HPP
#define ICU4XPluralOperands_HPP

#include "ICU4XPluralOperands.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XFixedDecimal.hpp"
#include "ICU4XFixedDecimalParseError.hpp"


namespace capi {
    extern "C" {
    
    struct ICU4XPluralOperands_create_from_string_result {union {ICU4XPluralOperands* ok; ICU4XFixedDecimalParseError err;}; bool is_ok;};
    struct ICU4XPluralOperands_create_from_string_result ICU4XPluralOperands_create_from_string(const char* s_data, size_t s_len);
    
    ICU4XPluralOperands* ICU4XPluralOperands_create_from_fixed_decimal(const ICU4XFixedDecimal* x);
    
    
    void ICU4XPluralOperands_destroy(ICU4XPluralOperands* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<ICU4XPluralOperands>, ICU4XFixedDecimalParseError> ICU4XPluralOperands::create_from_string(std::string_view s) {
  auto result = capi::ICU4XPluralOperands_create_from_string(s.data(),
    s.size());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XPluralOperands>, ICU4XFixedDecimalParseError>(diplomat::Ok<std::unique_ptr<ICU4XPluralOperands>>(std::unique_ptr<ICU4XPluralOperands>(ICU4XPluralOperands::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XPluralOperands>, ICU4XFixedDecimalParseError>(diplomat::Err<ICU4XFixedDecimalParseError>(ICU4XFixedDecimalParseError::FromFFI(result.err)));
}

inline std::unique_ptr<ICU4XPluralOperands> ICU4XPluralOperands::create_from_fixed_decimal(const ICU4XFixedDecimal& x) {
  auto result = capi::ICU4XPluralOperands_create_from_fixed_decimal(x.AsFFI());
  return std::unique_ptr<ICU4XPluralOperands>(ICU4XPluralOperands::FromFFI(result));
}

inline const capi::ICU4XPluralOperands* ICU4XPluralOperands::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XPluralOperands*>(this);
}

inline capi::ICU4XPluralOperands* ICU4XPluralOperands::AsFFI() {
  return reinterpret_cast<capi::ICU4XPluralOperands*>(this);
}

inline const ICU4XPluralOperands* ICU4XPluralOperands::FromFFI(const capi::ICU4XPluralOperands* ptr) {
  return reinterpret_cast<const ICU4XPluralOperands*>(ptr);
}

inline ICU4XPluralOperands* ICU4XPluralOperands::FromFFI(capi::ICU4XPluralOperands* ptr) {
  return reinterpret_cast<ICU4XPluralOperands*>(ptr);
}

inline void ICU4XPluralOperands::operator delete(void* ptr) {
  capi::ICU4XPluralOperands_destroy(reinterpret_cast<capi::ICU4XPluralOperands*>(ptr));
}


#endif // ICU4XPluralOperands_HPP
