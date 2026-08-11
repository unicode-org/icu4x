#ifndef ICU4X_LocaleNamesUnstable_HPP
#define ICU4X_LocaleNamesUnstable_HPP

#include "LocaleNamesUnstable.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "Locale.hpp"
#include "diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    extern "C" {

    typedef struct icu4x_LocaleNamesUnstable_for_region_with_compiled_data_light_mv1_result {union { icu4x::capi::DataError err;}; bool is_ok;} icu4x_LocaleNamesUnstable_for_region_with_compiled_data_light_mv1_result;
    icu4x_LocaleNamesUnstable_for_region_with_compiled_data_light_mv1_result icu4x_LocaleNamesUnstable_for_region_with_compiled_data_light_mv1(const icu4x::capi::Locale* locale, icu4x::diplomat::capi::DiplomatStringView region, icu4x::diplomat::capi::DiplomatWrite* write);

    typedef struct icu4x_LocaleNamesUnstable_for_region_with_provider_light_mv1_result {union { icu4x::capi::DataError err;}; bool is_ok;} icu4x_LocaleNamesUnstable_for_region_with_provider_light_mv1_result;
    icu4x_LocaleNamesUnstable_for_region_with_provider_light_mv1_result icu4x_LocaleNamesUnstable_for_region_with_provider_light_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::diplomat::capi::DiplomatStringView region, icu4x::diplomat::capi::DiplomatWrite* write);

    void icu4x_LocaleNamesUnstable_destroy_mv1(LocaleNamesUnstable* self);

    } // extern "C"
} // namespace capi
} // namespace

inline icu4x::diplomat::result<std::string, icu4x::DataError> icu4x::LocaleNamesUnstable::for_region_with_compiled_data_light(const icu4x::Locale& locale, std::string_view region) {
    std::string output;
    icu4x::diplomat::capi::DiplomatWrite write = icu4x::diplomat::WriteFromString(output);
    auto result = icu4x::capi::icu4x_LocaleNamesUnstable_for_region_with_compiled_data_light_mv1(locale.AsFFI(),
        {region.data(), region.size()},
        &write);
    return result.is_ok ? icu4x::diplomat::result<std::string, icu4x::DataError>(icu4x::diplomat::Ok<std::string>(std::move(output))) : icu4x::diplomat::result<std::string, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}
template<typename W>
inline icu4x::diplomat::result<std::monostate, icu4x::DataError> icu4x::LocaleNamesUnstable::for_region_with_compiled_data_light_write(const icu4x::Locale& locale, std::string_view region, W& writeable) {
    icu4x::diplomat::capi::DiplomatWrite write = icu4x::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = icu4x::capi::icu4x_LocaleNamesUnstable_for_region_with_compiled_data_light_mv1(locale.AsFFI(),
        {region.data(), region.size()},
        &write);
    return result.is_ok ? icu4x::diplomat::result<std::monostate, icu4x::DataError>(icu4x::diplomat::Ok<std::monostate>()) : icu4x::diplomat::result<std::monostate, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::string, icu4x::DataError> icu4x::LocaleNamesUnstable::for_region_with_provider_light(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::string_view region) {
    std::string output;
    icu4x::diplomat::capi::DiplomatWrite write = icu4x::diplomat::WriteFromString(output);
    auto result = icu4x::capi::icu4x_LocaleNamesUnstable_for_region_with_provider_light_mv1(provider.AsFFI(),
        locale.AsFFI(),
        {region.data(), region.size()},
        &write);
    return result.is_ok ? icu4x::diplomat::result<std::string, icu4x::DataError>(icu4x::diplomat::Ok<std::string>(std::move(output))) : icu4x::diplomat::result<std::string, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}
template<typename W>
inline icu4x::diplomat::result<std::monostate, icu4x::DataError> icu4x::LocaleNamesUnstable::for_region_with_provider_light_write(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::string_view region, W& writeable) {
    icu4x::diplomat::capi::DiplomatWrite write = icu4x::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = icu4x::capi::icu4x_LocaleNamesUnstable_for_region_with_provider_light_mv1(provider.AsFFI(),
        locale.AsFFI(),
        {region.data(), region.size()},
        &write);
    return result.is_ok ? icu4x::diplomat::result<std::monostate, icu4x::DataError>(icu4x::diplomat::Ok<std::monostate>()) : icu4x::diplomat::result<std::monostate, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline const icu4x::capi::LocaleNamesUnstable* icu4x::LocaleNamesUnstable::AsFFI() const {
    return reinterpret_cast<const icu4x::capi::LocaleNamesUnstable*>(this);
}

inline icu4x::capi::LocaleNamesUnstable* icu4x::LocaleNamesUnstable::AsFFI() {
    return reinterpret_cast<icu4x::capi::LocaleNamesUnstable*>(this);
}

inline const icu4x::LocaleNamesUnstable* icu4x::LocaleNamesUnstable::FromFFI(const icu4x::capi::LocaleNamesUnstable* ptr) {
    return reinterpret_cast<const icu4x::LocaleNamesUnstable*>(ptr);
}

inline icu4x::LocaleNamesUnstable* icu4x::LocaleNamesUnstable::FromFFI(icu4x::capi::LocaleNamesUnstable* ptr) {
    return reinterpret_cast<icu4x::LocaleNamesUnstable*>(ptr);
}

inline void icu4x::LocaleNamesUnstable::operator delete(void* ptr) {
    icu4x::capi::icu4x_LocaleNamesUnstable_destroy_mv1(reinterpret_cast<icu4x::capi::LocaleNamesUnstable*>(ptr));
}


#endif // ICU4X_LocaleNamesUnstable_HPP
