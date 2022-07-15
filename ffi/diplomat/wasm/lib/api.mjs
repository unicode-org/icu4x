import wasm from "./wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
const diplomat_alloc_destroy_registry = new FinalizationRegistry(obj => {
  wasm.diplomat_free(obj["ptr"], obj["size"], obj["align"]);
});

const ICU4XBidi_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XBidi_destroy(underlying);
});

export class ICU4XBidi {
  constructor(underlying) {
    this.underlying = underlying;
  }

  static try_new(provider) {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      const result_tag = {};
      diplomat_alloc_destroy_registry.register(result_tag, {
        ptr: diplomat_receive_buffer,
        size: 5,
        align: 4,
      });
      wasm.ICU4XBidi_try_new(diplomat_receive_buffer, provider.underlying);
      const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
      if (is_ok) {
        const ok_value = (() => {
          const out = new ICU4XBidi((new Uint32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]);
          out.owner = result_tag;
          return out;
        })();
        return ok_value;
      } else {
        const throw_value = ICU4XError_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    return diplomat_out;
  }

  for_text(text, default_level) {
    let text_diplomat_bytes = (new TextEncoder()).encode(text);
    let text_diplomat_ptr = wasm.diplomat_alloc(text_diplomat_bytes.length, 1);
    let text_diplomat_buf = new Uint8Array(wasm.memory.buffer, text_diplomat_ptr, text_diplomat_bytes.length);
    text_diplomat_buf.set(text_diplomat_bytes, 0);
    const diplomat_out = (() => {
      const out = (() => {
        const out = new ICU4XBidiInfo(wasm.ICU4XBidi_for_text(this.underlying, text_diplomat_ptr, text_diplomat_bytes.length, default_level));
        out.owner = null;
        return out;
      })();
      ICU4XBidiInfo_box_destroy_registry.register(out, out.underlying)
      return out;
    })();
    wasm.diplomat_free(text_diplomat_ptr, text_diplomat_bytes.length, 1);
    return diplomat_out;
  }

  static level_is_rtl(level) {
    const diplomat_out = wasm.ICU4XBidi_level_is_rtl(level);
    return diplomat_out;
  }

  static level_is_ltr(level) {
    const diplomat_out = wasm.ICU4XBidi_level_is_ltr(level);
    return diplomat_out;
  }

  static level_rtl() {
    const diplomat_out = wasm.ICU4XBidi_level_rtl();
    return diplomat_out;
  }

  static level_ltr() {
    const diplomat_out = wasm.ICU4XBidi_level_ltr();
    return diplomat_out;
  }
}

const ICU4XBidiDirection_js_to_rust = {
  "Ltr": 0,
  "Rtl": 1,
  "Mixed": 2,
};
const ICU4XBidiDirection_rust_to_js = {
  0: "Ltr",
  1: "Rtl",
  2: "Mixed",
};

const ICU4XBidiInfo_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XBidiInfo_destroy(underlying);
});

export class ICU4XBidiInfo {
  constructor(underlying) {
    this.underlying = underlying;
  }

  paragraph_count() {
    const diplomat_out = wasm.ICU4XBidiInfo_paragraph_count(this.underlying);
    return diplomat_out;
  }

  paragraph_at(n) {
    const diplomat_out = (() => {
      const option_value = wasm.ICU4XBidiInfo_paragraph_at(this.underlying, n)
      if (option_value !== 0) {
        const inhabited_value = (() => {
          const out = (() => {
            const out = new ICU4XBidiParagraph(option_value);
            out.owner = null;
            return out;
          })();
          ICU4XBidiParagraph_box_destroy_registry.register(out, out.underlying)
          return out;
        })();
        return inhabited_value;
      } else {
        return null;
      }
    })();
    return diplomat_out;
  }

  size() {
    const diplomat_out = wasm.ICU4XBidiInfo_size(this.underlying);
    return diplomat_out;
  }

  level_at(pos) {
    const diplomat_out = wasm.ICU4XBidiInfo_level_at(this.underlying, pos);
    return diplomat_out;
  }
}

const ICU4XBidiParagraph_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XBidiParagraph_destroy(underlying);
});

export class ICU4XBidiParagraph {
  constructor(underlying) {
    this.underlying = underlying;
  }

  set_paragraph_in_text(n) {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      const result_tag = {};
      diplomat_alloc_destroy_registry.register(result_tag, {
        ptr: diplomat_receive_buffer,
        size: 5,
        align: 4,
      });
      wasm.ICU4XBidiParagraph_set_paragraph_in_text(diplomat_receive_buffer, this.underlying, n);
      const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
      if (is_ok) {
        const ok_value = {};
        return ok_value;
      } else {
        const throw_value = ICU4XError_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    return diplomat_out;
  }

  direction() {
    const diplomat_out = ICU4XBidiDirection_rust_to_js[wasm.ICU4XBidiParagraph_direction(this.underlying)];
    return diplomat_out;
  }

  size() {
    const diplomat_out = wasm.ICU4XBidiParagraph_size(this.underlying);
    return diplomat_out;
  }

  range_start() {
    const diplomat_out = wasm.ICU4XBidiParagraph_range_start(this.underlying);
    return diplomat_out;
  }

  range_end() {
    const diplomat_out = wasm.ICU4XBidiParagraph_range_end(this.underlying);
    return diplomat_out;
  }

  reorder_line(range_start, range_end) {
    const diplomat_out = diplomatRuntime.withWriteable(wasm, (writeable) => {
      return (() => {
        const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
        const result_tag = {};
        diplomat_alloc_destroy_registry.register(result_tag, {
          ptr: diplomat_receive_buffer,
          size: 5,
          align: 4,
        });
        wasm.ICU4XBidiParagraph_reorder_line(diplomat_receive_buffer, this.underlying, range_start, range_end, writeable);
        const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
        if (is_ok) {
          const ok_value = {};
          return ok_value;
        } else {
          const throw_value = ICU4XError_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
          throw new diplomatRuntime.FFIError(throw_value);
        }
      })();
    });
    return diplomat_out;
  }

  level_at(pos) {
    const diplomat_out = wasm.ICU4XBidiParagraph_level_at(this.underlying, pos);
    return diplomat_out;
  }
}

const ICU4XCanonicalizationResult_js_to_rust = {
  "Modified": 0,
  "Unmodified": 1,
};
const ICU4XCanonicalizationResult_rust_to_js = {
  0: "Modified",
  1: "Unmodified",
};

const ICU4XCodePointMapData16_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XCodePointMapData16_destroy(underlying);
});

export class ICU4XCodePointMapData16 {
  constructor(underlying) {
    this.underlying = underlying;
  }

  static try_get_script(provider) {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      const result_tag = {};
      diplomat_alloc_destroy_registry.register(result_tag, {
        ptr: diplomat_receive_buffer,
        size: 5,
        align: 4,
      });
      wasm.ICU4XCodePointMapData16_try_get_script(diplomat_receive_buffer, provider.underlying);
      const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
      if (is_ok) {
        const ok_value = (() => {
          const out = new ICU4XCodePointMapData16((new Uint32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]);
          out.owner = result_tag;
          return out;
        })();
        return ok_value;
      } else {
        const throw_value = ICU4XError_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    return diplomat_out;
  }

  get(cp) {
    const diplomat_out = wasm.ICU4XCodePointMapData16_get(this.underlying, diplomatRuntime.extractCodePoint(cp, 'cp'));
    return diplomat_out;
  }
}

const ICU4XCodePointMapData16Response_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XCodePointMapData16Response_destroy(underlying);
});

export class ICU4XCodePointMapData16Response {
  constructor(underlying) {
    this.underlying = underlying;
  }

  get data() {
    return (() => {
      const out = new ICU4XCodePointMapData16((new Uint32Array(wasm.memory.buffer, this.underlying + 0, 1))[0]);
      out.owner = null;
      return out;
    })();
  }

  get success() {
    return (new Uint8Array(wasm.memory.buffer, this.underlying + 4, 1))[0] == 1;
  }
}

const ICU4XCodePointSetData_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XCodePointSetData_destroy(underlying);
});

export class ICU4XCodePointSetData {
  constructor(underlying) {
    this.underlying = underlying;
  }

  static try_get_ascii_hex_digit(provider) {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      const result_tag = {};
      diplomat_alloc_destroy_registry.register(result_tag, {
        ptr: diplomat_receive_buffer,
        size: 5,
        align: 4,
      });
      wasm.ICU4XCodePointSetData_try_get_ascii_hex_digit(diplomat_receive_buffer, provider.underlying);
      const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
      if (is_ok) {
        const ok_value = (() => {
          const out = new ICU4XCodePointSetData((new Uint32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]);
          out.owner = result_tag;
          return out;
        })();
        return ok_value;
      } else {
        const throw_value = ICU4XError_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    return diplomat_out;
  }

  contains(cp) {
    const diplomat_out = wasm.ICU4XCodePointSetData_contains(this.underlying, diplomatRuntime.extractCodePoint(cp, 'cp'));
    return diplomat_out;
  }
}

const ICU4XCreateDataProviderResult_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XCreateDataProviderResult_destroy(underlying);
});

