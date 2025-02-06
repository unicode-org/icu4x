#ifndef icu4x_ZonedDateTimeParser_HPP
#define icu4x_ZonedDateTimeParser_HPP

#include "ZonedDateTimeParser.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
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
    
    icu4x::capi::ZonedDateTimeParser* icu4x_ZonedDateTimeParser_create_mv1(void);
    
    typedef struct icu4x_ZonedDateTimeParser_create_with_provider_mv1_result {union {icu4x::capi::ZonedDateTimeParser* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_ZonedDateTimeParser_create_with_provider_mv1_result;
    icu4x_ZonedDateTimeParser_create_with_provider_mv1_result icu4x_ZonedDateTimeParser_create_with_provider_mv1(const icu4x::capi::DataProvider* provider);
    
    typedef struct icu4x_ZonedDateTimeParser_try_iso_from_str_mv1_result {union {icu4x::capi::ZonedIsoDateTime ok; icu4x::capi::CalendarParseError err;}; bool is_ok;} icu4x_ZonedDateTimeParser_try_iso_from_str_mv1_result;
    icu4x_ZonedDateTimeParser_try_iso_from_str_mv1_result icu4x_ZonedDateTimeParser_try_iso_from_str_mv1(const icu4x::capi::ZonedDateTimeParser* self, diplomat::capi::DiplomatStringView v);
    
    typedef struct icu4x_ZonedDateTimeParser_try_from_str_mv1_result {union {icu4x::capi::ZonedDateTime ok; icu4x::capi::CalendarParseError err;}; bool is_ok;} icu4x_ZonedDateTimeParser_try_from_str_mv1_result;
    icu4x_ZonedDateTimeParser_try_from_str_mv1_result icu4x_ZonedDateTimeParser_try_from_str_mv1(const icu4x::capi::ZonedDateTimeParser* self, diplomat::capi::DiplomatStringView v, const icu4x::capi::Calendar* calendar);
    
    
    void icu4x_ZonedDateTimeParser_destroy_mv1(ZonedDateTimeParser* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<icu4x::ZonedDateTimeParser> icu4x::ZonedDateTimeParser::create() {
  auto result = icu4x::capi::icu4x_ZonedDateTimeParser_create_mv1();
  return std::unique_ptr<icu4x::ZonedDateTimeParser>(icu4x::ZonedDateTimeParser::FromFFI(result));
}

inline diplomat::result<std::unique_ptr<icu4x::ZonedDateTimeParser>, icu4x::DataError> icu4x::ZonedDateTimeParser::create_with_provider(const icu4x::DataProvider& provider) {
  auto result = icu4x::capi::icu4x_ZonedDateTimeParser_create_with_provider_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::ZonedDateTimeParser>, icu4x::DataError>(diplomat::Ok<std::unique_ptr<icu4x::ZonedDateTimeParser>>(std::unique_ptr<icu4x::ZonedDateTimeParser>(icu4x::ZonedDateTimeParser::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::ZonedDateTimeParser>, icu4x::DataError>(diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline diplomat::result<icu4x::ZonedIsoDateTime, icu4x::CalendarParseError> icu4x::ZonedDateTimeParser::try_iso_from_str(std::string_view v) const {
  auto result = icu4x::capi::icu4x_ZonedDateTimeParser_try_iso_from_str_mv1(this->AsFFI(),
    {v.data(), v.size()});
  return result.is_ok ? diplomat::result<icu4x::ZonedIsoDateTime, icu4x::CalendarParseError>(diplomat::Ok<icu4x::ZonedIsoDateTime>(icu4x::ZonedIsoDateTime::FromFFI(result.ok))) : diplomat::result<icu4x::ZonedIsoDateTime, icu4x::CalendarParseError>(diplomat::Err<icu4x::CalendarParseError>(icu4x::CalendarParseError::FromFFI(result.err)));
}

inline diplomat::result<icu4x::ZonedDateTime, icu4x::CalendarParseError> icu4x::ZonedDateTimeParser::try_from_str(std::string_view v, const icu4x::Calendar& calendar) const {
  auto result = icu4x::capi::icu4x_ZonedDateTimeParser_try_from_str_mv1(this->AsFFI(),
    {v.data(), v.size()},
    calendar.AsFFI());
  return result.is_ok ? diplomat::result<icu4x::ZonedDateTime, icu4x::CalendarParseError>(diplomat::Ok<icu4x::ZonedDateTime>(icu4x::ZonedDateTime::FromFFI(result.ok))) : diplomat::result<icu4x::ZonedDateTime, icu4x::CalendarParseError>(diplomat::Err<icu4x::CalendarParseError>(icu4x::CalendarParseError::FromFFI(result.err)));
}

inline const icu4x::capi::ZonedDateTimeParser* icu4x::ZonedDateTimeParser::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::ZonedDateTimeParser*>(this);
}

inline icu4x::capi::ZonedDateTimeParser* icu4x::ZonedDateTimeParser::AsFFI() {
  return reinterpret_cast<icu4x::capi::ZonedDateTimeParser*>(this);
}

inline const icu4x::ZonedDateTimeParser* icu4x::ZonedDateTimeParser::FromFFI(const icu4x::capi::ZonedDateTimeParser* ptr) {
  return reinterpret_cast<const icu4x::ZonedDateTimeParser*>(ptr);
}

inline icu4x::ZonedDateTimeParser* icu4x::ZonedDateTimeParser::FromFFI(icu4x::capi::ZonedDateTimeParser* ptr) {
  return reinterpret_cast<icu4x::ZonedDateTimeParser*>(ptr);
}

inline void icu4x::ZonedDateTimeParser::operator delete(void* ptr) {
  icu4x::capi::icu4x_ZonedDateTimeParser_destroy_mv1(reinterpret_cast<icu4x::capi::ZonedDateTimeParser*>(ptr));
}


#endif // icu4x_ZonedDateTimeParser_HPP
