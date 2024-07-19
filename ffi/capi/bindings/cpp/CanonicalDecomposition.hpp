#ifndef CanonicalDecomposition_HPP
#define CanonicalDecomposition_HPP

#include "CanonicalDecomposition.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "Decomposed.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_CanonicalDecomposition_create_mv1_result {union {diplomat::capi::CanonicalDecomposition* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CanonicalDecomposition_create_mv1_result;
    icu4x_CanonicalDecomposition_create_mv1_result icu4x_CanonicalDecomposition_create_mv1(const diplomat::capi::DataProvider* provider);
    
    diplomat::capi::Decomposed icu4x_CanonicalDecomposition_decompose_mv1(const diplomat::capi::CanonicalDecomposition* self, char32_t c);
    
    
    void icu4x_CanonicalDecomposition_destroy_mv1(CanonicalDecomposition* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<CanonicalDecomposition>, DataError> CanonicalDecomposition::create(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CanonicalDecomposition_create_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CanonicalDecomposition>, DataError>(diplomat::Ok<std::unique_ptr<CanonicalDecomposition>>(std::unique_ptr<CanonicalDecomposition>(CanonicalDecomposition::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CanonicalDecomposition>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline Decomposed CanonicalDecomposition::decompose(char32_t c) const {
  auto result = diplomat::capi::icu4x_CanonicalDecomposition_decompose_mv1(this->AsFFI(),
    c);
  return Decomposed::FromFFI(result);
}

inline const diplomat::capi::CanonicalDecomposition* CanonicalDecomposition::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::CanonicalDecomposition*>(this);
}

inline diplomat::capi::CanonicalDecomposition* CanonicalDecomposition::AsFFI() {
  return reinterpret_cast<diplomat::capi::CanonicalDecomposition*>(this);
}

inline const CanonicalDecomposition* CanonicalDecomposition::FromFFI(const diplomat::capi::CanonicalDecomposition* ptr) {
  return reinterpret_cast<const CanonicalDecomposition*>(ptr);
}

inline CanonicalDecomposition* CanonicalDecomposition::FromFFI(diplomat::capi::CanonicalDecomposition* ptr) {
  return reinterpret_cast<CanonicalDecomposition*>(ptr);
}

inline void CanonicalDecomposition::operator delete(void* ptr) {
  diplomat::capi::icu4x_CanonicalDecomposition_destroy_mv1(reinterpret_cast<diplomat::capi::CanonicalDecomposition*>(ptr));
}


#endif // CanonicalDecomposition_HPP
