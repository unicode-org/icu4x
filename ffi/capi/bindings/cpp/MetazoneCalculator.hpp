#ifndef MetazoneCalculator_HPP
#define MetazoneCalculator_HPP

#include "MetazoneCalculator.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"


namespace capi {
    extern "C" {
    
    typedef struct ICU4XMetazoneCalculator_create_result {union {MetazoneCalculator* ok; DataError err;}; bool is_ok;} ICU4XMetazoneCalculator_create_result;
    ICU4XMetazoneCalculator_create_result ICU4XMetazoneCalculator_create(const DataProvider* provider);
    
    
    void ICU4XMetazoneCalculator_destroy(MetazoneCalculator* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<MetazoneCalculator>, DataError> MetazoneCalculator::create(const DataProvider& provider) {
  auto result = capi::ICU4XMetazoneCalculator_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<MetazoneCalculator>, DataError>(diplomat::Ok<std::unique_ptr<MetazoneCalculator>>(std::unique_ptr<MetazoneCalculator>(MetazoneCalculator::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<MetazoneCalculator>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline const capi::MetazoneCalculator* MetazoneCalculator::AsFFI() const {
  return reinterpret_cast<const capi::MetazoneCalculator*>(this);
}

inline capi::MetazoneCalculator* MetazoneCalculator::AsFFI() {
  return reinterpret_cast<capi::MetazoneCalculator*>(this);
}

inline const MetazoneCalculator* MetazoneCalculator::FromFFI(const capi::MetazoneCalculator* ptr) {
  return reinterpret_cast<const MetazoneCalculator*>(ptr);
}

inline MetazoneCalculator* MetazoneCalculator::FromFFI(capi::MetazoneCalculator* ptr) {
  return reinterpret_cast<MetazoneCalculator*>(ptr);
}

inline void MetazoneCalculator::operator delete(void* ptr) {
  capi::ICU4XMetazoneCalculator_destroy(reinterpret_cast<capi::MetazoneCalculator*>(ptr));
}


#endif // MetazoneCalculator_HPP
