#ifndef ICU4XDataStruct_HPP
#define ICU4XDataStruct_HPP

#include "ICU4XDataStruct.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataStruct.h"


inline std::unique_ptr<ICU4XDataStruct> ICU4XDataStruct::create_decimal_symbols_v1(std::string_view plus_sign_prefix, std::string_view plus_sign_suffix, std::string_view minus_sign_prefix, std::string_view minus_sign_suffix, std::string_view decimal_separator, std::string_view grouping_separator, uint8_t primary_group_size, uint8_t secondary_group_size, uint8_t min_group_size, diplomat::span<const char32_t> digits) {
  auto result = capi::ICU4XDataStruct_create_decimal_symbols_v1(plus_sign_prefix.data(),
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
  return std::unique_ptr<ICU4XDataStruct>(ICU4XDataStruct::FromFFI(result));
}

inline const capi::ICU4XDataStruct* ICU4XDataStruct::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XDataStruct*>(this);
}

inline capi::ICU4XDataStruct* ICU4XDataStruct::AsFFI() {
  return reinterpret_cast<capi::ICU4XDataStruct*>(this);
}

inline const ICU4XDataStruct* ICU4XDataStruct::FromFFI(const capi::ICU4XDataStruct* ptr) {
  return reinterpret_cast<const ICU4XDataStruct*>(ptr);
}

inline ICU4XDataStruct* ICU4XDataStruct::FromFFI(capi::ICU4XDataStruct* ptr) {
  return reinterpret_cast<ICU4XDataStruct*>(ptr);
}

inline void ICU4XDataStruct::operator delete(void* ptr) {
  capi::ICU4XDataStruct_destroy(reinterpret_cast<capi::ICU4XDataStruct*>(ptr));
}


#endif // ICU4XDataStruct_HPP
