#ifndef ICU4X_Parts_HPP
#define ICU4X_Parts_HPP

#include "Parts.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "PartSpan.hpp"
#include "diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    extern "C" {

    icu4x::capi::DiplomatPartSpanView icu4x_Parts_as_slice_mv1(const icu4x::capi::Parts* self);

    size_t icu4x_Parts_len_mv1(const icu4x::capi::Parts* self);

    bool icu4x_Parts_is_empty_mv1(const icu4x::capi::Parts* self);

    typedef struct icu4x_Parts_get_mv1_result {union {icu4x::capi::PartSpan ok; }; bool is_ok;} icu4x_Parts_get_mv1_result;
    icu4x_Parts_get_mv1_result icu4x_Parts_get_mv1(const icu4x::capi::Parts* self, size_t index);

    void icu4x_Parts_destroy_mv1(Parts* self);

    } // extern "C"
} // namespace capi
} // namespace

inline icu4x::diplomat::span<const icu4x::PartSpan> icu4x::Parts::as_slice() const DIPLOMAT_LIFETIME_BOUND {
    auto result = icu4x::capi::icu4x_Parts_as_slice_mv1(this->AsFFI());
    return icu4x::diplomat::span<const icu4x::PartSpan>(reinterpret_cast<const icu4x::PartSpan*>(result.data), result.len);
}

inline size_t icu4x::Parts::len() const {
    auto result = icu4x::capi::icu4x_Parts_len_mv1(this->AsFFI());
    return result;
}

inline bool icu4x::Parts::is_empty() const {
    auto result = icu4x::capi::icu4x_Parts_is_empty_mv1(this->AsFFI());
    return result;
}

inline std::optional<icu4x::PartSpan> icu4x::Parts::operator[](size_t index) const {
    auto result = icu4x::capi::icu4x_Parts_get_mv1(this->AsFFI(),
        index);
    return result.is_ok ? std::optional<icu4x::PartSpan>(icu4x::PartSpan::FromFFI(result.ok)) : std::nullopt;
}

inline const icu4x::capi::Parts* icu4x::Parts::AsFFI() const {
    return reinterpret_cast<const icu4x::capi::Parts*>(this);
}

inline icu4x::capi::Parts* icu4x::Parts::AsFFI() {
    return reinterpret_cast<icu4x::capi::Parts*>(this);
}

inline const icu4x::Parts* icu4x::Parts::FromFFI(const icu4x::capi::Parts* ptr) {
    return reinterpret_cast<const icu4x::Parts*>(ptr);
}

inline icu4x::Parts* icu4x::Parts::FromFFI(icu4x::capi::Parts* ptr) {
    return reinterpret_cast<icu4x::Parts*>(ptr);
}

inline void icu4x::Parts::operator delete(void* ptr) {
    icu4x::capi::icu4x_Parts_destroy_mv1(reinterpret_cast<icu4x::capi::Parts*>(ptr));
}


#endif // ICU4X_Parts_HPP
