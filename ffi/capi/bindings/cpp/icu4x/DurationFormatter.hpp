#ifndef ICU4X_DurationFormatter_HPP
#define ICU4X_DurationFormatter_HPP

#include "DurationFormatter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "DataProvider.hpp"
#include "Duration.hpp"
#include "DurationFormatterError.hpp"
#include "DurationFormatterOptions.hpp"
#include "Locale.hpp"
#include "Parts.hpp"
#include "diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    extern "C" {

    typedef struct icu4x_DurationFormatter_create_mv1_result {union {icu4x::capi::DurationFormatter* ok; icu4x::capi::DurationFormatterError err;}; bool is_ok;} icu4x_DurationFormatter_create_mv1_result;
    icu4x_DurationFormatter_create_mv1_result icu4x_DurationFormatter_create_mv1(const icu4x::capi::Locale* locale, icu4x::capi::DurationFormatterOptions options);

    typedef struct icu4x_DurationFormatter_create_with_provider_mv1_result {union {icu4x::capi::DurationFormatter* ok; icu4x::capi::DurationFormatterError err;}; bool is_ok;} icu4x_DurationFormatter_create_with_provider_mv1_result;
    icu4x_DurationFormatter_create_with_provider_mv1_result icu4x_DurationFormatter_create_with_provider_mv1(const icu4x::capi::DataProvider* provider, const icu4x::capi::Locale* locale, icu4x::capi::DurationFormatterOptions options);

    void icu4x_DurationFormatter_format_mv1(const icu4x::capi::DurationFormatter* self, icu4x::capi::Duration duration, icu4x::diplomat::capi::DiplomatWrite* write);

    void icu4x_DurationFormatter_format_utf8_to_parts_mv1(const icu4x::capi::DurationFormatter* self, icu4x::capi::Duration duration, icu4x::capi::Parts* parts, icu4x::diplomat::capi::DiplomatWrite* write);

    void icu4x_DurationFormatter_format_utf16_to_parts_mv1(const icu4x::capi::DurationFormatter* self, icu4x::capi::Duration duration, icu4x::capi::Parts* parts, icu4x::diplomat::capi::DiplomatWrite* write);

    void icu4x_DurationFormatter_destroy_mv1(DurationFormatter* self);

    } // extern "C"
} // namespace capi
} // namespace

