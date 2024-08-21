#ifndef icu4x_SentenceBreakOptionsV1_D_HPP
#define icu4x_SentenceBreakOptionsV1_D_HPP

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
namespace capi { struct SentenceBreakOptionsV1; }
class SentenceBreakOptionsV1;
}


namespace icu4x {
namespace capi {
    struct SentenceBreakOptionsV1;
} // namespace capi
} // namespace

namespace icu4x {
class SentenceBreakOptionsV1 {
public:

  inline static std::unique_ptr<icu4x::SentenceBreakOptionsV1> create(const icu4x::Locale& locale);

  inline const icu4x::capi::SentenceBreakOptionsV1* AsFFI() const;
  inline icu4x::capi::SentenceBreakOptionsV1* AsFFI();
  inline static const icu4x::SentenceBreakOptionsV1* FromFFI(const icu4x::capi::SentenceBreakOptionsV1* ptr);
  inline static icu4x::SentenceBreakOptionsV1* FromFFI(icu4x::capi::SentenceBreakOptionsV1* ptr);
  inline static void operator delete(void* ptr);
private:
  SentenceBreakOptionsV1() = delete;
  SentenceBreakOptionsV1(const icu4x::SentenceBreakOptionsV1&) = delete;
  SentenceBreakOptionsV1(icu4x::SentenceBreakOptionsV1&&) noexcept = delete;
  SentenceBreakOptionsV1 operator=(const icu4x::SentenceBreakOptionsV1&) = delete;
  SentenceBreakOptionsV1 operator=(icu4x::SentenceBreakOptionsV1&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // icu4x_SentenceBreakOptionsV1_D_HPP
