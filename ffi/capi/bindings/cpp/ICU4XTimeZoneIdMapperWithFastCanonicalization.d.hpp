#ifndef ICU4XTimeZoneIdMapperWithFastCanonicalization_D_HPP
#define ICU4XTimeZoneIdMapperWithFastCanonicalization_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataError.d.hpp"
#include "ICU4XTimeZoneInvalidIdError.d.hpp"

class ICU4XDataProvider;
class ICU4XDataError;
class ICU4XTimeZoneInvalidIdError;


namespace capi {
    typedef struct ICU4XTimeZoneIdMapperWithFastCanonicalization ICU4XTimeZoneIdMapperWithFastCanonicalization;
}

class ICU4XTimeZoneIdMapperWithFastCanonicalization {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XTimeZoneIdMapperWithFastCanonicalization>, ICU4XDataError> create(const ICU4XDataProvider& provider);

  inline diplomat::result<diplomat::result<std::string, ICU4XTimeZoneInvalidIdError>, diplomat::Utf8Error> canonicalize_iana(std::string_view value) const;

  inline diplomat::result<std::string, ICU4XTimeZoneInvalidIdError> canonical_iana_from_bcp47(std::string_view value) const;

  inline const capi::ICU4XTimeZoneIdMapperWithFastCanonicalization* AsFFI() const;
  inline capi::ICU4XTimeZoneIdMapperWithFastCanonicalization* AsFFI();
  inline static const ICU4XTimeZoneIdMapperWithFastCanonicalization* FromFFI(const capi::ICU4XTimeZoneIdMapperWithFastCanonicalization* ptr);
  inline static ICU4XTimeZoneIdMapperWithFastCanonicalization* FromFFI(capi::ICU4XTimeZoneIdMapperWithFastCanonicalization* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XTimeZoneIdMapperWithFastCanonicalization() = delete;
  ICU4XTimeZoneIdMapperWithFastCanonicalization(const ICU4XTimeZoneIdMapperWithFastCanonicalization&) = delete;
  ICU4XTimeZoneIdMapperWithFastCanonicalization(ICU4XTimeZoneIdMapperWithFastCanonicalization&&) noexcept = delete;
  ICU4XTimeZoneIdMapperWithFastCanonicalization operator=(const ICU4XTimeZoneIdMapperWithFastCanonicalization&) = delete;
  ICU4XTimeZoneIdMapperWithFastCanonicalization operator=(ICU4XTimeZoneIdMapperWithFastCanonicalization&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XTimeZoneIdMapperWithFastCanonicalization_D_HPP
