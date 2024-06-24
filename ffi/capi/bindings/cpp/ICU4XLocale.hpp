#ifndef ICU4XLocale_HPP
#define ICU4XLocale_HPP

#include "ICU4XLocale.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XLocaleParseError.hpp"


namespace capi {
    extern "C" {
    
    typedef struct ICU4XLocale_create_from_string_result {union {ICU4XLocale* ok; ICU4XLocaleParseError err;}; bool is_ok;} ICU4XLocale_create_from_string_result;
    ICU4XLocale_create_from_string_result ICU4XLocale_create_from_string(const char* name_data, size_t name_len);
    
    ICU4XLocale* ICU4XLocale_create_und();
    
    ICU4XLocale* ICU4XLocale_clone(const ICU4XLocale* self);
    
    void ICU4XLocale_basename(const ICU4XLocale* self, DiplomatWrite* write);
    
    typedef struct ICU4XLocale_get_unicode_extension_result { bool is_ok;} ICU4XLocale_get_unicode_extension_result;
    ICU4XLocale_get_unicode_extension_result ICU4XLocale_get_unicode_extension(const ICU4XLocale* self, const char* s_data, size_t s_len, DiplomatWrite* write);
    
    void ICU4XLocale_language(const ICU4XLocale* self, DiplomatWrite* write);
    
    typedef struct ICU4XLocale_set_language_result {union { ICU4XLocaleParseError err;}; bool is_ok;} ICU4XLocale_set_language_result;
    ICU4XLocale_set_language_result ICU4XLocale_set_language(ICU4XLocale* self, const char* s_data, size_t s_len);
    
    typedef struct ICU4XLocale_region_result { bool is_ok;} ICU4XLocale_region_result;
    ICU4XLocale_region_result ICU4XLocale_region(const ICU4XLocale* self, DiplomatWrite* write);
    
    typedef struct ICU4XLocale_set_region_result {union { ICU4XLocaleParseError err;}; bool is_ok;} ICU4XLocale_set_region_result;
    ICU4XLocale_set_region_result ICU4XLocale_set_region(ICU4XLocale* self, const char* s_data, size_t s_len);
    
    typedef struct ICU4XLocale_script_result { bool is_ok;} ICU4XLocale_script_result;
    ICU4XLocale_script_result ICU4XLocale_script(const ICU4XLocale* self, DiplomatWrite* write);
    
    typedef struct ICU4XLocale_set_script_result {union { ICU4XLocaleParseError err;}; bool is_ok;} ICU4XLocale_set_script_result;
    ICU4XLocale_set_script_result ICU4XLocale_set_script(ICU4XLocale* self, const char* s_data, size_t s_len);
    
    typedef struct ICU4XLocale_canonicalize_result {union { ICU4XLocaleParseError err;}; bool is_ok;} ICU4XLocale_canonicalize_result;
    ICU4XLocale_canonicalize_result ICU4XLocale_canonicalize(const char* s_data, size_t s_len, DiplomatWrite* write);
    
    void ICU4XLocale_to_string(const ICU4XLocale* self, DiplomatWrite* write);
    
    bool ICU4XLocale_normalizing_eq(const ICU4XLocale* self, const char* other_data, size_t other_len);
    
    int8_t ICU4XLocale_strict_cmp_(const ICU4XLocale* self, const char* other_data, size_t other_len);
    
    int8_t ICU4XLocale_total_cmp_(const ICU4XLocale* self, const ICU4XLocale* other);
    
    
    void ICU4XLocale_destroy(ICU4XLocale* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<ICU4XLocale>, ICU4XLocaleParseError> ICU4XLocale::create_from_string(std::string_view name) {
  auto result = capi::ICU4XLocale_create_from_string(name.data(),
    name.size());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XLocale>, ICU4XLocaleParseError>(diplomat::Ok<std::unique_ptr<ICU4XLocale>>(std::unique_ptr<ICU4XLocale>(ICU4XLocale::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XLocale>, ICU4XLocaleParseError>(diplomat::Err<ICU4XLocaleParseError>(ICU4XLocaleParseError::FromFFI(result.err)));
}

inline std::unique_ptr<ICU4XLocale> ICU4XLocale::create_und() {
  auto result = capi::ICU4XLocale_create_und();
  return std::unique_ptr<ICU4XLocale>(ICU4XLocale::FromFFI(result));
}

inline std::unique_ptr<ICU4XLocale> ICU4XLocale::clone() const {
  auto result = capi::ICU4XLocale_clone(this->AsFFI());
  return std::unique_ptr<ICU4XLocale>(ICU4XLocale::FromFFI(result));
}

inline std::string ICU4XLocale::basename() const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XLocale_basename(this->AsFFI(),
    &write);
  return output;
}

inline std::optional<std::string> ICU4XLocale::get_unicode_extension(std::string_view s) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XLocale_get_unicode_extension(this->AsFFI(),
    s.data(),
    s.size(),
    &write);
  return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}

inline std::string ICU4XLocale::language() const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XLocale_language(this->AsFFI(),
    &write);
  return output;
}

inline diplomat::result<std::monostate, ICU4XLocaleParseError> ICU4XLocale::set_language(std::string_view s) {
  auto result = capi::ICU4XLocale_set_language(this->AsFFI(),
    s.data(),
    s.size());
  return result.is_ok ? diplomat::result<std::monostate, ICU4XLocaleParseError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, ICU4XLocaleParseError>(diplomat::Err<ICU4XLocaleParseError>(ICU4XLocaleParseError::FromFFI(result.err)));
}

inline std::optional<std::string> ICU4XLocale::region() const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XLocale_region(this->AsFFI(),
    &write);
  return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}

inline diplomat::result<std::monostate, ICU4XLocaleParseError> ICU4XLocale::set_region(std::string_view s) {
  auto result = capi::ICU4XLocale_set_region(this->AsFFI(),
    s.data(),
    s.size());
  return result.is_ok ? diplomat::result<std::monostate, ICU4XLocaleParseError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, ICU4XLocaleParseError>(diplomat::Err<ICU4XLocaleParseError>(ICU4XLocaleParseError::FromFFI(result.err)));
}

inline std::optional<std::string> ICU4XLocale::script() const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XLocale_script(this->AsFFI(),
    &write);
  return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}

inline diplomat::result<std::monostate, ICU4XLocaleParseError> ICU4XLocale::set_script(std::string_view s) {
  auto result = capi::ICU4XLocale_set_script(this->AsFFI(),
    s.data(),
    s.size());
  return result.is_ok ? diplomat::result<std::monostate, ICU4XLocaleParseError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, ICU4XLocaleParseError>(diplomat::Err<ICU4XLocaleParseError>(ICU4XLocaleParseError::FromFFI(result.err)));
}

inline diplomat::result<std::string, ICU4XLocaleParseError> ICU4XLocale::canonicalize(std::string_view s) {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XLocale_canonicalize(s.data(),
    s.size(),
    &write);
  return result.is_ok ? diplomat::result<std::string, ICU4XLocaleParseError>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, ICU4XLocaleParseError>(diplomat::Err<ICU4XLocaleParseError>(ICU4XLocaleParseError::FromFFI(result.err)));
}

inline std::string ICU4XLocale::to_string() const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XLocale_to_string(this->AsFFI(),
    &write);
  return output;
}

