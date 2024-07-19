#ifndef CanonicalCombiningClassMap_D_HPP
#define CanonicalCombiningClassMap_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace diplomat::capi { struct DataProvider; }
class DataProvider;
class DataError;


namespace diplomat {
namespace capi {
    struct CanonicalCombiningClassMap;
} // namespace capi
} // namespace

class CanonicalCombiningClassMap {
public:

  inline static diplomat::result<std::unique_ptr<CanonicalCombiningClassMap>, DataError> create(const DataProvider& provider);

  inline uint8_t get(char32_t ch) const;

  inline const diplomat::capi::CanonicalCombiningClassMap* AsFFI() const;
  inline diplomat::capi::CanonicalCombiningClassMap* AsFFI();
  inline static const CanonicalCombiningClassMap* FromFFI(const diplomat::capi::CanonicalCombiningClassMap* ptr);
  inline static CanonicalCombiningClassMap* FromFFI(diplomat::capi::CanonicalCombiningClassMap* ptr);
  inline static void operator delete(void* ptr);
private:
  CanonicalCombiningClassMap() = delete;
  CanonicalCombiningClassMap(const CanonicalCombiningClassMap&) = delete;
  CanonicalCombiningClassMap(CanonicalCombiningClassMap&&) noexcept = delete;
  CanonicalCombiningClassMap operator=(const CanonicalCombiningClassMap&) = delete;
  CanonicalCombiningClassMap operator=(CanonicalCombiningClassMap&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // CanonicalCombiningClassMap_D_HPP
