#ifndef ICU4XMetazoneCalculator_D_HPP
#define ICU4XMetazoneCalculator_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XError.d.hpp"
#include "ICU4XMetazoneCalculator.d.h"

class ICU4XDataProvider;
class ICU4XError;


class ICU4XMetazoneCalculator {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XMetazoneCalculator>, ICU4XError> create(const ICU4XDataProvider& provider);

  inline const capi::ICU4XMetazoneCalculator* AsFFI() const;
  inline capi::ICU4XMetazoneCalculator* AsFFI();
  inline static const ICU4XMetazoneCalculator* FromFFI(const capi::ICU4XMetazoneCalculator* ptr);
  inline static ICU4XMetazoneCalculator* FromFFI(capi::ICU4XMetazoneCalculator* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XMetazoneCalculator() = delete;
  ICU4XMetazoneCalculator(const ICU4XMetazoneCalculator&) = delete;
  ICU4XMetazoneCalculator(ICU4XMetazoneCalculator&&) noexcept = delete;
  ICU4XMetazoneCalculator operator=(const ICU4XMetazoneCalculator&) = delete;
  ICU4XMetazoneCalculator operator=(ICU4XMetazoneCalculator&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XMetazoneCalculator_D_HPP
