#ifndef ICU4XIanaToBcp47Mapper_D_HPP
#define ICU4XIanaToBcp47Mapper_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XError.d.hpp"
#include "ICU4XIanaToBcp47Mapper.d.h"

class ICU4XDataProvider;
class ICU4XError;


class ICU4XIanaToBcp47Mapper {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XIanaToBcp47Mapper>, ICU4XError> create(const ICU4XDataProvider& provider);

  inline diplomat::result<std::string, ICU4XError> get(std::string_view value) const;

  inline const capi::ICU4XIanaToBcp47Mapper* AsFFI() const;
  inline capi::ICU4XIanaToBcp47Mapper* AsFFI();
  inline static const ICU4XIanaToBcp47Mapper* FromFFI(const capi::ICU4XIanaToBcp47Mapper* ptr);
  inline static ICU4XIanaToBcp47Mapper* FromFFI(capi::ICU4XIanaToBcp47Mapper* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XIanaToBcp47Mapper() = delete;
  ICU4XIanaToBcp47Mapper(const ICU4XIanaToBcp47Mapper&) = delete;
  ICU4XIanaToBcp47Mapper(ICU4XIanaToBcp47Mapper&&) noexcept = delete;
  ICU4XIanaToBcp47Mapper operator=(const ICU4XIanaToBcp47Mapper&) = delete;
  ICU4XIanaToBcp47Mapper operator=(ICU4XIanaToBcp47Mapper&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XIanaToBcp47Mapper_D_HPP
