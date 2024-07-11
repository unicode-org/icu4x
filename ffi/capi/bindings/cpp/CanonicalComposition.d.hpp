#ifndef CanonicalComposition_D_HPP
#define CanonicalComposition_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {typedef struct DataProvider DataProvider; }
class DataProvider;
class DataError;


namespace diplomat {
namespace capi {
    typedef struct CanonicalComposition CanonicalComposition;
} // namespace capi
} // namespace

class CanonicalComposition {
public:

  inline static diplomat::result<std::unique_ptr<CanonicalComposition>, DataError> create(const DataProvider& provider);

  inline char32_t compose(char32_t starter, char32_t second) const;

  inline const diplomat::capi::CanonicalComposition* AsFFI() const;
  inline diplomat::capi::CanonicalComposition* AsFFI();
  inline static const CanonicalComposition* FromFFI(const diplomat::capi::CanonicalComposition* ptr);
  inline static CanonicalComposition* FromFFI(diplomat::capi::CanonicalComposition* ptr);
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
