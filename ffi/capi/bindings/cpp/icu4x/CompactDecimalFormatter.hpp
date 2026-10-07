#ifndef ICU4X_CompactDecimalFormatter_HPP
#define ICU4X_CompactDecimalFormatter_HPP

#include "CompactDecimalFormatter.d.hpp"

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
#include "Decimal.hpp"
#include "DecimalGroupingStrategy.hpp"
#include "Locale.hpp"
#include "diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    extern "C" {

    typedef struct icu4x_CompactDecimalFormatter_create_short_mv1_result {union {icu4x::capi::CompactDecimalFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_CompactDecimalFormatter_create_short_mv1_result;
    icu4x_CompactDecimalFormatter_create_short_mv1_result icu4x_CompactDecimalFormatter_create_short_mv1(const icu4x::capi::Locale* locale, icu4x::capi::DecimalGroupingStrategy_option grouping_strategy);

    typedef struct icu4x_CompactDecimalFormatter_create_short_with_provider_mv1_result {union {icu4x::capi::CompactDecimalFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_CompactDecimalFormatter_create_short_with_provider_mv1_result;
    icu4x_CompactDecimalFormatter_create_short_with_provider_mv1_result icu4x_CompactDecimalFormatter_create_short_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::DecimalGroupingStrategy_option grouping_strategy);

    typedef struct icu4x_CompactDecimalFormatter_create_long_mv1_result {union {icu4x::capi::CompactDecimalFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_CompactDecimalFormatter_create_long_mv1_result;
    icu4x_CompactDecimalFormatter_create_long_mv1_result icu4x_CompactDecimalFormatter_create_long_mv1(const icu4x::capi::Locale* locale, icu4x::capi::DecimalGroupingStrategy_option grouping_strategy);

    typedef struct icu4x_CompactDecimalFormatter_create_long_with_provider_mv1_result {union {icu4x::capi::CompactDecimalFormatter* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_CompactDecimalFormatter_create_long_with_provider_mv1_result;
    icu4x_CompactDecimalFormatter_create_long_with_provider_mv1_result icu4x_CompactDecimalFormatter_create_long_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::DecimalGroupingStrategy_option grouping_strategy);

    uint8_t icu4x_CompactDecimalFormatter_compact_exponent_for_magnitude_mv1(const icu4x::capi::CompactDecimalFormatter* self, int16_t magnitude);

    void icu4x_CompactDecimalFormatter_format_mv1(const icu4x::capi::CompactDecimalFormatter* self, const icu4x::capi::Decimal* value, icu4x::diplomat::capi::DiplomatWrite* write);

    void icu4x_CompactDecimalFormatter_destroy_mv1(CompactDecimalFormatter* self);

    } // extern "C"
} // namespace capi
} // namespace

inline icu4x::diplomat::result<std::unique_ptr<icu4x::CompactDecimalFormatter>, icu4x::DataError> icu4x::CompactDecimalFormatter::create_short(const icu4x::Locale& locale, std::optional<icu4x::DecimalGroupingStrategy> grouping_strategy) {
    auto result = icu4x::capi::icu4x_CompactDecimalFormatter_create_short_mv1(locale.AsFFI(),
        grouping_strategy.has_value() ? (icu4x::capi::DecimalGroupingStrategy_option{ { grouping_strategy.value().AsFFI() }, true }) : (icu4x::capi::DecimalGroupingStrategy_option{ {}, false }));
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::CompactDecimalFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::CompactDecimalFormatter>>(std::unique_ptr<icu4x::CompactDecimalFormatter>(icu4x::CompactDecimalFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::CompactDecimalFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::CompactDecimalFormatter>, icu4x::DataError> icu4x::CompactDecimalFormatter::create_short_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::optional<icu4x::DecimalGroupingStrategy> grouping_strategy) {
    auto result = icu4x::capi::icu4x_CompactDecimalFormatter_create_short_with_provider_mv1(provider.AsFFI(),
        locale.AsFFI(),
        grouping_strategy.has_value() ? (icu4x::capi::DecimalGroupingStrategy_option{ { grouping_strategy.value().AsFFI() }, true }) : (icu4x::capi::DecimalGroupingStrategy_option{ {}, false }));
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::CompactDecimalFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::CompactDecimalFormatter>>(std::unique_ptr<icu4x::CompactDecimalFormatter>(icu4x::CompactDecimalFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::CompactDecimalFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::CompactDecimalFormatter>, icu4x::DataError> icu4x::CompactDecimalFormatter::create_long(const icu4x::Locale& locale, std::optional<icu4x::DecimalGroupingStrategy> grouping_strategy) {
    auto result = icu4x::capi::icu4x_CompactDecimalFormatter_create_long_mv1(locale.AsFFI(),
        grouping_strategy.has_value() ? (icu4x::capi::DecimalGroupingStrategy_option{ { grouping_strategy.value().AsFFI() }, true }) : (icu4x::capi::DecimalGroupingStrategy_option{ {}, false }));
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::CompactDecimalFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::CompactDecimalFormatter>>(std::unique_ptr<icu4x::CompactDecimalFormatter>(icu4x::CompactDecimalFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::CompactDecimalFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::CompactDecimalFormatter>, icu4x::DataError> icu4x::CompactDecimalFormatter::create_long_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::optional<icu4x::DecimalGroupingStrategy> grouping_strategy) {
    auto result = icu4x::capi::icu4x_CompactDecimalFormatter_create_long_with_provider_mv1(provider.AsFFI(),
        locale.AsFFI(),
        grouping_strategy.has_value() ? (icu4x::capi::DecimalGroupingStrategy_option{ { grouping_strategy.value().AsFFI() }, true }) : (icu4x::capi::DecimalGroupingStrategy_option{ {}, false }));
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::CompactDecimalFormatter>, icu4x::DataError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::CompactDecimalFormatter>>(std::unique_ptr<icu4x::CompactDecimalFormatter>(icu4x::CompactDecimalFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::CompactDecimalFormatter>, icu4x::DataError>(icu4x::diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline uint8_t icu4x::CompactDecimalFormatter::compact_exponent_for_magnitude(int16_t magnitude) const {
    auto result = icu4x::capi::icu4x_CompactDecimalFormatter_compact_exponent_for_magnitude_mv1(this->AsFFI(),
        magnitude);
    return result;
}

inline std::string icu4x::CompactDecimalFormatter::format(const icu4x::Decimal& value) const {
    std::string output;
    icu4x::diplomat::capi::DiplomatWrite write = icu4x::diplomat::WriteFromString(output);
    icu4x::capi::icu4x_CompactDecimalFormatter_format_mv1(this->AsFFI(),
        value.AsFFI(),
        &write);
    return output;
}
template<typename W>
inline void icu4x::CompactDecimalFormatter::format_write(const icu4x::Decimal& value, W& writeable) const {
    icu4x::diplomat::capi::DiplomatWrite write = icu4x::diplomat::WriteTrait<W>::Construct(writeable);
    icu4x::capi::icu4x_CompactDecimalFormatter_format_mv1(this->AsFFI(),
        value.AsFFI(),
        &write);
}

inline const icu4x::capi::CompactDecimalFormatter* icu4x::CompactDecimalFormatter::AsFFI() const {
    return reinterpret_cast<const icu4x::capi::CompactDecimalFormatter*>(this);
}

inline icu4x::capi::CompactDecimalFormatter* icu4x::CompactDecimalFormatter::AsFFI() {
    return reinterpret_cast<icu4x::capi::CompactDecimalFormatter*>(this);
}

inline const icu4x::CompactDecimalFormatter* icu4x::CompactDecimalFormatter::FromFFI(const icu4x::capi::CompactDecimalFormatter* ptr) {
    return reinterpret_cast<const icu4x::CompactDecimalFormatter*>(ptr);
}

inline icu4x::CompactDecimalFormatter* icu4x::CompactDecimalFormatter::FromFFI(icu4x::capi::CompactDecimalFormatter* ptr) {
    return reinterpret_cast<icu4x::CompactDecimalFormatter*>(ptr);
}

inline void icu4x::CompactDecimalFormatter::operator delete(void* ptr) {
    icu4x::capi::icu4x_CompactDecimalFormatter_destroy_mv1(reinterpret_cast<icu4x::capi::CompactDecimalFormatter*>(ptr));
}


#endif // ICU4X_CompactDecimalFormatter_HPP
