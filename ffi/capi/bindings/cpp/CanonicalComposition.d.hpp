#ifndef CanonicalComposition_D_HPP
#define CanonicalComposition_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "DataError.d.hpp"

class DataProvider;
class DataError;


namespace capi {
    typedef struct CanonicalComposition CanonicalComposition;
}

class CanonicalComposition {
public:

  inline static diplomat::result<std::unique_ptr<CanonicalComposition>, DataError> create(const DataProvider& provider);

  inline char32_t compose(char32_t starter, char32_t second) const;

  inline const capi::CanonicalComposition* AsFFI() const;
  inline capi::CanonicalComposition* AsFFI();
  inline static const CanonicalComposition* FromFFI(const capi::CanonicalComposition* ptr);
  inline static CanonicalComposition* FromFFI(capi::CanonicalComposition* ptr);
  inline static void operator delete(void* ptr);
private:
  CanonicalComposition() = delete;
  CanonicalComposition(const CanonicalComposition&) = delete;
  CanonicalComposition(CanonicalComposition&&) noexcept = delete;
  CanonicalComposition operator=(const CanonicalComposition&) = delete;
  CanonicalComposition operator=(CanonicalComposition&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // CanonicalComposition_D_HPP
