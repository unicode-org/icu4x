#ifndef ICU4XCanonicalComposition_HPP
#define ICU4XCanonicalComposition_HPP

#include "ICU4XCanonicalComposition.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XCanonicalComposition.h"
#include "ICU4XDataError.hpp"
#include "ICU4XDataProvider.hpp"


inline diplomat::result<std::unique_ptr<ICU4XCanonicalComposition>, ICU4XDataError> ICU4XCanonicalComposition::create(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCanonicalComposition_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCanonicalComposition>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XCanonicalComposition>>(std::unique_ptr<ICU4XCanonicalComposition>(ICU4XCanonicalComposition::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCanonicalComposition>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline char32_t ICU4XCanonicalComposition::compose(char32_t starter, char32_t second) const {
  auto result = capi::ICU4XCanonicalComposition_compose(this->AsFFI(),
    starter,
    second);
  return result;
}

inline const capi::ICU4XCanonicalComposition* ICU4XCanonicalComposition::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XCanonicalComposition*>(this);
}

inline capi::ICU4XCanonicalComposition* ICU4XCanonicalComposition::AsFFI() {
  return reinterpret_cast<capi::ICU4XCanonicalComposition*>(this);
}

inline const ICU4XCanonicalComposition* ICU4XCanonicalComposition::FromFFI(const capi::ICU4XCanonicalComposition* ptr) {
  return reinterpret_cast<const ICU4XCanonicalComposition*>(ptr);
}

inline ICU4XCanonicalComposition* ICU4XCanonicalComposition::FromFFI(capi::ICU4XCanonicalComposition* ptr) {
  return reinterpret_cast<ICU4XCanonicalComposition*>(ptr);
}

inline void ICU4XCanonicalComposition::operator delete(void* ptr) {
  capi::ICU4XCanonicalComposition_destroy(reinterpret_cast<capi::ICU4XCanonicalComposition*>(ptr));
}


#endif // ICU4XCanonicalComposition_HPP
