#ifndef Collator_D_HPP
#define Collator_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace diplomat::capi { struct DataProvider; }
class DataProvider;
namespace diplomat::capi { struct Locale; }
class Locale;
struct CollatorOptionsV1;
struct CollatorResolvedOptionsV1;
class DataError;


namespace diplomat {
namespace capi {
    struct Collator;
} // namespace capi
} // namespace

class Collator {
public:

  inline static diplomat::result<std::unique_ptr<Collator>, DataError> create_v1(const DataProvider& provider, const Locale& locale, CollatorOptionsV1 options);

  inline int8_t compare(std::string_view left, std::string_view right) const;

  inline int8_t compare16(std::u16string_view left, std::u16string_view right) const;

  inline CollatorResolvedOptionsV1 resolved_options_v1() const;

  inline const diplomat::capi::Collator* AsFFI() const;
  inline diplomat::capi::Collator* AsFFI();
  inline static const Collator* FromFFI(const diplomat::capi::Collator* ptr);
  inline static Collator* FromFFI(diplomat::capi::Collator* ptr);
  inline static void operator delete(void* ptr);
private:
  Collator() = delete;
  Collator(const Collator&) = delete;
  Collator(Collator&&) noexcept = delete;
  Collator operator=(const Collator&) = delete;
  Collator operator=(Collator&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // Collator_D_HPP
