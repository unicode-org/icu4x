import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { DataError_js_to_rust, DataError_rust_to_js } from "./DataError.mjs"
import { LineBreakIteratorLatin1 } from "./LineBreakIteratorLatin1.mjs"
import { LineBreakIteratorUtf16 } from "./LineBreakIteratorUtf16.mjs"
import { LineBreakIteratorUtf8 } from "./LineBreakIteratorUtf8.mjs"
import { LineBreakStrictness_js_to_rust, LineBreakStrictness_rust_to_js } from "./LineBreakStrictness.mjs"
import { LineBreakWordOption_js_to_rust, LineBreakWordOption_rust_to_js } from "./LineBreakWordOption.mjs"

const LineSegmenter_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XLineSegmenter_destroy(underlying);
});

export class LineSegmenter {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      LineSegmenter_box_destroy_registry.register(this, underlying);
    }
  }

  static create_auto(arg_provider) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XLineSegmenter_create_auto(diplomat_receive_buffer, arg_provider.underlying);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new LineSegmenter(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = DataError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  static create_lstm(arg_provider) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XLineSegmenter_create_lstm(diplomat_receive_buffer, arg_provider.underlying);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new LineSegmenter(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = DataError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  static create_dictionary(arg_provider) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XLineSegmenter_create_dictionary(diplomat_receive_buffer, arg_provider.underlying);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new LineSegmenter(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = DataError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  static create_auto_with_options_v1(arg_provider, arg_options) {
    const field_strictness_arg_options = arg_options["strictness"];
    const field_word_option_arg_options = arg_options["word_option"];
    const field_ja_zh_arg_options = arg_options["ja_zh"];
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XLineSegmenter_create_auto_with_options_v1(diplomat_receive_buffer, arg_provider.underlying, LineBreakStrictness_js_to_rust[field_strictness_arg_options], LineBreakWordOption_js_to_rust[field_word_option_arg_options], field_ja_zh_arg_options);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new LineSegmenter(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = DataError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  static create_lstm_with_options_v1(arg_provider, arg_options) {
    const field_strictness_arg_options = arg_options["strictness"];
    const field_word_option_arg_options = arg_options["word_option"];
    const field_ja_zh_arg_options = arg_options["ja_zh"];
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XLineSegmenter_create_lstm_with_options_v1(diplomat_receive_buffer, arg_provider.underlying, LineBreakStrictness_js_to_rust[field_strictness_arg_options], LineBreakWordOption_js_to_rust[field_word_option_arg_options], field_ja_zh_arg_options);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new LineSegmenter(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = DataError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  static create_dictionary_with_options_v1(arg_provider, arg_options) {
    const field_strictness_arg_options = arg_options["strictness"];
    const field_word_option_arg_options = arg_options["word_option"];
    const field_ja_zh_arg_options = arg_options["ja_zh"];
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XLineSegmenter_create_dictionary_with_options_v1(diplomat_receive_buffer, arg_provider.underlying, LineBreakStrictness_js_to_rust[field_strictness_arg_options], LineBreakWordOption_js_to_rust[field_word_option_arg_options], field_ja_zh_arg_options);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new LineSegmenter(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
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
    const diplomat_out = new LineBreakIteratorUtf8(wasm.ICU4XLineSegmenter_segment_utf8(this.underlying, buf_arg_input.ptr, buf_arg_input.size), true, [this, buf_arg_input]);
    buf_arg_input.garbageCollect();
    return diplomat_out;
  }

  segment_utf16(arg_input) {
    const buf_arg_input = diplomatRuntime.DiplomatBuf.str16(wasm, arg_input);
    const diplomat_out = new LineBreakIteratorUtf16(wasm.ICU4XLineSegmenter_segment_utf16(this.underlying, buf_arg_input.ptr, buf_arg_input.size), true, [this, buf_arg_input]);
    buf_arg_input.garbageCollect();
    return diplomat_out;
  }

  segment_latin1(arg_input) {
    const buf_arg_input = diplomatRuntime.DiplomatBuf.slice(wasm, arg_input, "u8");
    const diplomat_out = new LineBreakIteratorLatin1(wasm.ICU4XLineSegmenter_segment_latin1(this.underlying, buf_arg_input.ptr, buf_arg_input.size), true, [this, buf_arg_input]);
    buf_arg_input.garbageCollect();
    return diplomat_out;
  }
}
