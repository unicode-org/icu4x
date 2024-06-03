#ifndef ICU4XUnitsConverter_D_HPP
#define ICU4XUnitsConverter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XUnitsConverter.d.h"


class ICU4XUnitsConverter {
public:

  inline double convert_f64(double value) const;

  inline std::unique_ptr<ICU4XUnitsConverter> clone() const;

  inline const capi::ICU4XUnitsConverter* AsFFI() const;
  inline capi::ICU4XUnitsConverter* AsFFI();
  inline static const ICU4XUnitsConverter* FromFFI(const capi::ICU4XUnitsConverter* ptr);
  inline static ICU4XUnitsConverter* FromFFI(capi::ICU4XUnitsConverter* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XUnitsConverter() = delete;
  ICU4XUnitsConverter(const ICU4XUnitsConverter&) = delete;
  ICU4XUnitsConverter(ICU4XUnitsConverter&&) noexcept = delete;
  ICU4XUnitsConverter operator=(const ICU4XUnitsConverter&) = delete;
  ICU4XUnitsConverter operator=(ICU4XUnitsConverter&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XUnitsConverter_D_HPP
