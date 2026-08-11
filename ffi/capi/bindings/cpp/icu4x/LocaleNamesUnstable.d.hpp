#ifndef ICU4X_LocaleNamesUnstable_D_HPP
#define ICU4X_LocaleNamesUnstable_D_HPP

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
namespace capi { struct DataProvider; }
class DataProvider;
namespace capi { struct Locale; }
class Locale;
class DataError;
class LanguageDisplay;
} // namespace icu4x



namespace icu4x {
namespace capi {
    struct LocaleNamesUnstable;
} // namespace capi
} // namespace

namespace icu4x {
/**
 * See the [Rust documentation for `RegionDisplayName`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.RegionDisplayName.html) for more information.
 *
 * See the [Rust documentation for `ScriptDisplayName`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.ScriptDisplayName.html) for more information.
 *
 * See the [Rust documentation for `VariantDisplayName`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.VariantDisplayName.html) for more information.
 *
 * See the [Rust documentation for `LanguageIdentifierDisplayName`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html) for more information.
 */
class LocaleNamesUnstable {
public:

  /**
   * See the [Rust documentation for `try_new_light`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.RegionDisplayName.html#method.try_new_light) for more information.
   */
  inline static icu4x::diplomat::result<std::string, icu4x::DataError> for_region_light(const icu4x::Locale& locale, std::string_view region);
  template<typename W>
  inline static icu4x::diplomat::result<std::monostate, icu4x::DataError> for_region_light_write(const icu4x::Locale& locale, std::string_view region, W& writeable_output);

  /**
   * See the [Rust documentation for `try_new_light`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.RegionDisplayName.html#method.try_new_light) for more information.
   */
  inline static icu4x::diplomat::result<std::string, icu4x::DataError> for_region_light_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::string_view region);
  template<typename W>
  inline static icu4x::diplomat::result<std::monostate, icu4x::DataError> for_region_light_with_provider_write(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::string_view region, W& writeable_output);

  /**
   * See the [Rust documentation for `try_new_tiny`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.RegionDisplayName.html#method.try_new_tiny) for more information.
   */
  inline static icu4x::diplomat::result<std::string, icu4x::DataError> for_region_tiny(const icu4x::Locale& locale, std::string_view region);
  template<typename W>
  inline static icu4x::diplomat::result<std::monostate, icu4x::DataError> for_region_tiny_write(const icu4x::Locale& locale, std::string_view region, W& writeable_output);

