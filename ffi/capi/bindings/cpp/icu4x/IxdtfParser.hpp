#ifndef icu4x_IxdtfParser_HPP
#define icu4x_IxdtfParser_HPP

#include "IxdtfParser.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"
#include "Calendar.hpp"
#include "CalendarParseError.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "ZonedDateTime.hpp"
#include "ZonedIsoDateTime.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    icu4x::capi::IxdtfParser* icu4x_IxdtfParser_create_mv1(void);
    
    typedef struct icu4x_IxdtfParser_create_with_provider_mv1_result {union {icu4x::capi::IxdtfParser* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_IxdtfParser_create_with_provider_mv1_result;
    icu4x_IxdtfParser_create_with_provider_mv1_result icu4x_IxdtfParser_create_with_provider_mv1(const icu4x::capi::DataProvider* provider);
    
    typedef struct icu4x_IxdtfParser_try_iso_from_str_mv1_result {union {icu4x::capi::ZonedIsoDateTime ok; icu4x::capi::CalendarParseError err;}; bool is_ok;} icu4x_IxdtfParser_try_iso_from_str_mv1_result;
    icu4x_IxdtfParser_try_iso_from_str_mv1_result icu4x_IxdtfParser_try_iso_from_str_mv1(const icu4x::capi::IxdtfParser* self, diplomat::capi::DiplomatStringView v);
    
    typedef struct icu4x_IxdtfParser_try_from_str_mv1_result {union {icu4x::capi::ZonedDateTime ok; icu4x::capi::CalendarParseError err;}; bool is_ok;} icu4x_IxdtfParser_try_from_str_mv1_result;
    icu4x_IxdtfParser_try_from_str_mv1_result icu4x_IxdtfParser_try_from_str_mv1(const icu4x::capi::IxdtfParser* self, diplomat::capi::DiplomatStringView v, const icu4x::capi::Calendar* calendar);
    
    
    void icu4x_IxdtfParser_destroy_mv1(IxdtfParser* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<icu4x::IxdtfParser> icu4x::IxdtfParser::create() {
  auto result = icu4x::capi::icu4x_IxdtfParser_create_mv1();
  return std::unique_ptr<icu4x::IxdtfParser>(icu4x::IxdtfParser::FromFFI(result));
}

inline diplomat::result<std::unique_ptr<icu4x::IxdtfParser>, icu4x::DataError> icu4x::IxdtfParser::create_with_provider(const icu4x::DataProvider& provider) {
  auto result = icu4x::capi::icu4x_IxdtfParser_create_with_provider_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::IxdtfParser>, icu4x::DataError>(diplomat::Ok<std::unique_ptr<icu4x::IxdtfParser>>(std::unique_ptr<icu4x::IxdtfParser>(icu4x::IxdtfParser::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::IxdtfParser>, icu4x::DataError>(diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline diplomat::result<icu4x::ZonedIsoDateTime, icu4x::CalendarParseError> icu4x::IxdtfParser::try_iso_from_str(std::string_view v) const {
  auto result = icu4x::capi::icu4x_IxdtfParser_try_iso_from_str_mv1(this->AsFFI(),
    {v.data(), v.size()});
  return result.is_ok ? diplomat::result<icu4x::ZonedIsoDateTime, icu4x::CalendarParseError>(diplomat::Ok<icu4x::ZonedIsoDateTime>(icu4x::ZonedIsoDateTime::FromFFI(result.ok))) : diplomat::result<icu4x::ZonedIsoDateTime, icu4x::CalendarParseError>(diplomat::Err<icu4x::CalendarParseError>(icu4x::CalendarParseError::FromFFI(result.err)));
}

inline diplomat::result<icu4x::ZonedDateTime, icu4x::CalendarParseError> icu4x::IxdtfParser::try_from_str(std::string_view v, const icu4x::Calendar& calendar) const {
  auto result = icu4x::capi::icu4x_IxdtfParser_try_from_str_mv1(this->AsFFI(),
    {v.data(), v.size()},
    calendar.AsFFI());
  return result.is_ok ? diplomat::result<icu4x::ZonedDateTime, icu4x::CalendarParseError>(diplomat::Ok<icu4x::ZonedDateTime>(icu4x::ZonedDateTime::FromFFI(result.ok))) : diplomat::result<icu4x::ZonedDateTime, icu4x::CalendarParseError>(diplomat::Err<icu4x::CalendarParseError>(icu4x::CalendarParseError::FromFFI(result.err)));
}

inline const icu4x::capi::IxdtfParser* icu4x::IxdtfParser::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::IxdtfParser*>(this);
}

inline icu4x::capi::IxdtfParser* icu4x::IxdtfParser::AsFFI() {
  return reinterpret_cast<icu4x::capi::IxdtfParser*>(this);
}

inline const icu4x::IxdtfParser* icu4x::IxdtfParser::FromFFI(const icu4x::capi::IxdtfParser* ptr) {
  return reinterpret_cast<const icu4x::IxdtfParser*>(ptr);
}

inline icu4x::IxdtfParser* icu4x::IxdtfParser::FromFFI(icu4x::capi::IxdtfParser* ptr) {
  return reinterpret_cast<icu4x::IxdtfParser*>(ptr);
}

inline void icu4x::IxdtfParser::operator delete(void* ptr) {
  icu4x::capi::icu4x_IxdtfParser_destroy_mv1(reinterpret_cast<icu4x::capi::IxdtfParser*>(ptr));
}


#endif // icu4x_IxdtfParser_HPP
