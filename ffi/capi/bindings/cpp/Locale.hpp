#ifndef Locale_HPP
#define Locale_HPP

#include "Locale.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "LocaleParseError.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_Locale_from_string_mv1_result {union {diplomat::capi::Locale* ok; diplomat::capi::LocaleParseError err;}; bool is_ok;} icu4x_Locale_from_string_mv1_result;
    icu4x_Locale_from_string_mv1_result icu4x_Locale_from_string_mv1(const char* name_data, size_t name_len);
    
    diplomat::capi::Locale* icu4x_Locale_und_mv1(void);
    
    diplomat::capi::Locale* icu4x_Locale_clone_mv1(const diplomat::capi::Locale* self);
    
    void icu4x_Locale_basename_mv1(const diplomat::capi::Locale* self, diplomat::capi::DiplomatWrite* write);
    
    typedef struct icu4x_Locale_get_unicode_extension_mv1_result { bool is_ok;} icu4x_Locale_get_unicode_extension_mv1_result;
    icu4x_Locale_get_unicode_extension_mv1_result icu4x_Locale_get_unicode_extension_mv1(const diplomat::capi::Locale* self, const char* s_data, size_t s_len, diplomat::capi::DiplomatWrite* write);
    
    void icu4x_Locale_language_mv1(const diplomat::capi::Locale* self, diplomat::capi::DiplomatWrite* write);
    
    typedef struct icu4x_Locale_set_language_mv1_result {union { diplomat::capi::LocaleParseError err;}; bool is_ok;} icu4x_Locale_set_language_mv1_result;
    icu4x_Locale_set_language_mv1_result icu4x_Locale_set_language_mv1(diplomat::capi::Locale* self, const char* s_data, size_t s_len);
    
    typedef struct icu4x_Locale_region_mv1_result { bool is_ok;} icu4x_Locale_region_mv1_result;
    icu4x_Locale_region_mv1_result icu4x_Locale_region_mv1(const diplomat::capi::Locale* self, diplomat::capi::DiplomatWrite* write);
    
    typedef struct icu4x_Locale_set_region_mv1_result {union { diplomat::capi::LocaleParseError err;}; bool is_ok;} icu4x_Locale_set_region_mv1_result;
    icu4x_Locale_set_region_mv1_result icu4x_Locale_set_region_mv1(diplomat::capi::Locale* self, const char* s_data, size_t s_len);
    
    typedef struct icu4x_Locale_script_mv1_result { bool is_ok;} icu4x_Locale_script_mv1_result;
    icu4x_Locale_script_mv1_result icu4x_Locale_script_mv1(const diplomat::capi::Locale* self, diplomat::capi::DiplomatWrite* write);
    
    typedef struct icu4x_Locale_set_script_mv1_result {union { diplomat::capi::LocaleParseError err;}; bool is_ok;} icu4x_Locale_set_script_mv1_result;
    icu4x_Locale_set_script_mv1_result icu4x_Locale_set_script_mv1(diplomat::capi::Locale* self, const char* s_data, size_t s_len);
    
    typedef struct icu4x_Locale_canonicalize_mv1_result {union { diplomat::capi::LocaleParseError err;}; bool is_ok;} icu4x_Locale_canonicalize_mv1_result;
    icu4x_Locale_canonicalize_mv1_result icu4x_Locale_canonicalize_mv1(const char* s_data, size_t s_len, diplomat::capi::DiplomatWrite* write);
    
    void icu4x_Locale_to_string_mv1(const diplomat::capi::Locale* self, diplomat::capi::DiplomatWrite* write);
    
    bool icu4x_Locale_normalizing_eq_mv1(const diplomat::capi::Locale* self, const char* other_data, size_t other_len);
    
    int8_t icu4x_Locale_compare_to_string_mv1(const diplomat::capi::Locale* self, const char* other_data, size_t other_len);
    
    int8_t icu4x_Locale_compare_to_mv1(const diplomat::capi::Locale* self, const diplomat::capi::Locale* other);
    
    
    void icu4x_Locale_destroy_mv1(Locale* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<Locale>, LocaleParseError> Locale::from_string(std::string_view name) {
  auto result = diplomat::capi::icu4x_Locale_from_string_mv1(name.data(),
    name.size());
  return result.is_ok ? diplomat::result<std::unique_ptr<Locale>, LocaleParseError>(diplomat::Ok<std::unique_ptr<Locale>>(std::unique_ptr<Locale>(Locale::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<Locale>, LocaleParseError>(diplomat::Err<LocaleParseError>(LocaleParseError::FromFFI(result.err)));
}

inline std::unique_ptr<Locale> Locale::und() {
  auto result = diplomat::capi::icu4x_Locale_und_mv1();
  return std::unique_ptr<Locale>(Locale::FromFFI(result));
}

inline std::unique_ptr<Locale> Locale::clone() const {
  auto result = diplomat::capi::icu4x_Locale_clone_mv1(this->AsFFI());
  return std::unique_ptr<Locale>(Locale::FromFFI(result));
}

inline std::string Locale::basename() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::icu4x_Locale_basename_mv1(this->AsFFI(),
    &write);
  return output;
}

inline std::optional<std::string> Locale::get_unicode_extension(std::string_view s) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::icu4x_Locale_get_unicode_extension_mv1(this->AsFFI(),
    s.data(),
    s.size(),
    &write);
  return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}

inline std::string Locale::language() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::icu4x_Locale_language_mv1(this->AsFFI(),
    &write);
  return output;
}

inline diplomat::result<std::monostate, LocaleParseError> Locale::set_language(std::string_view s) {
  auto result = diplomat::capi::icu4x_Locale_set_language_mv1(this->AsFFI(),
    s.data(),
    s.size());
  return result.is_ok ? diplomat::result<std::monostate, LocaleParseError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, LocaleParseError>(diplomat::Err<LocaleParseError>(LocaleParseError::FromFFI(result.err)));
}

inline std::optional<std::string> Locale::region() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::icu4x_Locale_region_mv1(this->AsFFI(),
    &write);
  return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}

