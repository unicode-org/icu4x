#ifndef ListFormatter_D_HPP
#define ListFormatter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {typedef struct DataProvider DataProvider; }
class DataProvider;
namespace capi {typedef struct Locale Locale; }
class Locale;
class DataError;
class ListLength;


namespace diplomat {
namespace capi {
    typedef struct ListFormatter ListFormatter;
} // namespace capi
} // namespace

class ListFormatter {
public:

  inline static diplomat::result<std::unique_ptr<ListFormatter>, DataError> create_and_with_length(const DataProvider& provider, const Locale& locale, ListLength length);

  inline static diplomat::result<std::unique_ptr<ListFormatter>, DataError> create_or_with_length(const DataProvider& provider, const Locale& locale, ListLength length);

  inline static diplomat::result<std::unique_ptr<ListFormatter>, DataError> create_unit_with_length(const DataProvider& provider, const Locale& locale, ListLength length);

  inline std::string format_valid_utf8(diplomat::span<const std::string_view> list) const;

  inline std::string format_utf8(diplomat::span<const std::string_view> list) const;

  inline std::string format_utf16(diplomat::span<const std::u16string_view> list) const;

  inline const diplomat::capi::ListFormatter* AsFFI() const;
  inline diplomat::capi::ListFormatter* AsFFI();
  inline static const ListFormatter* FromFFI(const diplomat::capi::ListFormatter* ptr);
  inline static ListFormatter* FromFFI(diplomat::capi::ListFormatter* ptr);
  inline static void operator delete(void* ptr);
private:
  ListFormatter() = delete;
  ListFormatter(const ListFormatter&) = delete;
  ListFormatter(ListFormatter&&) noexcept = delete;
  ListFormatter operator=(const ListFormatter&) = delete;
  ListFormatter operator=(ListFormatter&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ListFormatter_D_HPP
