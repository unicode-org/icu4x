#ifndef ListFormatter_HPP
#define ListFormatter_HPP

#include "ListFormatter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "ListLength.hpp"
#include "Locale.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_ListFormatter_create_and_with_length_mv1_result {union {diplomat::capi::ListFormatter* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_ListFormatter_create_and_with_length_mv1_result;
    icu4x_ListFormatter_create_and_with_length_mv1_result icu4x_ListFormatter_create_and_with_length_mv1(const diplomat::capi::DataProvider* provider, const diplomat::capi::Locale* locale, diplomat::capi::ListLength length);
    
    typedef struct icu4x_ListFormatter_create_or_with_length_mv1_result {union {diplomat::capi::ListFormatter* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_ListFormatter_create_or_with_length_mv1_result;
    icu4x_ListFormatter_create_or_with_length_mv1_result icu4x_ListFormatter_create_or_with_length_mv1(const diplomat::capi::DataProvider* provider, const diplomat::capi::Locale* locale, diplomat::capi::ListLength length);
    
    typedef struct icu4x_ListFormatter_create_unit_with_length_mv1_result {union {diplomat::capi::ListFormatter* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_ListFormatter_create_unit_with_length_mv1_result;
    icu4x_ListFormatter_create_unit_with_length_mv1_result icu4x_ListFormatter_create_unit_with_length_mv1(const diplomat::capi::DataProvider* provider, const diplomat::capi::Locale* locale, diplomat::capi::ListLength length);
    
    void icu4x_ListFormatter_format_utf8_mv1(const diplomat::capi::ListFormatter* self, diplomat::capi::DiplomatStringsView* list_data, size_t list_len, diplomat::capi::DiplomatWrite* write);
    
    void icu4x_ListFormatter_format_utf16_mv1(const diplomat::capi::ListFormatter* self, diplomat::capi::DiplomatStrings16View* list_data, size_t list_len, diplomat::capi::DiplomatWrite* write);
    
    
    void icu4x_ListFormatter_destroy_mv1(ListFormatter* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<ListFormatter>, DataError> ListFormatter::create_and_with_length(const DataProvider& provider, const Locale& locale, ListLength length) {
  auto result = diplomat::capi::icu4x_ListFormatter_create_and_with_length_mv1(provider.AsFFI(),
    locale.AsFFI(),
    length.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ListFormatter>, DataError>(diplomat::Ok<std::unique_ptr<ListFormatter>>(std::unique_ptr<ListFormatter>(ListFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ListFormatter>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ListFormatter>, DataError> ListFormatter::create_or_with_length(const DataProvider& provider, const Locale& locale, ListLength length) {
  auto result = diplomat::capi::icu4x_ListFormatter_create_or_with_length_mv1(provider.AsFFI(),
    locale.AsFFI(),
    length.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ListFormatter>, DataError>(diplomat::Ok<std::unique_ptr<ListFormatter>>(std::unique_ptr<ListFormatter>(ListFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ListFormatter>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ListFormatter>, DataError> ListFormatter::create_unit_with_length(const DataProvider& provider, const Locale& locale, ListLength length) {
  auto result = diplomat::capi::icu4x_ListFormatter_create_unit_with_length_mv1(provider.AsFFI(),
    locale.AsFFI(),
    length.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ListFormatter>, DataError>(diplomat::Ok<std::unique_ptr<ListFormatter>>(std::unique_ptr<ListFormatter>(ListFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ListFormatter>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline std::string ListFormatter::format(diplomat::span<const std::string_view> list) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::icu4x_ListFormatter_format_utf8_mv1(this->AsFFI(),
    list.data(),
    list.size(),
    &write);
  return output;
}

inline std::string ListFormatter::format16(diplomat::span<const std::u16string_view> list) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::icu4x_ListFormatter_format_utf16_mv1(this->AsFFI(),
    list.data(),
    list.size(),
    &write);
  return output;
}

inline const diplomat::capi::ListFormatter* ListFormatter::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::ListFormatter*>(this);
}

inline diplomat::capi::ListFormatter* ListFormatter::AsFFI() {
  return reinterpret_cast<diplomat::capi::ListFormatter*>(this);
}

inline const ListFormatter* ListFormatter::FromFFI(const diplomat::capi::ListFormatter* ptr) {
  return reinterpret_cast<const ListFormatter*>(ptr);
}

inline ListFormatter* ListFormatter::FromFFI(diplomat::capi::ListFormatter* ptr) {
  return reinterpret_cast<ListFormatter*>(ptr);
}

inline void ListFormatter::operator delete(void* ptr) {
  diplomat::capi::icu4x_ListFormatter_destroy_mv1(reinterpret_cast<diplomat::capi::ListFormatter*>(ptr));
}


#endif // ListFormatter_HPP
