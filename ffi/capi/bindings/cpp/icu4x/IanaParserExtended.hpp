#ifndef icu4x_IanaParserExtended_HPP
#define icu4x_IanaParserExtended_HPP

#include "IanaParserExtended.d.hpp"

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
    
    icu4x::capi::IanaParserExtended* icu4x_IanaParserExtended_create_mv1(void);
    
    typedef struct icu4x_IanaParserExtended_create_with_provider_mv1_result {union {icu4x::capi::IanaParserExtended* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_IanaParserExtended_create_with_provider_mv1_result;
    icu4x_IanaParserExtended_create_with_provider_mv1_result icu4x_IanaParserExtended_create_with_provider_mv1(const icu4x::capi::DataProvider* provider);
    
    typedef struct icu4x_IanaParserExtended_canonicalize_iana_mv1_result { bool is_ok;} icu4x_IanaParserExtended_canonicalize_iana_mv1_result;
    icu4x_IanaParserExtended_canonicalize_iana_mv1_result icu4x_IanaParserExtended_canonicalize_iana_mv1(const icu4x::capi::IanaParserExtended* self, diplomat::capi::DiplomatStringView value, diplomat::capi::DiplomatWrite* write);
    
    typedef struct icu4x_IanaParserExtended_canonical_iana_from_bcp47_mv1_result { bool is_ok;} icu4x_IanaParserExtended_canonical_iana_from_bcp47_mv1_result;
    icu4x_IanaParserExtended_canonical_iana_from_bcp47_mv1_result icu4x_IanaParserExtended_canonical_iana_from_bcp47_mv1(const icu4x::capi::IanaParserExtended* self, diplomat::capi::DiplomatStringView value, diplomat::capi::DiplomatWrite* write);
    
    
    void icu4x_IanaParserExtended_destroy_mv1(IanaParserExtended* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<icu4x::IanaParserExtended> icu4x::IanaParserExtended::create() {
  auto result = icu4x::capi::icu4x_IanaParserExtended_create_mv1();
  return std::unique_ptr<icu4x::IanaParserExtended>(icu4x::IanaParserExtended::FromFFI(result));
}

inline diplomat::result<std::unique_ptr<icu4x::IanaParserExtended>, icu4x::DataError> icu4x::IanaParserExtended::create_with_provider(const icu4x::DataProvider& provider) {
  auto result = icu4x::capi::icu4x_IanaParserExtended_create_with_provider_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::IanaParserExtended>, icu4x::DataError>(diplomat::Ok<std::unique_ptr<icu4x::IanaParserExtended>>(std::unique_ptr<icu4x::IanaParserExtended>(icu4x::IanaParserExtended::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::IanaParserExtended>, icu4x::DataError>(diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline diplomat::result<std::optional<std::string>, diplomat::Utf8Error> icu4x::IanaParserExtended::canonicalize_iana(std::string_view value) const {
  if (!diplomat::capi::diplomat_is_str(value.data(), value.size())) {
    return diplomat::Err<diplomat::Utf8Error>();
  }
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = icu4x::capi::icu4x_IanaParserExtended_canonicalize_iana_mv1(this->AsFFI(),
    {value.data(), value.size()},
    &write);
  return diplomat::Ok<std::optional<std::string>>(result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt);
}

inline std::optional<std::string> icu4x::IanaParserExtended::canonical_iana_from_bcp47(std::string_view value) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = icu4x::capi::icu4x_IanaParserExtended_canonical_iana_from_bcp47_mv1(this->AsFFI(),
    {value.data(), value.size()},
    &write);
  return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}

inline const icu4x::capi::IanaParserExtended* icu4x::IanaParserExtended::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::IanaParserExtended*>(this);
}

inline icu4x::capi::IanaParserExtended* icu4x::IanaParserExtended::AsFFI() {
  return reinterpret_cast<icu4x::capi::IanaParserExtended*>(this);
}

inline const icu4x::IanaParserExtended* icu4x::IanaParserExtended::FromFFI(const icu4x::capi::IanaParserExtended* ptr) {
  return reinterpret_cast<const icu4x::IanaParserExtended*>(ptr);
}

inline icu4x::IanaParserExtended* icu4x::IanaParserExtended::FromFFI(icu4x::capi::IanaParserExtended* ptr) {
  return reinterpret_cast<icu4x::IanaParserExtended*>(ptr);
}

inline void icu4x::IanaParserExtended::operator delete(void* ptr) {
  icu4x::capi::icu4x_IanaParserExtended_destroy_mv1(reinterpret_cast<icu4x::capi::IanaParserExtended*>(ptr));
}


#endif // icu4x_IanaParserExtended_HPP
