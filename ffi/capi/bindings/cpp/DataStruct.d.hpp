#ifndef DataStruct_D_HPP
#define DataStruct_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    typedef struct DataStruct DataStruct;
} // namespace capi
} // namespace

class DataStruct {
public:

  inline static std::unique_ptr<DataStruct> create_decimal_symbols_v1(std::string_view plus_sign_prefix, std::string_view plus_sign_suffix, std::string_view minus_sign_prefix, std::string_view minus_sign_suffix, std::string_view decimal_separator, std::string_view grouping_separator, uint8_t primary_group_size, uint8_t secondary_group_size, uint8_t min_group_size, diplomat::span<const char32_t> digits);

  inline const diplomat::capi::DataStruct* AsFFI() const;
  inline diplomat::capi::DataStruct* AsFFI();
  inline static const DataStruct* FromFFI(const diplomat::capi::DataStruct* ptr);
  inline static DataStruct* FromFFI(diplomat::capi::DataStruct* ptr);
  inline static void operator delete(void* ptr);
private:
  DataStruct() = delete;
  DataStruct(const DataStruct&) = delete;
  DataStruct(DataStruct&&) noexcept = delete;
  DataStruct operator=(const DataStruct&) = delete;
  DataStruct operator=(DataStruct&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // DataStruct_D_HPP
