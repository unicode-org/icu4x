#ifndef ICU4XDataStruct_D_HPP
#define ICU4XDataStruct_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataStruct.d.h"


class ICU4XDataStruct {
public:

  inline static std::unique_ptr<ICU4XDataStruct> create_decimal_symbols_v1(std::string_view plus_sign_prefix, std::string_view plus_sign_suffix, std::string_view minus_sign_prefix, std::string_view minus_sign_suffix, std::string_view decimal_separator, std::string_view grouping_separator, uint8_t primary_group_size, uint8_t secondary_group_size, uint8_t min_group_size, diplomat::span<const char32_t> digits);

  inline const capi::ICU4XDataStruct* AsFFI() const;
  inline capi::ICU4XDataStruct* AsFFI();
  inline static const ICU4XDataStruct* FromFFI(const capi::ICU4XDataStruct* ptr);
  inline static ICU4XDataStruct* FromFFI(capi::ICU4XDataStruct* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XDataStruct() = delete;
  ICU4XDataStruct(const ICU4XDataStruct&) = delete;
  ICU4XDataStruct(ICU4XDataStruct&&) noexcept = delete;
  ICU4XDataStruct operator=(const ICU4XDataStruct&) = delete;
  ICU4XDataStruct operator=(ICU4XDataStruct&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XDataStruct_D_HPP
