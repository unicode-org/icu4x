#ifndef CodePointSetBuilder_D_HPP
#define CodePointSetBuilder_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

class CodePointSetData;


namespace capi {
    typedef struct CodePointSetBuilder CodePointSetBuilder;
}

class CodePointSetBuilder {
public:

  inline static std::unique_ptr<CodePointSetBuilder> create();

  inline std::unique_ptr<CodePointSetData> build();

  inline void complement();

  inline bool is_empty() const;

  inline void add_char(char32_t ch);

  inline void add_inclusive_range(char32_t start, char32_t end);

  inline void add_set(const CodePointSetData& data);

  inline void remove_char(char32_t ch);

  inline void remove_inclusive_range(char32_t start, char32_t end);

  inline void remove_set(const CodePointSetData& data);

  inline void retain_char(char32_t ch);

  inline void retain_inclusive_range(char32_t start, char32_t end);

  inline void retain_set(const CodePointSetData& data);

  inline void complement_char(char32_t ch);

  inline void complement_inclusive_range(char32_t start, char32_t end);

  inline void complement_set(const CodePointSetData& data);

  inline const capi::CodePointSetBuilder* AsFFI() const;
  inline capi::CodePointSetBuilder* AsFFI();
  inline static const CodePointSetBuilder* FromFFI(const capi::CodePointSetBuilder* ptr);
  inline static CodePointSetBuilder* FromFFI(capi::CodePointSetBuilder* ptr);
  inline static void operator delete(void* ptr);
private:
  CodePointSetBuilder() = delete;
  CodePointSetBuilder(const CodePointSetBuilder&) = delete;
  CodePointSetBuilder(CodePointSetBuilder&&) noexcept = delete;
  CodePointSetBuilder operator=(const CodePointSetBuilder&) = delete;
  CodePointSetBuilder operator=(CodePointSetBuilder&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // CodePointSetBuilder_D_HPP
