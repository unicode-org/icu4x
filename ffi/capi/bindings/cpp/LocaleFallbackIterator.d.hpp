#ifndef LocaleFallbackIterator_D_HPP
#define LocaleFallbackIterator_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {typedef struct Locale Locale; }
class Locale;


namespace diplomat {
namespace capi {
    typedef struct LocaleFallbackIterator LocaleFallbackIterator;
} // namespace capi
} // namespace

class LocaleFallbackIterator {
public:

  inline std::unique_ptr<Locale> next();

  inline const diplomat::capi::LocaleFallbackIterator* AsFFI() const;
  inline diplomat::capi::LocaleFallbackIterator* AsFFI();
  inline static const LocaleFallbackIterator* FromFFI(const diplomat::capi::LocaleFallbackIterator* ptr);
  inline static LocaleFallbackIterator* FromFFI(diplomat::capi::LocaleFallbackIterator* ptr);
  inline static void operator delete(void* ptr);
private:
  LocaleFallbackIterator() = delete;
  LocaleFallbackIterator(const LocaleFallbackIterator&) = delete;
  LocaleFallbackIterator(LocaleFallbackIterator&&) noexcept = delete;
  LocaleFallbackIterator operator=(const LocaleFallbackIterator&) = delete;
  LocaleFallbackIterator operator=(LocaleFallbackIterator&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // LocaleFallbackIterator_D_HPP
