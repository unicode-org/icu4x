#ifndef BidiInfo_D_HPP
#define BidiInfo_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace diplomat::capi { struct BidiParagraph; }
class BidiParagraph;


namespace diplomat {
namespace capi {
    struct BidiInfo;
} // namespace capi
} // namespace

class BidiInfo {
public:

  inline size_t paragraph_count() const;

  inline std::unique_ptr<BidiParagraph> paragraph_at(size_t n) const;

  inline size_t size() const;

  inline uint8_t level_at(size_t pos) const;

  inline const diplomat::capi::BidiInfo* AsFFI() const;
  inline diplomat::capi::BidiInfo* AsFFI();
  inline static const BidiInfo* FromFFI(const diplomat::capi::BidiInfo* ptr);
  inline static BidiInfo* FromFFI(diplomat::capi::BidiInfo* ptr);
  inline static void operator delete(void* ptr);
private:
  BidiInfo() = delete;
  BidiInfo(const BidiInfo&) = delete;
  BidiInfo(BidiInfo&&) noexcept = delete;
  BidiInfo operator=(const BidiInfo&) = delete;
  BidiInfo operator=(BidiInfo&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // BidiInfo_D_HPP
