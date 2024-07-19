#ifndef CanonicalComposition_HPP
#define CanonicalComposition_HPP

#include "CanonicalComposition.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_CanonicalComposition_create_mv1_result {union {diplomat::capi::CanonicalComposition* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CanonicalComposition_create_mv1_result;
    icu4x_CanonicalComposition_create_mv1_result icu4x_CanonicalComposition_create_mv1(const diplomat::capi::DataProvider* provider);
    
    char32_t icu4x_CanonicalComposition_compose_mv1(const diplomat::capi::CanonicalComposition* self, char32_t starter, char32_t second);
    
    
    void icu4x_CanonicalComposition_destroy_mv1(CanonicalComposition* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<CanonicalComposition>, DataError> CanonicalComposition::create(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CanonicalComposition_create_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CanonicalComposition>, DataError>(diplomat::Ok<std::unique_ptr<CanonicalComposition>>(std::unique_ptr<CanonicalComposition>(CanonicalComposition::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CanonicalComposition>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline char32_t CanonicalComposition::compose(char32_t starter, char32_t second) const {
  auto result = diplomat::capi::icu4x_CanonicalComposition_compose_mv1(this->AsFFI(),
    starter,
    second);
  return result;
}

inline const diplomat::capi::CanonicalComposition* CanonicalComposition::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::CanonicalComposition*>(this);
}

inline diplomat::capi::CanonicalComposition* CanonicalComposition::AsFFI() {
  return reinterpret_cast<diplomat::capi::CanonicalComposition*>(this);
}

inline const CanonicalComposition* CanonicalComposition::FromFFI(const diplomat::capi::CanonicalComposition* ptr) {
  return reinterpret_cast<const CanonicalComposition*>(ptr);
}

inline CanonicalComposition* CanonicalComposition::FromFFI(diplomat::capi::CanonicalComposition* ptr) {
  return reinterpret_cast<CanonicalComposition*>(ptr);
}

inline void CanonicalComposition::operator delete(void* ptr) {
  diplomat::capi::icu4x_CanonicalComposition_destroy_mv1(reinterpret_cast<diplomat::capi::CanonicalComposition*>(ptr));
}


#endif // CanonicalComposition_HPP
