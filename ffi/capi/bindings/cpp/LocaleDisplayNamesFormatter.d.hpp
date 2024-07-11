#ifndef LocaleDisplayNamesFormatter_D_HPP
#define LocaleDisplayNamesFormatter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "DataError.d.hpp"
#include "DisplayNamesOptionsV1.d.hpp"

class DataProvider;
class Locale;
struct DisplayNamesOptionsV1;
class DataError;


namespace capi {
    typedef struct LocaleDisplayNamesFormatter LocaleDisplayNamesFormatter;
}

class LocaleDisplayNamesFormatter {
public:

  inline static diplomat::result<std::unique_ptr<LocaleDisplayNamesFormatter>, DataError> create(const DataProvider& provider, const Locale& locale, DisplayNamesOptionsV1 options);

  inline std::string of(const Locale& locale) const;

  inline const capi::LocaleDisplayNamesFormatter* AsFFI() const;
  inline capi::LocaleDisplayNamesFormatter* AsFFI();
  inline static const LocaleDisplayNamesFormatter* FromFFI(const capi::LocaleDisplayNamesFormatter* ptr);
  inline static LocaleDisplayNamesFormatter* FromFFI(capi::LocaleDisplayNamesFormatter* ptr);
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
