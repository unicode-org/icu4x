#ifndef ICU4XUnicodeSetData_D_HPP
#define ICU4XUnicodeSetData_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataError.d.hpp"

class ICU4XDataProvider;
class ICU4XLocale;
class ICU4XDataError;


namespace capi {
    typedef struct ICU4XUnicodeSetData ICU4XUnicodeSetData;
}

class ICU4XUnicodeSetData {
public:

  inline bool contains(std::string_view s) const;

  inline bool contains_char(char32_t cp) const;

  inline bool contains32(uint32_t cp) const;

  inline static diplomat::result<std::unique_ptr<ICU4XUnicodeSetData>, ICU4XDataError> load_basic_emoji(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XUnicodeSetData>, ICU4XDataError> load_exemplars_main(const ICU4XDataProvider& provider, const ICU4XLocale& locale);

  inline static diplomat::result<std::unique_ptr<ICU4XUnicodeSetData>, ICU4XDataError> load_exemplars_auxiliary(const ICU4XDataProvider& provider, const ICU4XLocale& locale);

  inline static diplomat::result<std::unique_ptr<ICU4XUnicodeSetData>, ICU4XDataError> load_exemplars_punctuation(const ICU4XDataProvider& provider, const ICU4XLocale& locale);

  inline static diplomat::result<std::unique_ptr<ICU4XUnicodeSetData>, ICU4XDataError> load_exemplars_numbers(const ICU4XDataProvider& provider, const ICU4XLocale& locale);

  inline static diplomat::result<std::unique_ptr<ICU4XUnicodeSetData>, ICU4XDataError> load_exemplars_index(const ICU4XDataProvider& provider, const ICU4XLocale& locale);

  inline const capi::ICU4XUnicodeSetData* AsFFI() const;
  inline capi::ICU4XUnicodeSetData* AsFFI();
  inline static const ICU4XUnicodeSetData* FromFFI(const capi::ICU4XUnicodeSetData* ptr);
  inline static ICU4XUnicodeSetData* FromFFI(capi::ICU4XUnicodeSetData* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XUnicodeSetData() = delete;
  ICU4XUnicodeSetData(const ICU4XUnicodeSetData&) = delete;
  ICU4XUnicodeSetData(ICU4XUnicodeSetData&&) noexcept = delete;
  ICU4XUnicodeSetData operator=(const ICU4XUnicodeSetData&) = delete;
  ICU4XUnicodeSetData operator=(ICU4XUnicodeSetData&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XUnicodeSetData_D_HPP
