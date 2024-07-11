#ifndef SegmenterWordType_D_HPP
#define SegmenterWordType_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    typedef enum SegmenterWordType {
      SegmenterWordType_None = 0,
      SegmenterWordType_Number = 1,
      SegmenterWordType_Letter = 2,
    } SegmenterWordType;
} // namespace capi
} // namespace

class SegmenterWordType {
public:
  enum Value {
    None = 0,
    Number = 1,
    Letter = 2,
  };

  SegmenterWordType() = default;
  // Implicit conversions between enum and ::Value
  constexpr SegmenterWordType(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline bool is_word_like();

  inline diplomat::capi::SegmenterWordType AsFFI() const;
  inline static SegmenterWordType FromFFI(diplomat::capi::SegmenterWordType c_enum);
private:
    Value value;
};


#endif // SegmenterWordType_D_HPP
