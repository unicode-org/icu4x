#ifndef LocaleDisplayNamesFormatter_D_HPP
#define LocaleDisplayNamesFormatter_D_HPP

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
struct DisplayNamesOptionsV1;
class DataError;


namespace diplomat {
namespace capi {
    typedef struct LocaleDisplayNamesFormatter LocaleDisplayNamesFormatter;
} // namespace capi
} // namespace

class LocaleDisplayNamesFormatter {
public:

  inline static diplomat::result<std::unique_ptr<LocaleDisplayNamesFormatter>, DataError> create(const DataProvider& provider, const Locale& locale, DisplayNamesOptionsV1 options);

  inline std::string of(const Locale& locale) const;

  inline const diplomat::capi::LocaleDisplayNamesFormatter* AsFFI() const;
  inline diplomat::capi::LocaleDisplayNamesFormatter* AsFFI();
  inline static const LocaleDisplayNamesFormatter* FromFFI(const diplomat::capi::LocaleDisplayNamesFormatter* ptr);
  inline static LocaleDisplayNamesFormatter* FromFFI(diplomat::capi::LocaleDisplayNamesFormatter* ptr);
  inline static void operator delete(void* ptr);
private:
  LocaleDisplayNamesFormatter() = delete;
  LocaleDisplayNamesFormatter(const LocaleDisplayNamesFormatter&) = delete;
  LocaleDisplayNamesFormatter(LocaleDisplayNamesFormatter&&) noexcept = delete;
  LocaleDisplayNamesFormatter operator=(const LocaleDisplayNamesFormatter&) = delete;
  LocaleDisplayNamesFormatter operator=(LocaleDisplayNamesFormatter&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // LocaleDisplayNamesFormatter_D_HPP
