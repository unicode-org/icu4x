#ifndef ICU4X_PartsSink_HPP
#define ICU4X_PartsSink_HPP

#include "PartsSink.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "Parts.hpp"
#include "diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    extern "C" {

    icu4x::capi::PartsSink* icu4x_PartsSink_create_mv1(void);

    icu4x::capi::PartsSink* icu4x_PartsSink_create_with_capacity_mv1(size_t capacity);

    icu4x::capi::Parts* icu4x_PartsSink_drain_mv1(icu4x::capi::PartsSink* self);

    void icu4x_PartsSink_destroy_mv1(PartsSink* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<icu4x::PartsSink> icu4x::PartsSink::create() {
    auto result = icu4x::capi::icu4x_PartsSink_create_mv1();
    return std::unique_ptr<icu4x::PartsSink>(icu4x::PartsSink::FromFFI(result));
}

inline std::unique_ptr<icu4x::PartsSink> icu4x::PartsSink::create_with_capacity(size_t capacity) {
    auto result = icu4x::capi::icu4x_PartsSink_create_with_capacity_mv1(capacity);
    return std::unique_ptr<icu4x::PartsSink>(icu4x::PartsSink::FromFFI(result));
}

inline std::unique_ptr<icu4x::Parts> icu4x::PartsSink::drain() {
    auto result = icu4x::capi::icu4x_PartsSink_drain_mv1(this->AsFFI());
    return std::unique_ptr<icu4x::Parts>(icu4x::Parts::FromFFI(result));
}

inline const icu4x::capi::PartsSink* icu4x::PartsSink::AsFFI() const {
    return reinterpret_cast<const icu4x::capi::PartsSink*>(this);
}

inline icu4x::capi::PartsSink* icu4x::PartsSink::AsFFI() {
    return reinterpret_cast<icu4x::capi::PartsSink*>(this);
}

inline const icu4x::PartsSink* icu4x::PartsSink::FromFFI(const icu4x::capi::PartsSink* ptr) {
    return reinterpret_cast<const icu4x::PartsSink*>(ptr);
}

inline icu4x::PartsSink* icu4x::PartsSink::FromFFI(icu4x::capi::PartsSink* ptr) {
    return reinterpret_cast<icu4x::PartsSink*>(ptr);
}

inline void icu4x::PartsSink::operator delete(void* ptr) {
    icu4x::capi::icu4x_PartsSink_destroy_mv1(reinterpret_cast<icu4x::capi::PartsSink*>(ptr));
}


#endif // ICU4X_PartsSink_HPP