inline diplomat::result<std::monostate, LocaleParseError> Locale::set_region(std::string_view s) {
  auto result = diplomat::capi::icu4x_Locale_set_region_mv1(this->AsFFI(),
    s.data(),
    s.size());
  return result.is_ok ? diplomat::result<std::monostate, LocaleParseError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, LocaleParseError>(diplomat::Err<LocaleParseError>(LocaleParseError::FromFFI(result.err)));
}

inline std::optional<std::string> Locale::script() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::icu4x_Locale_script_mv1(this->AsFFI(),
    &write);
  return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}

inline diplomat::result<std::monostate, LocaleParseError> Locale::set_script(std::string_view s) {
  auto result = diplomat::capi::icu4x_Locale_set_script_mv1(this->AsFFI(),
    s.data(),
    s.size());
  return result.is_ok ? diplomat::result<std::monostate, LocaleParseError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, LocaleParseError>(diplomat::Err<LocaleParseError>(LocaleParseError::FromFFI(result.err)));
}

inline diplomat::result<std::string, LocaleParseError> Locale::canonicalize(std::string_view s) {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::icu4x_Locale_canonicalize_mv1(s.data(),
    s.size(),
    &write);
  return result.is_ok ? diplomat::result<std::string, LocaleParseError>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, LocaleParseError>(diplomat::Err<LocaleParseError>(LocaleParseError::FromFFI(result.err)));
}

inline std::string Locale::to_string() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::icu4x_Locale_to_string_mv1(this->AsFFI(),
    &write);
  return output;
}

inline bool Locale::normalizing_eq(std::string_view other) const {
  auto result = diplomat::capi::icu4x_Locale_normalizing_eq_mv1(this->AsFFI(),
    other.data(),
    other.size());
  return result;
}

inline int8_t Locale::compare_to_string(std::string_view other) const {
  auto result = diplomat::capi::icu4x_Locale_compare_to_string_mv1(this->AsFFI(),
    other.data(),
    other.size());
  return result;
}

inline int8_t Locale::compare_to(const Locale& other) const {
  auto result = diplomat::capi::icu4x_Locale_compare_to_mv1(this->AsFFI(),
    other.AsFFI());
  return result;
}

inline const diplomat::capi::Locale* Locale::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::Locale*>(this);
}

inline diplomat::capi::Locale* Locale::AsFFI() {
  return reinterpret_cast<diplomat::capi::Locale*>(this);
}

inline const Locale* Locale::FromFFI(const diplomat::capi::Locale* ptr) {
  return reinterpret_cast<const Locale*>(ptr);
}

inline Locale* Locale::FromFFI(diplomat::capi::Locale* ptr) {
  return reinterpret_cast<Locale*>(ptr);
}

inline void Locale::operator delete(void* ptr) {
  diplomat::capi::icu4x_Locale_destroy_mv1(reinterpret_cast<diplomat::capi::Locale*>(ptr));
}


#endif // Locale_HPP
