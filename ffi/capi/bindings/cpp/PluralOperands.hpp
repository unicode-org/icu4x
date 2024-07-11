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


namespace capi {
    extern "C" {
    
    typedef struct ICU4XPluralOperands_create_from_string_result {union {PluralOperands* ok; FixedDecimalParseError err;}; bool is_ok;} ICU4XPluralOperands_create_from_string_result;
    ICU4XPluralOperands_create_from_string_result ICU4XPluralOperands_create_from_string(const char* s_data, size_t s_len);
    
    PluralOperands* ICU4XPluralOperands_create_from_fixed_decimal(const FixedDecimal* x);
    
    
    void ICU4XPluralOperands_destroy(PluralOperands* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<PluralOperands>, FixedDecimalParseError> PluralOperands::create_from_string(std::string_view s) {
  auto result = capi::ICU4XPluralOperands_create_from_string(s.data(),
    s.size());
  return result.is_ok ? diplomat::result<std::unique_ptr<PluralOperands>, FixedDecimalParseError>(diplomat::Ok<std::unique_ptr<PluralOperands>>(std::unique_ptr<PluralOperands>(PluralOperands::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<PluralOperands>, FixedDecimalParseError>(diplomat::Err<FixedDecimalParseError>(FixedDecimalParseError::FromFFI(result.err)));
}

inline std::unique_ptr<PluralOperands> PluralOperands::create_from_fixed_decimal(const FixedDecimal& x) {
  auto result = capi::ICU4XPluralOperands_create_from_fixed_decimal(x.AsFFI());
  return std::unique_ptr<PluralOperands>(PluralOperands::FromFFI(result));
}

inline const capi::PluralOperands* PluralOperands::AsFFI() const {
  return reinterpret_cast<const capi::PluralOperands*>(this);
}

inline capi::PluralOperands* PluralOperands::AsFFI() {
  return reinterpret_cast<capi::PluralOperands*>(this);
}

inline const PluralOperands* PluralOperands::FromFFI(const capi::PluralOperands* ptr) {
  return reinterpret_cast<const PluralOperands*>(ptr);
}

inline PluralOperands* PluralOperands::FromFFI(capi::PluralOperands* ptr) {
  return reinterpret_cast<PluralOperands*>(ptr);
}

inline void PluralOperands::operator delete(void* ptr) {
  capi::ICU4XPluralOperands_destroy(reinterpret_cast<capi::PluralOperands*>(ptr));
}


#endif // PluralOperands_HPP
