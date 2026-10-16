#ifndef ICU4X_PartsSink_D_HPP
#define ICU4X_PartsSink_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"
namespace icu4x {
namespace capi { struct Parts; }
class Parts;
namespace capi { struct PartsSink; }
class PartsSink;
} // namespace icu4x



namespace icu4x {
namespace capi {
    struct PartsSink;
} // namespace capi
} // namespace

namespace icu4x {
/**
 * A mutable sink that collects {@link PartSpan}s produced during `*_to_parts` formatting methods,
 * and can be drained into an immutable {@link Parts} collection via {@link PartsSink::drain}.
 *
 * See the [Rust documentation for `PartsWrite`](https://docs.rs/writeable/0.6.4/writeable/trait.PartsWrite.html) for more information.
 *
 * 🚧 This API is unstable and may experience breaking changes outside major releases.
 */
class PartsSink {
public:

  /**
   * Construct a new empty {@link PartsSink}.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static std::unique_ptr<icu4x::PartsSink> create();

  /**
   * Construct a new empty {@link PartsSink} with pre-allocated capacity.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static std::unique_ptr<icu4x::PartsSink> create_with_capacity(size_t capacity);

  /**
   * Drains the collected {@link PartSpan}s into an immutable {@link Parts} collection without
   * copying the underlying span buffer, leaving `self` empty.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline std::unique_ptr<icu4x::Parts> drain();

    inline const icu4x::capi::PartsSink* AsFFI() const;
    inline icu4x::capi::PartsSink* AsFFI();
    inline static const icu4x::PartsSink* FromFFI(const icu4x::capi::PartsSink* ptr);
    inline static icu4x::PartsSink* FromFFI(icu4x::capi::PartsSink* ptr);
    inline static void operator delete(void* ptr);
private:
    PartsSink() = delete;
    PartsSink(const icu4x::PartsSink&) = delete;
    PartsSink(icu4x::PartsSink&&) noexcept = delete;
    PartsSink operator=(const icu4x::PartsSink&) = delete;
    PartsSink operator=(icu4x::PartsSink&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // ICU4X_PartsSink_D_HPP
