#ifndef icu4x_IanaParserExtended_D_HPP
#define icu4x_IanaParserExtended_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../diplomat_runtime.hpp"

namespace icu4x {
namespace capi { struct DataProvider; }
class DataProvider;
namespace capi { struct IanaParserExtended; }
class IanaParserExtended;
class DataError;
}


namespace icu4x {
namespace capi {
    struct IanaParserExtended;
} // namespace capi
} // namespace

namespace icu4x {
class IanaParserExtended {
public:

  inline static std::unique_ptr<icu4x::IanaParserExtended> create();

  inline static diplomat::result<std::unique_ptr<icu4x::IanaParserExtended>, icu4x::DataError> create_with_provider(const icu4x::DataProvider& provider);

  inline diplomat::result<std::optional<std::string>, diplomat::Utf8Error> canonicalize_iana(std::string_view value) const;

  inline std::optional<std::string> canonical_iana_from_bcp47(std::string_view value) const;

  inline const icu4x::capi::IanaParserExtended* AsFFI() const;
  inline icu4x::capi::IanaParserExtended* AsFFI();
  inline static const icu4x::IanaParserExtended* FromFFI(const icu4x::capi::IanaParserExtended* ptr);
  inline static icu4x::IanaParserExtended* FromFFI(icu4x::capi::IanaParserExtended* ptr);
  inline static void operator delete(void* ptr);
private:
  IanaParserExtended() = delete;
  IanaParserExtended(const icu4x::IanaParserExtended&) = delete;
  IanaParserExtended(icu4x::IanaParserExtended&&) noexcept = delete;
  IanaParserExtended operator=(const icu4x::IanaParserExtended&) = delete;
  IanaParserExtended operator=(icu4x::IanaParserExtended&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // icu4x_IanaParserExtended_D_HPP
