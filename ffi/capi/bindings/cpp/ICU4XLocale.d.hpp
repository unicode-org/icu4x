#ifndef ICU4XLocale_D_HPP
#define ICU4XLocale_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XError.d.hpp"
#include "ICU4XLocale.d.h"

class ICU4XError;


class ICU4XLocale {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XLocale>, ICU4XError> create_from_string(std::string_view name);

  inline static std::unique_ptr<ICU4XLocale> create_und();

  inline std::unique_ptr<ICU4XLocale> clone() const;

  inline std::string basename() const;

  inline std::optional<std::string> get_unicode_extension(std::string_view bytes) const;

  inline std::string language() const;

  inline diplomat::result<std::monostate, ICU4XError> set_language(std::string_view bytes);

  inline std::optional<std::string> region() const;

  inline diplomat::result<std::monostate, ICU4XError> set_region(std::string_view bytes);

  inline std::optional<std::string> script() const;

  inline diplomat::result<std::monostate, ICU4XError> set_script(std::string_view bytes);

  inline static diplomat::result<std::string, ICU4XError> canonicalize(std::string_view bytes);

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
