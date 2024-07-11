#ifndef CanonicalDecomposition_D_HPP
#define CanonicalDecomposition_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {typedef struct DataProvider DataProvider; }
class DataProvider;
struct Decomposed;
class DataError;


namespace diplomat {
namespace capi {
    typedef struct CanonicalDecomposition CanonicalDecomposition;
} // namespace capi
} // namespace

class CanonicalDecomposition {
public:

  inline static diplomat::result<std::unique_ptr<CanonicalDecomposition>, DataError> create(const DataProvider& provider);

  inline Decomposed decompose(char32_t c) const;

  inline const diplomat::capi::CanonicalDecomposition* AsFFI() const;
  inline diplomat::capi::CanonicalDecomposition* AsFFI();
  inline static const CanonicalDecomposition* FromFFI(const diplomat::capi::CanonicalDecomposition* ptr);
  inline static CanonicalDecomposition* FromFFI(diplomat::capi::CanonicalDecomposition* ptr);
  inline static void operator delete(void* ptr);
private:
  CanonicalDecomposition() = delete;
  CanonicalDecomposition(const CanonicalDecomposition&) = delete;
  CanonicalDecomposition(CanonicalDecomposition&&) noexcept = delete;
  CanonicalDecomposition operator=(const CanonicalDecomposition&) = delete;
  CanonicalDecomposition operator=(CanonicalDecomposition&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // CanonicalDecomposition_D_HPP
