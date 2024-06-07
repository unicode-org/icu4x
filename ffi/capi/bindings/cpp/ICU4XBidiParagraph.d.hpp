#ifndef ICU4XBidiParagraph_D_HPP
#define ICU4XBidiParagraph_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XBidiDirection.d.hpp"
#include "ICU4XBidiParagraph.d.h"

class ICU4XBidiDirection;


class ICU4XBidiParagraph {
public:

  inline bool set_paragraph_in_text(size_t n);

  inline ICU4XBidiDirection direction() const;

  inline size_t size() const;

  inline size_t range_start() const;

  inline size_t range_end() const;

  inline std::optional<std::string> reorder_line(size_t range_start, size_t range_end) const;

  inline uint8_t level_at(size_t pos) const;

  inline const capi::ICU4XBidiParagraph* AsFFI() const;
  inline capi::ICU4XBidiParagraph* AsFFI();
  inline static const ICU4XBidiParagraph* FromFFI(const capi::ICU4XBidiParagraph* ptr);
  inline static ICU4XBidiParagraph* FromFFI(capi::ICU4XBidiParagraph* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XBidiParagraph() = delete;
  ICU4XBidiParagraph(const ICU4XBidiParagraph&) = delete;
  ICU4XBidiParagraph(ICU4XBidiParagraph&&) noexcept = delete;
  ICU4XBidiParagraph operator=(const ICU4XBidiParagraph&) = delete;
  ICU4XBidiParagraph operator=(ICU4XBidiParagraph&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XBidiParagraph_D_HPP
