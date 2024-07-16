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
    
    typedef struct ICU4XListFormatter_create_and_with_length_result {union {diplomat::capi::ListFormatter* ok; diplomat::capi::DataError err;}; bool is_ok;} ICU4XListFormatter_create_and_with_length_result;
    ICU4XListFormatter_create_and_with_length_result ICU4XListFormatter_create_and_with_length(const diplomat::capi::DataProvider* provider, const diplomat::capi::Locale* locale, diplomat::capi::ListLength length);
    
    typedef struct ICU4XListFormatter_create_or_with_length_result {union {diplomat::capi::ListFormatter* ok; diplomat::capi::DataError err;}; bool is_ok;} ICU4XListFormatter_create_or_with_length_result;
    ICU4XListFormatter_create_or_with_length_result ICU4XListFormatter_create_or_with_length(const diplomat::capi::DataProvider* provider, const diplomat::capi::Locale* locale, diplomat::capi::ListLength length);
    
    typedef struct ICU4XListFormatter_create_unit_with_length_result {union {diplomat::capi::ListFormatter* ok; diplomat::capi::DataError err;}; bool is_ok;} ICU4XListFormatter_create_unit_with_length_result;
    ICU4XListFormatter_create_unit_with_length_result ICU4XListFormatter_create_unit_with_length(const diplomat::capi::DataProvider* provider, const diplomat::capi::Locale* locale, diplomat::capi::ListLength length);
    
    void ICU4XListFormatter_format_valid_utf8(const diplomat::capi::ListFormatter* self, DiplomatStringsView* list_data, size_t list_len, diplomat::capi::DiplomatWrite* write);
    
    void ICU4XListFormatter_format_utf8(const diplomat::capi::ListFormatter* self, DiplomatStringsView* list_data, size_t list_len, diplomat::capi::DiplomatWrite* write);
    
    void ICU4XListFormatter_format_utf16(const diplomat::capi::ListFormatter* self, DiplomatStrings16View* list_data, size_t list_len, diplomat::capi::DiplomatWrite* write);
    
    
    void ICU4XListFormatter_destroy(ListFormatter* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<ListFormatter>, DataError> ListFormatter::create_and_with_length(const DataProvider& provider, const Locale& locale, ListLength length) {
  auto result = diplomat::capi::ICU4XListFormatter_create_and_with_length(provider.AsFFI(),
    locale.AsFFI(),
    length.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ListFormatter>, DataError>(diplomat::Ok<std::unique_ptr<ListFormatter>>(std::unique_ptr<ListFormatter>(ListFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ListFormatter>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ListFormatter>, DataError> ListFormatter::create_or_with_length(const DataProvider& provider, const Locale& locale, ListLength length) {
  auto result = diplomat::capi::ICU4XListFormatter_create_or_with_length(provider.AsFFI(),
    locale.AsFFI(),
    length.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ListFormatter>, DataError>(diplomat::Ok<std::unique_ptr<ListFormatter>>(std::unique_ptr<ListFormatter>(ListFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ListFormatter>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ListFormatter>, DataError> ListFormatter::create_unit_with_length(const DataProvider& provider, const Locale& locale, ListLength length) {
  auto result = diplomat::capi::ICU4XListFormatter_create_unit_with_length(provider.AsFFI(),
    locale.AsFFI(),
    length.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ListFormatter>, DataError>(diplomat::Ok<std::unique_ptr<ListFormatter>>(std::unique_ptr<ListFormatter>(ListFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ListFormatter>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline std::string ListFormatter::format_valid_utf8(diplomat::span<const std::string_view> list) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::ICU4XListFormatter_format_valid_utf8(this->AsFFI(),
    list.data(),
    list.size(),
    &write);
  return output;
}

inline std::string ListFormatter::format_utf8(diplomat::span<const std::string_view> list) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::ICU4XListFormatter_format_utf8(this->AsFFI(),
    list.data(),
    list.size(),
    &write);
  return output;
}

inline std::string ListFormatter::format_utf16(diplomat::span<const std::u16string_view> list) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::ICU4XListFormatter_format_utf16(this->AsFFI(),
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
  diplomat::capi::ICU4XListFormatter_destroy(reinterpret_cast<diplomat::capi::ListFormatter*>(ptr));
}


#endif // ListFormatter_HPP
