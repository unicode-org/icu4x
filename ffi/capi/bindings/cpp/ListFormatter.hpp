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


namespace capi {
    extern "C" {
    
    typedef struct ICU4XListFormatter_create_and_with_length_result {union {ListFormatter* ok; DataError err;}; bool is_ok;} ICU4XListFormatter_create_and_with_length_result;
    ICU4XListFormatter_create_and_with_length_result ICU4XListFormatter_create_and_with_length(const DataProvider* provider, const Locale* locale, ListLength length);
    
    typedef struct ICU4XListFormatter_create_or_with_length_result {union {ListFormatter* ok; DataError err;}; bool is_ok;} ICU4XListFormatter_create_or_with_length_result;
    ICU4XListFormatter_create_or_with_length_result ICU4XListFormatter_create_or_with_length(const DataProvider* provider, const Locale* locale, ListLength length);
    
    typedef struct ICU4XListFormatter_create_unit_with_length_result {union {ListFormatter* ok; DataError err;}; bool is_ok;} ICU4XListFormatter_create_unit_with_length_result;
    ICU4XListFormatter_create_unit_with_length_result ICU4XListFormatter_create_unit_with_length(const DataProvider* provider, const Locale* locale, ListLength length);
    
    void ICU4XListFormatter_format_valid_utf8(const ListFormatter* self, DiplomatStringsView* list_data, size_t list_len, DiplomatWrite* write);
    
    void ICU4XListFormatter_format_utf8(const ListFormatter* self, DiplomatStringsView* list_data, size_t list_len, DiplomatWrite* write);
    
    void ICU4XListFormatter_format_utf16(const ListFormatter* self, DiplomatStrings16View* list_data, size_t list_len, DiplomatWrite* write);
    
    
    void ICU4XListFormatter_destroy(ListFormatter* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<ListFormatter>, DataError> ListFormatter::create_and_with_length(const DataProvider& provider, const Locale& locale, ListLength length) {
  auto result = capi::ICU4XListFormatter_create_and_with_length(provider.AsFFI(),
    locale.AsFFI(),
    length.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ListFormatter>, DataError>(diplomat::Ok<std::unique_ptr<ListFormatter>>(std::unique_ptr<ListFormatter>(ListFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ListFormatter>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ListFormatter>, DataError> ListFormatter::create_or_with_length(const DataProvider& provider, const Locale& locale, ListLength length) {
  auto result = capi::ICU4XListFormatter_create_or_with_length(provider.AsFFI(),
    locale.AsFFI(),
    length.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ListFormatter>, DataError>(diplomat::Ok<std::unique_ptr<ListFormatter>>(std::unique_ptr<ListFormatter>(ListFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ListFormatter>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ListFormatter>, DataError> ListFormatter::create_unit_with_length(const DataProvider& provider, const Locale& locale, ListLength length) {
  auto result = capi::ICU4XListFormatter_create_unit_with_length(provider.AsFFI(),
    locale.AsFFI(),
    length.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ListFormatter>, DataError>(diplomat::Ok<std::unique_ptr<ListFormatter>>(std::unique_ptr<ListFormatter>(ListFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ListFormatter>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline std::string ListFormatter::format_valid_utf8(diplomat::span<const std::string_view> list) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XListFormatter_format_valid_utf8(this->AsFFI(),
    list.data(),
    list.size(),
    &write);
  return output;
}

inline std::string ListFormatter::format_utf8(diplomat::span<const std::string_view> list) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XListFormatter_format_utf8(this->AsFFI(),
    list.data(),
    list.size(),
    &write);
  return output;
}

inline std::string ListFormatter::format_utf16(diplomat::span<const std::u16string_view> list) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XListFormatter_format_utf16(this->AsFFI(),
    list.data(),
    list.size(),
    &write);
  return output;
}

inline const capi::ListFormatter* ListFormatter::AsFFI() const {
  return reinterpret_cast<const capi::ListFormatter*>(this);
}

inline capi::ListFormatter* ListFormatter::AsFFI() {
  return reinterpret_cast<capi::ListFormatter*>(this);
}

inline const ListFormatter* ListFormatter::FromFFI(const capi::ListFormatter* ptr) {
  return reinterpret_cast<const ListFormatter*>(ptr);
}

inline ListFormatter* ListFormatter::FromFFI(capi::ListFormatter* ptr) {
  return reinterpret_cast<ListFormatter*>(ptr);
}

inline void ListFormatter::operator delete(void* ptr) {
  capi::ICU4XListFormatter_destroy(reinterpret_cast<capi::ListFormatter*>(ptr));
}


#endif // ListFormatter_HPP