inline icu4x::diplomat::result<std::unique_ptr<icu4x::DurationFormatter>, icu4x::DurationFormatterError> icu4x::DurationFormatter::create(const icu4x::Locale& locale, icu4x::DurationFormatterOptions options) {
    auto result = icu4x::capi::icu4x_DurationFormatter_create_mv1(locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::DurationFormatter>, icu4x::DurationFormatterError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::DurationFormatter>>(std::unique_ptr<icu4x::DurationFormatter>(icu4x::DurationFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::DurationFormatter>, icu4x::DurationFormatterError>(icu4x::diplomat::Err<icu4x::DurationFormatterError>(icu4x::DurationFormatterError::FromFFI(result.err)));
}

inline icu4x::diplomat::result<std::unique_ptr<icu4x::DurationFormatter>, icu4x::DurationFormatterError> icu4x::DurationFormatter::create_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::DurationFormatterOptions options) {
    auto result = icu4x::capi::icu4x_DurationFormatter_create_with_provider_mv1(provider.AsFFI(),
        locale.AsFFI(),
        options.AsFFI());
    return result.is_ok ? icu4x::diplomat::result<std::unique_ptr<icu4x::DurationFormatter>, icu4x::DurationFormatterError>(icu4x::diplomat::Ok<std::unique_ptr<icu4x::DurationFormatter>>(std::unique_ptr<icu4x::DurationFormatter>(icu4x::DurationFormatter::FromFFI(result.ok)))) : icu4x::diplomat::result<std::unique_ptr<icu4x::DurationFormatter>, icu4x::DurationFormatterError>(icu4x::diplomat::Err<icu4x::DurationFormatterError>(icu4x::DurationFormatterError::FromFFI(result.err)));
}

inline std::string icu4x::DurationFormatter::format(icu4x::Duration duration) const {
    std::string output;
    icu4x::diplomat::capi::DiplomatWrite write = icu4x::diplomat::WriteFromString(output);
    icu4x::capi::icu4x_DurationFormatter_format_mv1(this->AsFFI(),
        duration.AsFFI(),
        &write);
    return output;
}
template<typename W>
inline void icu4x::DurationFormatter::format_write(icu4x::Duration duration, W& writeable) const {
    icu4x::diplomat::capi::DiplomatWrite write = icu4x::diplomat::WriteTrait<W>::Construct(writeable);
    icu4x::capi::icu4x_DurationFormatter_format_mv1(this->AsFFI(),
        duration.AsFFI(),
        &write);
}

inline std::string icu4x::DurationFormatter::format_to_parts(icu4x::Duration duration, icu4x::Parts& parts) const {
    std::string output;
    icu4x::diplomat::capi::DiplomatWrite write = icu4x::diplomat::WriteFromString(output);
    icu4x::capi::icu4x_DurationFormatter_format_utf8_to_parts_mv1(this->AsFFI(),
        duration.AsFFI(),
        parts.AsFFI(),
        &write);
    return output;
}
template<typename W>
inline void icu4x::DurationFormatter::format_to_parts_write(icu4x::Duration duration, icu4x::Parts& parts, W& writeable) const {
    icu4x::diplomat::capi::DiplomatWrite write = icu4x::diplomat::WriteTrait<W>::Construct(writeable);
    icu4x::capi::icu4x_DurationFormatter_format_utf8_to_parts_mv1(this->AsFFI(),
        duration.AsFFI(),
        parts.AsFFI(),
        &write);
}

inline std::u16string icu4x::DurationFormatter::format_to_parts16(icu4x::Duration duration, icu4x::Parts& parts) const {
    std::u16string output;
    icu4x::diplomat::capi::DiplomatWrite write = icu4x::diplomat::WriteFromString16(output);
    icu4x::capi::icu4x_DurationFormatter_format_utf16_to_parts_mv1(this->AsFFI(),
        duration.AsFFI(),
        parts.AsFFI(),
        &write);
    return output;
}
template<typename W>
inline void icu4x::DurationFormatter::format_to_parts16_write(icu4x::Duration duration, icu4x::Parts& parts, W& writeable) const {
    icu4x::diplomat::capi::DiplomatWrite write = icu4x::diplomat::WriteTrait<W>::Construct(writeable);
    icu4x::capi::icu4x_DurationFormatter_format_utf16_to_parts_mv1(this->AsFFI(),
        duration.AsFFI(),
        parts.AsFFI(),
        &write);
}

inline const icu4x::capi::DurationFormatter* icu4x::DurationFormatter::AsFFI() const {
    return reinterpret_cast<const icu4x::capi::DurationFormatter*>(this);
}

inline icu4x::capi::DurationFormatter* icu4x::DurationFormatter::AsFFI() {
    return reinterpret_cast<icu4x::capi::DurationFormatter*>(this);
}

inline const icu4x::DurationFormatter* icu4x::DurationFormatter::FromFFI(const icu4x::capi::DurationFormatter* ptr) {
    return reinterpret_cast<const icu4x::DurationFormatter*>(ptr);
}

inline icu4x::DurationFormatter* icu4x::DurationFormatter::FromFFI(icu4x::capi::DurationFormatter* ptr) {
    return reinterpret_cast<icu4x::DurationFormatter*>(ptr);
}

inline void icu4x::DurationFormatter::operator delete(void* ptr) {
    icu4x::capi::icu4x_DurationFormatter_destroy_mv1(reinterpret_cast<icu4x::capi::DurationFormatter*>(ptr));
}


#endif // ICU4X_DurationFormatter_HPP
