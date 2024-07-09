#ifndef ICU4XCodePointSetBuilder_D_HPP
#define ICU4XCodePointSetBuilder_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

class ICU4XCodePointSetData;


namespace capi {
    typedef struct ICU4XCodePointSetBuilder ICU4XCodePointSetBuilder;
}

class ICU4XCodePointSetBuilder {
public:

  inline static std::unique_ptr<ICU4XCodePointSetBuilder> create();

  inline std::unique_ptr<ICU4XCodePointSetData> build();

  inline void complement();

  inline bool is_empty() const;

  inline void add_char(char32_t ch);

  inline void add_inclusive_range(char32_t start, char32_t end);

  inline void add_set(const ICU4XCodePointSetData& data);

  inline void remove_char(char32_t ch);

  inline void remove_inclusive_range(char32_t start, char32_t end);

  inline void remove_set(const ICU4XCodePointSetData& data);

  inline void retain_char(char32_t ch);

  inline void retain_inclusive_range(char32_t start, char32_t end);

  inline void retain_set(const ICU4XCodePointSetData& data);

  inline void complement_char(char32_t ch);

  inline void complement_inclusive_range(char32_t start, char32_t end);

  inline void complement_set(const ICU4XCodePointSetData& data);

  inline const capi::ICU4XCodePointSetBuilder* AsFFI() const;
  inline capi::ICU4XCodePointSetBuilder* AsFFI();
  inline static const ICU4XCodePointSetBuilder* FromFFI(const capi::ICU4XCodePointSetBuilder* ptr);
  inline static ICU4XCodePointSetBuilder* FromFFI(capi::ICU4XCodePointSetBuilder* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XCodePointSetBuilder() = delete;
  ICU4XCodePointSetBuilder(const ICU4XCodePointSetBuilder&) = delete;
  ICU4XCodePointSetBuilder(ICU4XCodePointSetBuilder&&) noexcept = delete;
  ICU4XCodePointSetBuilder operator=(const ICU4XCodePointSetBuilder&) = delete;
  ICU4XCodePointSetBuilder operator=(ICU4XCodePointSetBuilder&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XCodePointSetBuilder_D_HPP
