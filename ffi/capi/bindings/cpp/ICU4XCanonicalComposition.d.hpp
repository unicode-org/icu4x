#ifndef ICU4XCanonicalComposition_D_HPP
#define ICU4XCanonicalComposition_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataError.d.hpp"

class ICU4XDataProvider;
class ICU4XDataError;


namespace capi {
    typedef struct ICU4XCanonicalComposition ICU4XCanonicalComposition;
}

class ICU4XCanonicalComposition {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XCanonicalComposition>, ICU4XDataError> create(const ICU4XDataProvider& provider);

  inline char32_t compose(char32_t starter, char32_t second) const;

  inline const capi::ICU4XCanonicalComposition* AsFFI() const;
  inline capi::ICU4XCanonicalComposition* AsFFI();
  inline static const ICU4XCanonicalComposition* FromFFI(const capi::ICU4XCanonicalComposition* ptr);
  inline static ICU4XCanonicalComposition* FromFFI(capi::ICU4XCanonicalComposition* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XCanonicalComposition() = delete;
  ICU4XCanonicalComposition(const ICU4XCanonicalComposition&) = delete;
  ICU4XCanonicalComposition(ICU4XCanonicalComposition&&) noexcept = delete;
  ICU4XCanonicalComposition operator=(const ICU4XCanonicalComposition&) = delete;
  ICU4XCanonicalComposition operator=(ICU4XCanonicalComposition&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XCanonicalComposition_D_HPP
