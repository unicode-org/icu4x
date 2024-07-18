#ifndef DataStruct_HPP
#define DataStruct_HPP

#include "DataStruct.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    diplomat::capi::DataStruct* icu4x_DataStruct_create_decimal_symbols_v1_mv1(const char* plus_sign_prefix_data, size_t plus_sign_prefix_len, const char* plus_sign_suffix_data, size_t plus_sign_suffix_len, const char* minus_sign_prefix_data, size_t minus_sign_prefix_len, const char* minus_sign_suffix_data, size_t minus_sign_suffix_len, const char* decimal_separator_data, size_t decimal_separator_len, const char* grouping_separator_data, size_t grouping_separator_len, uint8_t primary_group_size, uint8_t secondary_group_size, uint8_t min_group_size, const char32_t* digits_data, size_t digits_len);
    
    
    void icu4x_DataStruct_destroy_mv1(DataStruct* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<DataStruct> DataStruct::create_decimal_symbols_v1(std::string_view plus_sign_prefix, std::string_view plus_sign_suffix, std::string_view minus_sign_prefix, std::string_view minus_sign_suffix, std::string_view decimal_separator, std::string_view grouping_separator, uint8_t primary_group_size, uint8_t secondary_group_size, uint8_t min_group_size, diplomat::span<const char32_t> digits) {
  auto result = diplomat::capi::icu4x_DataStruct_create_decimal_symbols_v1_mv1(plus_sign_prefix.data(),
    plus_sign_prefix.size(),
    plus_sign_suffix.data(),
    plus_sign_suffix.size(),
    minus_sign_prefix.data(),
    minus_sign_prefix.size(),
    minus_sign_suffix.data(),
    minus_sign_suffix.size(),
    decimal_separator.data(),
    decimal_separator.size(),
    grouping_separator.data(),
    grouping_separator.size(),
    primary_group_size,
    secondary_group_size,
    min_group_size,
    digits.data(),
    digits.size());
  return std::unique_ptr<DataStruct>(DataStruct::FromFFI(result));
}

inline const diplomat::capi::DataStruct* DataStruct::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::DataStruct*>(this);
}

inline diplomat::capi::DataStruct* DataStruct::AsFFI() {
  return reinterpret_cast<diplomat::capi::DataStruct*>(this);
}

inline const DataStruct* DataStruct::FromFFI(const diplomat::capi::DataStruct* ptr) {
  return reinterpret_cast<const DataStruct*>(ptr);
}

inline DataStruct* DataStruct::FromFFI(diplomat::capi::DataStruct* ptr) {
  return reinterpret_cast<DataStruct*>(ptr);
}

inline void DataStruct::operator delete(void* ptr) {
  diplomat::capi::icu4x_DataStruct_destroy_mv1(reinterpret_cast<diplomat::capi::DataStruct*>(ptr));
}


#endif // DataStruct_HPP
