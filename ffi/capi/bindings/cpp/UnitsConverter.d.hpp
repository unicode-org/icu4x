#ifndef UnitsConverter_D_HPP
#define UnitsConverter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef struct UnitsConverter UnitsConverter;
}

class UnitsConverter {
public:

  inline double convert_f64(double value) const;

  inline std::unique_ptr<UnitsConverter> clone() const;

  inline const capi::UnitsConverter* AsFFI() const;
  inline capi::UnitsConverter* AsFFI();
  inline static const UnitsConverter* FromFFI(const capi::UnitsConverter* ptr);
  inline static UnitsConverter* FromFFI(capi::UnitsConverter* ptr);
  inline static void operator delete(void* ptr);
private:
  UnitsConverter() = delete;
  UnitsConverter(const UnitsConverter&) = delete;
  UnitsConverter(UnitsConverter&&) noexcept = delete;
  UnitsConverter operator=(const UnitsConverter&) = delete;
  UnitsConverter operator=(UnitsConverter&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // UnitsConverter_D_HPP
