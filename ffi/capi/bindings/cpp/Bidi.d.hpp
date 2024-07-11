#ifndef Bidi_D_HPP
#define Bidi_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {typedef struct BidiInfo BidiInfo; }
class BidiInfo;
namespace capi {typedef struct DataProvider DataProvider; }
class DataProvider;
namespace capi {typedef struct ReorderedIndexMap ReorderedIndexMap; }
class ReorderedIndexMap;
class DataError;


namespace diplomat {
namespace capi {
    typedef struct Bidi Bidi;
} // namespace capi
} // namespace

class Bidi {
public:

  inline static diplomat::result<std::unique_ptr<Bidi>, DataError> create(const DataProvider& provider);

  inline std::unique_ptr<BidiInfo> for_text(std::string_view text, uint8_t default_level) const;

  inline std::unique_ptr<ReorderedIndexMap> reorder_visual(diplomat::span<const uint8_t> levels) const;

  inline static bool level_is_rtl(uint8_t level);

  inline static bool level_is_ltr(uint8_t level);

  inline static uint8_t level_rtl();

  inline static uint8_t level_ltr();

  inline const diplomat::capi::Bidi* AsFFI() const;
  inline diplomat::capi::Bidi* AsFFI();
  inline static const Bidi* FromFFI(const diplomat::capi::Bidi* ptr);
  inline static Bidi* FromFFI(diplomat::capi::Bidi* ptr);
  inline static void operator delete(void* ptr);
private:
  Bidi() = delete;
  Bidi(const Bidi&) = delete;
  Bidi(Bidi&&) noexcept = delete;
  Bidi operator=(const Bidi&) = delete;
  Bidi operator=(Bidi&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // Bidi_D_HPP
