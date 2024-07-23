#ifndef ReorderedIndexMap_D_HPP
#define ReorderedIndexMap_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    struct ReorderedIndexMap;
} // namespace capi
} // namespace

class ReorderedIndexMap {
public:

  inline diplomat::span<const size_t> as_slice() const;

  inline size_t len() const;

  inline bool is_empty() const;

  inline size_t get(size_t index) const;

  inline const diplomat::capi::ReorderedIndexMap* AsFFI() const;
  inline diplomat::capi::ReorderedIndexMap* AsFFI();
  inline static const ReorderedIndexMap* FromFFI(const diplomat::capi::ReorderedIndexMap* ptr);
  inline static ReorderedIndexMap* FromFFI(diplomat::capi::ReorderedIndexMap* ptr);
  inline static void operator delete(void* ptr);
private:
  ReorderedIndexMap() = delete;
  ReorderedIndexMap(const ReorderedIndexMap&) = delete;
  ReorderedIndexMap(ReorderedIndexMap&&) noexcept = delete;
  ReorderedIndexMap operator=(const ReorderedIndexMap&) = delete;
  ReorderedIndexMap operator=(ReorderedIndexMap&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ReorderedIndexMap_D_HPP
