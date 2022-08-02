import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.js"
import { ICU4XCodePointMapData16 } from "./ICU4XCodePointMapData16.js"

export class ICU4XCodePointMapData16Response {
  constructor(underlying) {
    this.data = (() => {
      const option_ptr = diplomatRuntime.ptrRead(wasm, underlying);
      return (option_ptr == 0) ? null : new ICU4XCodePointMapData16(option_ptr, true, []);
    })();
    this.success = (new Uint8Array(wasm.memory.buffer, underlying + 4, 1))[0] == 1;
  }
}
