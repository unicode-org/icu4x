#ifndef ICU4XLocaleDisplayNamesFormatter_D_HPP
#define ICU4XLocaleDisplayNamesFormatter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataError.d.hpp"
#include "ICU4XDisplayNamesOptionsV1.d.hpp"

class ICU4XDataProvider;
class ICU4XLocale;
struct ICU4XDisplayNamesOptionsV1;
class ICU4XDataError;


namespace capi {
    typedef struct ICU4XLocaleDisplayNamesFormatter ICU4XLocaleDisplayNamesFormatter;
}

class ICU4XLocaleDisplayNamesFormatter {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XLocaleDisplayNamesFormatter>, ICU4XDataError> create(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XDisplayNamesOptionsV1 options);

  inline std::string of(const ICU4XLocale& locale) const;

  inline const capi::ICU4XLocaleDisplayNamesFormatter* AsFFI() const;
  inline capi::ICU4XLocaleDisplayNamesFormatter* AsFFI();
  inline static const ICU4XLocaleDisplayNamesFormatter* FromFFI(const capi::ICU4XLocaleDisplayNamesFormatter* ptr);
  inline static ICU4XLocaleDisplayNamesFormatter* FromFFI(capi::ICU4XLocaleDisplayNamesFormatter* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XLocaleDisplayNamesFormatter() = delete;
  ICU4XLocaleDisplayNamesFormatter(const ICU4XLocaleDisplayNamesFormatter&) = delete;
  ICU4XLocaleDisplayNamesFormatter(ICU4XLocaleDisplayNamesFormatter&&) noexcept = delete;
  ICU4XLocaleDisplayNamesFormatter operator=(const ICU4XLocaleDisplayNamesFormatter&) = delete;
  ICU4XLocaleDisplayNamesFormatter operator=(ICU4XLocaleDisplayNamesFormatter&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XLocaleDisplayNamesFormatter_D_HPP
