#ifndef icu4x_UnicodeSetData_D_HPP
#define icu4x_UnicodeSetData_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"

namespace icu4x {
namespace capi { struct DataProvider; }
class DataProvider;
namespace capi { struct UnicodeSetData; }
class UnicodeSetData;
class DataError;
}


namespace icu4x {
namespace capi {
    struct UnicodeSetData;
} // namespace capi
} // namespace

namespace icu4x {
class UnicodeSetData {
public:

  inline bool contains(std::string_view s) const;

  inline bool contains_char(char32_t cp) const;

  inline static diplomat::result<std::unique_ptr<icu4x::UnicodeSetData>, icu4x::DataError> load_basic_emoji(const icu4x::DataProvider& provider);

  inline const icu4x::capi::UnicodeSetData* AsFFI() const;
  inline icu4x::capi::UnicodeSetData* AsFFI();
  inline static const icu4x::UnicodeSetData* FromFFI(const icu4x::capi::UnicodeSetData* ptr);
  inline static icu4x::UnicodeSetData* FromFFI(icu4x::capi::UnicodeSetData* ptr);
  inline static void operator delete(void* ptr);
private:
  UnicodeSetData() = delete;
  UnicodeSetData(const icu4x::UnicodeSetData&) = delete;
  UnicodeSetData(icu4x::UnicodeSetData&&) noexcept = delete;
  UnicodeSetData operator=(const icu4x::UnicodeSetData&) = delete;
  UnicodeSetData operator=(icu4x::UnicodeSetData&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // icu4x_UnicodeSetData_D_HPP
