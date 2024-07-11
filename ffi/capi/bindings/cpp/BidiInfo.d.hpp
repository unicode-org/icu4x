#ifndef BidiInfo_D_HPP
#define BidiInfo_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

class BidiParagraph;


namespace capi {
    typedef struct BidiInfo BidiInfo;
}

class BidiInfo {
public:

  inline size_t paragraph_count() const;

  inline std::unique_ptr<BidiParagraph> paragraph_at(size_t n) const;

  inline size_t size() const;

  inline uint8_t level_at(size_t pos) const;

  inline const capi::BidiInfo* AsFFI() const;
  inline capi::BidiInfo* AsFFI();
  inline static const BidiInfo* FromFFI(const capi::BidiInfo* ptr);
  inline static BidiInfo* FromFFI(capi::BidiInfo* ptr);
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
