#ifndef ICU4XCanonicalDecomposition_HPP
#define ICU4XCanonicalDecomposition_HPP

#include "ICU4XCanonicalDecomposition.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataError.hpp"
#include "ICU4XDataProvider.hpp"
#include "ICU4XDecomposed.hpp"


namespace capi {
    extern "C" {
    
    typedef struct ICU4XCanonicalDecomposition_create_result {union {ICU4XCanonicalDecomposition* ok; ICU4XDataError err;}; bool is_ok;} ICU4XCanonicalDecomposition_create_result;
    ICU4XCanonicalDecomposition_create_result ICU4XCanonicalDecomposition_create(const ICU4XDataProvider* provider);
    
    ICU4XDecomposed ICU4XCanonicalDecomposition_decompose(const ICU4XCanonicalDecomposition* self, char32_t c);
    
    
    void ICU4XCanonicalDecomposition_destroy(ICU4XCanonicalDecomposition* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<ICU4XCanonicalDecomposition>, ICU4XDataError> ICU4XCanonicalDecomposition::create(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCanonicalDecomposition_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCanonicalDecomposition>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XCanonicalDecomposition>>(std::unique_ptr<ICU4XCanonicalDecomposition>(ICU4XCanonicalDecomposition::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCanonicalDecomposition>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline ICU4XDecomposed ICU4XCanonicalDecomposition::decompose(char32_t c) const {
  auto result = capi::ICU4XCanonicalDecomposition_decompose(this->AsFFI(),
    c);
  return ICU4XDecomposed::FromFFI(result);
}

inline const capi::ICU4XCanonicalDecomposition* ICU4XCanonicalDecomposition::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XCanonicalDecomposition*>(this);
}

inline capi::ICU4XCanonicalDecomposition* ICU4XCanonicalDecomposition::AsFFI() {
  return reinterpret_cast<capi::ICU4XCanonicalDecomposition*>(this);
}

inline const ICU4XCanonicalDecomposition* ICU4XCanonicalDecomposition::FromFFI(const capi::ICU4XCanonicalDecomposition* ptr) {
  return reinterpret_cast<const ICU4XCanonicalDecomposition*>(ptr);
}

inline ICU4XCanonicalDecomposition* ICU4XCanonicalDecomposition::FromFFI(capi::ICU4XCanonicalDecomposition* ptr) {
  return reinterpret_cast<ICU4XCanonicalDecomposition*>(ptr);
}

inline void ICU4XCanonicalDecomposition::operator delete(void* ptr) {
  capi::ICU4XCanonicalDecomposition_destroy(reinterpret_cast<capi::ICU4XCanonicalDecomposition*>(ptr));
}


#endif // ICU4XCanonicalDecomposition_HPP
