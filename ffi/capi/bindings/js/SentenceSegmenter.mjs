import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { DataError_js_to_rust, DataError_rust_to_js } from "./DataError.mjs"
import { SentenceBreakIteratorLatin1 } from "./SentenceBreakIteratorLatin1.mjs"
import { SentenceBreakIteratorUtf16 } from "./SentenceBreakIteratorUtf16.mjs"
import { SentenceBreakIteratorUtf8 } from "./SentenceBreakIteratorUtf8.mjs"

const SentenceSegmenter_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XSentenceSegmenter_destroy(underlying);
});

export class SentenceSegmenter {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      SentenceSegmenter_box_destroy_registry.register(this, underlying);
    }
  }

  static create(arg_provider) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XSentenceSegmenter_create(diplomat_receive_buffer, arg_provider.underlying);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new SentenceSegmenter(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = DataError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  segment_utf8(arg_input) {
    const buf_arg_input = diplomatRuntime.DiplomatBuf.str8(wasm, arg_input);
    const diplomat_out = new SentenceBreakIteratorUtf8(wasm.ICU4XSentenceSegmenter_segment_utf8(this.underlying, buf_arg_input.ptr, buf_arg_input.size), true, [this, buf_arg_input]);
    buf_arg_input.garbageCollect();
    return diplomat_out;
  }

  segment_utf16(arg_input) {
    const buf_arg_input = diplomatRuntime.DiplomatBuf.str16(wasm, arg_input);
    const diplomat_out = new SentenceBreakIteratorUtf16(wasm.ICU4XSentenceSegmenter_segment_utf16(this.underlying, buf_arg_input.ptr, buf_arg_input.size), true, [this, buf_arg_input]);
    buf_arg_input.garbageCollect();
    return diplomat_out;
  }

  segment_latin1(arg_input) {
    const buf_arg_input = diplomatRuntime.DiplomatBuf.slice(wasm, arg_input, "u8");
    const diplomat_out = new SentenceBreakIteratorLatin1(wasm.ICU4XSentenceSegmenter_segment_latin1(this.underlying, buf_arg_input.ptr, buf_arg_input.size), true, [this, buf_arg_input]);
    buf_arg_input.garbageCollect();
    return diplomat_out;
  }
}
