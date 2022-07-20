import wasm from "../wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.js"
import { ICU4XDataProvider } from "./ICU4XDataProvider.js"

export class ICU4XCreateDataProviderResult {
  constructor(underlying) {
    this.provider = (() => {
      const option_ptr = diplomatRuntime.ptrRead(wasm, underlying);
      return (option_ptr == 0) ? null : new ICU4XDataProvider(option_ptr, true, []);
    })();
    this.success = (new Uint8Array(wasm.memory.buffer, underlying + 4, 1))[0] == 1;
  }
}