  /**
   * See the [Rust documentation for `try_new_tiny`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.RegionDisplayName.html#method.try_new_tiny) for more information.
   */
  inline static icu4x::diplomat::result<std::string, icu4x::DataError> for_region_tiny_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::string_view region);
  template<typename W>
  inline static icu4x::diplomat::result<std::monostate, icu4x::DataError> for_region_tiny_with_provider_write(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::string_view region, W& writeable_output);

  /**
   * See the [Rust documentation for `try_new_short_tiny`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.RegionDisplayName.html#method.try_new_short_tiny) for more information.
   */
  inline static icu4x::diplomat::result<std::string, icu4x::DataError> for_region_short_tiny(const icu4x::Locale& locale, std::string_view region);
  template<typename W>
  inline static icu4x::diplomat::result<std::monostate, icu4x::DataError> for_region_short_tiny_write(const icu4x::Locale& locale, std::string_view region, W& writeable_output);

  /**
   * See the [Rust documentation for `try_new_short_tiny`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.RegionDisplayName.html#method.try_new_short_tiny) for more information.
   */
  inline static icu4x::diplomat::result<std::string, icu4x::DataError> for_region_short_tiny_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::string_view region);
  template<typename W>
  inline static icu4x::diplomat::result<std::monostate, icu4x::DataError> for_region_short_tiny_with_provider_write(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::string_view region, W& writeable_output);

  /**
   * See the [Rust documentation for `try_new_short_light`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.RegionDisplayName.html#method.try_new_short_light) for more information.
   */
  inline static icu4x::diplomat::result<std::string, icu4x::DataError> for_region_short_light(const icu4x::Locale& locale, std::string_view region);
  template<typename W>
  inline static icu4x::diplomat::result<std::monostate, icu4x::DataError> for_region_short_light_write(const icu4x::Locale& locale, std::string_view region, W& writeable_output);

  /**
   * See the [Rust documentation for `try_new_short_light`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.RegionDisplayName.html#method.try_new_short_light) for more information.
   */
  inline static icu4x::diplomat::result<std::string, icu4x::DataError> for_region_short_light_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::string_view region);
  template<typename W>
  inline static icu4x::diplomat::result<std::monostate, icu4x::DataError> for_region_short_light_with_provider_write(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::string_view region, W& writeable_output);

  /**
   * See the [Rust documentation for `try_new_light`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.ScriptDisplayName.html#method.try_new_light) for more information.
   */
  inline static icu4x::diplomat::result<std::string, icu4x::DataError> for_script_light(const icu4x::Locale& locale, std::string_view script);
  template<typename W>
  inline static icu4x::diplomat::result<std::monostate, icu4x::DataError> for_script_light_write(const icu4x::Locale& locale, std::string_view script, W& writeable_output);

  /**
   * See the [Rust documentation for `try_new_light`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.ScriptDisplayName.html#method.try_new_light) for more information.
   */
  inline static icu4x::diplomat::result<std::string, icu4x::DataError> for_script_light_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::string_view script);
  template<typename W>
  inline static icu4x::diplomat::result<std::monostate, icu4x::DataError> for_script_light_with_provider_write(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::string_view script, W& writeable_output);

  /**
   * See the [Rust documentation for `try_new_tiny`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.ScriptDisplayName.html#method.try_new_tiny) for more information.
   */
  inline static icu4x::diplomat::result<std::string, icu4x::DataError> for_script_tiny(const icu4x::Locale& locale, std::string_view script);
  template<typename W>
  inline static icu4x::diplomat::result<std::monostate, icu4x::DataError> for_script_tiny_write(const icu4x::Locale& locale, std::string_view script, W& writeable_output);

  /**
   * See the [Rust documentation for `try_new_tiny`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.ScriptDisplayName.html#method.try_new_tiny) for more information.
   */
  inline static icu4x::diplomat::result<std::string, icu4x::DataError> for_script_tiny_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::string_view script);
  template<typename W>
  inline static icu4x::diplomat::result<std::monostate, icu4x::DataError> for_script_tiny_with_provider_write(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::string_view script, W& writeable_output);

  /**
   * See the [Rust documentation for `try_new_heavy`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.ScriptDisplayName.html#method.try_new_heavy) for more information.
   */
  inline static icu4x::diplomat::result<std::string, icu4x::DataError> for_script_heavy(const icu4x::Locale& locale, std::string_view script);
  template<typename W>
  inline static icu4x::diplomat::result<std::monostate, icu4x::DataError> for_script_heavy_write(const icu4x::Locale& locale, std::string_view script, W& writeable_output);

  /**
   * See the [Rust documentation for `try_new_heavy`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.ScriptDisplayName.html#method.try_new_heavy) for more information.
   */
  inline static icu4x::diplomat::result<std::string, icu4x::DataError> for_script_heavy_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::string_view script);
  template<typename W>
  inline static icu4x::diplomat::result<std::monostate, icu4x::DataError> for_script_heavy_with_provider_write(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::string_view script, W& writeable_output);

  /**
   * See the [Rust documentation for `try_new_short_heavy`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.ScriptDisplayName.html#method.try_new_short_heavy) for more information.
   */
  inline static icu4x::diplomat::result<std::string, icu4x::DataError> for_script_short_heavy(const icu4x::Locale& locale, std::string_view script);
  template<typename W>
  inline static icu4x::diplomat::result<std::monostate, icu4x::DataError> for_script_short_heavy_write(const icu4x::Locale& locale, std::string_view script, W& writeable_output);

  /**
   * See the [Rust documentation for `try_new_short_heavy`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.ScriptDisplayName.html#method.try_new_short_heavy) for more information.
   */
  inline static icu4x::diplomat::result<std::string, icu4x::DataError> for_script_short_heavy_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::string_view script);
  template<typename W>
  inline static icu4x::diplomat::result<std::monostate, icu4x::DataError> for_script_short_heavy_with_provider_write(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::string_view script, W& writeable_output);

  /**
   * See the [Rust documentation for `try_new_heavy`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.VariantDisplayName.html#method.try_new_heavy) for more information.
   */
  inline static icu4x::diplomat::result<std::string, icu4x::DataError> for_variant_heavy(const icu4x::Locale& locale, std::string_view variant);
  template<typename W>
  inline static icu4x::diplomat::result<std::monostate, icu4x::DataError> for_variant_heavy_write(const icu4x::Locale& locale, std::string_view variant, W& writeable_output);

  /**
   * See the [Rust documentation for `try_new_heavy`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.VariantDisplayName.html#method.try_new_heavy) for more information.
   */
  inline static icu4x::diplomat::result<std::string, icu4x::DataError> for_variant_heavy_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::string_view variant);
  template<typename W>
  inline static icu4x::diplomat::result<std::monostate, icu4x::DataError> for_variant_heavy_with_provider_write(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::string_view variant, W& writeable_output);

  /**
   * See the [Rust documentation for `try_new_light`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_light) for more information.
   */
  inline static icu4x::diplomat::result<std::string, icu4x::DataError> for_language_identifier_light(const icu4x::Locale& locale, const icu4x::Locale& langid, icu4x::LanguageDisplay language_display);
  template<typename W>
  inline static icu4x::diplomat::result<std::monostate, icu4x::DataError> for_language_identifier_light_write(const icu4x::Locale& locale, const icu4x::Locale& langid, icu4x::LanguageDisplay language_display, W& writeable_output);

  /**
   * See the [Rust documentation for `try_new_light`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_light) for more information.
   */
  inline static icu4x::diplomat::result<std::string, icu4x::DataError> for_language_identifier_light_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::Locale& langid, icu4x::LanguageDisplay language_display);
  template<typename W>
  inline static icu4x::diplomat::result<std::monostate, icu4x::DataError> for_language_identifier_light_with_provider_write(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::Locale& langid, icu4x::LanguageDisplay language_display, W& writeable_output);

  /**
   * See the [Rust documentation for `try_new_tiny`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_tiny) for more information.
   */
  inline static icu4x::diplomat::result<std::string, icu4x::DataError> for_language_identifier_tiny(const icu4x::Locale& locale, const icu4x::Locale& langid, icu4x::LanguageDisplay language_display);
  template<typename W>
  inline static icu4x::diplomat::result<std::monostate, icu4x::DataError> for_language_identifier_tiny_write(const icu4x::Locale& locale, const icu4x::Locale& langid, icu4x::LanguageDisplay language_display, W& writeable_output);

  /**
   * See the [Rust documentation for `try_new_tiny`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_tiny) for more information.
   */
  inline static icu4x::diplomat::result<std::string, icu4x::DataError> for_language_identifier_tiny_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::Locale& langid, icu4x::LanguageDisplay language_display);
  template<typename W>
  inline static icu4x::diplomat::result<std::monostate, icu4x::DataError> for_language_identifier_tiny_with_provider_write(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::Locale& langid, icu4x::LanguageDisplay language_display, W& writeable_output);

  /**
   * See the [Rust documentation for `try_new_short_light`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_short_light) for more information.
   */
  inline static icu4x::diplomat::result<std::string, icu4x::DataError> for_language_identifier_short_light(const icu4x::Locale& locale, const icu4x::Locale& langid, icu4x::LanguageDisplay language_display);
  template<typename W>
  inline static icu4x::diplomat::result<std::monostate, icu4x::DataError> for_language_identifier_short_light_write(const icu4x::Locale& locale, const icu4x::Locale& langid, icu4x::LanguageDisplay language_display, W& writeable_output);

  /**
   * See the [Rust documentation for `try_new_short_light`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_short_light) for more information.
   */
  inline static icu4x::diplomat::result<std::string, icu4x::DataError> for_language_identifier_short_light_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::Locale& langid, icu4x::LanguageDisplay language_display);
  template<typename W>
  inline static icu4x::diplomat::result<std::monostate, icu4x::DataError> for_language_identifier_short_light_with_provider_write(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::Locale& langid, icu4x::LanguageDisplay language_display, W& writeable_output);

  /**
   * See the [Rust documentation for `try_new_long_light`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_long_light) for more information.
   */
  inline static icu4x::diplomat::result<std::string, icu4x::DataError> for_language_identifier_long_light(const icu4x::Locale& locale, const icu4x::Locale& langid, icu4x::LanguageDisplay language_display);
  template<typename W>
  inline static icu4x::diplomat::result<std::monostate, icu4x::DataError> for_language_identifier_long_light_write(const icu4x::Locale& locale, const icu4x::Locale& langid, icu4x::LanguageDisplay language_display, W& writeable_output);

  /**
   * See the [Rust documentation for `try_new_long_light`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_long_light) for more information.
   */
  inline static icu4x::diplomat::result<std::string, icu4x::DataError> for_language_identifier_long_light_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::Locale& langid, icu4x::LanguageDisplay language_display);
  template<typename W>
  inline static icu4x::diplomat::result<std::monostate, icu4x::DataError> for_language_identifier_long_light_with_provider_write(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::Locale& langid, icu4x::LanguageDisplay language_display, W& writeable_output);

  /**
   * See the [Rust documentation for `try_new_menu_light`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_menu_light) for more information.
   */
  inline static icu4x::diplomat::result<std::string, icu4x::DataError> for_language_identifier_menu_light(const icu4x::Locale& locale, const icu4x::Locale& langid, icu4x::LanguageDisplay language_display);
  template<typename W>
  inline static icu4x::diplomat::result<std::monostate, icu4x::DataError> for_language_identifier_menu_light_write(const icu4x::Locale& locale, const icu4x::Locale& langid, icu4x::LanguageDisplay language_display, W& writeable_output);

  /**
   * See the [Rust documentation for `try_new_menu_light`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_menu_light) for more information.
   */
  inline static icu4x::diplomat::result<std::string, icu4x::DataError> for_language_identifier_menu_light_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::Locale& langid, icu4x::LanguageDisplay language_display);
  template<typename W>
  inline static icu4x::diplomat::result<std::monostate, icu4x::DataError> for_language_identifier_menu_light_with_provider_write(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::Locale& langid, icu4x::LanguageDisplay language_display, W& writeable_output);

  /**
   * See the [Rust documentation for `try_new_short_menu_light`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_short_menu_light) for more information.
   */
  inline static icu4x::diplomat::result<std::string, icu4x::DataError> for_language_identifier_short_menu_light(const icu4x::Locale& locale, const icu4x::Locale& langid, icu4x::LanguageDisplay language_display);
  template<typename W>
  inline static icu4x::diplomat::result<std::monostate, icu4x::DataError> for_language_identifier_short_menu_light_write(const icu4x::Locale& locale, const icu4x::Locale& langid, icu4x::LanguageDisplay language_display, W& writeable_output);

  /**
   * See the [Rust documentation for `try_new_short_menu_light`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_short_menu_light) for more information.
   */
  inline static icu4x::diplomat::result<std::string, icu4x::DataError> for_language_identifier_short_menu_light_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::Locale& langid, icu4x::LanguageDisplay language_display);
  template<typename W>
  inline static icu4x::diplomat::result<std::monostate, icu4x::DataError> for_language_identifier_short_menu_light_with_provider_write(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::Locale& langid, icu4x::LanguageDisplay language_display, W& writeable_output);

  /**
   * See the [Rust documentation for `try_new_heavy`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_heavy) for more information.
   */
  inline static icu4x::diplomat::result<std::string, icu4x::DataError> for_language_identifier_heavy(const icu4x::Locale& locale, const icu4x::Locale& langid, icu4x::LanguageDisplay language_display);
  template<typename W>
  inline static icu4x::diplomat::result<std::monostate, icu4x::DataError> for_language_identifier_heavy_write(const icu4x::Locale& locale, const icu4x::Locale& langid, icu4x::LanguageDisplay language_display, W& writeable_output);

  /**
   * See the [Rust documentation for `try_new_heavy`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_heavy) for more information.
   */
  inline static icu4x::diplomat::result<std::string, icu4x::DataError> for_language_identifier_heavy_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::Locale& langid, icu4x::LanguageDisplay language_display);
  template<typename W>
  inline static icu4x::diplomat::result<std::monostate, icu4x::DataError> for_language_identifier_heavy_with_provider_write(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::Locale& langid, icu4x::LanguageDisplay language_display, W& writeable_output);

  /**
   * See the [Rust documentation for `try_new_short_heavy`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_short_heavy) for more information.
   */
  inline static icu4x::diplomat::result<std::string, icu4x::DataError> for_language_identifier_short_heavy(const icu4x::Locale& locale, const icu4x::Locale& langid, icu4x::LanguageDisplay language_display);
  template<typename W>
  inline static icu4x::diplomat::result<std::monostate, icu4x::DataError> for_language_identifier_short_heavy_write(const icu4x::Locale& locale, const icu4x::Locale& langid, icu4x::LanguageDisplay language_display, W& writeable_output);

  /**
   * See the [Rust documentation for `try_new_short_heavy`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_short_heavy) for more information.
   */
  inline static icu4x::diplomat::result<std::string, icu4x::DataError> for_language_identifier_short_heavy_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::Locale& langid, icu4x::LanguageDisplay language_display);
  template<typename W>
  inline static icu4x::diplomat::result<std::monostate, icu4x::DataError> for_language_identifier_short_heavy_with_provider_write(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::Locale& langid, icu4x::LanguageDisplay language_display, W& writeable_output);

  /**
   * See the [Rust documentation for `try_new_long_heavy`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_long_heavy) for more information.
   */
  inline static icu4x::diplomat::result<std::string, icu4x::DataError> for_language_identifier_long_heavy(const icu4x::Locale& locale, const icu4x::Locale& langid, icu4x::LanguageDisplay language_display);
  template<typename W>
  inline static icu4x::diplomat::result<std::monostate, icu4x::DataError> for_language_identifier_long_heavy_write(const icu4x::Locale& locale, const icu4x::Locale& langid, icu4x::LanguageDisplay language_display, W& writeable_output);

  /**
   * See the [Rust documentation for `try_new_long_heavy`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_long_heavy) for more information.
   */
  inline static icu4x::diplomat::result<std::string, icu4x::DataError> for_language_identifier_long_heavy_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::Locale& langid, icu4x::LanguageDisplay language_display);
  template<typename W>
  inline static icu4x::diplomat::result<std::monostate, icu4x::DataError> for_language_identifier_long_heavy_with_provider_write(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::Locale& langid, icu4x::LanguageDisplay language_display, W& writeable_output);

  /**
   * See the [Rust documentation for `try_new_menu_heavy`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_menu_heavy) for more information.
   */
  inline static icu4x::diplomat::result<std::string, icu4x::DataError> for_language_identifier_menu_heavy(const icu4x::Locale& locale, const icu4x::Locale& langid, icu4x::LanguageDisplay language_display);
  template<typename W>
  inline static icu4x::diplomat::result<std::monostate, icu4x::DataError> for_language_identifier_menu_heavy_write(const icu4x::Locale& locale, const icu4x::Locale& langid, icu4x::LanguageDisplay language_display, W& writeable_output);

  /**
   * See the [Rust documentation for `try_new_menu_heavy`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_menu_heavy) for more information.
   */
  inline static icu4x::diplomat::result<std::string, icu4x::DataError> for_language_identifier_menu_heavy_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::Locale& langid, icu4x::LanguageDisplay language_display);
  template<typename W>
  inline static icu4x::diplomat::result<std::monostate, icu4x::DataError> for_language_identifier_menu_heavy_with_provider_write(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::Locale& langid, icu4x::LanguageDisplay language_display, W& writeable_output);

  /**
   * See the [Rust documentation for `try_new_short_menu_heavy`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_short_menu_heavy) for more information.
   */
  inline static icu4x::diplomat::result<std::string, icu4x::DataError> for_language_identifier_short_menu_heavy(const icu4x::Locale& locale, const icu4x::Locale& langid, icu4x::LanguageDisplay language_display);
  template<typename W>
  inline static icu4x::diplomat::result<std::monostate, icu4x::DataError> for_language_identifier_short_menu_heavy_write(const icu4x::Locale& locale, const icu4x::Locale& langid, icu4x::LanguageDisplay language_display, W& writeable_output);

  /**
   * See the [Rust documentation for `try_new_short_menu_heavy`](https://docs.rs/icu/2.2.0/icu/locale/names/struct.LanguageIdentifierDisplayName.html#method.try_new_short_menu_heavy) for more information.
   */
  inline static icu4x::diplomat::result<std::string, icu4x::DataError> for_language_identifier_short_menu_heavy_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::Locale& langid, icu4x::LanguageDisplay language_display);
  template<typename W>
  inline static icu4x::diplomat::result<std::monostate, icu4x::DataError> for_language_identifier_short_menu_heavy_with_provider_write(const icu4x::DataProvider& provider, const icu4x::Locale& locale, const icu4x::Locale& langid, icu4x::LanguageDisplay language_display, W& writeable_output);

    inline const icu4x::capi::LocaleNamesUnstable* AsFFI() const;
    inline icu4x::capi::LocaleNamesUnstable* AsFFI();
    inline static const icu4x::LocaleNamesUnstable* FromFFI(const icu4x::capi::LocaleNamesUnstable* ptr);
    inline static icu4x::LocaleNamesUnstable* FromFFI(icu4x::capi::LocaleNamesUnstable* ptr);
    inline static void operator delete(void* ptr);
private:
    LocaleNamesUnstable() = delete;
    LocaleNamesUnstable(const icu4x::LocaleNamesUnstable&) = delete;
    LocaleNamesUnstable(icu4x::LocaleNamesUnstable&&) noexcept = delete;
    LocaleNamesUnstable operator=(const icu4x::LocaleNamesUnstable&) = delete;
    LocaleNamesUnstable operator=(icu4x::LocaleNamesUnstable&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // ICU4X_LocaleNamesUnstable_D_HPP
