#ifndef ICU4X_Parts_D_HPP
#define ICU4X_Parts_D_HPP

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
struct PartSpan;
} // namespace icu4x



namespace icu4x {
namespace capi {
    struct Parts;
} // namespace capi
} // namespace

namespace icu4x {
/**
 * A mutable collection of {@link PartSpan}s populated by `*_to_parts` formatting methods.
 *
 * Calling a `*_to_parts` method clears any previously recorded spans in {@link Parts}
 * and records the new spans in pre-order, reusing the underlying allocation.
 *
 * See the [Rust documentation for `PartsWrite`](https://docs.rs/writeable/0.6.4/writeable/trait.PartsWrite.html) for more information.
 *
 * 🚧 This API is unstable and may experience breaking changes outside major releases.
 */
class Parts {
public:

  /**
   * Construct a new empty {@link Parts}.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static std::unique_ptr<icu4x::Parts> create();

  /**
   * Construct a new empty {@link Parts} with pre-allocated capacity.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static std::unique_ptr<icu4x::Parts> create_with_capacity(size_t capacity);

  /**
   * Returns the number of recorded {@link PartSpan}s.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline size_t len() const;

  /**
   * Returns whether there are no recorded {@link PartSpan}s.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline bool is_empty() const;

  /**
   * Returns the {@link PartSpan} at `index`, or `None` if out of bounds.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline std::optional<icu4x::PartSpan> operator[](size_t index) const;

    inline const icu4x::capi::Parts* AsFFI() const;
    inline icu4x::capi::Parts* AsFFI();
    inline static const icu4x::Parts* FromFFI(const icu4x::capi::Parts* ptr);
    inline static icu4x::Parts* FromFFI(icu4x::capi::Parts* ptr);
    inline static void operator delete(void* ptr);
private:
    Parts() = delete;
    Parts(const icu4x::Parts&) = delete;
    Parts(icu4x::Parts&&) noexcept = delete;
    Parts operator=(const icu4x::Parts&) = delete;
    Parts operator=(icu4x::Parts&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // ICU4X_Parts_D_HPP