export class ICU4XCreateDataProviderResult {
  constructor(underlying) {
    this.underlying = underlying;
  }

  get provider() {
    return (() => {
      const out = new ICU4XDataProvider((new Uint32Array(wasm.memory.buffer, this.underlying + 0, 1))[0]);
      out.owner = null;
      return out;
    })();
  }

  get success() {
    return (new Uint8Array(wasm.memory.buffer, this.underlying + 4, 1))[0] == 1;
  }
}

const ICU4XDataProvider_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XDataProvider_destroy(underlying);
});

export class ICU4XDataProvider {
  constructor(underlying) {
    this.underlying = underlying;
  }

  static create_fs(path) {
    let path_diplomat_bytes = (new TextEncoder()).encode(path);
    let path_diplomat_ptr = wasm.diplomat_alloc(path_diplomat_bytes.length, 1);
    let path_diplomat_buf = new Uint8Array(wasm.memory.buffer, path_diplomat_ptr, path_diplomat_bytes.length);
    path_diplomat_buf.set(path_diplomat_bytes, 0);
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      const result_tag = {};
      diplomat_alloc_destroy_registry.register(result_tag, {
        ptr: diplomat_receive_buffer,
        size: 5,
        align: 4,
      });
      wasm.ICU4XDataProvider_create_fs(diplomat_receive_buffer, path_diplomat_ptr, path_diplomat_bytes.length);
      const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
      if (is_ok) {
        const ok_value = (() => {
          const out = new ICU4XDataProvider((new Uint32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]);
          out.owner = result_tag;
          return out;
        })();
        return ok_value;
      } else {
        const throw_value = ICU4XError_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    wasm.diplomat_free(path_diplomat_ptr, path_diplomat_bytes.length, 1);
    return diplomat_out;
  }

  static create_test() {
    const diplomat_out = (() => {
      const out = (() => {
        const out = new ICU4XDataProvider(wasm.ICU4XDataProvider_create_test());
        out.owner = null;
        return out;
      })();
      ICU4XDataProvider_box_destroy_registry.register(out, out.underlying)
      return out;
    })();
    return diplomat_out;
  }

  static create_from_byte_slice(blob) {
    let blob_diplomat_bytes = new Uint8Array(blob);
    let blob_diplomat_ptr = wasm.diplomat_alloc(blob_diplomat_bytes.length, 1);
    let blob_diplomat_buf = new Uint8Array(wasm.memory.buffer, blob_diplomat_ptr, blob_diplomat_bytes.length);
    blob_diplomat_buf.set(blob_diplomat_bytes, 0);
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      const result_tag = {};
      diplomat_alloc_destroy_registry.register(result_tag, {
        ptr: diplomat_receive_buffer,
        size: 5,
        align: 4,
      });
      wasm.ICU4XDataProvider_create_from_byte_slice(diplomat_receive_buffer, blob_diplomat_ptr, blob_diplomat_bytes.length);
      const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
      if (is_ok) {
        const ok_value = (() => {
          const out = new ICU4XDataProvider((new Uint32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]);
          out.owner = result_tag;
          return out;
        })();
        return ok_value;
      } else {
        const throw_value = ICU4XError_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    wasm.diplomat_free(blob_diplomat_ptr, blob_diplomat_bytes.length, 1);
    return diplomat_out;
  }

  static create_empty() {
    const diplomat_out = (() => {
      const out = (() => {
        const out = new ICU4XDataProvider(wasm.ICU4XDataProvider_create_empty());
        out.owner = null;
        return out;
      })();
      ICU4XDataProvider_box_destroy_registry.register(out, out.underlying)
      return out;
    })();
    return diplomat_out;
  }
}

const ICU4XDataStruct_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XDataStruct_destroy(underlying);
});

export class ICU4XDataStruct {
  constructor(underlying) {
    this.underlying = underlying;
  }

  static create_decimal_symbols_v1(plus_sign_prefix, plus_sign_suffix, minus_sign_prefix, minus_sign_suffix, decimal_separator, grouping_separator, primary_group_size, secondary_group_size, min_group_size, digits) {
    let plus_sign_prefix_diplomat_bytes = (new TextEncoder()).encode(plus_sign_prefix);
    let plus_sign_prefix_diplomat_ptr = wasm.diplomat_alloc(plus_sign_prefix_diplomat_bytes.length, 1);
    let plus_sign_prefix_diplomat_buf = new Uint8Array(wasm.memory.buffer, plus_sign_prefix_diplomat_ptr, plus_sign_prefix_diplomat_bytes.length);
    plus_sign_prefix_diplomat_buf.set(plus_sign_prefix_diplomat_bytes, 0);
    let plus_sign_suffix_diplomat_bytes = (new TextEncoder()).encode(plus_sign_suffix);
    let plus_sign_suffix_diplomat_ptr = wasm.diplomat_alloc(plus_sign_suffix_diplomat_bytes.length, 1);
    let plus_sign_suffix_diplomat_buf = new Uint8Array(wasm.memory.buffer, plus_sign_suffix_diplomat_ptr, plus_sign_suffix_diplomat_bytes.length);
    plus_sign_suffix_diplomat_buf.set(plus_sign_suffix_diplomat_bytes, 0);
    let minus_sign_prefix_diplomat_bytes = (new TextEncoder()).encode(minus_sign_prefix);
    let minus_sign_prefix_diplomat_ptr = wasm.diplomat_alloc(minus_sign_prefix_diplomat_bytes.length, 1);
    let minus_sign_prefix_diplomat_buf = new Uint8Array(wasm.memory.buffer, minus_sign_prefix_diplomat_ptr, minus_sign_prefix_diplomat_bytes.length);
    minus_sign_prefix_diplomat_buf.set(minus_sign_prefix_diplomat_bytes, 0);
    let minus_sign_suffix_diplomat_bytes = (new TextEncoder()).encode(minus_sign_suffix);
    let minus_sign_suffix_diplomat_ptr = wasm.diplomat_alloc(minus_sign_suffix_diplomat_bytes.length, 1);
    let minus_sign_suffix_diplomat_buf = new Uint8Array(wasm.memory.buffer, minus_sign_suffix_diplomat_ptr, minus_sign_suffix_diplomat_bytes.length);
    minus_sign_suffix_diplomat_buf.set(minus_sign_suffix_diplomat_bytes, 0);
    let decimal_separator_diplomat_bytes = (new TextEncoder()).encode(decimal_separator);
    let decimal_separator_diplomat_ptr = wasm.diplomat_alloc(decimal_separator_diplomat_bytes.length, 1);
    let decimal_separator_diplomat_buf = new Uint8Array(wasm.memory.buffer, decimal_separator_diplomat_ptr, decimal_separator_diplomat_bytes.length);
    decimal_separator_diplomat_buf.set(decimal_separator_diplomat_bytes, 0);
    let grouping_separator_diplomat_bytes = (new TextEncoder()).encode(grouping_separator);
    let grouping_separator_diplomat_ptr = wasm.diplomat_alloc(grouping_separator_diplomat_bytes.length, 1);
    let grouping_separator_diplomat_buf = new Uint8Array(wasm.memory.buffer, grouping_separator_diplomat_ptr, grouping_separator_diplomat_bytes.length);
    grouping_separator_diplomat_buf.set(grouping_separator_diplomat_bytes, 0);
    let digits_diplomat_bytes = new Uint8Array(digits);
    let digits_diplomat_ptr = wasm.diplomat_alloc(digits_diplomat_bytes.length, 4);
    let digits_diplomat_buf = new Uint8Array(wasm.memory.buffer, digits_diplomat_ptr, digits_diplomat_bytes.length);
    digits_diplomat_buf.set(digits_diplomat_bytes, 0);
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      const result_tag = {};
      diplomat_alloc_destroy_registry.register(result_tag, {
        ptr: diplomat_receive_buffer,
        size: 5,
        align: 4,
      });
      wasm.ICU4XDataStruct_create_decimal_symbols_v1(diplomat_receive_buffer, plus_sign_prefix_diplomat_ptr, plus_sign_prefix_diplomat_bytes.length, plus_sign_suffix_diplomat_ptr, plus_sign_suffix_diplomat_bytes.length, minus_sign_prefix_diplomat_ptr, minus_sign_prefix_diplomat_bytes.length, minus_sign_suffix_diplomat_ptr, minus_sign_suffix_diplomat_bytes.length, decimal_separator_diplomat_ptr, decimal_separator_diplomat_bytes.length, grouping_separator_diplomat_ptr, grouping_separator_diplomat_bytes.length, primary_group_size, secondary_group_size, min_group_size, digits_diplomat_ptr, digits_diplomat_bytes.length);
      const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
      if (is_ok) {
        const ok_value = (() => {
          const out = new ICU4XDataStruct((new Uint32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]);
          out.owner = result_tag;
          return out;
        })();
        return ok_value;
      } else {
        const throw_value = ICU4XError_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    wasm.diplomat_free(plus_sign_prefix_diplomat_ptr, plus_sign_prefix_diplomat_bytes.length, 1);
    wasm.diplomat_free(plus_sign_suffix_diplomat_ptr, plus_sign_suffix_diplomat_bytes.length, 1);
    wasm.diplomat_free(minus_sign_prefix_diplomat_ptr, minus_sign_prefix_diplomat_bytes.length, 1);
    wasm.diplomat_free(minus_sign_suffix_diplomat_ptr, minus_sign_suffix_diplomat_bytes.length, 1);
    wasm.diplomat_free(decimal_separator_diplomat_ptr, decimal_separator_diplomat_bytes.length, 1);
    wasm.diplomat_free(grouping_separator_diplomat_ptr, grouping_separator_diplomat_bytes.length, 1);
    wasm.diplomat_free(digits_diplomat_ptr, digits_diplomat_bytes.length, 4);
    return diplomat_out;
  }
}

const ICU4XError_js_to_rust = {
  "UnknownError": 0,
  "WriteableError": 1,
  "OutOfBoundsError": 2,
  "DataMissingResourceKeyError": 256,
  "DataMissingVariantError": 257,
  "DataMissingLocaleError": 258,
  "DataMissingResourceOptionsError": 259,
  "DataNeedsVariantError": 260,
  "DataNeedsLocaleError": 261,
  "DataExtraneousResourceOptionsError": 262,
  "DataFilteredResourceError": 263,
  "DataMismatchedTypeError": 264,
  "DataMissingPayloadError": 265,
  "DataInvalidStateError": 266,
  "DataCustomError": 267,
  "DataIoError": 268,
  "DataUnavailableBufferFormatError": 269,
  "LocaleUndefinedSubtagError": 512,
  "LocaleParserError": 513,
  "DataStructValidityError": 768,
  "PropertyUnknownScriptIdError": 1024,
  "PropertyUnknownGeneralCategoryGroupError": 1025,
  "DecimalLimitError": 1280,
  "DecimalSyntaxError": 1281,
  "PluralParserError": 1536,
};
const ICU4XError_rust_to_js = {
  0: "UnknownError",
  1: "WriteableError",
  2: "OutOfBoundsError",
  256: "DataMissingResourceKeyError",
  257: "DataMissingVariantError",
  258: "DataMissingLocaleError",
  259: "DataMissingResourceOptionsError",
  260: "DataNeedsVariantError",
  261: "DataNeedsLocaleError",
  262: "DataExtraneousResourceOptionsError",
  263: "DataFilteredResourceError",
  264: "DataMismatchedTypeError",
  265: "DataMissingPayloadError",
  266: "DataInvalidStateError",
  267: "DataCustomError",
  268: "DataIoError",
  269: "DataUnavailableBufferFormatError",
  512: "LocaleUndefinedSubtagError",
  513: "LocaleParserError",
  768: "DataStructValidityError",
  1024: "PropertyUnknownScriptIdError",
  1025: "PropertyUnknownGeneralCategoryGroupError",
  1280: "DecimalLimitError",
  1281: "DecimalSyntaxError",
  1536: "PluralParserError",
};

const ICU4XFixedDecimal_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XFixedDecimal_destroy(underlying);
});

