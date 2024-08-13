#ifndef icu4x_MetazoneCalculator_D_HPP
#define icu4x_MetazoneCalculator_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"

namespace icu4x {
namespace capi { struct DataProvider; }
class DataProvider;
namespace capi { struct MetazoneCalculator; }
class MetazoneCalculator;
class DataError;
}


namespace icu4x {
namespace capi {
    struct MetazoneCalculator;
} // namespace capi
} // namespace

namespace icu4x {
class MetazoneCalculator {
public:

  inline static diplomat::result<std::unique_ptr<icu4x::MetazoneCalculator>, icu4x::DataError> create(const icu4x::DataProvider& provider);

  inline const icu4x::capi::MetazoneCalculator* AsFFI() const;
  inline icu4x::capi::MetazoneCalculator* AsFFI();
  inline static const icu4x::MetazoneCalculator* FromFFI(const icu4x::capi::MetazoneCalculator* ptr);
  inline static icu4x::MetazoneCalculator* FromFFI(icu4x::capi::MetazoneCalculator* ptr);
  inline static void operator delete(void* ptr);
private:
  MetazoneCalculator() = delete;
  MetazoneCalculator(const icu4x::MetazoneCalculator&) = delete;
  MetazoneCalculator(icu4x::MetazoneCalculator&&) noexcept = delete;
  MetazoneCalculator operator=(const icu4x::MetazoneCalculator&) = delete;
  MetazoneCalculator operator=(icu4x::MetazoneCalculator&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // icu4x_MetazoneCalculator_D_HPP
