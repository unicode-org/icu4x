#ifndef ICU4XBcp47ToIanaMapper_D_HPP
#define ICU4XBcp47ToIanaMapper_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XBcp47ToIanaMapper.d.h"
#include "ICU4XError.d.hpp"

class ICU4XDataProvider;
class ICU4XError;


class ICU4XBcp47ToIanaMapper {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XBcp47ToIanaMapper>, ICU4XError> create(const ICU4XDataProvider& provider);

  inline diplomat::result<std::string, ICU4XError> get(std::string_view value) const;

  inline const capi::ICU4XBcp47ToIanaMapper* AsFFI() const;
  inline capi::ICU4XBcp47ToIanaMapper* AsFFI();
  inline static const ICU4XBcp47ToIanaMapper* FromFFI(const capi::ICU4XBcp47ToIanaMapper* ptr);
  inline static ICU4XBcp47ToIanaMapper* FromFFI(capi::ICU4XBcp47ToIanaMapper* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XBcp47ToIanaMapper() = delete;
  ICU4XBcp47ToIanaMapper(const ICU4XBcp47ToIanaMapper&) = delete;
  ICU4XBcp47ToIanaMapper(ICU4XBcp47ToIanaMapper&&) noexcept = delete;
  ICU4XBcp47ToIanaMapper operator=(const ICU4XBcp47ToIanaMapper&) = delete;
  ICU4XBcp47ToIanaMapper operator=(ICU4XBcp47ToIanaMapper&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XBcp47ToIanaMapper_D_HPP
