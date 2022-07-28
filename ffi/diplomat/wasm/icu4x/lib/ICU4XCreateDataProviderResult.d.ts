import { ICU4XDataProvider } from "./ICU4XDataProvider";

/**

 * A result type for `ICU4XDataProvider::create`.
 */
export class ICU4XCreateDataProviderResult {
  provider?: ICU4XDataProvider;
  success: boolean;
}
