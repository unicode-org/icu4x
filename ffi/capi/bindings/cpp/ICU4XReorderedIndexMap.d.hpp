#ifndef ICU4XReorderedIndexMap_D_HPP
#define ICU4XReorderedIndexMap_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XReorderedIndexMap.d.h"


class ICU4XReorderedIndexMap {
public:

  inline diplomat::span<const size_t> as_slice() const;

  inline size_t len() const;

  inline bool is_empty() const;

  inline size_t get(size_t index) const;

  inline const capi::ICU4XReorderedIndexMap* AsFFI() const;
  inline capi::ICU4XReorderedIndexMap* AsFFI();
  inline static const ICU4XReorderedIndexMap* FromFFI(const capi::ICU4XReorderedIndexMap* ptr);
  inline static ICU4XReorderedIndexMap* FromFFI(capi::ICU4XReorderedIndexMap* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XReorderedIndexMap() = delete;
  ICU4XReorderedIndexMap(const ICU4XReorderedIndexMap&) = delete;
  ICU4XReorderedIndexMap(ICU4XReorderedIndexMap&&) noexcept = delete;
  ICU4XReorderedIndexMap operator=(const ICU4XReorderedIndexMap&) = delete;
  ICU4XReorderedIndexMap operator=(ICU4XReorderedIndexMap&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XReorderedIndexMap_D_HPP
