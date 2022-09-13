import { ICU4XLocaleFallbackPriority } from "./ICU4XLocaleFallbackPriority";

/**

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbackConfig.html Rust documentation for `LocaleFallbackConfig`} for more information.
 */
export class ICU4XLocaleFallbackConfig {
  priority: ICU4XLocaleFallbackPriority;
  extension_key: string;
}
