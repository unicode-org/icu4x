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


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_MetazoneCalculator_create_mv1_result {union {diplomat::capi::MetazoneCalculator* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_MetazoneCalculator_create_mv1_result;
    icu4x_MetazoneCalculator_create_mv1_result icu4x_MetazoneCalculator_create_mv1(const diplomat::capi::DataProvider* provider);
    
    
    void icu4x_MetazoneCalculator_destroy_mv1(MetazoneCalculator* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<MetazoneCalculator>, DataError> MetazoneCalculator::create(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_MetazoneCalculator_create_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<MetazoneCalculator>, DataError>(diplomat::Ok<std::unique_ptr<MetazoneCalculator>>(std::unique_ptr<MetazoneCalculator>(MetazoneCalculator::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<MetazoneCalculator>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline const diplomat::capi::MetazoneCalculator* MetazoneCalculator::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::MetazoneCalculator*>(this);
}

inline diplomat::capi::MetazoneCalculator* MetazoneCalculator::AsFFI() {
  return reinterpret_cast<diplomat::capi::MetazoneCalculator*>(this);
}

inline const MetazoneCalculator* MetazoneCalculator::FromFFI(const diplomat::capi::MetazoneCalculator* ptr) {
  return reinterpret_cast<const MetazoneCalculator*>(ptr);
}

inline MetazoneCalculator* MetazoneCalculator::FromFFI(diplomat::capi::MetazoneCalculator* ptr) {
  return reinterpret_cast<MetazoneCalculator*>(ptr);
}

inline void MetazoneCalculator::operator delete(void* ptr) {
  diplomat::capi::icu4x_MetazoneCalculator_destroy_mv1(reinterpret_cast<diplomat::capi::MetazoneCalculator*>(ptr));
}


#endif // MetazoneCalculator_HPP
