#ifndef ICU4XLocale_D_HPP
#define ICU4XLocale_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XLocaleParseError.d.hpp"

class ICU4XLocaleParseError;


namespace capi {
    typedef struct ICU4XLocale ICU4XLocale;
}

class ICU4XLocale {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XLocale>, ICU4XLocaleParseError> create_from_string(std::string_view name);

  inline static std::unique_ptr<ICU4XLocale> create_und();

  inline std::unique_ptr<ICU4XLocale> clone() const;

  inline std::string basename() const;

  inline std::optional<std::string> get_unicode_extension(std::string_view s) const;

  inline std::string language() const;

  inline diplomat::result<std::monostate, ICU4XLocaleParseError> set_language(std::string_view s);

  inline std::optional<std::string> region() const;

  inline diplomat::result<std::monostate, ICU4XLocaleParseError> set_region(std::string_view s);

  inline std::optional<std::string> script() const;

  inline diplomat::result<std::monostate, ICU4XLocaleParseError> set_script(std::string_view s);

  inline static diplomat::result<std::string, ICU4XLocaleParseError> canonicalize(std::string_view s);

  inline std::string to_string() const;

  inline bool normalizing_eq(std::string_view other) const;

  inline int8_t strict_cmp_(std::string_view other) const;

  inline int8_t total_cmp_(const ICU4XLocale& other) const;

  inline const capi::ICU4XLocale* AsFFI() const;
  inline capi::ICU4XLocale* AsFFI();
  inline static const ICU4XLocale* FromFFI(const capi::ICU4XLocale* ptr);
  inline static ICU4XLocale* FromFFI(capi::ICU4XLocale* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XLocale() = delete;
  ICU4XLocale(const ICU4XLocale&) = delete;
  ICU4XLocale(ICU4XLocale&&) noexcept = delete;
  ICU4XLocale operator=(const ICU4XLocale&) = delete;
  ICU4XLocale operator=(ICU4XLocale&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XLocale_D_HPP
