#ifndef Collator_D_HPP
#define Collator_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "CollatorOptionsV1.d.hpp"
#include "CollatorResolvedOptionsV1.d.hpp"
#include "DataError.d.hpp"

class DataProvider;
class Locale;
struct CollatorOptionsV1;
struct CollatorResolvedOptionsV1;
class DataError;


namespace capi {
    typedef struct Collator Collator;
}

class Collator {
public:

  inline static diplomat::result<std::unique_ptr<Collator>, DataError> create_v1(const DataProvider& provider, const Locale& locale, CollatorOptionsV1 options);

  inline int8_t compare16(std::u16string_view left, std::u16string_view right) const;

  inline int8_t compare(std::string_view left, std::string_view right) const;

  inline CollatorResolvedOptionsV1 resolved_options() const;

  inline const capi::Collator* AsFFI() const;
  inline capi::Collator* AsFFI();
  inline static const Collator* FromFFI(const capi::Collator* ptr);
  inline static Collator* FromFFI(capi::Collator* ptr);
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
