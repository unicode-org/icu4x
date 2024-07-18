#ifndef Locale_D_HPP
#define Locale_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

class LocaleParseError;


namespace diplomat {
namespace capi {
    struct Locale;
} // namespace capi
} // namespace

class Locale {
public:

  inline static diplomat::result<std::unique_ptr<Locale>, LocaleParseError> from_string(std::string_view name);

  inline static std::unique_ptr<Locale> und();

  inline std::unique_ptr<Locale> clone() const;

  inline std::string basename() const;

  inline std::optional<std::string> get_unicode_extension(std::string_view s) const;

  inline std::string language() const;

  inline diplomat::result<std::monostate, LocaleParseError> set_language(std::string_view s);

  inline std::optional<std::string> region() const;

  inline diplomat::result<std::monostate, LocaleParseError> set_region(std::string_view s);

  inline std::optional<std::string> script() const;

  inline diplomat::result<std::monostate, LocaleParseError> set_script(std::string_view s);

  inline static diplomat::result<std::string, LocaleParseError> canonicalize(std::string_view s);

  inline std::string to_string() const;

  inline bool normalizing_eq(std::string_view other) const;

  inline int8_t compare_to_string(std::string_view other) const;

  inline int8_t compare_to(const Locale& other) const;

  inline const diplomat::capi::Locale* AsFFI() const;
  inline diplomat::capi::Locale* AsFFI();
  inline static const Locale* FromFFI(const diplomat::capi::Locale* ptr);
  inline static Locale* FromFFI(diplomat::capi::Locale* ptr);
  inline static void operator delete(void* ptr);
private:
  Locale() = delete;
  Locale(const Locale&) = delete;
  Locale(Locale&&) noexcept = delete;
  Locale operator=(const Locale&) = delete;
  Locale operator=(Locale&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // Locale_D_HPP
