
/**

 * An object allowing control over the logging used
 */
export class ICU4XLogger {

  /**

   * Initialize the logger using `simple_logger`, or console.log/warn in WASM.

   * Returns `false` if there was already a logger set, or if logging has not been compiled into the platform
   */
  static init_simple_logger(): boolean;
}
