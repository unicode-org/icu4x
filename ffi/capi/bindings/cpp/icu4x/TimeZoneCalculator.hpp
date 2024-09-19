#ifndef icu4x_TimeZoneCalculator_HPP
#define icu4x_TimeZoneCalculator_HPP

#include "TimeZoneCalculator.d.hpp"

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
    
    typedef struct icu4x_TimeZoneCalculator_create_mv1_result {union {icu4x::capi::TimeZoneCalculator* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_TimeZoneCalculator_create_mv1_result;
    icu4x_TimeZoneCalculator_create_mv1_result icu4x_TimeZoneCalculator_create_mv1(const icu4x::capi::DataProvider* provider);
    
    
    void icu4x_TimeZoneCalculator_destroy_mv1(TimeZoneCalculator* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<icu4x::TimeZoneCalculator>, icu4x::DataError> icu4x::TimeZoneCalculator::create(const icu4x::DataProvider& provider) {
  auto result = icu4x::capi::icu4x_TimeZoneCalculator_create_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::TimeZoneCalculator>, icu4x::DataError>(diplomat::Ok<std::unique_ptr<icu4x::TimeZoneCalculator>>(std::unique_ptr<icu4x::TimeZoneCalculator>(icu4x::TimeZoneCalculator::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::TimeZoneCalculator>, icu4x::DataError>(diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline const icu4x::capi::TimeZoneCalculator* icu4x::TimeZoneCalculator::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::TimeZoneCalculator*>(this);
}

inline icu4x::capi::TimeZoneCalculator* icu4x::TimeZoneCalculator::AsFFI() {
  return reinterpret_cast<icu4x::capi::TimeZoneCalculator*>(this);
}

inline const icu4x::TimeZoneCalculator* icu4x::TimeZoneCalculator::FromFFI(const icu4x::capi::TimeZoneCalculator* ptr) {
  return reinterpret_cast<const icu4x::TimeZoneCalculator*>(ptr);
}

inline icu4x::TimeZoneCalculator* icu4x::TimeZoneCalculator::FromFFI(icu4x::capi::TimeZoneCalculator* ptr) {
  return reinterpret_cast<icu4x::TimeZoneCalculator*>(ptr);
}

inline void icu4x::TimeZoneCalculator::operator delete(void* ptr) {
  icu4x::capi::icu4x_TimeZoneCalculator_destroy_mv1(reinterpret_cast<icu4x::capi::TimeZoneCalculator*>(ptr));
}


#endif // icu4x_TimeZoneCalculator_HPP
