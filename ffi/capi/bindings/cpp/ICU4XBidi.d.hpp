#ifndef ICU4XBidi_D_HPP
#define ICU4XBidi_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XBidi.d.h"
#include "ICU4XError.d.hpp"

class ICU4XBidiInfo;
class ICU4XDataProvider;
class ICU4XReorderedIndexMap;
class ICU4XError;


class ICU4XBidi {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XBidi>, ICU4XError> create(const ICU4XDataProvider& provider);

  inline std::unique_ptr<ICU4XBidiInfo> for_text(std::string_view text, uint8_t default_level) const;

  inline std::unique_ptr<ICU4XReorderedIndexMap> reorder_visual(diplomat::span<const uint8_t> levels) const;

  inline static bool level_is_rtl(uint8_t level);

  inline static bool level_is_ltr(uint8_t level);

  inline static uint8_t level_rtl();

  inline static uint8_t level_ltr();

  inline const capi::ICU4XBidi* AsFFI() const;
  inline capi::ICU4XBidi* AsFFI();
  inline static const ICU4XBidi* FromFFI(const capi::ICU4XBidi* ptr);
  inline static ICU4XBidi* FromFFI(capi::ICU4XBidi* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XBidi() = delete;
  ICU4XBidi(const ICU4XBidi&) = delete;
  ICU4XBidi(ICU4XBidi&&) noexcept = delete;
  ICU4XBidi operator=(const ICU4XBidi&) = delete;
  ICU4XBidi operator=(ICU4XBidi&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XBidi_D_HPP
