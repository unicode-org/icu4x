#ifndef icu4x_IanaParser_HPP
#define icu4x_IanaParser_HPP

#include "IanaParser.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../diplomat_runtime.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    icu4x::capi::IanaParser* icu4x_IanaParser_create_mv1(void);
    
    typedef struct icu4x_IanaParser_create_with_provider_mv1_result {union {icu4x::capi::IanaParser* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_IanaParser_create_with_provider_mv1_result;
    icu4x_IanaParser_create_with_provider_mv1_result icu4x_IanaParser_create_with_provider_mv1(const icu4x::capi::DataProvider* provider);
    
    void icu4x_IanaParser_iana_to_bcp47_mv1(const icu4x::capi::IanaParser* self, diplomat::capi::DiplomatStringView value, diplomat::capi::DiplomatWrite* write);
    
    typedef struct icu4x_IanaParser_normalize_iana_mv1_result { bool is_ok;} icu4x_IanaParser_normalize_iana_mv1_result;
    icu4x_IanaParser_normalize_iana_mv1_result icu4x_IanaParser_normalize_iana_mv1(const icu4x::capi::IanaParser* self, diplomat::capi::DiplomatStringView value, diplomat::capi::DiplomatWrite* write);
    
    typedef struct icu4x_IanaParser_canonicalize_iana_mv1_result { bool is_ok;} icu4x_IanaParser_canonicalize_iana_mv1_result;
    icu4x_IanaParser_canonicalize_iana_mv1_result icu4x_IanaParser_canonicalize_iana_mv1(const icu4x::capi::IanaParser* self, diplomat::capi::DiplomatStringView value, diplomat::capi::DiplomatWrite* write);
    
    typedef struct icu4x_IanaParser_find_canonical_iana_from_bcp47_mv1_result { bool is_ok;} icu4x_IanaParser_find_canonical_iana_from_bcp47_mv1_result;
    icu4x_IanaParser_find_canonical_iana_from_bcp47_mv1_result icu4x_IanaParser_find_canonical_iana_from_bcp47_mv1(const icu4x::capi::IanaParser* self, diplomat::capi::DiplomatStringView value, diplomat::capi::DiplomatWrite* write);
    
    
    void icu4x_IanaParser_destroy_mv1(IanaParser* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<icu4x::IanaParser> icu4x::IanaParser::create() {
  auto result = icu4x::capi::icu4x_IanaParser_create_mv1();
  return std::unique_ptr<icu4x::IanaParser>(icu4x::IanaParser::FromFFI(result));
}

inline diplomat::result<std::unique_ptr<icu4x::IanaParser>, icu4x::DataError> icu4x::IanaParser::create_with_provider(const icu4x::DataProvider& provider) {
  auto result = icu4x::capi::icu4x_IanaParser_create_with_provider_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::IanaParser>, icu4x::DataError>(diplomat::Ok<std::unique_ptr<icu4x::IanaParser>>(std::unique_ptr<icu4x::IanaParser>(icu4x::IanaParser::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::IanaParser>, icu4x::DataError>(diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline std::string icu4x::IanaParser::iana_to_bcp47(std::string_view value) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  icu4x::capi::icu4x_IanaParser_iana_to_bcp47_mv1(this->AsFFI(),
    {value.data(), value.size()},
    &write);
  return output;
}

inline diplomat::result<std::optional<std::string>, diplomat::Utf8Error> icu4x::IanaParser::normalize_iana(std::string_view value) const {
  if (!diplomat::capi::diplomat_is_str(value.data(), value.size())) {
    return diplomat::Err<diplomat::Utf8Error>();
  }
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = icu4x::capi::icu4x_IanaParser_normalize_iana_mv1(this->AsFFI(),
    {value.data(), value.size()},
    &write);
  return diplomat::Ok<std::optional<std::string>>(result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt);
}

inline diplomat::result<std::optional<std::string>, diplomat::Utf8Error> icu4x::IanaParser::canonicalize_iana(std::string_view value) const {
  if (!diplomat::capi::diplomat_is_str(value.data(), value.size())) {
    return diplomat::Err<diplomat::Utf8Error>();
  }
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = icu4x::capi::icu4x_IanaParser_canonicalize_iana_mv1(this->AsFFI(),
    {value.data(), value.size()},
    &write);
  return diplomat::Ok<std::optional<std::string>>(result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt);
}

inline std::optional<std::string> icu4x::IanaParser::find_canonical_iana_from_bcp47(std::string_view value) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = icu4x::capi::icu4x_IanaParser_find_canonical_iana_from_bcp47_mv1(this->AsFFI(),
    {value.data(), value.size()},
    &write);
  return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}

inline const icu4x::capi::IanaParser* icu4x::IanaParser::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::IanaParser*>(this);
}

inline icu4x::capi::IanaParser* icu4x::IanaParser::AsFFI() {
  return reinterpret_cast<icu4x::capi::IanaParser*>(this);
}

inline const icu4x::IanaParser* icu4x::IanaParser::FromFFI(const icu4x::capi::IanaParser* ptr) {
  return reinterpret_cast<const icu4x::IanaParser*>(ptr);
}

inline icu4x::IanaParser* icu4x::IanaParser::FromFFI(icu4x::capi::IanaParser* ptr) {
  return reinterpret_cast<icu4x::IanaParser*>(ptr);
}

inline void icu4x::IanaParser::operator delete(void* ptr) {
  icu4x::capi::icu4x_IanaParser_destroy_mv1(reinterpret_cast<icu4x::capi::IanaParser*>(ptr));
}


#endif // icu4x_IanaParser_HPP
