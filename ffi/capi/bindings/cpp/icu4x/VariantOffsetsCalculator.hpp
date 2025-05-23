#ifndef icu4x_VariantOffsetsCalculator_HPP
#define icu4x_VariantOffsetsCalculator_HPP

#include "VariantOffsetsCalculator.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "IsoDate.hpp"
#include "Time.hpp"
#include "TimeZone.hpp"
#include "VariantOffsets.hpp"


namespace icu4x {
namespace capi {
    extern "C" {

    icu4x::capi::VariantOffsetsCalculator* icu4x_VariantOffsetsCalculator_create_mv1(void);

    typedef struct icu4x_VariantOffsetsCalculator_create_with_provider_mv1_result {union {icu4x::capi::VariantOffsetsCalculator* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_VariantOffsetsCalculator_create_with_provider_mv1_result;
    icu4x_VariantOffsetsCalculator_create_with_provider_mv1_result icu4x_VariantOffsetsCalculator_create_with_provider_mv1(const icu4x::capi::DataProvider* provider);

    typedef struct icu4x_VariantOffsetsCalculator_compute_offsets_from_time_zone_and_date_time_mv1_result {union {icu4x::capi::VariantOffsets ok; }; bool is_ok;} icu4x_VariantOffsetsCalculator_compute_offsets_from_time_zone_and_date_time_mv1_result;
    icu4x_VariantOffsetsCalculator_compute_offsets_from_time_zone_and_date_time_mv1_result icu4x_VariantOffsetsCalculator_compute_offsets_from_time_zone_and_date_time_mv1(const icu4x::capi::VariantOffsetsCalculator* self, const icu4x::capi::TimeZone* time_zone, const icu4x::capi::IsoDate* local_date, const icu4x::capi::Time* local_time);

    void icu4x_VariantOffsetsCalculator_destroy_mv1(VariantOffsetsCalculator* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<icu4x::VariantOffsetsCalculator> icu4x::VariantOffsetsCalculator::create() {
  auto result = icu4x::capi::icu4x_VariantOffsetsCalculator_create_mv1();
  return std::unique_ptr<icu4x::VariantOffsetsCalculator>(icu4x::VariantOffsetsCalculator::FromFFI(result));
}

inline diplomat::result<std::unique_ptr<icu4x::VariantOffsetsCalculator>, icu4x::DataError> icu4x::VariantOffsetsCalculator::create_with_provider(const icu4x::DataProvider& provider) {
  auto result = icu4x::capi::icu4x_VariantOffsetsCalculator_create_with_provider_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::VariantOffsetsCalculator>, icu4x::DataError>(diplomat::Ok<std::unique_ptr<icu4x::VariantOffsetsCalculator>>(std::unique_ptr<icu4x::VariantOffsetsCalculator>(icu4x::VariantOffsetsCalculator::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::VariantOffsetsCalculator>, icu4x::DataError>(diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline std::optional<icu4x::VariantOffsets> icu4x::VariantOffsetsCalculator::compute_offsets_from_time_zone_and_date_time(const icu4x::TimeZone& time_zone, const icu4x::IsoDate& local_date, const icu4x::Time& local_time) const {
  auto result = icu4x::capi::icu4x_VariantOffsetsCalculator_compute_offsets_from_time_zone_and_date_time_mv1(this->AsFFI(),
    time_zone.AsFFI(),
    local_date.AsFFI(),
    local_time.AsFFI());
  return result.is_ok ? std::optional<icu4x::VariantOffsets>(icu4x::VariantOffsets::FromFFI(result.ok)) : std::nullopt;
}

inline const icu4x::capi::VariantOffsetsCalculator* icu4x::VariantOffsetsCalculator::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::VariantOffsetsCalculator*>(this);
}

inline icu4x::capi::VariantOffsetsCalculator* icu4x::VariantOffsetsCalculator::AsFFI() {
  return reinterpret_cast<icu4x::capi::VariantOffsetsCalculator*>(this);
}

inline const icu4x::VariantOffsetsCalculator* icu4x::VariantOffsetsCalculator::FromFFI(const icu4x::capi::VariantOffsetsCalculator* ptr) {
  return reinterpret_cast<const icu4x::VariantOffsetsCalculator*>(ptr);
}

inline icu4x::VariantOffsetsCalculator* icu4x::VariantOffsetsCalculator::FromFFI(icu4x::capi::VariantOffsetsCalculator* ptr) {
  return reinterpret_cast<icu4x::VariantOffsetsCalculator*>(ptr);
}

inline void icu4x::VariantOffsetsCalculator::operator delete(void* ptr) {
  icu4x::capi::icu4x_VariantOffsetsCalculator_destroy_mv1(reinterpret_cast<icu4x::capi::VariantOffsetsCalculator*>(ptr));
}


#endif // icu4x_VariantOffsetsCalculator_HPP