inline bool ICU4XLocale::normalizing_eq(std::string_view other) const {
  auto result = capi::ICU4XLocale_normalizing_eq(this->AsFFI(),
    other.data(),
    other.size());
  return result;
}

inline int8_t ICU4XLocale::strict_cmp_(std::string_view other) const {
  auto result = capi::ICU4XLocale_strict_cmp_(this->AsFFI(),
    other.data(),
    other.size());
  return result;
}

inline int8_t ICU4XLocale::total_cmp_(const ICU4XLocale& other) const {
  auto result = capi::ICU4XLocale_total_cmp_(this->AsFFI(),
    other.AsFFI());
  return result;
}

inline const capi::ICU4XLocale* ICU4XLocale::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XLocale*>(this);
}

inline capi::ICU4XLocale* ICU4XLocale::AsFFI() {
  return reinterpret_cast<capi::ICU4XLocale*>(this);
}

inline const ICU4XLocale* ICU4XLocale::FromFFI(const capi::ICU4XLocale* ptr) {
  return reinterpret_cast<const ICU4XLocale*>(ptr);
}

inline ICU4XLocale* ICU4XLocale::FromFFI(capi::ICU4XLocale* ptr) {
  return reinterpret_cast<ICU4XLocale*>(ptr);
}

inline void ICU4XLocale::operator delete(void* ptr) {
  capi::ICU4XLocale_destroy(reinterpret_cast<capi::ICU4XLocale*>(ptr));
}


#endif // ICU4XLocale_HPP
