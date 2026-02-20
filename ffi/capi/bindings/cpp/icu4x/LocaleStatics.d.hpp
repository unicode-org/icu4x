#ifndef ICU4X_LocaleStatics_D_HPP
#define ICU4X_LocaleStatics_D_HPP

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
    struct LocaleStatics;
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
 * For now, this only supports locales that have special meaning to `icu_casemap`,
 * since it only cares about a small set of locales and locale parsing takes
 * up a relatively large amount of binary size in the context of casemapping.
 *
 * Please file an issue on ICU4X if you have a use case for more locales here.
 */
class LocaleStatics {
public:

  /**
   * Construct a {@link Locale} "und".
   */
  inline static const icu4x::Locale& und();

  /**
   * Construct a {@link Locale} "az".
   */
  inline static const icu4x::Locale& az();

  /**
   * Construct a {@link Locale} "el".
   */
  inline static const icu4x::Locale& el();

  /**
   * Construct a {@link Locale} "hy".
   */
  inline static const icu4x::Locale& hy();

  /**
   * Construct a {@link Locale} "lt".
   */
  inline static const icu4x::Locale& lt();

  /**
   * Construct a {@link Locale} "nl".
   */
  inline static const icu4x::Locale& nl();

  /**
   * Construct a {@link Locale} "tr".
   */
  inline static const icu4x::Locale& tr();

    inline const icu4x::capi::LocaleStatics* AsFFI() const;
    inline icu4x::capi::LocaleStatics* AsFFI();
    inline static const icu4x::LocaleStatics* FromFFI(const icu4x::capi::LocaleStatics* ptr);
    inline static icu4x::LocaleStatics* FromFFI(icu4x::capi::LocaleStatics* ptr);
    inline static void operator delete(void* ptr);
private:
    LocaleStatics() = delete;
    LocaleStatics(const icu4x::LocaleStatics&) = delete;
    LocaleStatics(icu4x::LocaleStatics&&) noexcept = delete;
    LocaleStatics operator=(const icu4x::LocaleStatics&) = delete;
    LocaleStatics operator=(icu4x::LocaleStatics&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // ICU4X_LocaleStatics_D_HPP
