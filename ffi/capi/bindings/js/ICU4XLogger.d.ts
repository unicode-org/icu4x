
/**

 * An object allowing control over the logging used
 */
export class ICU4XLogger {

  /**

   * Initialize the logger using `simple_logger`

   * Requires the `simple_logger` Cargo feature.

   * Returns `false` if there was already a logger set.
   */
  static init_simple_logger(): boolean;
}
