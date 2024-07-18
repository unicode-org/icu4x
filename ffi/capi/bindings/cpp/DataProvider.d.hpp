#ifndef DataProvider_D_HPP
#define DataProvider_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace diplomat::capi { struct LocaleFallbacker; }
class LocaleFallbacker;
class DataError;


namespace diplomat {
namespace capi {
    struct DataProvider;
} // namespace capi
} // namespace

class DataProvider {
public:

  inline static std::unique_ptr<DataProvider> compiled();

  inline static diplomat::result<std::unique_ptr<DataProvider>, DataError> from_fs(std::string_view path);

  inline static diplomat::result<std::unique_ptr<DataProvider>, DataError> from_byte_slice(diplomat::span<const uint8_t> blob);

  inline static std::unique_ptr<DataProvider> empty();

  inline diplomat::result<std::monostate, DataError> fork_by_key(DataProvider& other);

  inline diplomat::result<std::monostate, DataError> fork_by_locale(DataProvider& other);

  inline diplomat::result<std::monostate, DataError> enable_locale_fallback_with(const LocaleFallbacker& fallbacker);

  inline const diplomat::capi::DataProvider* AsFFI() const;
  inline diplomat::capi::DataProvider* AsFFI();
  inline static const DataProvider* FromFFI(const diplomat::capi::DataProvider* ptr);
  inline static DataProvider* FromFFI(diplomat::capi::DataProvider* ptr);
  inline static void operator delete(void* ptr);
private:
  DataProvider() = delete;
  DataProvider(const DataProvider&) = delete;
  DataProvider(DataProvider&&) noexcept = delete;
  DataProvider operator=(const DataProvider&) = delete;
  DataProvider operator=(DataProvider&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // DataProvider_D_HPP
