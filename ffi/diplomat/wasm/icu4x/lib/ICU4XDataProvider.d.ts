import { FFIError } from "./diplomat-runtime"
import { ICU4XError } from "./ICU4XError";

/**

 * An ICU4X data provider, capable of loading ICU4X data keys from some source.

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_provider/index.html Rust documentation for `icu_provider`} for more information.
 */
export class ICU4XDataProvider {

  /**

   * Constructs an `FsDataProvider` and returns it as an {@link ICU4XDataProvider `ICU4XDataProvider`}. Requires the `provider_fs` feature. Not supported in WASM.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_provider_fs/struct.FsDataProvider.html Rust documentation for `FsDataProvider`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static create_fs(path: string): ICU4XDataProvider | never;

  /**

   * Constructs a testdata provider and returns it as an {@link ICU4XDataProvider `ICU4XDataProvider`}. Requires the `provider_test` feature.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_testdata/index.html Rust documentation for `icu_testdata`} for more information.
   */
  static create_test(): ICU4XDataProvider;

  /**

   * Constructs a `BlobDataProvider` and returns it as an {@link ICU4XDataProvider `ICU4XDataProvider`}.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_provider_blob/struct.BlobDataProvider.html Rust documentation for `BlobDataProvider`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static create_from_byte_slice(blob: Uint8Array): ICU4XDataProvider | never;

  /**

   * Constructs an empty `StaticDataProvider` and returns it as an {@link ICU4XDataProvider `ICU4XDataProvider`}.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/empty/struct.EmptyDataProvider.html Rust documentation for `EmptyDataProvider`} for more information.
   */
  static create_empty(): ICU4XDataProvider;

  /**

   * Creates a provider that tries the current provider and then, if the current provider doesn't support the data key, another provider `other`.

   * This takes ownership of the `other` provider, leaving an empty provider in its place.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fork/type.ForkByKeyProvider.html Rust documentation for `ForkByKeyProvider`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  fork_by_key(other: ICU4XDataProvider): void | never;
}