export class ICU4XFixedDecimal {
  constructor(underlying) {
    this.underlying = underlying;
  }

  static create(v) {
    const diplomat_out = (() => {
      const out = (() => {
        const out = new ICU4XFixedDecimal(wasm.ICU4XFixedDecimal_create(v));
        out.owner = null;
        return out;
      })();
      ICU4XFixedDecimal_box_destroy_registry.register(out, out.underlying)
      return out;
    })();
    return diplomat_out;
  }

  static create_from_f64_with_max_precision(f) {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      const result_tag = {};
      diplomat_alloc_destroy_registry.register(result_tag, {
        ptr: diplomat_receive_buffer,
        size: 5,
        align: 4,
      });
      wasm.ICU4XFixedDecimal_create_from_f64_with_max_precision(diplomat_receive_buffer, f);
      const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
      if (is_ok) {
        const ok_value = (() => {
          const out = new ICU4XFixedDecimal((new Uint32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]);
          out.owner = result_tag;
          return out;
        })();
        return ok_value;
      } else {
        const throw_value = ICU4XError_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    return diplomat_out;
  }

  static create_from_f64_with_lower_magnitude(f, precision) {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      const result_tag = {};
      diplomat_alloc_destroy_registry.register(result_tag, {
        ptr: diplomat_receive_buffer,
        size: 5,
        align: 4,
      });
      wasm.ICU4XFixedDecimal_create_from_f64_with_lower_magnitude(diplomat_receive_buffer, f, precision);
      const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
      if (is_ok) {
        const ok_value = (() => {
          const out = new ICU4XFixedDecimal((new Uint32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]);
          out.owner = result_tag;
          return out;
        })();
        return ok_value;
      } else {
        const throw_value = ICU4XError_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    return diplomat_out;
  }

  static create_from_f64_with_significant_digits(f, digits) {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      const result_tag = {};
      diplomat_alloc_destroy_registry.register(result_tag, {
        ptr: diplomat_receive_buffer,
        size: 5,
        align: 4,
      });
      wasm.ICU4XFixedDecimal_create_from_f64_with_significant_digits(diplomat_receive_buffer, f, digits);
      const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
      if (is_ok) {
        const ok_value = (() => {
          const out = new ICU4XFixedDecimal((new Uint32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]);
          out.owner = result_tag;
          return out;
        })();
        return ok_value;
      } else {
        const throw_value = ICU4XError_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    return diplomat_out;
  }

  static create_fromstr(v) {
    let v_diplomat_bytes = (new TextEncoder()).encode(v);
    let v_diplomat_ptr = wasm.diplomat_alloc(v_diplomat_bytes.length, 1);
    let v_diplomat_buf = new Uint8Array(wasm.memory.buffer, v_diplomat_ptr, v_diplomat_bytes.length);
    v_diplomat_buf.set(v_diplomat_bytes, 0);
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      const result_tag = {};
      diplomat_alloc_destroy_registry.register(result_tag, {
        ptr: diplomat_receive_buffer,
        size: 5,
        align: 4,
      });
      wasm.ICU4XFixedDecimal_create_fromstr(diplomat_receive_buffer, v_diplomat_ptr, v_diplomat_bytes.length);
      const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
      if (is_ok) {
        const ok_value = (() => {
          const out = new ICU4XFixedDecimal((new Uint32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]);
          out.owner = result_tag;
          return out;
        })();
        return ok_value;
      } else {
        const throw_value = ICU4XError_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    wasm.diplomat_free(v_diplomat_ptr, v_diplomat_bytes.length, 1);
    return diplomat_out;
  }

  multiply_pow10(power) {
    const diplomat_out = wasm.ICU4XFixedDecimal_multiply_pow10(this.underlying, power);
    return diplomat_out;
  }

  set_sign(sign) {
    const diplomat_out = wasm.ICU4XFixedDecimal_set_sign(this.underlying, ICU4XFixedDecimalSign_js_to_rust[sign]);
  }

  pad_left(position) {
    const diplomat_out = wasm.ICU4XFixedDecimal_pad_left(this.underlying, position);
  }

  truncate_left(position) {
    const diplomat_out = wasm.ICU4XFixedDecimal_truncate_left(this.underlying, position);
  }

  pad_right(position) {
    const diplomat_out = wasm.ICU4XFixedDecimal_pad_right(this.underlying, position);
  }

  to_string() {
    const diplomat_out = diplomatRuntime.withWriteable(wasm, (writeable) => {
      return wasm.ICU4XFixedDecimal_to_string(this.underlying, writeable);
    });
    return diplomat_out;
  }
}

const ICU4XFixedDecimalFormat_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XFixedDecimalFormat_destroy(underlying);
});

export class ICU4XFixedDecimalFormat {
  constructor(underlying) {
    this.underlying = underlying;
  }

  static try_new(locale, provider, grouping_strategy) {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      const result_tag = {};
      diplomat_alloc_destroy_registry.register(result_tag, {
        ptr: diplomat_receive_buffer,
        size: 5,
        align: 4,
      });
      wasm.ICU4XFixedDecimalFormat_try_new(diplomat_receive_buffer, locale.underlying, provider.underlying, ICU4XFixedDecimalGroupingStrategy_js_to_rust[grouping_strategy]);
      const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
      if (is_ok) {
        const ok_value = (() => {
          const out = new ICU4XFixedDecimalFormat((new Uint32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]);
          out.owner = result_tag;
          return out;
        })();
        return ok_value;
      } else {
        const throw_value = ICU4XError_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    return diplomat_out;
  }

  static try_new_from_decimal_symbols_v1(data_struct, grouping_strategy) {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      const result_tag = {};
      diplomat_alloc_destroy_registry.register(result_tag, {
        ptr: diplomat_receive_buffer,
        size: 5,
        align: 4,
      });
      wasm.ICU4XFixedDecimalFormat_try_new_from_decimal_symbols_v1(diplomat_receive_buffer, data_struct.underlying, ICU4XFixedDecimalGroupingStrategy_js_to_rust[grouping_strategy]);
      const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
      if (is_ok) {
        const ok_value = (() => {
          const out = new ICU4XFixedDecimalFormat((new Uint32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]);
          out.owner = result_tag;
          return out;
        })();
        return ok_value;
      } else {
        const throw_value = ICU4XError_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    return diplomat_out;
  }

  format(value) {
    const diplomat_out = diplomatRuntime.withWriteable(wasm, (writeable) => {
      return (() => {
        const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
        const result_tag = {};
        diplomat_alloc_destroy_registry.register(result_tag, {
          ptr: diplomat_receive_buffer,
          size: 5,
          align: 4,
        });
        wasm.ICU4XFixedDecimalFormat_format(diplomat_receive_buffer, this.underlying, value.underlying, writeable);
        const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
        if (is_ok) {
          const ok_value = {};
          return ok_value;
        } else {
          const throw_value = ICU4XError_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
          throw new diplomatRuntime.FFIError(throw_value);
        }
      })();
    });
    return diplomat_out;
  }
}

const ICU4XFixedDecimalGroupingStrategy_js_to_rust = {
  "Auto": 0,
  "Never": 1,
  "Always": 2,
  "Min2": 3,
};
const ICU4XFixedDecimalGroupingStrategy_rust_to_js = {
  0: "Auto",
  1: "Never",
  2: "Always",
  3: "Min2",
};

const ICU4XFixedDecimalSign_js_to_rust = {
  "None": 0,
  "Negative": 1,
  "Positive": 2,
};
const ICU4XFixedDecimalSign_rust_to_js = {
  0: "None",
  1: "Negative",
  2: "Positive",
};

const ICU4XGraphemeClusterBreakIteratorLatin1_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XGraphemeClusterBreakIteratorLatin1_destroy(underlying);
});

export class ICU4XGraphemeClusterBreakIteratorLatin1 {
  constructor(underlying) {
    this.underlying = underlying;
  }

  next() {
    const diplomat_out = wasm.ICU4XGraphemeClusterBreakIteratorLatin1_next(this.underlying);
    return diplomat_out;
  }
}

const ICU4XGraphemeClusterBreakIteratorUtf16_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XGraphemeClusterBreakIteratorUtf16_destroy(underlying);
});

