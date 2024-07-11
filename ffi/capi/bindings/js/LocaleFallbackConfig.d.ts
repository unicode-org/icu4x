import { LocaleFallbackPriority } from "./LocaleFallbackPriority";
import { LocaleFallbackSupplement } from "./LocaleFallbackSupplement";

/**

 * Collection of configurations for the ICU4X fallback algorithm.

 * See the {@link https://docs.rs/icu/latest/icu/locale/fallback/struct.LocaleFallbackConfig.html Rust documentation for `LocaleFallbackConfig`} for more information.
 */
export class LocaleFallbackConfig {
  priority: LocaleFallbackPriority;
  extension_key: string;
  fallback_supplement: LocaleFallbackSupplement;
}
