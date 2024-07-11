#ifndef FixedDecimalFormatter_D_HPP
#define FixedDecimalFormatter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {typedef struct DataProvider DataProvider; }
class DataProvider;
namespace capi {typedef struct DataStruct DataStruct; }
class DataStruct;
namespace capi {typedef struct FixedDecimal FixedDecimal; }
class FixedDecimal;
namespace capi {typedef struct Locale Locale; }
class Locale;
class DataError;
class FixedDecimalGroupingStrategy;


namespace diplomat {
namespace capi {
    typedef struct FixedDecimalFormatter FixedDecimalFormatter;
} // namespace capi
} // namespace

class FixedDecimalFormatter {
public:

  inline static diplomat::result<std::unique_ptr<FixedDecimalFormatter>, DataError> create_with_grouping_strategy(const DataProvider& provider, const Locale& locale, FixedDecimalGroupingStrategy grouping_strategy);

  inline static diplomat::result<std::unique_ptr<FixedDecimalFormatter>, DataError> create_with_decimal_symbols_v1(const DataStruct& data_struct, FixedDecimalGroupingStrategy grouping_strategy);

  inline std::string format(const FixedDecimal& value) const;

  inline const diplomat::capi::FixedDecimalFormatter* AsFFI() const;
  inline diplomat::capi::FixedDecimalFormatter* AsFFI();
  inline static const FixedDecimalFormatter* FromFFI(const diplomat::capi::FixedDecimalFormatter* ptr);
  inline static FixedDecimalFormatter* FromFFI(diplomat::capi::FixedDecimalFormatter* ptr);
  inline static void operator delete(void* ptr);
private:
  FixedDecimalFormatter() = delete;
  FixedDecimalFormatter(const FixedDecimalFormatter&) = delete;
  FixedDecimalFormatter(FixedDecimalFormatter&&) noexcept = delete;
  FixedDecimalFormatter operator=(const FixedDecimalFormatter&) = delete;
  FixedDecimalFormatter operator=(FixedDecimalFormatter&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // FixedDecimalFormatter_D_HPP