export class ICU4XGraphemeClusterBreakIteratorUtf16 {
  constructor(underlying) {
    this.underlying = underlying;
  }

  next() {
    const diplomat_out = wasm.ICU4XGraphemeClusterBreakIteratorUtf16_next(this.underlying);
    return diplomat_out;
  }
}

const ICU4XGraphemeClusterBreakIteratorUtf8_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XGraphemeClusterBreakIteratorUtf8_destroy(underlying);
});

export class ICU4XGraphemeClusterBreakIteratorUtf8 {
  constructor(underlying) {
    this.underlying = underlying;
  }

  next() {
    const diplomat_out = wasm.ICU4XGraphemeClusterBreakIteratorUtf8_next(this.underlying);
    return diplomat_out;
  }
}

const ICU4XGraphemeClusterBreakSegmenter_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XGraphemeClusterBreakSegmenter_destroy(underlying);
});

export class ICU4XGraphemeClusterBreakSegmenter {
  constructor(underlying) {
    this.underlying = underlying;
  }

  static try_new(provider) {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      const result_tag = {};
      diplomat_alloc_destroy_registry.register(result_tag, {
        ptr: diplomat_receive_buffer,
        size: 5,
        align: 4,
      });
      wasm.ICU4XGraphemeClusterBreakSegmenter_try_new(diplomat_receive_buffer, provider.underlying);
      const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
      if (is_ok) {
        const ok_value = (() => {
          const out = new ICU4XGraphemeClusterBreakSegmenter((new Uint32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]);
          out.owner = result_tag;
          return out;
        })();
        return ok_value;
      } else {
        const throw_value = ICU4XError_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    return diplomat_out;
  }

  segment_utf8(input) {
    let input_diplomat_bytes = (new TextEncoder()).encode(input);
    let input_diplomat_ptr = wasm.diplomat_alloc(input_diplomat_bytes.length, 1);
    let input_diplomat_buf = new Uint8Array(wasm.memory.buffer, input_diplomat_ptr, input_diplomat_bytes.length);
    input_diplomat_buf.set(input_diplomat_bytes, 0);
    const diplomat_out = (() => {
      const out = (() => {
        const out = new ICU4XGraphemeClusterBreakIteratorUtf8(wasm.ICU4XGraphemeClusterBreakSegmenter_segment_utf8(this.underlying, input_diplomat_ptr, input_diplomat_bytes.length));
        out.owner = null;
        return out;
      })();
      ICU4XGraphemeClusterBreakIteratorUtf8_box_destroy_registry.register(out, out.underlying)
      return out;
    })();
    wasm.diplomat_free(input_diplomat_ptr, input_diplomat_bytes.length, 1);
    return diplomat_out;
  }

  segment_utf16(input) {
    let input_diplomat_bytes = new Uint8Array(input);
    let input_diplomat_ptr = wasm.diplomat_alloc(input_diplomat_bytes.length, 2);
    let input_diplomat_buf = new Uint8Array(wasm.memory.buffer, input_diplomat_ptr, input_diplomat_bytes.length);
    input_diplomat_buf.set(input_diplomat_bytes, 0);
    const diplomat_out = (() => {
      const out = (() => {
        const out = new ICU4XGraphemeClusterBreakIteratorUtf16(wasm.ICU4XGraphemeClusterBreakSegmenter_segment_utf16(this.underlying, input_diplomat_ptr, input_diplomat_bytes.length));
        out.owner = null;
        return out;
      })();
      ICU4XGraphemeClusterBreakIteratorUtf16_box_destroy_registry.register(out, out.underlying)
      return out;
    })();
    wasm.diplomat_free(input_diplomat_ptr, input_diplomat_bytes.length, 2);
    return diplomat_out;
  }

  segment_latin1(input) {
    let input_diplomat_bytes = new Uint8Array(input);
    let input_diplomat_ptr = wasm.diplomat_alloc(input_diplomat_bytes.length, 1);
    let input_diplomat_buf = new Uint8Array(wasm.memory.buffer, input_diplomat_ptr, input_diplomat_bytes.length);
    input_diplomat_buf.set(input_diplomat_bytes, 0);
    const diplomat_out = (() => {
      const out = (() => {
        const out = new ICU4XGraphemeClusterBreakIteratorLatin1(wasm.ICU4XGraphemeClusterBreakSegmenter_segment_latin1(this.underlying, input_diplomat_ptr, input_diplomat_bytes.length));
        out.owner = null;
        return out;
      })();
      ICU4XGraphemeClusterBreakIteratorLatin1_box_destroy_registry.register(out, out.underlying)
      return out;
    })();
    wasm.diplomat_free(input_diplomat_ptr, input_diplomat_bytes.length, 1);
    return diplomat_out;
  }
}

const ICU4XLineBreakIteratorLatin1_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XLineBreakIteratorLatin1_destroy(underlying);
});

export class ICU4XLineBreakIteratorLatin1 {
  constructor(underlying) {
    this.underlying = underlying;
  }

  next() {
    const diplomat_out = wasm.ICU4XLineBreakIteratorLatin1_next(this.underlying);
    return diplomat_out;
  }
}

const ICU4XLineBreakIteratorUtf16_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XLineBreakIteratorUtf16_destroy(underlying);
});

export class ICU4XLineBreakIteratorUtf16 {
  constructor(underlying) {
    this.underlying = underlying;
  }

