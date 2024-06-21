#ifndef ICU4XLogger_D_HPP
#define ICU4XLogger_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef struct ICU4XLogger ICU4XLogger;
}

class ICU4XLogger {
public:

  inline static bool init_simple_logger();

  inline const capi::ICU4XLogger* AsFFI() const;
  inline capi::ICU4XLogger* AsFFI();
  inline static const ICU4XLogger* FromFFI(const capi::ICU4XLogger* ptr);
  inline static ICU4XLogger* FromFFI(capi::ICU4XLogger* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XLogger() = delete;
  ICU4XLogger(const ICU4XLogger&) = delete;
  ICU4XLogger(ICU4XLogger&&) noexcept = delete;
  ICU4XLogger operator=(const ICU4XLogger&) = delete;
  ICU4XLogger operator=(ICU4XLogger&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XLogger_D_HPP
