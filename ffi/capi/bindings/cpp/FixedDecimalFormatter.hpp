#ifndef FixedDecimalFormatter_HPP
#define FixedDecimalFormatter_HPP

#include "FixedDecimalFormatter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "FixedDecimal.hpp"
#include "FixedDecimalGroupingStrategy.hpp"
#include "Locale.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_FixedDecimalFormatter_create_with_grouping_strategy_mv1_result {union {diplomat::capi::FixedDecimalFormatter* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_FixedDecimalFormatter_create_with_grouping_strategy_mv1_result;
    icu4x_FixedDecimalFormatter_create_with_grouping_strategy_mv1_result icu4x_FixedDecimalFormatter_create_with_grouping_strategy_mv1(const diplomat::capi::DataProvider* provider, const diplomat::capi::Locale* locale, diplomat::capi::FixedDecimalGroupingStrategy grouping_strategy);
    
    typedef struct icu4x_FixedDecimalFormatter_create_with_manual_data_mv1_result {union {diplomat::capi::FixedDecimalFormatter* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_FixedDecimalFormatter_create_with_manual_data_mv1_result;
    icu4x_FixedDecimalFormatter_create_with_manual_data_mv1_result icu4x_FixedDecimalFormatter_create_with_manual_data_mv1(const char* plus_sign_prefix_data, size_t plus_sign_prefix_len, const char* plus_sign_suffix_data, size_t plus_sign_suffix_len, const char* minus_sign_prefix_data, size_t minus_sign_prefix_len, const char* minus_sign_suffix_data, size_t minus_sign_suffix_len, const char* decimal_separator_data, size_t decimal_separator_len, const char* grouping_separator_data, size_t grouping_separator_len, uint8_t primary_group_size, uint8_t secondary_group_size, uint8_t min_group_size, const char32_t* digits_data, size_t digits_len, diplomat::capi::FixedDecimalGroupingStrategy grouping_strategy);
    
    void icu4x_FixedDecimalFormatter_format_mv1(const diplomat::capi::FixedDecimalFormatter* self, const diplomat::capi::FixedDecimal* value, diplomat::capi::DiplomatWrite* write);
    
    
    void icu4x_FixedDecimalFormatter_destroy_mv1(FixedDecimalFormatter* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<FixedDecimalFormatter>, DataError> FixedDecimalFormatter::create_with_grouping_strategy(const DataProvider& provider, const Locale& locale, FixedDecimalGroupingStrategy grouping_strategy) {
  auto result = diplomat::capi::icu4x_FixedDecimalFormatter_create_with_grouping_strategy_mv1(provider.AsFFI(),
    locale.AsFFI(),
    grouping_strategy.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<FixedDecimalFormatter>, DataError>(diplomat::Ok<std::unique_ptr<FixedDecimalFormatter>>(std::unique_ptr<FixedDecimalFormatter>(FixedDecimalFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<FixedDecimalFormatter>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<FixedDecimalFormatter>, DataError> FixedDecimalFormatter::create_with_manual_data(std::string_view plus_sign_prefix, std::string_view plus_sign_suffix, std::string_view minus_sign_prefix, std::string_view minus_sign_suffix, std::string_view decimal_separator, std::string_view grouping_separator, uint8_t primary_group_size, uint8_t secondary_group_size, uint8_t min_group_size, diplomat::span<const char32_t> digits, FixedDecimalGroupingStrategy grouping_strategy) {
  auto result = diplomat::capi::icu4x_FixedDecimalFormatter_create_with_manual_data_mv1(plus_sign_prefix.data(),
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
    digits.size(),
    grouping_strategy.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<FixedDecimalFormatter>, DataError>(diplomat::Ok<std::unique_ptr<FixedDecimalFormatter>>(std::unique_ptr<FixedDecimalFormatter>(FixedDecimalFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<FixedDecimalFormatter>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline std::string FixedDecimalFormatter::format(const FixedDecimal& value) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::icu4x_FixedDecimalFormatter_format_mv1(this->AsFFI(),
    value.AsFFI(),
    &write);
  return output;
}

inline const diplomat::capi::FixedDecimalFormatter* FixedDecimalFormatter::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::FixedDecimalFormatter*>(this);
}

inline diplomat::capi::FixedDecimalFormatter* FixedDecimalFormatter::AsFFI() {
  return reinterpret_cast<diplomat::capi::FixedDecimalFormatter*>(this);
}

inline const FixedDecimalFormatter* FixedDecimalFormatter::FromFFI(const diplomat::capi::FixedDecimalFormatter* ptr) {
  return reinterpret_cast<const FixedDecimalFormatter*>(ptr);
}

inline FixedDecimalFormatter* FixedDecimalFormatter::FromFFI(diplomat::capi::FixedDecimalFormatter* ptr) {
  return reinterpret_cast<FixedDecimalFormatter*>(ptr);
}

inline void FixedDecimalFormatter::operator delete(void* ptr) {
  diplomat::capi::icu4x_FixedDecimalFormatter_destroy_mv1(reinterpret_cast<diplomat::capi::FixedDecimalFormatter*>(ptr));
}


#endif // FixedDecimalFormatter_HPP
