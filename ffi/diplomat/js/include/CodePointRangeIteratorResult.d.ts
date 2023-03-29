import { u32 } from "./diplomat-runtime"

/**

 * Result of a single iteration of {@link CodePointRangeIterator `CodePointRangeIterator`}. Logically can be considered to be an `Option<RangeInclusive<u32>>`,

 * `start` and `end` represent an inclusive range of code points start, end, and `done` will be true when the iterator finishes.
 */
export class CodePointRangeIteratorResult {
  start: u32;
  end: u32;
  done: boolean;
}
