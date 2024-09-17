#ifndef icu4x_MetazoneCalculator_HPP
#define icu4x_MetazoneCalculator_HPP

#include "MetazoneCalculator.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_MetazoneCalculator_create_mv1_result {union {icu4x::capi::MetazoneCalculator* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_MetazoneCalculator_create_mv1_result;
    icu4x_MetazoneCalculator_create_mv1_result icu4x_MetazoneCalculator_create_mv1(const icu4x::capi::DataProvider* provider);
    
    
    void icu4x_MetazoneCalculator_destroy_mv1(MetazoneCalculator* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<icu4x::MetazoneCalculator>, icu4x::DataError> icu4x::MetazoneCalculator::create(const icu4x::DataProvider& provider) {
  auto result = icu4x::capi::icu4x_MetazoneCalculator_create_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::MetazoneCalculator>, icu4x::DataError>(diplomat::Ok<std::unique_ptr<icu4x::MetazoneCalculator>>(std::unique_ptr<icu4x::MetazoneCalculator>(icu4x::MetazoneCalculator::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::MetazoneCalculator>, icu4x::DataError>(diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline const icu4x::capi::MetazoneCalculator* icu4x::MetazoneCalculator::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::MetazoneCalculator*>(this);
}

inline icu4x::capi::MetazoneCalculator* icu4x::MetazoneCalculator::AsFFI() {
  return reinterpret_cast<icu4x::capi::MetazoneCalculator*>(this);
}

inline const icu4x::MetazoneCalculator* icu4x::MetazoneCalculator::FromFFI(const icu4x::capi::MetazoneCalculator* ptr) {
  return reinterpret_cast<const icu4x::MetazoneCalculator*>(ptr);
}

inline icu4x::MetazoneCalculator* icu4x::MetazoneCalculator::FromFFI(icu4x::capi::MetazoneCalculator* ptr) {
  return reinterpret_cast<icu4x::MetazoneCalculator*>(ptr);
}

inline void icu4x::MetazoneCalculator::operator delete(void* ptr) {
  icu4x::capi::icu4x_MetazoneCalculator_destroy_mv1(reinterpret_cast<icu4x::capi::MetazoneCalculator*>(ptr));
}


#endif // icu4x_MetazoneCalculator_HPP
