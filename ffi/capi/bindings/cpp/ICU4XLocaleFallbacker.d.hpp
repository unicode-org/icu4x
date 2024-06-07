#ifndef ICU4XLocaleFallbacker_D_HPP
#define ICU4XLocaleFallbacker_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XError.d.hpp"
#include "ICU4XLocaleFallbackConfig.d.hpp"
#include "ICU4XLocaleFallbacker.d.h"

class ICU4XDataProvider;
class ICU4XLocaleFallbackerWithConfig;
struct ICU4XLocaleFallbackConfig;
class ICU4XError;


class ICU4XLocaleFallbacker {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XLocaleFallbacker>, ICU4XError> create(const ICU4XDataProvider& provider);

  inline static std::unique_ptr<ICU4XLocaleFallbacker> create_without_data();

  inline diplomat::result<std::unique_ptr<ICU4XLocaleFallbackerWithConfig>, ICU4XError> for_config(ICU4XLocaleFallbackConfig config) const;

  inline const capi::ICU4XLocaleFallbacker* AsFFI() const;
  inline capi::ICU4XLocaleFallbacker* AsFFI();
  inline static const ICU4XLocaleFallbacker* FromFFI(const capi::ICU4XLocaleFallbacker* ptr);
  inline static ICU4XLocaleFallbacker* FromFFI(capi::ICU4XLocaleFallbacker* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XLocaleFallbacker() = delete;
  ICU4XLocaleFallbacker(const ICU4XLocaleFallbacker&) = delete;
  ICU4XLocaleFallbacker(ICU4XLocaleFallbacker&&) noexcept = delete;
  ICU4XLocaleFallbacker operator=(const ICU4XLocaleFallbacker&) = delete;
  ICU4XLocaleFallbacker operator=(ICU4XLocaleFallbacker&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XLocaleFallbacker_D_HPP