  next() {
    const diplomat_out = wasm.ICU4XLineBreakIteratorUtf16_next(this.underlying);
    return diplomat_out;
  }
}

const ICU4XLineBreakIteratorUtf8_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XLineBreakIteratorUtf8_destroy(underlying);
});

export class ICU4XLineBreakIteratorUtf8 {
  constructor(underlying) {
    this.underlying = underlying;
  }

  next() {
    const diplomat_out = wasm.ICU4XLineBreakIteratorUtf8_next(this.underlying);
    return diplomat_out;
  }
}

const ICU4XLineBreakOptions_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XLineBreakOptions_destroy(underlying);
});

export class ICU4XLineBreakOptions {
  constructor(underlying) {
    this.underlying = underlying;
  }

  get line_break_rule() {
    return ICU4XLineBreakRule_rust_to_js[(new Int32Array(wasm.memory.buffer, this.underlying + 0, 1))[0]];
  }

  get word_break_rule() {
    return ICU4XWordBreakRule_rust_to_js[(new Int32Array(wasm.memory.buffer, this.underlying + 4, 1))[0]];
  }

  get ja_zh() {
    return (new Uint8Array(wasm.memory.buffer, this.underlying + 8, 1))[0] == 1;
  }
}

const ICU4XLineBreakRule_js_to_rust = {
  "Loose": 0,
  "Normal": 1,
  "Strict": 2,
  "Anywhere": 3,
};
const ICU4XLineBreakRule_rust_to_js = {
  0: "Loose",
  1: "Normal",
  2: "Strict",
  3: "Anywhere",
};

const ICU4XLineBreakSegmenter_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XLineBreakSegmenter_destroy(underlying);
});

export class ICU4XLineBreakSegmenter {
  constructor(underlying) {
    this.underlying = underlying;
  }

  static try_new(provider) {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      const result_tag = {};
      diplomat_alloc_destroy_registry.register(result_tag, {
        ptr: diplomat_receive_buffer,
        size: 5,
        align: 4,
      });
      wasm.ICU4XLineBreakSegmenter_try_new(diplomat_receive_buffer, provider.underlying);
      const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
      if (is_ok) {
        const ok_value = (() => {
          const out = new ICU4XLineBreakSegmenter((new Uint32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]);
          out.owner = result_tag;
          return out;
        })();
        return ok_value;
      } else {
        const throw_value = ICU4XError_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    return diplomat_out;
  }

  static try_new_with_options(provider, options) {
    const diplomat_ICU4XLineBreakOptions_extracted_line_break_rule = options["line_break_rule"];
    const diplomat_ICU4XLineBreakOptions_extracted_word_break_rule = options["word_break_rule"];
    const diplomat_ICU4XLineBreakOptions_extracted_ja_zh = options["ja_zh"];
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      const result_tag = {};
      diplomat_alloc_destroy_registry.register(result_tag, {
        ptr: diplomat_receive_buffer,
        size: 5,
        align: 4,
      });
      wasm.ICU4XLineBreakSegmenter_try_new_with_options(diplomat_receive_buffer, provider.underlying, ICU4XLineBreakRule_js_to_rust[diplomat_ICU4XLineBreakOptions_extracted_line_break_rule], ICU4XWordBreakRule_js_to_rust[diplomat_ICU4XLineBreakOptions_extracted_word_break_rule], diplomat_ICU4XLineBreakOptions_extracted_ja_zh);
      const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
      if (is_ok) {
        const ok_value = (() => {
          const out = new ICU4XLineBreakSegmenter((new Uint32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]);
          out.owner = result_tag;
          return out;
        })();
        return ok_value;
      } else {
        const throw_value = ICU4XError_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    return diplomat_out;
  }

  segment_utf8(input) {
    let input_diplomat_bytes = (new TextEncoder()).encode(input);
    let input_diplomat_ptr = wasm.diplomat_alloc(input_diplomat_bytes.length, 1);
    let input_diplomat_buf = new Uint8Array(wasm.memory.buffer, input_diplomat_ptr, input_diplomat_bytes.length);
    input_diplomat_buf.set(input_diplomat_bytes, 0);
    const diplomat_out = (() => {
      const out = (() => {
        const out = new ICU4XLineBreakIteratorUtf8(wasm.ICU4XLineBreakSegmenter_segment_utf8(this.underlying, input_diplomat_ptr, input_diplomat_bytes.length));
        out.owner = null;
        return out;
      })();
      ICU4XLineBreakIteratorUtf8_box_destroy_registry.register(out, out.underlying)
      return out;
    })();
    wasm.diplomat_free(input_diplomat_ptr, input_diplomat_bytes.length, 1);
    return diplomat_out;
  }

  segment_utf16(input) {
    let input_diplomat_bytes = new Uint8Array(input);
    let input_diplomat_ptr = wasm.diplomat_alloc(input_diplomat_bytes.length, 2);
    let input_diplomat_buf = new Uint8Array(wasm.memory.buffer, input_diplomat_ptr, input_diplomat_bytes.length);
    input_diplomat_buf.set(input_diplomat_bytes, 0);
    const diplomat_out = (() => {
      const out = (() => {
        const out = new ICU4XLineBreakIteratorUtf16(wasm.ICU4XLineBreakSegmenter_segment_utf16(this.underlying, input_diplomat_ptr, input_diplomat_bytes.length));
        out.owner = null;
        return out;
      })();
      ICU4XLineBreakIteratorUtf16_box_destroy_registry.register(out, out.underlying)
      return out;
    })();
    wasm.diplomat_free(input_diplomat_ptr, input_diplomat_bytes.length, 2);
    return diplomat_out;
  }

  segment_latin1(input) {
    let input_diplomat_bytes = new Uint8Array(input);
    let input_diplomat_ptr = wasm.diplomat_alloc(input_diplomat_bytes.length, 1);
    let input_diplomat_buf = new Uint8Array(wasm.memory.buffer, input_diplomat_ptr, input_diplomat_bytes.length);
    input_diplomat_buf.set(input_diplomat_bytes, 0);
    const diplomat_out = (() => {
      const out = (() => {
        const out = new ICU4XLineBreakIteratorLatin1(wasm.ICU4XLineBreakSegmenter_segment_latin1(this.underlying, input_diplomat_ptr, input_diplomat_bytes.length));
        out.owner = null;
        return out;
      })();
      ICU4XLineBreakIteratorLatin1_box_destroy_registry.register(out, out.underlying)
      return out;
    })();
    wasm.diplomat_free(input_diplomat_ptr, input_diplomat_bytes.length, 1);
    return diplomat_out;
  }
}

const ICU4XLocale_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XLocale_destroy(underlying);
});

export class ICU4XLocale {
  constructor(underlying) {
    this.underlying = underlying;
  }

  static create(name) {
    let name_diplomat_bytes = (new TextEncoder()).encode(name);
    let name_diplomat_ptr = wasm.diplomat_alloc(name_diplomat_bytes.length, 1);
    let name_diplomat_buf = new Uint8Array(wasm.memory.buffer, name_diplomat_ptr, name_diplomat_bytes.length);
    name_diplomat_buf.set(name_diplomat_bytes, 0);
    const diplomat_out = (() => {
      const option_value = wasm.ICU4XLocale_create(name_diplomat_ptr, name_diplomat_bytes.length)
      if (option_value !== 0) {
        const inhabited_value = (() => {
          const out = (() => {
            const out = new ICU4XLocale(option_value);
            out.owner = null;
            return out;
          })();
          ICU4XLocale_box_destroy_registry.register(out, out.underlying)
          return out;
        })();
        return inhabited_value;
      } else {
        return null;
      }
    })();
    wasm.diplomat_free(name_diplomat_ptr, name_diplomat_bytes.length, 1);
    return diplomat_out;
  }

  static create_en() {
    const diplomat_out = (() => {
      const out = (() => {
        const out = new ICU4XLocale(wasm.ICU4XLocale_create_en());
        out.owner = null;
        return out;
      })();
      ICU4XLocale_box_destroy_registry.register(out, out.underlying)
      return out;
    })();
    return diplomat_out;
  }

  static create_bn() {
    const diplomat_out = (() => {
      const out = (() => {
        const out = new ICU4XLocale(wasm.ICU4XLocale_create_bn());
        out.owner = null;
        return out;
      })();
      ICU4XLocale_box_destroy_registry.register(out, out.underlying)
      return out;
    })();
    return diplomat_out;
  }

  static und() {
    const diplomat_out = (() => {
      const out = (() => {
        const out = new ICU4XLocale(wasm.ICU4XLocale_und());
        out.owner = null;
        return out;
      })();
      ICU4XLocale_box_destroy_registry.register(out, out.underlying)
      return out;
    })();
    return diplomat_out;
  }

