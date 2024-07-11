#ifndef MeasureUnit_D_HPP
#define MeasureUnit_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    typedef struct MeasureUnit MeasureUnit;
} // namespace capi
} // namespace

class MeasureUnit {
public:

  inline const diplomat::capi::MeasureUnit* AsFFI() const;
  inline diplomat::capi::MeasureUnit* AsFFI();
  inline static const MeasureUnit* FromFFI(const diplomat::capi::MeasureUnit* ptr);
  inline static MeasureUnit* FromFFI(diplomat::capi::MeasureUnit* ptr);
  inline static void operator delete(void* ptr);
private:
  MeasureUnit() = delete;
  MeasureUnit(const MeasureUnit&) = delete;
  MeasureUnit(MeasureUnit&&) noexcept = delete;
  MeasureUnit operator=(const MeasureUnit&) = delete;
  MeasureUnit operator=(MeasureUnit&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // MeasureUnit_D_HPP
