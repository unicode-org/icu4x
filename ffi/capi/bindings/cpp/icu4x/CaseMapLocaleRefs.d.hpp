#ifndef ICU4X_CaseMapLocaleRefs_D_HPP
#define ICU4X_CaseMapLocaleRefs_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"
namespace icu4x {
namespace capi { struct Locale; }
class Locale;
} // namespace icu4x



namespace icu4x {
namespace capi {
    struct CaseMapLocaleRefs;
} // namespace capi
} // namespace

namespace icu4x {
/**
 * A type that provides access to `'static` Locales, to avoid
 * needing to parse user locales when the code knows what locale it wants.
 *
 * In most cases, you should be taking a locale from the user, but in some
 * limited cases you may be parsing from a limited set of locales and not
 * wish to pull in full-fledged parsing code.
 *
 * This type is for locales that have special meaning to `CaseMap`,
 * since it only cares about a small set of locales and locale parsing takes
 * up a relatively large amount of binary size in the context of casemapping.
 */
class CaseMapLocaleRefs {
public:

  /**
   * Returns a borrowed "az" {@link Locale}.
   */
  inline static const icu4x::Locale& az();

  /**
   * Returns a borrowed "el" {@link Locale}.
   */
  inline static const icu4x::Locale& el();

  /**
   * Returns a borrowed "hy" {@link Locale}.
   */
  inline static const icu4x::Locale& hy();

  /**
   * Returns a borrowed "lt" {@link Locale}.
   */
  inline static const icu4x::Locale& lt();

  /**
   * Returns a borrowed "nl" {@link Locale}.
   */
  inline static const icu4x::Locale& nl();

  /**
   * Returns a borrowed "tr" {@link Locale}.
   */
  inline static const icu4x::Locale& tr();

    inline const icu4x::capi::CaseMapLocaleRefs* AsFFI() const;
    inline icu4x::capi::CaseMapLocaleRefs* AsFFI();
    inline static const icu4x::CaseMapLocaleRefs* FromFFI(const icu4x::capi::CaseMapLocaleRefs* ptr);
    inline static icu4x::CaseMapLocaleRefs* FromFFI(icu4x::capi::CaseMapLocaleRefs* ptr);
    inline static void operator delete(void* ptr);
private:
    CaseMapLocaleRefs() = delete;
    CaseMapLocaleRefs(const icu4x::CaseMapLocaleRefs&) = delete;
    CaseMapLocaleRefs(icu4x::CaseMapLocaleRefs&&) noexcept = delete;
    CaseMapLocaleRefs operator=(const icu4x::CaseMapLocaleRefs&) = delete;
    CaseMapLocaleRefs operator=(icu4x::CaseMapLocaleRefs&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // ICU4X_CaseMapLocaleRefs_D_HPP