  clone() {
    const diplomat_out = (() => {
      const out = (() => {
        const out = new ICU4XLocale(wasm.ICU4XLocale_clone(this.underlying));
        out.owner = null;
        return out;
      })();
      ICU4XLocale_box_destroy_registry.register(out, out.underlying)
      return out;
    })();
    return diplomat_out;
  }

  basename() {
    const diplomat_out = diplomatRuntime.withWriteable(wasm, (writeable) => {
      return (() => {
        const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
        const result_tag = {};
        diplomat_alloc_destroy_registry.register(result_tag, {
          ptr: diplomat_receive_buffer,
          size: 5,
          align: 4,
        });
        wasm.ICU4XLocale_basename(diplomat_receive_buffer, this.underlying, writeable);
        const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
        if (is_ok) {
          const ok_value = {};
          return ok_value;
        } else {
          const throw_value = ICU4XError_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
          throw new diplomatRuntime.FFIError(throw_value);
        }
      })();
    });
    return diplomat_out;
  }

  get_unicode_extension(bytes) {
    let bytes_diplomat_bytes = (new TextEncoder()).encode(bytes);
    let bytes_diplomat_ptr = wasm.diplomat_alloc(bytes_diplomat_bytes.length, 1);
    let bytes_diplomat_buf = new Uint8Array(wasm.memory.buffer, bytes_diplomat_ptr, bytes_diplomat_bytes.length);
    bytes_diplomat_buf.set(bytes_diplomat_bytes, 0);
    const diplomat_out = diplomatRuntime.withWriteable(wasm, (writeable) => {
      return (() => {
        const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
        const result_tag = {};
        diplomat_alloc_destroy_registry.register(result_tag, {
          ptr: diplomat_receive_buffer,
          size: 5,
          align: 4,
        });
        wasm.ICU4XLocale_get_unicode_extension(diplomat_receive_buffer, this.underlying, bytes_diplomat_ptr, bytes_diplomat_bytes.length, writeable);
        const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
        if (is_ok) {
          const ok_value = {};
          return ok_value;
        } else {
          const throw_value = ICU4XError_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
          throw new diplomatRuntime.FFIError(throw_value);
        }
      })();
    });
    wasm.diplomat_free(bytes_diplomat_ptr, bytes_diplomat_bytes.length, 1);
    return diplomat_out;
  }

  language() {
    const diplomat_out = diplomatRuntime.withWriteable(wasm, (writeable) => {
      return (() => {
        const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
        const result_tag = {};
        diplomat_alloc_destroy_registry.register(result_tag, {
          ptr: diplomat_receive_buffer,
          size: 5,
          align: 4,
        });
        wasm.ICU4XLocale_language(diplomat_receive_buffer, this.underlying, writeable);
        const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
        if (is_ok) {
          const ok_value = {};
          return ok_value;
        } else {
          const throw_value = ICU4XError_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
          throw new diplomatRuntime.FFIError(throw_value);
        }
      })();
    });
    return diplomat_out;
  }

  set_language(bytes) {
    let bytes_diplomat_bytes = (new TextEncoder()).encode(bytes);
    let bytes_diplomat_ptr = wasm.diplomat_alloc(bytes_diplomat_bytes.length, 1);
    let bytes_diplomat_buf = new Uint8Array(wasm.memory.buffer, bytes_diplomat_ptr, bytes_diplomat_bytes.length);
    bytes_diplomat_buf.set(bytes_diplomat_bytes, 0);
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      const result_tag = {};
      diplomat_alloc_destroy_registry.register(result_tag, {
        ptr: diplomat_receive_buffer,
        size: 5,
        align: 4,
      });
      wasm.ICU4XLocale_set_language(diplomat_receive_buffer, this.underlying, bytes_diplomat_ptr, bytes_diplomat_bytes.length);
      const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
      if (is_ok) {
        const ok_value = {};
        return ok_value;
      } else {
        const throw_value = ICU4XError_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    wasm.diplomat_free(bytes_diplomat_ptr, bytes_diplomat_bytes.length, 1);
    return diplomat_out;
  }

  region() {
    const diplomat_out = diplomatRuntime.withWriteable(wasm, (writeable) => {
      return (() => {
        const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
        const result_tag = {};
        diplomat_alloc_destroy_registry.register(result_tag, {
          ptr: diplomat_receive_buffer,
          size: 5,
          align: 4,
        });
        wasm.ICU4XLocale_region(diplomat_receive_buffer, this.underlying, writeable);
        const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
        if (is_ok) {
          const ok_value = {};
          return ok_value;
        } else {
          const throw_value = ICU4XError_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
          throw new diplomatRuntime.FFIError(throw_value);
        }
      })();
    });
    return diplomat_out;
  }

  set_region(bytes) {
    let bytes_diplomat_bytes = (new TextEncoder()).encode(bytes);
    let bytes_diplomat_ptr = wasm.diplomat_alloc(bytes_diplomat_bytes.length, 1);
    let bytes_diplomat_buf = new Uint8Array(wasm.memory.buffer, bytes_diplomat_ptr, bytes_diplomat_bytes.length);
    bytes_diplomat_buf.set(bytes_diplomat_bytes, 0);
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      const result_tag = {};
      diplomat_alloc_destroy_registry.register(result_tag, {
        ptr: diplomat_receive_buffer,
        size: 5,
        align: 4,
      });
      wasm.ICU4XLocale_set_region(diplomat_receive_buffer, this.underlying, bytes_diplomat_ptr, bytes_diplomat_bytes.length);
      const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
      if (is_ok) {
        const ok_value = {};
        return ok_value;
      } else {
        const throw_value = ICU4XError_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    wasm.diplomat_free(bytes_diplomat_ptr, bytes_diplomat_bytes.length, 1);
    return diplomat_out;
  }

  script() {
    const diplomat_out = diplomatRuntime.withWriteable(wasm, (writeable) => {
      return (() => {
        const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
        const result_tag = {};
        diplomat_alloc_destroy_registry.register(result_tag, {
          ptr: diplomat_receive_buffer,
          size: 5,
          align: 4,
        });
        wasm.ICU4XLocale_script(diplomat_receive_buffer, this.underlying, writeable);
        const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
        if (is_ok) {
          const ok_value = {};
          return ok_value;
        } else {
          const throw_value = ICU4XError_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
          throw new diplomatRuntime.FFIError(throw_value);
        }
      })();
    });
    return diplomat_out;
  }

  set_script(bytes) {
    let bytes_diplomat_bytes = (new TextEncoder()).encode(bytes);
    let bytes_diplomat_ptr = wasm.diplomat_alloc(bytes_diplomat_bytes.length, 1);
    let bytes_diplomat_buf = new Uint8Array(wasm.memory.buffer, bytes_diplomat_ptr, bytes_diplomat_bytes.length);
    bytes_diplomat_buf.set(bytes_diplomat_bytes, 0);
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      const result_tag = {};
      diplomat_alloc_destroy_registry.register(result_tag, {
        ptr: diplomat_receive_buffer,
        size: 5,
        align: 4,
      });
      wasm.ICU4XLocale_set_script(diplomat_receive_buffer, this.underlying, bytes_diplomat_ptr, bytes_diplomat_bytes.length);
      const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
      if (is_ok) {
        const ok_value = {};
        return ok_value;
      } else {
        const throw_value = ICU4XError_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    wasm.diplomat_free(bytes_diplomat_ptr, bytes_diplomat_bytes.length, 1);
    return diplomat_out;
  }

  tostring() {
    const diplomat_out = diplomatRuntime.withWriteable(wasm, (writeable) => {
      return (() => {
        const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
        const result_tag = {};
        diplomat_alloc_destroy_registry.register(result_tag, {
          ptr: diplomat_receive_buffer,
          size: 5,
          align: 4,
        });
        wasm.ICU4XLocale_tostring(diplomat_receive_buffer, this.underlying, writeable);
        const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
        if (is_ok) {
          const ok_value = {};
          return ok_value;
        } else {
          const throw_value = ICU4XError_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
          throw new diplomatRuntime.FFIError(throw_value);
        }
      })();
    });
    return diplomat_out;
  }
}

const ICU4XLocaleCanonicalizer_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XLocaleCanonicalizer_destroy(underlying);
});

export class ICU4XLocaleCanonicalizer {
  constructor(underlying) {
    this.underlying = underlying;
  }

