#ifndef Logger_D_HPP
#define Logger_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    struct Logger;
} // namespace capi
} // namespace

class Logger {
public:

  inline static bool init_simple_logger();

  inline const diplomat::capi::Logger* AsFFI() const;
  inline diplomat::capi::Logger* AsFFI();
  inline static const Logger* FromFFI(const diplomat::capi::Logger* ptr);
  inline static Logger* FromFFI(diplomat::capi::Logger* ptr);
  inline static void operator delete(void* ptr);
private:
  Logger() = delete;
  Logger(const Logger&) = delete;
  Logger(Logger&&) noexcept = delete;
  Logger operator=(const Logger&) = delete;
  Logger operator=(Logger&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // Logger_D_HPP
