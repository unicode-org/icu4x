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
#include "ICU4XCanonicalDecomposition.h"
#include "ICU4XDataProvider.hpp"
#include "ICU4XDecomposed.hpp"
#include "ICU4XError.hpp"


inline diplomat::result<std::unique_ptr<ICU4XCanonicalDecomposition>, ICU4XError> ICU4XCanonicalDecomposition::create(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCanonicalDecomposition_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCanonicalDecomposition>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCanonicalDecomposition>>(std::unique_ptr<ICU4XCanonicalDecomposition>(ICU4XCanonicalDecomposition::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCanonicalDecomposition>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
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