  static create(provider) {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      const result_tag = {};
      diplomat_alloc_destroy_registry.register(result_tag, {
        ptr: diplomat_receive_buffer,
        size: 5,
        align: 4,
      });
      wasm.ICU4XLocaleCanonicalizer_create(diplomat_receive_buffer, provider.underlying);
      const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
      if (is_ok) {
        const ok_value = (() => {
          const out = new ICU4XLocaleCanonicalizer((new Uint32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]);
          out.owner = result_tag;
          return out;
        })();
        return ok_value;
      } else {
        const throw_value = ICU4XError_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    return diplomat_out;
  }

  canonicalize(locale) {
    const diplomat_out = ICU4XCanonicalizationResult_rust_to_js[wasm.ICU4XLocaleCanonicalizer_canonicalize(this.underlying, locale.underlying)];
    return diplomat_out;
  }

  maximize(locale) {
    const diplomat_out = ICU4XCanonicalizationResult_rust_to_js[wasm.ICU4XLocaleCanonicalizer_maximize(this.underlying, locale.underlying)];
    return diplomat_out;
  }

  minimize(locale) {
    const diplomat_out = ICU4XCanonicalizationResult_rust_to_js[wasm.ICU4XLocaleCanonicalizer_minimize(this.underlying, locale.underlying)];
    return diplomat_out;
  }
}

const ICU4XPluralCategories_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XPluralCategories_destroy(underlying);
});

export class ICU4XPluralCategories {
  constructor(underlying) {
    this.underlying = underlying;
  }

  get zero() {
    return (new Uint8Array(wasm.memory.buffer, this.underlying + 0, 1))[0] == 1;
  }

  get one() {
    return (new Uint8Array(wasm.memory.buffer, this.underlying + 1, 1))[0] == 1;
  }

  get two() {
    return (new Uint8Array(wasm.memory.buffer, this.underlying + 2, 1))[0] == 1;
  }

  get few() {
    return (new Uint8Array(wasm.memory.buffer, this.underlying + 3, 1))[0] == 1;
  }

  get many() {
    return (new Uint8Array(wasm.memory.buffer, this.underlying + 4, 1))[0] == 1;
  }

  get other() {
    return (new Uint8Array(wasm.memory.buffer, this.underlying + 5, 1))[0] == 1;
  }
}

const ICU4XPluralCategory_js_to_rust = {
  "Zero": 0,
  "One": 1,
  "Two": 2,
  "Few": 3,
  "Many": 4,
  "Other": 5,
};
const ICU4XPluralCategory_rust_to_js = {
  0: "Zero",
  1: "One",
  2: "Two",
  3: "Few",
  4: "Many",
  5: "Other",
};

const ICU4XPluralOperands_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XPluralOperands_destroy(underlying);
});

export class ICU4XPluralOperands {
  constructor(underlying) {
    this.underlying = underlying;
  }

  static create(s) {
    let s_diplomat_bytes = (new TextEncoder()).encode(s);
    let s_diplomat_ptr = wasm.diplomat_alloc(s_diplomat_bytes.length, 1);
    let s_diplomat_buf = new Uint8Array(wasm.memory.buffer, s_diplomat_ptr, s_diplomat_bytes.length);
    s_diplomat_buf.set(s_diplomat_bytes, 0);
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(37, 8);
      const result_tag = {};
      diplomat_alloc_destroy_registry.register(result_tag, {
        ptr: diplomat_receive_buffer,
        size: 37,
        align: 8,
      });
      wasm.ICU4XPluralOperands_create(diplomat_receive_buffer, s_diplomat_ptr, s_diplomat_bytes.length);
      const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 36, 1))[0] == 1;
      if (is_ok) {
        const ok_value = (() => {
          const out = new ICU4XPluralOperands(diplomat_receive_buffer);
          out.owner = result_tag;
          return out;
        })();
        return ok_value;
      } else {
        const throw_value = ICU4XError_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    wasm.diplomat_free(s_diplomat_ptr, s_diplomat_bytes.length, 1);
    return diplomat_out;
  }

  get i() {
    return (new BigUint64Array(wasm.memory.buffer, this.underlying + 0, 1))[0];
  }

  get v() {
    return (new Uint32Array(wasm.memory.buffer, this.underlying + 8, 1))[0];
  }

  get w() {
    return (new Uint32Array(wasm.memory.buffer, this.underlying + 12, 1))[0];
  }

  get f() {
    return (new BigUint64Array(wasm.memory.buffer, this.underlying + 16, 1))[0];
  }

  get t() {
    return (new BigUint64Array(wasm.memory.buffer, this.underlying + 24, 1))[0];
  }

  get c() {
    return (new Uint32Array(wasm.memory.buffer, this.underlying + 32, 1))[0];
  }
}

const ICU4XPluralRules_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XPluralRules_destroy(underlying);
});

export class ICU4XPluralRules {
  constructor(underlying) {
    this.underlying = underlying;
  }

  static try_new_cardinal(locale, provider) {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      const result_tag = {};
      diplomat_alloc_destroy_registry.register(result_tag, {
        ptr: diplomat_receive_buffer,
        size: 5,
        align: 4,
      });
      wasm.ICU4XPluralRules_try_new_cardinal(diplomat_receive_buffer, locale.underlying, provider.underlying);
      const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
      if (is_ok) {
        const ok_value = (() => {
          const out = new ICU4XPluralRules((new Uint32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]);
          out.owner = result_tag;
          return out;
        })();
        return ok_value;
      } else {
        const throw_value = ICU4XError_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    return diplomat_out;
  }

  static try_new_ordinal(locale, provider) {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      const result_tag = {};
      diplomat_alloc_destroy_registry.register(result_tag, {
        ptr: diplomat_receive_buffer,
        size: 5,
        align: 4,
      });
      wasm.ICU4XPluralRules_try_new_ordinal(diplomat_receive_buffer, locale.underlying, provider.underlying);
      const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
      if (is_ok) {
        const ok_value = (() => {
          const out = new ICU4XPluralRules((new Uint32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]);
          out.owner = result_tag;
          return out;
        })();
        return ok_value;
      } else {
        const throw_value = ICU4XError_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    return diplomat_out;
  }

  select(op) {
    const diplomat_ICU4XPluralOperands_extracted_i = op["i"];
    const diplomat_ICU4XPluralOperands_extracted_v = op["v"];
    const diplomat_ICU4XPluralOperands_extracted_w = op["w"];
    const diplomat_ICU4XPluralOperands_extracted_f = op["f"];
    const diplomat_ICU4XPluralOperands_extracted_t = op["t"];
    const diplomat_ICU4XPluralOperands_extracted_c = op["c"];
    const diplomat_out = ICU4XPluralCategory_rust_to_js[wasm.ICU4XPluralRules_select(this.underlying, diplomat_ICU4XPluralOperands_extracted_i, diplomat_ICU4XPluralOperands_extracted_v, diplomat_ICU4XPluralOperands_extracted_w, diplomat_ICU4XPluralOperands_extracted_f, diplomat_ICU4XPluralOperands_extracted_t, diplomat_ICU4XPluralOperands_extracted_c)];
    return diplomat_out;
  }

  categories() {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(6, 1);
      wasm.ICU4XPluralRules_categories(diplomat_receive_buffer, this.underlying);
      const out = new ICU4XPluralCategories(diplomat_receive_buffer);
      diplomat_alloc_destroy_registry.register(out, {
        ptr: out.underlying,
        size: 6,
        align: 1,
      });
      return out;
    })();
    return diplomat_out;
  }
}

const ICU4XSentenceBreakIteratorLatin1_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XSentenceBreakIteratorLatin1_destroy(underlying);
});

export class ICU4XSentenceBreakIteratorLatin1 {
  constructor(underlying) {
    this.underlying = underlying;
  }

  next() {
    const diplomat_out = wasm.ICU4XSentenceBreakIteratorLatin1_next(this.underlying);
    return diplomat_out;
  }
}

const ICU4XSentenceBreakIteratorUtf16_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XSentenceBreakIteratorUtf16_destroy(underlying);
});

export class ICU4XSentenceBreakIteratorUtf16 {
  constructor(underlying) {
    this.underlying = underlying;
  }

  next() {
    const diplomat_out = wasm.ICU4XSentenceBreakIteratorUtf16_next(this.underlying);
    return diplomat_out;
  }
}

const ICU4XSentenceBreakIteratorUtf8_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XSentenceBreakIteratorUtf8_destroy(underlying);
});

export class ICU4XSentenceBreakIteratorUtf8 {
  constructor(underlying) {
    this.underlying = underlying;
  }

  next() {
    const diplomat_out = wasm.ICU4XSentenceBreakIteratorUtf8_next(this.underlying);
    return diplomat_out;
  }
}

const ICU4XSentenceBreakSegmenter_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XSentenceBreakSegmenter_destroy(underlying);
});

export class ICU4XSentenceBreakSegmenter {
  constructor(underlying) {
    this.underlying = underlying;
  }

