#ifndef ICU4X_CaseMapLocaleRefs_HPP
#define ICU4X_CaseMapLocaleRefs_HPP

#include "CaseMapLocaleRefs.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "Locale.hpp"
#include "diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    extern "C" {

    const icu4x::capi::Locale* icu4x_CaseMapLocaleRefs_az_mv1(void);

    const icu4x::capi::Locale* icu4x_CaseMapLocaleRefs_el_mv1(void);

    const icu4x::capi::Locale* icu4x_CaseMapLocaleRefs_hy_mv1(void);

    const icu4x::capi::Locale* icu4x_CaseMapLocaleRefs_lt_mv1(void);

    const icu4x::capi::Locale* icu4x_CaseMapLocaleRefs_nl_mv1(void);

    const icu4x::capi::Locale* icu4x_CaseMapLocaleRefs_tr_mv1(void);

    void icu4x_CaseMapLocaleRefs_destroy_mv1(CaseMapLocaleRefs* self);

    } // extern "C"
} // namespace capi
} // namespace

inline const icu4x::Locale& icu4x::CaseMapLocaleRefs::az() {
    auto result = icu4x::capi::icu4x_CaseMapLocaleRefs_az_mv1();
    return *icu4x::Locale::FromFFI(result);
}

inline const icu4x::Locale& icu4x::CaseMapLocaleRefs::el() {
    auto result = icu4x::capi::icu4x_CaseMapLocaleRefs_el_mv1();
    return *icu4x::Locale::FromFFI(result);
}

inline const icu4x::Locale& icu4x::CaseMapLocaleRefs::hy() {
    auto result = icu4x::capi::icu4x_CaseMapLocaleRefs_hy_mv1();
    return *icu4x::Locale::FromFFI(result);
}

inline const icu4x::Locale& icu4x::CaseMapLocaleRefs::lt() {
    auto result = icu4x::capi::icu4x_CaseMapLocaleRefs_lt_mv1();
    return *icu4x::Locale::FromFFI(result);
}

inline const icu4x::Locale& icu4x::CaseMapLocaleRefs::nl() {
    auto result = icu4x::capi::icu4x_CaseMapLocaleRefs_nl_mv1();
    return *icu4x::Locale::FromFFI(result);
}

inline const icu4x::Locale& icu4x::CaseMapLocaleRefs::tr() {
    auto result = icu4x::capi::icu4x_CaseMapLocaleRefs_tr_mv1();
    return *icu4x::Locale::FromFFI(result);
}

inline const icu4x::capi::CaseMapLocaleRefs* icu4x::CaseMapLocaleRefs::AsFFI() const {
    return reinterpret_cast<const icu4x::capi::CaseMapLocaleRefs*>(this);
}

inline icu4x::capi::CaseMapLocaleRefs* icu4x::CaseMapLocaleRefs::AsFFI() {
    return reinterpret_cast<icu4x::capi::CaseMapLocaleRefs*>(this);
}

inline const icu4x::CaseMapLocaleRefs* icu4x::CaseMapLocaleRefs::FromFFI(const icu4x::capi::CaseMapLocaleRefs* ptr) {
    return reinterpret_cast<const icu4x::CaseMapLocaleRefs*>(ptr);
}

inline icu4x::CaseMapLocaleRefs* icu4x::CaseMapLocaleRefs::FromFFI(icu4x::capi::CaseMapLocaleRefs* ptr) {
    return reinterpret_cast<icu4x::CaseMapLocaleRefs*>(ptr);
}

inline void icu4x::CaseMapLocaleRefs::operator delete(void* ptr) {
    icu4x::capi::icu4x_CaseMapLocaleRefs_destroy_mv1(reinterpret_cast<icu4x::capi::CaseMapLocaleRefs*>(ptr));
}


#endif // ICU4X_CaseMapLocaleRefs_HPP
