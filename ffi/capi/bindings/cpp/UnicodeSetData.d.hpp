#ifndef UnicodeSetData_D_HPP
#define UnicodeSetData_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace diplomat::capi { struct DataProvider; }
class DataProvider;
namespace diplomat::capi { struct Locale; }
class Locale;
class DataError;


namespace diplomat {
namespace capi {
    struct UnicodeSetData;
} // namespace capi
} // namespace

class UnicodeSetData {
public:

  inline bool contains(std::string_view s) const;

  inline bool contains_char(char32_t cp) const;

  inline static diplomat::result<std::unique_ptr<UnicodeSetData>, DataError> load_basic_emoji(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<UnicodeSetData>, DataError> load_exemplars_main(const DataProvider& provider, const Locale& locale);

  inline static diplomat::result<std::unique_ptr<UnicodeSetData>, DataError> load_exemplars_auxiliary(const DataProvider& provider, const Locale& locale);

  inline static diplomat::result<std::unique_ptr<UnicodeSetData>, DataError> load_exemplars_punctuation(const DataProvider& provider, const Locale& locale);

  inline static diplomat::result<std::unique_ptr<UnicodeSetData>, DataError> load_exemplars_numbers(const DataProvider& provider, const Locale& locale);

  inline static diplomat::result<std::unique_ptr<UnicodeSetData>, DataError> load_exemplars_index(const DataProvider& provider, const Locale& locale);

  inline const diplomat::capi::UnicodeSetData* AsFFI() const;
  inline diplomat::capi::UnicodeSetData* AsFFI();
  inline static const UnicodeSetData* FromFFI(const diplomat::capi::UnicodeSetData* ptr);
  inline static UnicodeSetData* FromFFI(diplomat::capi::UnicodeSetData* ptr);
  inline static void operator delete(void* ptr);
private:
  UnicodeSetData() = delete;
  UnicodeSetData(const UnicodeSetData&) = delete;
  UnicodeSetData(UnicodeSetData&&) noexcept = delete;
  UnicodeSetData operator=(const UnicodeSetData&) = delete;
  UnicodeSetData operator=(UnicodeSetData&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // UnicodeSetData_D_HPP
