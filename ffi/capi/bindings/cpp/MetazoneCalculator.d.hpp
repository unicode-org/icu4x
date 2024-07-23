#ifndef MetazoneCalculator_D_HPP
#define MetazoneCalculator_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace diplomat::capi { struct DataProvider; }
class DataProvider;
class DataError;


namespace diplomat {
namespace capi {
    struct MetazoneCalculator;
} // namespace capi
} // namespace

class MetazoneCalculator {
public:

  inline static diplomat::result<std::unique_ptr<MetazoneCalculator>, DataError> create(const DataProvider& provider);

  inline const diplomat::capi::MetazoneCalculator* AsFFI() const;
  inline diplomat::capi::MetazoneCalculator* AsFFI();
  inline static const MetazoneCalculator* FromFFI(const diplomat::capi::MetazoneCalculator* ptr);
  inline static MetazoneCalculator* FromFFI(diplomat::capi::MetazoneCalculator* ptr);
  inline static void operator delete(void* ptr);
private:
  MetazoneCalculator() = delete;
  MetazoneCalculator(const MetazoneCalculator&) = delete;
  MetazoneCalculator(MetazoneCalculator&&) noexcept = delete;
  MetazoneCalculator operator=(const MetazoneCalculator&) = delete;
  MetazoneCalculator operator=(MetazoneCalculator&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // MetazoneCalculator_D_HPP