  static try_new(provider) {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      const result_tag = {};
      diplomat_alloc_destroy_registry.register(result_tag, {
        ptr: diplomat_receive_buffer,
        size: 5,
        align: 4,
      });
      wasm.ICU4XSentenceBreakSegmenter_try_new(diplomat_receive_buffer, provider.underlying);
      const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
      if (is_ok) {
        const ok_value = (() => {
          const out = new ICU4XSentenceBreakSegmenter((new Uint32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]);
          out.owner = result_tag;
          return out;
        })();
        return ok_value;
      } else {
        const throw_value = ICU4XError_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    return diplomat_out;
  }

  segment_utf8(input) {
    let input_diplomat_bytes = (new TextEncoder()).encode(input);
    let input_diplomat_ptr = wasm.diplomat_alloc(input_diplomat_bytes.length, 1);
    let input_diplomat_buf = new Uint8Array(wasm.memory.buffer, input_diplomat_ptr, input_diplomat_bytes.length);
    input_diplomat_buf.set(input_diplomat_bytes, 0);
    const diplomat_out = (() => {
      const out = (() => {
        const out = new ICU4XSentenceBreakIteratorUtf8(wasm.ICU4XSentenceBreakSegmenter_segment_utf8(this.underlying, input_diplomat_ptr, input_diplomat_bytes.length));
        out.owner = null;
        return out;
      })();
      ICU4XSentenceBreakIteratorUtf8_box_destroy_registry.register(out, out.underlying)
      return out;
    })();
    wasm.diplomat_free(input_diplomat_ptr, input_diplomat_bytes.length, 1);
    return diplomat_out;
  }

  segment_utf16(input) {
    let input_diplomat_bytes = new Uint8Array(input);
    let input_diplomat_ptr = wasm.diplomat_alloc(input_diplomat_bytes.length, 2);
    let input_diplomat_buf = new Uint8Array(wasm.memory.buffer, input_diplomat_ptr, input_diplomat_bytes.length);
    input_diplomat_buf.set(input_diplomat_bytes, 0);
    const diplomat_out = (() => {
      const out = (() => {
        const out = new ICU4XSentenceBreakIteratorUtf16(wasm.ICU4XSentenceBreakSegmenter_segment_utf16(this.underlying, input_diplomat_ptr, input_diplomat_bytes.length));
        out.owner = null;
        return out;
      })();
      ICU4XSentenceBreakIteratorUtf16_box_destroy_registry.register(out, out.underlying)
      return out;
    })();
    wasm.diplomat_free(input_diplomat_ptr, input_diplomat_bytes.length, 2);
    return diplomat_out;
  }

  segment_latin1(input) {
    let input_diplomat_bytes = new Uint8Array(input);
    let input_diplomat_ptr = wasm.diplomat_alloc(input_diplomat_bytes.length, 1);
    let input_diplomat_buf = new Uint8Array(wasm.memory.buffer, input_diplomat_ptr, input_diplomat_bytes.length);
    input_diplomat_buf.set(input_diplomat_bytes, 0);
    const diplomat_out = (() => {
      const out = (() => {
        const out = new ICU4XSentenceBreakIteratorLatin1(wasm.ICU4XSentenceBreakSegmenter_segment_latin1(this.underlying, input_diplomat_ptr, input_diplomat_bytes.length));
        out.owner = null;
        return out;
      })();
      ICU4XSentenceBreakIteratorLatin1_box_destroy_registry.register(out, out.underlying)
      return out;
    })();
    wasm.diplomat_free(input_diplomat_ptr, input_diplomat_bytes.length, 1);
    return diplomat_out;
  }
}

const ICU4XWordBreakIteratorLatin1_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XWordBreakIteratorLatin1_destroy(underlying);
});

export class ICU4XWordBreakIteratorLatin1 {
  constructor(underlying) {
    this.underlying = underlying;
  }

  next() {
    const diplomat_out = wasm.ICU4XWordBreakIteratorLatin1_next(this.underlying);
    return diplomat_out;
  }
}

const ICU4XWordBreakIteratorUtf16_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XWordBreakIteratorUtf16_destroy(underlying);
});

export class ICU4XWordBreakIteratorUtf16 {
  constructor(underlying) {
    this.underlying = underlying;
  }

  next() {
    const diplomat_out = wasm.ICU4XWordBreakIteratorUtf16_next(this.underlying);
    return diplomat_out;
  }
}

const ICU4XWordBreakIteratorUtf8_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XWordBreakIteratorUtf8_destroy(underlying);
});

export class ICU4XWordBreakIteratorUtf8 {
  constructor(underlying) {
    this.underlying = underlying;
  }

  next() {
    const diplomat_out = wasm.ICU4XWordBreakIteratorUtf8_next(this.underlying);
    return diplomat_out;
  }
}

const ICU4XWordBreakRule_js_to_rust = {
  "Normal": 0,
  "BreakAll": 1,
  "KeepAll": 2,
};
const ICU4XWordBreakRule_rust_to_js = {
  0: "Normal",
  1: "BreakAll",
  2: "KeepAll",
};

const ICU4XWordBreakSegmenter_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XWordBreakSegmenter_destroy(underlying);
});

export class ICU4XWordBreakSegmenter {
  constructor(underlying) {
    this.underlying = underlying;
  }

  static try_new(provider) {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      const result_tag = {};
      diplomat_alloc_destroy_registry.register(result_tag, {
        ptr: diplomat_receive_buffer,
        size: 5,
        align: 4,
      });
      wasm.ICU4XWordBreakSegmenter_try_new(diplomat_receive_buffer, provider.underlying);
      const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
      if (is_ok) {
        const ok_value = (() => {
          const out = new ICU4XWordBreakSegmenter((new Uint32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]);
          out.owner = result_tag;
          return out;
        })();
        return ok_value;
      } else {
        const throw_value = ICU4XError_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    return diplomat_out;
  }

  segment_utf8(input) {
    let input_diplomat_bytes = (new TextEncoder()).encode(input);
    let input_diplomat_ptr = wasm.diplomat_alloc(input_diplomat_bytes.length, 1);
    let input_diplomat_buf = new Uint8Array(wasm.memory.buffer, input_diplomat_ptr, input_diplomat_bytes.length);
    input_diplomat_buf.set(input_diplomat_bytes, 0);
    const diplomat_out = (() => {
      const out = (() => {
        const out = new ICU4XWordBreakIteratorUtf8(wasm.ICU4XWordBreakSegmenter_segment_utf8(this.underlying, input_diplomat_ptr, input_diplomat_bytes.length));
        out.owner = null;
        return out;
      })();
      ICU4XWordBreakIteratorUtf8_box_destroy_registry.register(out, out.underlying)
      return out;
    })();
    wasm.diplomat_free(input_diplomat_ptr, input_diplomat_bytes.length, 1);
    return diplomat_out;
  }

  segment_utf16(input) {
    let input_diplomat_bytes = new Uint8Array(input);
    let input_diplomat_ptr = wasm.diplomat_alloc(input_diplomat_bytes.length, 2);
    let input_diplomat_buf = new Uint8Array(wasm.memory.buffer, input_diplomat_ptr, input_diplomat_bytes.length);
    input_diplomat_buf.set(input_diplomat_bytes, 0);
    const diplomat_out = (() => {
      const out = (() => {
        const out = new ICU4XWordBreakIteratorUtf16(wasm.ICU4XWordBreakSegmenter_segment_utf16(this.underlying, input_diplomat_ptr, input_diplomat_bytes.length));
        out.owner = null;
        return out;
      })();
      ICU4XWordBreakIteratorUtf16_box_destroy_registry.register(out, out.underlying)
      return out;
    })();
    wasm.diplomat_free(input_diplomat_ptr, input_diplomat_bytes.length, 2);
    return diplomat_out;
  }

  segment_latin1(input) {
    let input_diplomat_bytes = new Uint8Array(input);
    let input_diplomat_ptr = wasm.diplomat_alloc(input_diplomat_bytes.length, 1);
    let input_diplomat_buf = new Uint8Array(wasm.memory.buffer, input_diplomat_ptr, input_diplomat_bytes.length);
    input_diplomat_buf.set(input_diplomat_bytes, 0);
    const diplomat_out = (() => {
      const out = (() => {
        const out = new ICU4XWordBreakIteratorLatin1(wasm.ICU4XWordBreakSegmenter_segment_latin1(this.underlying, input_diplomat_ptr, input_diplomat_bytes.length));
        out.owner = null;
        return out;
      })();
      ICU4XWordBreakIteratorLatin1_box_destroy_registry.register(out, out.underlying)
      return out;
    })();
    wasm.diplomat_free(input_diplomat_ptr, input_diplomat_bytes.length, 1);
    return diplomat_out;
  }
}
