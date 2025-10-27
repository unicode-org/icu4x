#ifndef ICU4X_CollationSortKey_D_HPP
#define ICU4X_CollationSortKey_D_HPP

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
namespace capi {
    struct CollationSortKey;
} // namespace capi
} // namespace

namespace icu4x {
/**
 * See the [Rust documentation for `write_sort_key_to`](https://docs.rs/icu/2.0.0/icu/collator/struct.CollatorBorrowed.html#method.write_sort_key_to) for more information.
 */
class CollationSortKey {
public:

  inline icu4x::diplomat::span<const uint8_t> as_bytes() const;

    inline const icu4x::capi::CollationSortKey* AsFFI() const;
    inline icu4x::capi::CollationSortKey* AsFFI();
    inline static const icu4x::CollationSortKey* FromFFI(const icu4x::capi::CollationSortKey* ptr);
    inline static icu4x::CollationSortKey* FromFFI(icu4x::capi::CollationSortKey* ptr);
    inline static void operator delete(void* ptr);
private:
    CollationSortKey() = delete;
    CollationSortKey(const icu4x::CollationSortKey&) = delete;
    CollationSortKey(icu4x::CollationSortKey&&) noexcept = delete;
    CollationSortKey operator=(const icu4x::CollationSortKey&) = delete;
    CollationSortKey operator=(icu4x::CollationSortKey&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // ICU4X_CollationSortKey_D_HPP
