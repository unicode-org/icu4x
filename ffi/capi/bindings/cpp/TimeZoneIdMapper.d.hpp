#ifndef TimeZoneIdMapper_D_HPP
#define TimeZoneIdMapper_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {typedef struct DataProvider DataProvider; }
class DataProvider;
class DataError;
class TimeZoneInvalidIdError;


namespace diplomat {
namespace capi {
    typedef struct TimeZoneIdMapper TimeZoneIdMapper;
} // namespace capi
} // namespace

class TimeZoneIdMapper {
public:

  inline static diplomat::result<std::unique_ptr<TimeZoneIdMapper>, DataError> create(const DataProvider& provider);

  inline diplomat::result<std::string, TimeZoneInvalidIdError> iana_to_bcp47(std::string_view value) const;

  inline diplomat::result<diplomat::result<std::string, TimeZoneInvalidIdError>, diplomat::Utf8Error> normalize_iana(std::string_view value) const;

  inline diplomat::result<diplomat::result<std::string, TimeZoneInvalidIdError>, diplomat::Utf8Error> canonicalize_iana(std::string_view value) const;

  inline diplomat::result<std::string, TimeZoneInvalidIdError> find_canonical_iana_from_bcp47(std::string_view value) const;

  inline const diplomat::capi::TimeZoneIdMapper* AsFFI() const;
  inline diplomat::capi::TimeZoneIdMapper* AsFFI();
  inline static const TimeZoneIdMapper* FromFFI(const diplomat::capi::TimeZoneIdMapper* ptr);
  inline static TimeZoneIdMapper* FromFFI(diplomat::capi::TimeZoneIdMapper* ptr);
  inline static void operator delete(void* ptr);
private:
  TimeZoneIdMapper() = delete;
  TimeZoneIdMapper(const TimeZoneIdMapper&) = delete;
  TimeZoneIdMapper(TimeZoneIdMapper&&) noexcept = delete;
  TimeZoneIdMapper operator=(const TimeZoneIdMapper&) = delete;
  TimeZoneIdMapper operator=(TimeZoneIdMapper&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // TimeZoneIdMapper_D_HPP
