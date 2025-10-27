#ifndef ICU4X_CollationSortKey_HPP
#define ICU4X_CollationSortKey_HPP

#include "CollationSortKey.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    extern "C" {

    icu4x::diplomat::capi::DiplomatU8View icu4x_CollationSortKey_as_bytes_mv1(const icu4x::capi::CollationSortKey* self);

    void icu4x_CollationSortKey_destroy_mv1(CollationSortKey* self);

    } // extern "C"
} // namespace capi
} // namespace

inline icu4x::diplomat::span<const uint8_t> icu4x::CollationSortKey::as_bytes() const {
    auto result = icu4x::capi::icu4x_CollationSortKey_as_bytes_mv1(this->AsFFI());
    return icu4x::diplomat::span<const uint8_t>(result.data, result.len);
}

inline const icu4x::capi::CollationSortKey* icu4x::CollationSortKey::AsFFI() const {
    return reinterpret_cast<const icu4x::capi::CollationSortKey*>(this);
}

inline icu4x::capi::CollationSortKey* icu4x::CollationSortKey::AsFFI() {
    return reinterpret_cast<icu4x::capi::CollationSortKey*>(this);
}

inline const icu4x::CollationSortKey* icu4x::CollationSortKey::FromFFI(const icu4x::capi::CollationSortKey* ptr) {
    return reinterpret_cast<const icu4x::CollationSortKey*>(ptr);
}

inline icu4x::CollationSortKey* icu4x::CollationSortKey::FromFFI(icu4x::capi::CollationSortKey* ptr) {
    return reinterpret_cast<icu4x::CollationSortKey*>(ptr);
}

inline void icu4x::CollationSortKey::operator delete(void* ptr) {
    icu4x::capi::icu4x_CollationSortKey_destroy_mv1(reinterpret_cast<icu4x::capi::CollationSortKey*>(ptr));
}


#endif // ICU4X_CollationSortKey_HPP
