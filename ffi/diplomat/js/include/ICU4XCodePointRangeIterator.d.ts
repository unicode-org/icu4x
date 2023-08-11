import { ICU4XCodePointRangeIteratorResult } from "./ICU4XCodePointRangeIteratorResult";

/**

 * An iterator over code point ranges, produced by `ICU4XCodePointSetData` or one of the `ICU4XCodePointMapData` types
 */
export class ICU4XCodePointRangeIterator {

  /**

   * Advance the iterator by one and return the next range.

   * If the iterator is out of items, `done` will be true
   */
  next(): ICU4XCodePointRangeIteratorResult;
}
