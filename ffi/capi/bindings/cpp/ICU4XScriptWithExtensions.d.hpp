#ifndef ICU4XScriptWithExtensions_D_HPP
#define ICU4XScriptWithExtensions_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataError.d.hpp"
#include "ICU4XScriptWithExtensions.d.h"

class ICU4XCodePointRangeIterator;
class ICU4XDataProvider;
class ICU4XScriptWithExtensionsBorrowed;
class ICU4XDataError;


class ICU4XScriptWithExtensions {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XScriptWithExtensions>, ICU4XDataError> create(const ICU4XDataProvider& provider);

  inline uint16_t get_script_val(uint32_t code_point) const;

  inline bool has_script(uint32_t code_point, uint16_t script) const;

  inline std::unique_ptr<ICU4XScriptWithExtensionsBorrowed> as_borrowed() const;

  inline std::unique_ptr<ICU4XCodePointRangeIterator> iter_ranges_for_script(uint16_t script) const;

  inline const capi::ICU4XScriptWithExtensions* AsFFI() const;
  inline capi::ICU4XScriptWithExtensions* AsFFI();
  inline static const ICU4XScriptWithExtensions* FromFFI(const capi::ICU4XScriptWithExtensions* ptr);
  inline static ICU4XScriptWithExtensions* FromFFI(capi::ICU4XScriptWithExtensions* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XScriptWithExtensions() = delete;
  ICU4XScriptWithExtensions(const ICU4XScriptWithExtensions&) = delete;
  ICU4XScriptWithExtensions(ICU4XScriptWithExtensions&&) noexcept = delete;
  ICU4XScriptWithExtensions operator=(const ICU4XScriptWithExtensions&) = delete;
  ICU4XScriptWithExtensions operator=(ICU4XScriptWithExtensions&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XScriptWithExtensions_D_HPP
