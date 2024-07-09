#ifndef ICU4XCanonicalDecomposition_D_HPP
#define ICU4XCanonicalDecomposition_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataError.d.hpp"
#include "ICU4XDecomposed.d.hpp"

class ICU4XDataProvider;
struct ICU4XDecomposed;
class ICU4XDataError;


namespace capi {
    typedef struct ICU4XCanonicalDecomposition ICU4XCanonicalDecomposition;
}

class ICU4XCanonicalDecomposition {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XCanonicalDecomposition>, ICU4XDataError> create(const ICU4XDataProvider& provider);

  inline ICU4XDecomposed decompose(char32_t c) const;

  inline const capi::ICU4XCanonicalDecomposition* AsFFI() const;
  inline capi::ICU4XCanonicalDecomposition* AsFFI();
  inline static const ICU4XCanonicalDecomposition* FromFFI(const capi::ICU4XCanonicalDecomposition* ptr);
  inline static ICU4XCanonicalDecomposition* FromFFI(capi::ICU4XCanonicalDecomposition* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XCanonicalDecomposition() = delete;
  ICU4XCanonicalDecomposition(const ICU4XCanonicalDecomposition&) = delete;
  ICU4XCanonicalDecomposition(ICU4XCanonicalDecomposition&&) noexcept = delete;
  ICU4XCanonicalDecomposition operator=(const ICU4XCanonicalDecomposition&) = delete;
  ICU4XCanonicalDecomposition operator=(ICU4XCanonicalDecomposition&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XCanonicalDecomposition_D_HPP
