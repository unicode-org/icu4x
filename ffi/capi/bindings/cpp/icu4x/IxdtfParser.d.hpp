#ifndef icu4x_IxdtfParser_D_HPP
#define icu4x_IxdtfParser_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"

namespace icu4x {
namespace capi { struct Calendar; }
class Calendar;
namespace capi { struct DataProvider; }
class DataProvider;
namespace capi { struct IxdtfParser; }
class IxdtfParser;
struct ZonedDateTime;
struct ZonedIsoDateTime;
class CalendarParseError;
class DataError;
}


namespace icu4x {
namespace capi {
    struct IxdtfParser;
} // namespace capi
} // namespace

namespace icu4x {
class IxdtfParser {
public:

  inline static std::unique_ptr<icu4x::IxdtfParser> create();

  inline static diplomat::result<std::unique_ptr<icu4x::IxdtfParser>, icu4x::DataError> create_with_provider(const icu4x::DataProvider& provider);

  inline diplomat::result<icu4x::ZonedIsoDateTime, icu4x::CalendarParseError> try_iso_from_str(std::string_view v) const;

  inline diplomat::result<icu4x::ZonedDateTime, icu4x::CalendarParseError> try_from_str(std::string_view v, const icu4x::Calendar& calendar) const;

  inline const icu4x::capi::IxdtfParser* AsFFI() const;
  inline icu4x::capi::IxdtfParser* AsFFI();
  inline static const icu4x::IxdtfParser* FromFFI(const icu4x::capi::IxdtfParser* ptr);
  inline static icu4x::IxdtfParser* FromFFI(icu4x::capi::IxdtfParser* ptr);
  inline static void operator delete(void* ptr);
private:
  IxdtfParser() = delete;
  IxdtfParser(const icu4x::IxdtfParser&) = delete;
  IxdtfParser(icu4x::IxdtfParser&&) noexcept = delete;
  IxdtfParser operator=(const icu4x::IxdtfParser&) = delete;
  IxdtfParser operator=(icu4x::IxdtfParser&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // icu4x_IxdtfParser_D_HPP
