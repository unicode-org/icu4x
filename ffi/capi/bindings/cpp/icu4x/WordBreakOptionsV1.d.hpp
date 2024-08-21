#ifndef icu4x_WordBreakOptionsV1_D_HPP
#define icu4x_WordBreakOptionsV1_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"

namespace icu4x {
namespace capi { struct Locale; }
class Locale;
namespace capi { struct WordBreakOptionsV1; }
class WordBreakOptionsV1;
}


namespace icu4x {
namespace capi {
    struct WordBreakOptionsV1;
} // namespace capi
} // namespace

namespace icu4x {
class WordBreakOptionsV1 {
public:

  inline static std::unique_ptr<icu4x::WordBreakOptionsV1> create(const icu4x::Locale& locale);

  inline const icu4x::capi::WordBreakOptionsV1* AsFFI() const;
  inline icu4x::capi::WordBreakOptionsV1* AsFFI();
  inline static const icu4x::WordBreakOptionsV1* FromFFI(const icu4x::capi::WordBreakOptionsV1* ptr);
  inline static icu4x::WordBreakOptionsV1* FromFFI(icu4x::capi::WordBreakOptionsV1* ptr);
  inline static void operator delete(void* ptr);
private:
  WordBreakOptionsV1() = delete;
  WordBreakOptionsV1(const icu4x::WordBreakOptionsV1&) = delete;
  WordBreakOptionsV1(icu4x::WordBreakOptionsV1&&) noexcept = delete;
  WordBreakOptionsV1 operator=(const icu4x::WordBreakOptionsV1&) = delete;
  WordBreakOptionsV1 operator=(icu4x::WordBreakOptionsV1&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // icu4x_WordBreakOptionsV1_D_HPP
