#ifndef icu4x_CanonicalComposition_HPP
#define icu4x_CanonicalComposition_HPP

#include "CanonicalComposition.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_CanonicalComposition_create_mv1_result {union {icu4x::capi::CanonicalComposition* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_CanonicalComposition_create_mv1_result;
    icu4x_CanonicalComposition_create_mv1_result icu4x_CanonicalComposition_create_mv1(const icu4x::capi::DataProvider* provider);
    
    char32_t icu4x_CanonicalComposition_compose_mv1(const icu4x::capi::CanonicalComposition* self, char32_t starter, char32_t second);
    
    
    void icu4x_CanonicalComposition_destroy_mv1(CanonicalComposition* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<icu4x::CanonicalComposition>, icu4x::DataError> icu4x::CanonicalComposition::create(const icu4x::DataProvider& provider) {
  auto result = icu4x::capi::icu4x_CanonicalComposition_create_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::CanonicalComposition>, icu4x::DataError>(diplomat::Ok<std::unique_ptr<icu4x::CanonicalComposition>>(std::unique_ptr<icu4x::CanonicalComposition>(icu4x::CanonicalComposition::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::CanonicalComposition>, icu4x::DataError>(diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline char32_t icu4x::CanonicalComposition::compose(char32_t starter, char32_t second) const {
  auto result = icu4x::capi::icu4x_CanonicalComposition_compose_mv1(this->AsFFI(),
    starter,
    second);
  return result;
}

inline const icu4x::capi::CanonicalComposition* icu4x::CanonicalComposition::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::CanonicalComposition*>(this);
}

inline icu4x::capi::CanonicalComposition* icu4x::CanonicalComposition::AsFFI() {
  return reinterpret_cast<icu4x::capi::CanonicalComposition*>(this);
}

inline const icu4x::CanonicalComposition* icu4x::CanonicalComposition::FromFFI(const icu4x::capi::CanonicalComposition* ptr) {
  return reinterpret_cast<const icu4x::CanonicalComposition*>(ptr);
}

inline icu4x::CanonicalComposition* icu4x::CanonicalComposition::FromFFI(icu4x::capi::CanonicalComposition* ptr) {
  return reinterpret_cast<icu4x::CanonicalComposition*>(ptr);
}

inline void icu4x::CanonicalComposition::operator delete(void* ptr) {
  icu4x::capi::icu4x_CanonicalComposition_destroy_mv1(reinterpret_cast<icu4x::capi::CanonicalComposition*>(ptr));
}


#endif // icu4x_CanonicalComposition_HPP
