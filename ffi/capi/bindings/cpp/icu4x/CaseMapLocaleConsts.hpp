#ifndef ICU4X_CaseMapLocaleConsts_HPP
#define ICU4X_CaseMapLocaleConsts_HPP

#include "CaseMapLocaleConsts.d.hpp"

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

    const icu4x::capi::Locale* icu4x_CaseMapLocaleConsts_az_mv1(void);

    const icu4x::capi::Locale* icu4x_CaseMapLocaleConsts_el_mv1(void);

    const icu4x::capi::Locale* icu4x_CaseMapLocaleConsts_hy_mv1(void);

    const icu4x::capi::Locale* icu4x_CaseMapLocaleConsts_lt_mv1(void);

    const icu4x::capi::Locale* icu4x_CaseMapLocaleConsts_nl_mv1(void);

    const icu4x::capi::Locale* icu4x_CaseMapLocaleConsts_tr_mv1(void);

    void icu4x_CaseMapLocaleConsts_destroy_mv1(CaseMapLocaleConsts* self);

    } // extern "C"
} // namespace capi
} // namespace

inline const icu4x::Locale& icu4x::CaseMapLocaleConsts::az() {
    auto result = icu4x::capi::icu4x_CaseMapLocaleConsts_az_mv1();
    return *icu4x::Locale::FromFFI(result);
}

inline const icu4x::Locale& icu4x::CaseMapLocaleConsts::el() {
    auto result = icu4x::capi::icu4x_CaseMapLocaleConsts_el_mv1();
    return *icu4x::Locale::FromFFI(result);
}

inline const icu4x::Locale& icu4x::CaseMapLocaleConsts::hy() {
    auto result = icu4x::capi::icu4x_CaseMapLocaleConsts_hy_mv1();
    return *icu4x::Locale::FromFFI(result);
}

inline const icu4x::Locale& icu4x::CaseMapLocaleConsts::lt() {
    auto result = icu4x::capi::icu4x_CaseMapLocaleConsts_lt_mv1();
    return *icu4x::Locale::FromFFI(result);
}

inline const icu4x::Locale& icu4x::CaseMapLocaleConsts::nl() {
    auto result = icu4x::capi::icu4x_CaseMapLocaleConsts_nl_mv1();
    return *icu4x::Locale::FromFFI(result);
}

inline const icu4x::Locale& icu4x::CaseMapLocaleConsts::tr() {
    auto result = icu4x::capi::icu4x_CaseMapLocaleConsts_tr_mv1();
    return *icu4x::Locale::FromFFI(result);
}

inline const icu4x::capi::CaseMapLocaleConsts* icu4x::CaseMapLocaleConsts::AsFFI() const {
    return reinterpret_cast<const icu4x::capi::CaseMapLocaleConsts*>(this);
}

inline icu4x::capi::CaseMapLocaleConsts* icu4x::CaseMapLocaleConsts::AsFFI() {
    return reinterpret_cast<icu4x::capi::CaseMapLocaleConsts*>(this);
}

inline const icu4x::CaseMapLocaleConsts* icu4x::CaseMapLocaleConsts::FromFFI(const icu4x::capi::CaseMapLocaleConsts* ptr) {
    return reinterpret_cast<const icu4x::CaseMapLocaleConsts*>(ptr);
}

inline icu4x::CaseMapLocaleConsts* icu4x::CaseMapLocaleConsts::FromFFI(icu4x::capi::CaseMapLocaleConsts* ptr) {
    return reinterpret_cast<icu4x::CaseMapLocaleConsts*>(ptr);
}

inline void icu4x::CaseMapLocaleConsts::operator delete(void* ptr) {
    icu4x::capi::icu4x_CaseMapLocaleConsts_destroy_mv1(reinterpret_cast<icu4x::capi::CaseMapLocaleConsts*>(ptr));
}


#endif // ICU4X_CaseMapLocaleConsts_HPP
