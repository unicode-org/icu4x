#ifndef ICU4XTimeZoneIdMapper_D_HPP
#define ICU4XTimeZoneIdMapper_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataError.d.hpp"
#include "ICU4XTimeZoneIdMapper.d.h"
#include "ICU4XTimeZoneInvalidIdError.d.hpp"

class ICU4XDataProvider;
class ICU4XDataError;
class ICU4XTimeZoneInvalidIdError;


class ICU4XTimeZoneIdMapper {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XTimeZoneIdMapper>, ICU4XDataError> create(const ICU4XDataProvider& provider);

  inline diplomat::result<std::string, ICU4XTimeZoneInvalidIdError> iana_to_bcp47(std::string_view value) const;

  inline diplomat::result<diplomat::result<std::string, ICU4XTimeZoneInvalidIdError>, diplomat::Utf8Error> normalize_iana(std::string_view value) const;

  inline diplomat::result<diplomat::result<std::string, ICU4XTimeZoneInvalidIdError>, diplomat::Utf8Error> canonicalize_iana(std::string_view value) const;

  inline diplomat::result<std::string, ICU4XTimeZoneInvalidIdError> find_canonical_iana_from_bcp47(std::string_view value) const;

  inline const capi::ICU4XTimeZoneIdMapper* AsFFI() const;
  inline capi::ICU4XTimeZoneIdMapper* AsFFI();
  inline static const ICU4XTimeZoneIdMapper* FromFFI(const capi::ICU4XTimeZoneIdMapper* ptr);
  inline static ICU4XTimeZoneIdMapper* FromFFI(capi::ICU4XTimeZoneIdMapper* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XTimeZoneIdMapper() = delete;
  ICU4XTimeZoneIdMapper(const ICU4XTimeZoneIdMapper&) = delete;
  ICU4XTimeZoneIdMapper(ICU4XTimeZoneIdMapper&&) noexcept = delete;
  ICU4XTimeZoneIdMapper operator=(const ICU4XTimeZoneIdMapper&) = delete;
  ICU4XTimeZoneIdMapper operator=(ICU4XTimeZoneIdMapper&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XTimeZoneIdMapper_D_HPP
