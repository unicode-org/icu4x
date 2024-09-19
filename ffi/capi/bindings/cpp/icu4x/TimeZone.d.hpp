#ifndef icu4x_TimeZone_D_HPP
#define icu4x_TimeZone_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"

namespace icu4x {
namespace capi { struct TimeZone; }
class TimeZone;
struct TimeZoneInvalidIdError;
struct TimeZoneInvalidOffsetError;
struct TimeZoneUnknownError;
}


namespace icu4x {
namespace capi {
    struct TimeZone;
} // namespace capi
} // namespace

namespace icu4x {
class TimeZone {
public:

  inline static diplomat::result<std::unique_ptr<icu4x::TimeZone>, icu4x::TimeZoneUnknownError> from_string(std::string_view s);

  inline static std::unique_ptr<icu4x::TimeZone> utc();

  inline static std::unique_ptr<icu4x::TimeZone> create(int32_t offset_seconds, std::string_view id);

  inline static diplomat::result<std::unique_ptr<icu4x::TimeZone>, icu4x::TimeZoneInvalidOffsetError> create_from_offset_seconds(int32_t offset_seconds);

  inline static diplomat::result<std::unique_ptr<icu4x::TimeZone>, icu4x::TimeZoneInvalidIdError> create_from_bcp47_id(std::string_view id);

  inline std::optional<int8_t> offset_eighths_of_hour() const;

  inline std::optional<int32_t> offset_seconds() const;

  inline std::optional<bool> is_offset_positive() const;

  inline std::optional<bool> is_offset_zero() const;

  inline std::optional<bool> offset_has_minutes() const;

  inline std::optional<bool> offset_has_seconds() const;

  inline std::optional<std::string> bcp47_id() const;

  inline const icu4x::capi::TimeZone* AsFFI() const;
  inline icu4x::capi::TimeZone* AsFFI();
  inline static const icu4x::TimeZone* FromFFI(const icu4x::capi::TimeZone* ptr);
  inline static icu4x::TimeZone* FromFFI(icu4x::capi::TimeZone* ptr);
  inline static void operator delete(void* ptr);
private:
  TimeZone() = delete;
  TimeZone(const icu4x::TimeZone&) = delete;
  TimeZone(icu4x::TimeZone&&) noexcept = delete;
  TimeZone operator=(const icu4x::TimeZone&) = delete;
  TimeZone operator=(icu4x::TimeZone&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // icu4x_TimeZone_D_HPP
