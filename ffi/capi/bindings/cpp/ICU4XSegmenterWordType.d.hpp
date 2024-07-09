#ifndef ICU4XSegmenterWordType_D_HPP
#define ICU4XSegmenterWordType_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef enum ICU4XSegmenterWordType {
      ICU4XSegmenterWordType_None = 0,
      ICU4XSegmenterWordType_Number = 1,
      ICU4XSegmenterWordType_Letter = 2,
    } ICU4XSegmenterWordType;
}

class ICU4XSegmenterWordType {
public:
  enum Value {
    None = 0,
    Number = 1,
    Letter = 2,
  };

  ICU4XSegmenterWordType() = default;
  // Implicit conversions between enum and ::Value
  constexpr ICU4XSegmenterWordType(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline bool is_word_like();

  inline capi::ICU4XSegmenterWordType AsFFI() const;
  inline static ICU4XSegmenterWordType FromFFI(capi::ICU4XSegmenterWordType c_enum);
private:
    Value value;
};


#endif // ICU4XSegmenterWordType_D_HPP
