#ifndef ICU4XCodePointMapData16_D_HPP
#define ICU4XCodePointMapData16_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XCodePointMapData16.d.h"
#include "ICU4XDataError.d.hpp"

class ICU4XCodePointRangeIterator;
class ICU4XCodePointSetData;
class ICU4XDataProvider;
class ICU4XDataError;


class ICU4XCodePointMapData16 {
public:

  inline uint16_t get(char32_t cp) const;

  inline uint16_t get32(uint32_t cp) const;

  inline std::unique_ptr<ICU4XCodePointRangeIterator> iter_ranges_for_value(uint16_t value) const;

  inline std::unique_ptr<ICU4XCodePointRangeIterator> iter_ranges_for_value_complemented(uint16_t value) const;

  inline std::unique_ptr<ICU4XCodePointSetData> get_set_for_value(uint16_t value) const;

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointMapData16>, ICU4XDataError> load_script(const ICU4XDataProvider& provider);

  inline const capi::ICU4XCodePointMapData16* AsFFI() const;
  inline capi::ICU4XCodePointMapData16* AsFFI();
  inline static const ICU4XCodePointMapData16* FromFFI(const capi::ICU4XCodePointMapData16* ptr);
  inline static ICU4XCodePointMapData16* FromFFI(capi::ICU4XCodePointMapData16* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XCodePointMapData16() = delete;
  ICU4XCodePointMapData16(const ICU4XCodePointMapData16&) = delete;
  ICU4XCodePointMapData16(ICU4XCodePointMapData16&&) noexcept = delete;
  ICU4XCodePointMapData16 operator=(const ICU4XCodePointMapData16&) = delete;
  ICU4XCodePointMapData16 operator=(ICU4XCodePointMapData16&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XCodePointMapData16_D_HPP
