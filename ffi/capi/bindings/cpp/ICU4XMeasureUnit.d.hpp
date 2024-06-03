#ifndef ICU4XMeasureUnit_D_HPP
#define ICU4XMeasureUnit_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XMeasureUnit.d.h"


class ICU4XMeasureUnit {
public:

  inline const capi::ICU4XMeasureUnit* AsFFI() const;
  inline capi::ICU4XMeasureUnit* AsFFI();
  inline static const ICU4XMeasureUnit* FromFFI(const capi::ICU4XMeasureUnit* ptr);
  inline static ICU4XMeasureUnit* FromFFI(capi::ICU4XMeasureUnit* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XMeasureUnit() = delete;
  ICU4XMeasureUnit(const ICU4XMeasureUnit&) = delete;
  ICU4XMeasureUnit(ICU4XMeasureUnit&&) noexcept = delete;
  ICU4XMeasureUnit operator=(const ICU4XMeasureUnit&) = delete;
  ICU4XMeasureUnit operator=(ICU4XMeasureUnit&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XMeasureUnit_D_HPP
