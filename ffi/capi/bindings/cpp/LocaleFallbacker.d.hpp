#ifndef LocaleFallbacker_D_HPP
#define LocaleFallbacker_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace diplomat::capi { struct DataProvider; }
class DataProvider;
namespace diplomat::capi { struct LocaleFallbackerWithConfig; }
class LocaleFallbackerWithConfig;
struct LocaleFallbackConfig;
class DataError;
class LocaleParseError;


namespace diplomat {
namespace capi {
    struct LocaleFallbacker;
} // namespace capi
} // namespace

class LocaleFallbacker {
public:

  inline static diplomat::result<std::unique_ptr<LocaleFallbacker>, DataError> create(const DataProvider& provider);

  inline static std::unique_ptr<LocaleFallbacker> without_data();

  inline diplomat::result<std::unique_ptr<LocaleFallbackerWithConfig>, LocaleParseError> for_config(LocaleFallbackConfig config) const;

  inline const diplomat::capi::LocaleFallbacker* AsFFI() const;
  inline diplomat::capi::LocaleFallbacker* AsFFI();
  inline static const LocaleFallbacker* FromFFI(const diplomat::capi::LocaleFallbacker* ptr);
  inline static LocaleFallbacker* FromFFI(diplomat::capi::LocaleFallbacker* ptr);
  inline static void operator delete(void* ptr);
private:
  LocaleFallbacker() = delete;
  LocaleFallbacker(const LocaleFallbacker&) = delete;
  LocaleFallbacker(LocaleFallbacker&&) noexcept = delete;
  LocaleFallbacker operator=(const LocaleFallbacker&) = delete;
  LocaleFallbacker operator=(LocaleFallbacker&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // LocaleFallbacker_D_HPP
