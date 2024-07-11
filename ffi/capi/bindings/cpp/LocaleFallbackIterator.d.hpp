#ifndef LocaleFallbackIterator_D_HPP
#define LocaleFallbackIterator_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

class Locale;


namespace capi {
    typedef struct LocaleFallbackIterator LocaleFallbackIterator;
}

class LocaleFallbackIterator {
public:

  inline std::unique_ptr<Locale> next();

  inline const capi::LocaleFallbackIterator* AsFFI() const;
  inline capi::LocaleFallbackIterator* AsFFI();
  inline static const LocaleFallbackIterator* FromFFI(const capi::LocaleFallbackIterator* ptr);
  inline static LocaleFallbackIterator* FromFFI(capi::LocaleFallbackIterator* ptr);
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
