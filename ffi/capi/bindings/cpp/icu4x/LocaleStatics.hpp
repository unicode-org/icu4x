#ifndef ICU4X_LocaleStatics_HPP
#define ICU4X_LocaleStatics_HPP

#include "LocaleStatics.d.hpp"

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

    const icu4x::capi::Locale* icu4x_LocaleStatics_und_mv1(void);

    const icu4x::capi::Locale* icu4x_LocaleStatics_az_mv1(void);

    const icu4x::capi::Locale* icu4x_LocaleStatics_el_mv1(void);

    const icu4x::capi::Locale* icu4x_LocaleStatics_hy_mv1(void);

    const icu4x::capi::Locale* icu4x_LocaleStatics_lt_mv1(void);

    const icu4x::capi::Locale* icu4x_LocaleStatics_nl_mv1(void);

    const icu4x::capi::Locale* icu4x_LocaleStatics_tr_mv1(void);

    void icu4x_LocaleStatics_destroy_mv1(LocaleStatics* self);

    } // extern "C"
} // namespace capi
} // namespace

inline const icu4x::Locale& icu4x::LocaleStatics::und() {
    auto result = icu4x::capi::icu4x_LocaleStatics_und_mv1();
    return *icu4x::Locale::FromFFI(result);
}

inline const icu4x::Locale& icu4x::LocaleStatics::az() {
    auto result = icu4x::capi::icu4x_LocaleStatics_az_mv1();
    return *icu4x::Locale::FromFFI(result);
}

inline const icu4x::Locale& icu4x::LocaleStatics::el() {
    auto result = icu4x::capi::icu4x_LocaleStatics_el_mv1();
    return *icu4x::Locale::FromFFI(result);
}

inline const icu4x::Locale& icu4x::LocaleStatics::hy() {
    auto result = icu4x::capi::icu4x_LocaleStatics_hy_mv1();
    return *icu4x::Locale::FromFFI(result);
}

inline const icu4x::Locale& icu4x::LocaleStatics::lt() {
    auto result = icu4x::capi::icu4x_LocaleStatics_lt_mv1();
    return *icu4x::Locale::FromFFI(result);
}

inline const icu4x::Locale& icu4x::LocaleStatics::nl() {
    auto result = icu4x::capi::icu4x_LocaleStatics_nl_mv1();
    return *icu4x::Locale::FromFFI(result);
}

inline const icu4x::Locale& icu4x::LocaleStatics::tr() {
    auto result = icu4x::capi::icu4x_LocaleStatics_tr_mv1();
    return *icu4x::Locale::FromFFI(result);
}

inline const icu4x::capi::LocaleStatics* icu4x::LocaleStatics::AsFFI() const {
    return reinterpret_cast<const icu4x::capi::LocaleStatics*>(this);
}

inline icu4x::capi::LocaleStatics* icu4x::LocaleStatics::AsFFI() {
    return reinterpret_cast<icu4x::capi::LocaleStatics*>(this);
}

inline const icu4x::LocaleStatics* icu4x::LocaleStatics::FromFFI(const icu4x::capi::LocaleStatics* ptr) {
    return reinterpret_cast<const icu4x::LocaleStatics*>(ptr);
}

inline icu4x::LocaleStatics* icu4x::LocaleStatics::FromFFI(icu4x::capi::LocaleStatics* ptr) {
    return reinterpret_cast<icu4x::LocaleStatics*>(ptr);
}

inline void icu4x::LocaleStatics::operator delete(void* ptr) {
    icu4x::capi::icu4x_LocaleStatics_destroy_mv1(reinterpret_cast<icu4x::capi::LocaleStatics*>(ptr));
}


#endif // ICU4X_LocaleStatics_HPP
