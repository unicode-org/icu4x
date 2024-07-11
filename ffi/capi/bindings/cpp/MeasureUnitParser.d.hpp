#ifndef MeasureUnitParser_D_HPP
#define MeasureUnitParser_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

class MeasureUnit;


namespace capi {
    typedef struct MeasureUnitParser MeasureUnitParser;
}

class MeasureUnitParser {
public:

  inline std::unique_ptr<MeasureUnit> parse(std::string_view unit_id) const;

  inline const capi::MeasureUnitParser* AsFFI() const;
  inline capi::MeasureUnitParser* AsFFI();
  inline static const MeasureUnitParser* FromFFI(const capi::MeasureUnitParser* ptr);
  inline static MeasureUnitParser* FromFFI(capi::MeasureUnitParser* ptr);
  inline static void operator delete(void* ptr);
private:
  MeasureUnitParser() = delete;
  MeasureUnitParser(const MeasureUnitParser&) = delete;
  MeasureUnitParser(MeasureUnitParser&&) noexcept = delete;
  MeasureUnitParser operator=(const MeasureUnitParser&) = delete;
  MeasureUnitParser operator=(MeasureUnitParser&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // MeasureUnitParser_D_HPP
