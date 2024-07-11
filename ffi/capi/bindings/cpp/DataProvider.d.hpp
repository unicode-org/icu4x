#ifndef DataProvider_D_HPP
#define DataProvider_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "DataError.d.hpp"

class LocaleFallbacker;
class DataError;


namespace capi {
    typedef struct DataProvider DataProvider;
}

class DataProvider {
public:

  inline static std::unique_ptr<DataProvider> create_compiled();

  inline static diplomat::result<std::unique_ptr<DataProvider>, DataError> create_fs(std::string_view path);

  inline static diplomat::result<std::unique_ptr<DataProvider>, DataError> create_from_byte_slice(diplomat::span<const uint8_t> blob);

  inline static std::unique_ptr<DataProvider> create_empty();

  inline diplomat::result<std::monostate, DataError> fork_by_key(DataProvider& other);

  inline diplomat::result<std::monostate, DataError> fork_by_locale(DataProvider& other);

  inline diplomat::result<std::monostate, DataError> enable_locale_fallback_with(const LocaleFallbacker& fallbacker);

  inline const capi::DataProvider* AsFFI() const;
  inline capi::DataProvider* AsFFI();
  inline static const DataProvider* FromFFI(const capi::DataProvider* ptr);
  inline static DataProvider* FromFFI(capi::DataProvider* ptr);
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
