#ifndef Locale_HPP
#define Locale_HPP

#include "Locale.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "LocaleParseError.hpp"


namespace capi {
    extern "C" {
    
    typedef struct ICU4XLocale_create_from_string_result {union {Locale* ok; LocaleParseError err;}; bool is_ok;} ICU4XLocale_create_from_string_result;
    ICU4XLocale_create_from_string_result ICU4XLocale_create_from_string(const char* name_data, size_t name_len);
    
    Locale* ICU4XLocale_create_und();
    
    Locale* ICU4XLocale_clone(const Locale* self);
    
    void ICU4XLocale_basename(const Locale* self, DiplomatWrite* write);
    
    typedef struct ICU4XLocale_get_unicode_extension_result { bool is_ok;} ICU4XLocale_get_unicode_extension_result;
    ICU4XLocale_get_unicode_extension_result ICU4XLocale_get_unicode_extension(const Locale* self, const char* s_data, size_t s_len, DiplomatWrite* write);
    
    void ICU4XLocale_language(const Locale* self, DiplomatWrite* write);
    
    typedef struct ICU4XLocale_set_language_result {union { LocaleParseError err;}; bool is_ok;} ICU4XLocale_set_language_result;
    ICU4XLocale_set_language_result ICU4XLocale_set_language(Locale* self, const char* s_data, size_t s_len);
    
    typedef struct ICU4XLocale_region_result { bool is_ok;} ICU4XLocale_region_result;
    ICU4XLocale_region_result ICU4XLocale_region(const Locale* self, DiplomatWrite* write);
    
    typedef struct ICU4XLocale_set_region_result {union { LocaleParseError err;}; bool is_ok;} ICU4XLocale_set_region_result;
    ICU4XLocale_set_region_result ICU4XLocale_set_region(Locale* self, const char* s_data, size_t s_len);
    
    typedef struct ICU4XLocale_script_result { bool is_ok;} ICU4XLocale_script_result;
    ICU4XLocale_script_result ICU4XLocale_script(const Locale* self, DiplomatWrite* write);
    
    typedef struct ICU4XLocale_set_script_result {union { LocaleParseError err;}; bool is_ok;} ICU4XLocale_set_script_result;
    ICU4XLocale_set_script_result ICU4XLocale_set_script(Locale* self, const char* s_data, size_t s_len);
    
    typedef struct ICU4XLocale_canonicalize_result {union { LocaleParseError err;}; bool is_ok;} ICU4XLocale_canonicalize_result;
    ICU4XLocale_canonicalize_result ICU4XLocale_canonicalize(const char* s_data, size_t s_len, DiplomatWrite* write);
    
    void ICU4XLocale_to_string(const Locale* self, DiplomatWrite* write);
    
    bool ICU4XLocale_normalizing_eq(const Locale* self, const char* other_data, size_t other_len);
    
    int8_t ICU4XLocale_strict_cmp_(const Locale* self, const char* other_data, size_t other_len);
    
    int8_t ICU4XLocale_total_cmp_(const Locale* self, const Locale* other);
    
    
    void ICU4XLocale_destroy(Locale* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<Locale>, LocaleParseError> Locale::create_from_string(std::string_view name) {
  auto result = capi::ICU4XLocale_create_from_string(name.data(),
    name.size());
  return result.is_ok ? diplomat::result<std::unique_ptr<Locale>, LocaleParseError>(diplomat::Ok<std::unique_ptr<Locale>>(std::unique_ptr<Locale>(Locale::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<Locale>, LocaleParseError>(diplomat::Err<LocaleParseError>(LocaleParseError::FromFFI(result.err)));
}

inline std::unique_ptr<Locale> Locale::create_und() {
  auto result = capi::ICU4XLocale_create_und();
  return std::unique_ptr<Locale>(Locale::FromFFI(result));
}

inline std::unique_ptr<Locale> Locale::clone() const {
  auto result = capi::ICU4XLocale_clone(this->AsFFI());
  return std::unique_ptr<Locale>(Locale::FromFFI(result));
}

inline std::string Locale::basename() const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XLocale_basename(this->AsFFI(),
    &write);
  return output;
}

inline std::optional<std::string> Locale::get_unicode_extension(std::string_view s) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XLocale_get_unicode_extension(this->AsFFI(),
    s.data(),
    s.size(),
    &write);
  return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}

inline std::string Locale::language() const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XLocale_language(this->AsFFI(),
    &write);
  return output;
}

inline diplomat::result<std::monostate, LocaleParseError> Locale::set_language(std::string_view s) {
  auto result = capi::ICU4XLocale_set_language(this->AsFFI(),
    s.data(),
    s.size());
  return result.is_ok ? diplomat::result<std::monostate, LocaleParseError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, LocaleParseError>(diplomat::Err<LocaleParseError>(LocaleParseError::FromFFI(result.err)));
}

inline std::optional<std::string> Locale::region() const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XLocale_region(this->AsFFI(),
    &write);
  return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}

inline diplomat::result<std::monostate, LocaleParseError> Locale::set_region(std::string_view s) {
  auto result = capi::ICU4XLocale_set_region(this->AsFFI(),
    s.data(),
    s.size());
  return result.is_ok ? diplomat::result<std::monostate, LocaleParseError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, LocaleParseError>(diplomat::Err<LocaleParseError>(LocaleParseError::FromFFI(result.err)));
}

inline std::optional<std::string> Locale::script() const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XLocale_script(this->AsFFI(),
    &write);
  return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}

inline diplomat::result<std::monostate, LocaleParseError> Locale::set_script(std::string_view s) {
  auto result = capi::ICU4XLocale_set_script(this->AsFFI(),
    s.data(),
    s.size());
  return result.is_ok ? diplomat::result<std::monostate, LocaleParseError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, LocaleParseError>(diplomat::Err<LocaleParseError>(LocaleParseError::FromFFI(result.err)));
}

inline diplomat::result<std::string, LocaleParseError> Locale::canonicalize(std::string_view s) {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XLocale_canonicalize(s.data(),
    s.size(),
    &write);
  return result.is_ok ? diplomat::result<std::string, LocaleParseError>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, LocaleParseError>(diplomat::Err<LocaleParseError>(LocaleParseError::FromFFI(result.err)));
}

inline std::string Locale::to_string() const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XLocale_to_string(this->AsFFI(),
    &write);
  return output;
}

inline bool Locale::normalizing_eq(std::string_view other) const {
  auto result = capi::ICU4XLocale_normalizing_eq(this->AsFFI(),
    other.data(),
    other.size());
  return result;
}

inline int8_t Locale::strict_cmp_(std::string_view other) const {
  auto result = capi::ICU4XLocale_strict_cmp_(this->AsFFI(),
    other.data(),
    other.size());
  return result;
}

inline int8_t Locale::total_cmp_(const Locale& other) const {
  auto result = capi::ICU4XLocale_total_cmp_(this->AsFFI(),
    other.AsFFI());
  return result;
}

inline const capi::Locale* Locale::AsFFI() const {
  return reinterpret_cast<const capi::Locale*>(this);
}

inline capi::Locale* Locale::AsFFI() {
  return reinterpret_cast<capi::Locale*>(this);
}

inline const Locale* Locale::FromFFI(const capi::Locale* ptr) {
  return reinterpret_cast<const Locale*>(ptr);
}

inline Locale* Locale::FromFFI(capi::Locale* ptr) {
  return reinterpret_cast<Locale*>(ptr);
}

inline void Locale::operator delete(void* ptr) {
  capi::ICU4XLocale_destroy(reinterpret_cast<capi::Locale*>(ptr));
}


#endif // Locale_HPP
