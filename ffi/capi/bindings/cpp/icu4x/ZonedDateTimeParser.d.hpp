#ifndef icu4x_ZonedDateTimeParser_D_HPP
#define icu4x_ZonedDateTimeParser_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../diplomat_runtime.hpp"

namespace icu4x {
namespace capi { struct Calendar; }
class Calendar;
namespace capi { struct DataProvider; }
class DataProvider;
namespace capi { struct ZonedDateTimeParser; }
class ZonedDateTimeParser;
struct ZonedDateTime;
struct ZonedIsoDateTime;
class CalendarParseError;
class DataError;
}


namespace icu4x {
namespace capi {
    struct ZonedDateTimeParser;
} // namespace capi
} // namespace

namespace icu4x {
class ZonedDateTimeParser {
public:

  inline static std::unique_ptr<icu4x::ZonedDateTimeParser> create();

  inline static diplomat::result<std::unique_ptr<icu4x::ZonedDateTimeParser>, icu4x::DataError> create_with_provider(const icu4x::DataProvider& provider);

  inline diplomat::result<icu4x::ZonedIsoDateTime, icu4x::CalendarParseError> try_iso_from_str(std::string_view v) const;

  inline diplomat::result<icu4x::ZonedDateTime, icu4x::CalendarParseError> try_from_str(std::string_view v, const icu4x::Calendar& calendar) const;

  inline const icu4x::capi::ZonedDateTimeParser* AsFFI() const;
  inline icu4x::capi::ZonedDateTimeParser* AsFFI();
  inline static const icu4x::ZonedDateTimeParser* FromFFI(const icu4x::capi::ZonedDateTimeParser* ptr);
  inline static icu4x::ZonedDateTimeParser* FromFFI(icu4x::capi::ZonedDateTimeParser* ptr);
  inline static void operator delete(void* ptr);
private:
  ZonedDateTimeParser() = delete;
  ZonedDateTimeParser(const icu4x::ZonedDateTimeParser&) = delete;
  ZonedDateTimeParser(icu4x::ZonedDateTimeParser&&) noexcept = delete;
  ZonedDateTimeParser operator=(const icu4x::ZonedDateTimeParser&) = delete;
  ZonedDateTimeParser operator=(icu4x::ZonedDateTimeParser&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // icu4x_ZonedDateTimeParser_D_HPP
