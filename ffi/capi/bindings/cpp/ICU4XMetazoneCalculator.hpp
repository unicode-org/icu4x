#ifndef ICU4XMetazoneCalculator_HPP
#define ICU4XMetazoneCalculator_HPP

#include "ICU4XMetazoneCalculator.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataProvider.hpp"
#include "ICU4XError.hpp"
#include "ICU4XMetazoneCalculator.h"


inline diplomat::result<std::unique_ptr<ICU4XMetazoneCalculator>, ICU4XError> ICU4XMetazoneCalculator::create(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XMetazoneCalculator_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XMetazoneCalculator>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XMetazoneCalculator>>(std::unique_ptr<ICU4XMetazoneCalculator>(ICU4XMetazoneCalculator::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XMetazoneCalculator>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline const capi::ICU4XMetazoneCalculator* ICU4XMetazoneCalculator::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XMetazoneCalculator*>(this);
}

inline capi::ICU4XMetazoneCalculator* ICU4XMetazoneCalculator::AsFFI() {
  return reinterpret_cast<capi::ICU4XMetazoneCalculator*>(this);
}

inline const ICU4XMetazoneCalculator* ICU4XMetazoneCalculator::FromFFI(const capi::ICU4XMetazoneCalculator* ptr) {
  return reinterpret_cast<const ICU4XMetazoneCalculator*>(ptr);
}

inline ICU4XMetazoneCalculator* ICU4XMetazoneCalculator::FromFFI(capi::ICU4XMetazoneCalculator* ptr) {
  return reinterpret_cast<ICU4XMetazoneCalculator*>(ptr);
}

inline void ICU4XMetazoneCalculator::operator delete(void* ptr) {
  capi::ICU4XMetazoneCalculator_destroy(reinterpret_cast<capi::ICU4XMetazoneCalculator*>(ptr));
}


#endif // ICU4XMetazoneCalculator_HPP
