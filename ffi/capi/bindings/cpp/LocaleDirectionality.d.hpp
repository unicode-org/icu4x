#ifndef LocaleDirectionality_D_HPP
#define LocaleDirectionality_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {typedef struct DataProvider DataProvider; }
class DataProvider;
namespace capi {typedef struct Locale Locale; }
class Locale;
namespace capi {typedef struct LocaleExpander LocaleExpander; }
class LocaleExpander;
class DataError;
class LocaleDirection;


namespace diplomat {
namespace capi {
    typedef struct LocaleDirectionality LocaleDirectionality;
} // namespace capi
} // namespace

class LocaleDirectionality {
public:

  inline static diplomat::result<std::unique_ptr<LocaleDirectionality>, DataError> create(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<LocaleDirectionality>, DataError> create_with_expander(const DataProvider& provider, const LocaleExpander& expander);

  inline LocaleDirection get(const Locale& locale) const;

  inline bool is_left_to_right(const Locale& locale) const;

  inline bool is_right_to_left(const Locale& locale) const;

  inline const diplomat::capi::LocaleDirectionality* AsFFI() const;
  inline diplomat::capi::LocaleDirectionality* AsFFI();
  inline static const LocaleDirectionality* FromFFI(const diplomat::capi::LocaleDirectionality* ptr);
  inline static LocaleDirectionality* FromFFI(diplomat::capi::LocaleDirectionality* ptr);
  inline static void operator delete(void* ptr);
private:
  LocaleDirectionality() = delete;
  LocaleDirectionality(const LocaleDirectionality&) = delete;
  LocaleDirectionality(LocaleDirectionality&&) noexcept = delete;
  LocaleDirectionality operator=(const LocaleDirectionality&) = delete;
  LocaleDirectionality operator=(LocaleDirectionality&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // LocaleDirectionality_D_HPP
