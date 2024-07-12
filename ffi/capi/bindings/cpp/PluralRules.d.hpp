#ifndef PluralRules_D_HPP
#define PluralRules_D_HPP

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
namespace diplomat::capi { struct PluralOperands; }
class PluralOperands;
struct PluralCategories;
class DataError;
class PluralCategory;


namespace diplomat {
namespace capi {
    struct PluralRules;
} // namespace capi
} // namespace

class PluralRules {
public:

  inline static diplomat::result<std::unique_ptr<PluralRules>, DataError> create_cardinal(const DataProvider& provider, const Locale& locale);

  inline static diplomat::result<std::unique_ptr<PluralRules>, DataError> create_ordinal(const DataProvider& provider, const Locale& locale);

  inline PluralCategory category_for(const PluralOperands& op) const;

  inline PluralCategories categories() const;

  inline const diplomat::capi::PluralRules* AsFFI() const;
  inline diplomat::capi::PluralRules* AsFFI();
  inline static const PluralRules* FromFFI(const diplomat::capi::PluralRules* ptr);
  inline static PluralRules* FromFFI(diplomat::capi::PluralRules* ptr);
  inline static void operator delete(void* ptr);
private:
  PluralRules() = delete;
  PluralRules(const PluralRules&) = delete;
  PluralRules(PluralRules&&) noexcept = delete;
  PluralRules operator=(const PluralRules&) = delete;
  PluralRules operator=(PluralRules&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // PluralRules_D_HPP
