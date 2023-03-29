#ifndef CodePointRangeIteratorResult_HPP
#define CodePointRangeIteratorResult_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "CodePointRangeIteratorResult.h"



/**
 * Result of a single iteration of [`CodePointRangeIterator`].
 * Logically can be considered to be an `Option<RangeInclusive<u32>>`,
 * 
 * `start` and `end` represent an inclusive range of code points [start, end],
 * and `done` will be true when the iterator finishes.
 */
struct CodePointRangeIteratorResult {
 public:
  uint32_t start;
  uint32_t end;
  bool done;
};


#endif
