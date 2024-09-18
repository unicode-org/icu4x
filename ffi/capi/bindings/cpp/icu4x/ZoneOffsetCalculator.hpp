#ifndef icu4x_ZoneOffsetCalculator_HPP
#define icu4x_ZoneOffsetCalculator_HPP

#include "ZoneOffsetCalculator.d.hpp"

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
    
    typedef struct icu4x_ZoneOffsetCalculator_create_mv1_result {union {icu4x::capi::ZoneOffsetCalculator* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_ZoneOffsetCalculator_create_mv1_result;
    icu4x_ZoneOffsetCalculator_create_mv1_result icu4x_ZoneOffsetCalculator_create_mv1(const icu4x::capi::DataProvider* provider);
    
    
    void icu4x_ZoneOffsetCalculator_destroy_mv1(ZoneOffsetCalculator* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<icu4x::ZoneOffsetCalculator>, icu4x::DataError> icu4x::ZoneOffsetCalculator::create(const icu4x::DataProvider& provider) {
  auto result = icu4x::capi::icu4x_ZoneOffsetCalculator_create_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::ZoneOffsetCalculator>, icu4x::DataError>(diplomat::Ok<std::unique_ptr<icu4x::ZoneOffsetCalculator>>(std::unique_ptr<icu4x::ZoneOffsetCalculator>(icu4x::ZoneOffsetCalculator::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::ZoneOffsetCalculator>, icu4x::DataError>(diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline const icu4x::capi::ZoneOffsetCalculator* icu4x::ZoneOffsetCalculator::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::ZoneOffsetCalculator*>(this);
}

inline icu4x::capi::ZoneOffsetCalculator* icu4x::ZoneOffsetCalculator::AsFFI() {
  return reinterpret_cast<icu4x::capi::ZoneOffsetCalculator*>(this);
}

inline const icu4x::ZoneOffsetCalculator* icu4x::ZoneOffsetCalculator::FromFFI(const icu4x::capi::ZoneOffsetCalculator* ptr) {
  return reinterpret_cast<const icu4x::ZoneOffsetCalculator*>(ptr);
}

inline icu4x::ZoneOffsetCalculator* icu4x::ZoneOffsetCalculator::FromFFI(icu4x::capi::ZoneOffsetCalculator* ptr) {
  return reinterpret_cast<icu4x::ZoneOffsetCalculator*>(ptr);
}

inline void icu4x::ZoneOffsetCalculator::operator delete(void* ptr) {
  icu4x::capi::icu4x_ZoneOffsetCalculator_destroy_mv1(reinterpret_cast<icu4x::capi::ZoneOffsetCalculator*>(ptr));
}


#endif // icu4x_ZoneOffsetCalculator_HPP
