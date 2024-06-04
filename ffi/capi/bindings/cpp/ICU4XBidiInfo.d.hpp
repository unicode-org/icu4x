#ifndef ICU4XBidiInfo_D_HPP
#define ICU4XBidiInfo_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XBidiInfo.d.h"

class ICU4XBidiParagraph;


class ICU4XBidiInfo {
public:

  inline size_t paragraph_count() const;

  inline std::unique_ptr<ICU4XBidiParagraph> paragraph_at(size_t n) const;

  inline size_t size() const;

  inline uint8_t level_at(size_t pos) const;

  inline const capi::ICU4XBidiInfo* AsFFI() const;
  inline capi::ICU4XBidiInfo* AsFFI();
  inline static const ICU4XBidiInfo* FromFFI(const capi::ICU4XBidiInfo* ptr);
  inline static ICU4XBidiInfo* FromFFI(capi::ICU4XBidiInfo* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XBidiInfo() = delete;
  ICU4XBidiInfo(const ICU4XBidiInfo&) = delete;
  ICU4XBidiInfo(ICU4XBidiInfo&&) noexcept = delete;
  ICU4XBidiInfo operator=(const ICU4XBidiInfo&) = delete;
  ICU4XBidiInfo operator=(ICU4XBidiInfo&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XBidiInfo_D_HPP
