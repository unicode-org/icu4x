#ifndef CanonicalComposition_HPP
#define CanonicalComposition_HPP

#include "CanonicalComposition.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"


namespace capi {
    extern "C" {
    
    typedef struct ICU4XCanonicalComposition_create_result {union {CanonicalComposition* ok; DataError err;}; bool is_ok;} ICU4XCanonicalComposition_create_result;
    ICU4XCanonicalComposition_create_result ICU4XCanonicalComposition_create(const DataProvider* provider);
    
    char32_t ICU4XCanonicalComposition_compose(const CanonicalComposition* self, char32_t starter, char32_t second);
    
    
    void ICU4XCanonicalComposition_destroy(CanonicalComposition* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<CanonicalComposition>, DataError> CanonicalComposition::create(const DataProvider& provider) {
  auto result = capi::ICU4XCanonicalComposition_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CanonicalComposition>, DataError>(diplomat::Ok<std::unique_ptr<CanonicalComposition>>(std::unique_ptr<CanonicalComposition>(CanonicalComposition::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CanonicalComposition>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline char32_t CanonicalComposition::compose(char32_t starter, char32_t second) const {
  auto result = capi::ICU4XCanonicalComposition_compose(this->AsFFI(),
    starter,
    second);
  return result;
}

inline const capi::CanonicalComposition* CanonicalComposition::AsFFI() const {
  return reinterpret_cast<const capi::CanonicalComposition*>(this);
}

inline capi::CanonicalComposition* CanonicalComposition::AsFFI() {
  return reinterpret_cast<capi::CanonicalComposition*>(this);
}

inline const CanonicalComposition* CanonicalComposition::FromFFI(const capi::CanonicalComposition* ptr) {
  return reinterpret_cast<const CanonicalComposition*>(ptr);
}

inline CanonicalComposition* CanonicalComposition::FromFFI(capi::CanonicalComposition* ptr) {
  return reinterpret_cast<CanonicalComposition*>(ptr);
}

inline void CanonicalComposition::operator delete(void* ptr) {
  capi::ICU4XCanonicalComposition_destroy(reinterpret_cast<capi::CanonicalComposition*>(ptr));
}


#endif // CanonicalComposition_HPP
