#ifndef icu4x_WeekInformation_D_HPP
#define icu4x_WeekInformation_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../diplomat_runtime.hpp"

namespace icu4x {
namespace capi { struct DataProvider; }
class DataProvider;
namespace capi { struct Locale; }
class Locale;
namespace capi { struct WeekInformation; }
class WeekInformation;
namespace capi { struct WeekdaySetIterator; }
class WeekdaySetIterator;
class DataError;
class Weekday;
}


namespace icu4x {
namespace capi {
    struct WeekInformation;
} // namespace capi
} // namespace

namespace icu4x {
class WeekInformation {
public:

  inline static diplomat::result<std::unique_ptr<icu4x::WeekInformation>, icu4x::DataError> create(const icu4x::Locale& locale);

  inline static diplomat::result<std::unique_ptr<icu4x::WeekInformation>, icu4x::DataError> create_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale);

  inline icu4x::Weekday first_weekday() const;

  inline bool is_weekend(icu4x::Weekday day) const;

  inline std::unique_ptr<icu4x::WeekdaySetIterator> weekend() const;

  inline const icu4x::capi::WeekInformation* AsFFI() const;
  inline icu4x::capi::WeekInformation* AsFFI();
  inline static const icu4x::WeekInformation* FromFFI(const icu4x::capi::WeekInformation* ptr);
  inline static icu4x::WeekInformation* FromFFI(icu4x::capi::WeekInformation* ptr);
  inline static void operator delete(void* ptr);
private:
  WeekInformation() = delete;
  WeekInformation(const icu4x::WeekInformation&) = delete;
  WeekInformation(icu4x::WeekInformation&&) noexcept = delete;
  WeekInformation operator=(const icu4x::WeekInformation&) = delete;
  WeekInformation operator=(icu4x::WeekInformation&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // icu4x_WeekInformation_D_HPP
