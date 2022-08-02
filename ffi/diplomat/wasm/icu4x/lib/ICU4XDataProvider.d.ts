import { FFIError } from "./diplomat-runtime"
import { ICU4XError } from "./ICU4XError";

/**

 * An ICU4X data provider, capable of loading ICU4X data keys from some source.

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_provider/index.html Rust documentation} for more information.
 */
export class ICU4XDataProvider {

  /**

   * Constructs an `FsDataProvider` and returns it as an {@link ICU4XDataProvider `ICU4XDataProvider`}. Requires the `provider_fs` feature. Not supported in WASM.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_provider_fs/struct.FsDataProvider.html Rust documentation} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static create_fs(path: string): ICU4XDataProvider | never;

  /**

   * Constructs a testdata provider and returns it as an {@link ICU4XDataProvider `ICU4XDataProvider`}. Requires the `provider_test` feature.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_testdata/index.html Rust documentation} for more information.
   */
  static create_test(): ICU4XDataProvider;

  /**

   * Constructs a `BlobDataProvider` and returns it as an {@link ICU4XDataProvider `ICU4XDataProvider`}.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_provider_blob/struct.BlobDataProvider.html Rust documentation} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static create_from_byte_slice(blob: Uint8Array): ICU4XDataProvider | never;

  /**

   * Constructs an empty `StaticDataProvider` and returns it as an {@link ICU4XDataProvider `ICU4XDataProvider`}.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_provider_blob/struct.StaticDataProvider.html Rust documentation} for more information.
   */
  static create_empty(): ICU4XDataProvider;
}
