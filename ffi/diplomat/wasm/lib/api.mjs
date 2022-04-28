import wasm from "./wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
const diplomat_alloc_destroy_registry = new FinalizationRegistry(obj => {
  wasm.diplomat_free(obj["ptr"], obj["size"], obj["align"]);
});

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
      wasm.ICU4XCodePointMapData16_try_get_script(diplomat_receive_buffer, provider.underlying);
      const out = new ICU4XCodePointMapData16Response(diplomat_receive_buffer);
      if (out.data.underlying !== 0) {
        const out_data_value = out.data;
        ICU4XCodePointMapData16_box_destroy_registry.register(out_data_value, out_data_value.underlying);
        Object.defineProperty(out, "data", { value: out_data_value });
      } else {
        Object.defineProperty(out, "data", { value: null });
      }
      diplomat_alloc_destroy_registry.register(out, {
        ptr: out.underlying,
        size: 5,
        align: 4,
      });
      return out;
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
      wasm.ICU4XCodePointSetData_try_get_ascii_hex_digit(diplomat_receive_buffer, provider.underlying);
      const out = new ICU4XCodePointSetDataResult(diplomat_receive_buffer);
      if (out.data.underlying !== 0) {
        const out_data_value = out.data;
        ICU4XCodePointSetData_box_destroy_registry.register(out_data_value, out_data_value.underlying);
        Object.defineProperty(out, "data", { value: out_data_value });
      } else {
        Object.defineProperty(out, "data", { value: null });
      }
      diplomat_alloc_destroy_registry.register(out, {
        ptr: out.underlying,
        size: 5,
        align: 4,
      });
      return out;
    })();
    return diplomat_out;
  }

  contains(cp) {
    const diplomat_out = wasm.ICU4XCodePointSetData_contains(this.underlying, diplomatRuntime.extractCodePoint(cp, 'cp'));
    return diplomat_out;
  }
}

const ICU4XCodePointSetDataResult_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XCodePointSetDataResult_destroy(underlying);
});

export class ICU4XCodePointSetDataResult {
  constructor(underlying) {
    this.underlying = underlying;
  }

  get data() {
    return (() => {
      const out = new ICU4XCodePointSetData((new Uint32Array(wasm.memory.buffer, this.underlying + 0, 1))[0]);
      out.owner = null;
      return out;
    })();
  }

  get success() {
    return (new Uint8Array(wasm.memory.buffer, this.underlying + 4, 1))[0] == 1;
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

const ICU4XCreateFixedDecimalResult_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XCreateFixedDecimalResult_destroy(underlying);
});

export class ICU4XCreateFixedDecimalResult {
  constructor(underlying) {
    this.underlying = underlying;
  }

  get fd() {
    return (() => {
      const out = new ICU4XFixedDecimal((new Uint32Array(wasm.memory.buffer, this.underlying + 0, 1))[0]);
      out.owner = null;
      return out;
    })();
  }

  get success() {
    return (new Uint8Array(wasm.memory.buffer, this.underlying + 4, 1))[0] == 1;
  }
}

const ICU4XCreatePluralOperandsResult_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XCreatePluralOperandsResult_destroy(underlying);
});

export class ICU4XCreatePluralOperandsResult {
  constructor(underlying) {
    this.underlying = underlying;
  }

  get operands() {
    return (() => {
      const out = new ICU4XPluralOperands(this.underlying + 0);
      out.owner = null;
      return out;
    })();
  }

  get success() {
    return (new Uint8Array(wasm.memory.buffer, this.underlying + 36, 1))[0] == 1;
  }
}

const ICU4XCreatePluralRulesResult_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XCreatePluralRulesResult_destroy(underlying);
});

export class ICU4XCreatePluralRulesResult {
  constructor(underlying) {
    this.underlying = underlying;
  }

  get rules() {
    return (() => {
      const out = new ICU4XPluralRules((new Uint32Array(wasm.memory.buffer, this.underlying + 0, 1))[0]);
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
      wasm.ICU4XDataProvider_create_fs(diplomat_receive_buffer, path_diplomat_ptr, path_diplomat_bytes.length);
      const out = new ICU4XCreateDataProviderResult(diplomat_receive_buffer);
      if (out.provider.underlying !== 0) {
        const out_provider_value = out.provider;
        ICU4XDataProvider_box_destroy_registry.register(out_provider_value, out_provider_value.underlying);
        Object.defineProperty(out, "provider", { value: out_provider_value });
      } else {
        Object.defineProperty(out, "provider", { value: null });
      }
      diplomat_alloc_destroy_registry.register(out, {
        ptr: out.underlying,
        size: 5,
        align: 4,
      });
      return out;
    })();
    wasm.diplomat_free(path_diplomat_ptr, path_diplomat_bytes.length, 1);
    return diplomat_out;
  }

  static create_test() {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XDataProvider_create_test(diplomat_receive_buffer);
      const out = new ICU4XCreateDataProviderResult(diplomat_receive_buffer);
      if (out.provider.underlying !== 0) {
        const out_provider_value = out.provider;
        ICU4XDataProvider_box_destroy_registry.register(out_provider_value, out_provider_value.underlying);
        Object.defineProperty(out, "provider", { value: out_provider_value });
      } else {
        Object.defineProperty(out, "provider", { value: null });
      }
      diplomat_alloc_destroy_registry.register(out, {
        ptr: out.underlying,
        size: 5,
        align: 4,
      });
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
      wasm.ICU4XDataProvider_create_from_byte_slice(diplomat_receive_buffer, blob_diplomat_ptr, blob_diplomat_bytes.length);
      const out = new ICU4XCreateDataProviderResult(diplomat_receive_buffer);
      if (out.provider.underlying !== 0) {
        const out_provider_value = out.provider;
        ICU4XDataProvider_box_destroy_registry.register(out_provider_value, out_provider_value.underlying);
        Object.defineProperty(out, "provider", { value: out_provider_value });
      } else {
        Object.defineProperty(out, "provider", { value: null });
      }
      diplomat_alloc_destroy_registry.register(out, {
        ptr: out.underlying,
        size: 5,
        align: 4,
      });
      return out;
    })();
    wasm.diplomat_free(blob_diplomat_ptr, blob_diplomat_bytes.length, 1);
    return diplomat_out;
  }

  static create_empty() {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XDataProvider_create_empty(diplomat_receive_buffer);
      const out = new ICU4XCreateDataProviderResult(diplomat_receive_buffer);
      if (out.provider.underlying !== 0) {
        const out_provider_value = out.provider;
        ICU4XDataProvider_box_destroy_registry.register(out_provider_value, out_provider_value.underlying);
        Object.defineProperty(out, "provider", { value: out_provider_value });
      } else {
        Object.defineProperty(out, "provider", { value: null });
      }
      diplomat_alloc_destroy_registry.register(out, {
        ptr: out.underlying,
        size: 5,
        align: 4,
      });
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
        const throw_value = {};
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
      const option_value = wasm.ICU4XFixedDecimal_create_from_f64_with_max_precision(f)
      if (option_value !== 0) {
        const inhabited_value = (() => {
          const out = (() => {
            const out = new ICU4XFixedDecimal(option_value);
            out.owner = null;
            return out;
          })();
          ICU4XFixedDecimal_box_destroy_registry.register(out, out.underlying)
          return out;
        })();
        return inhabited_value;
      } else {
        return null;
      }
    })();
    return diplomat_out;
  }

  static create_from_f64_with_lower_magnitude(f, precision, rounding_mode) {
    const diplomat_out = (() => {
      const option_value = wasm.ICU4XFixedDecimal_create_from_f64_with_lower_magnitude(f, precision, ICU4XFixedDecimalRoundingMode_js_to_rust[rounding_mode])
      if (option_value !== 0) {
        const inhabited_value = (() => {
          const out = (() => {
            const out = new ICU4XFixedDecimal(option_value);
            out.owner = null;
            return out;
          })();
          ICU4XFixedDecimal_box_destroy_registry.register(out, out.underlying)
          return out;
        })();
        return inhabited_value;
      } else {
        return null;
      }
    })();
    return diplomat_out;
  }

  static create_from_f64_with_significant_digits(f, digits, rounding_mode) {
    const diplomat_out = (() => {
      const option_value = wasm.ICU4XFixedDecimal_create_from_f64_with_significant_digits(f, digits, ICU4XFixedDecimalRoundingMode_js_to_rust[rounding_mode])
      if (option_value !== 0) {
        const inhabited_value = (() => {
          const out = (() => {
            const out = new ICU4XFixedDecimal(option_value);
            out.owner = null;
            return out;
          })();
          ICU4XFixedDecimal_box_destroy_registry.register(out, out.underlying)
          return out;
        })();
        return inhabited_value;
      } else {
        return null;
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
      wasm.ICU4XFixedDecimal_create_fromstr(diplomat_receive_buffer, v_diplomat_ptr, v_diplomat_bytes.length);
      const out = new ICU4XCreateFixedDecimalResult(diplomat_receive_buffer);
      if (out.fd.underlying !== 0) {
        const out_fd_value = out.fd;
        ICU4XFixedDecimal_box_destroy_registry.register(out_fd_value, out_fd_value.underlying);
        Object.defineProperty(out, "fd", { value: out_fd_value });
      } else {
        Object.defineProperty(out, "fd", { value: null });
      }
      diplomat_alloc_destroy_registry.register(out, {
        ptr: out.underlying,
        size: 5,
        align: 4,
      });
      return out;
    })();
    wasm.diplomat_free(v_diplomat_ptr, v_diplomat_bytes.length, 1);
    return diplomat_out;
  }

  multiply_pow10(power) {
    const diplomat_out = wasm.ICU4XFixedDecimal_multiply_pow10(this.underlying, power);
    return diplomat_out;
  }

  negate() {
    const diplomat_out = wasm.ICU4XFixedDecimal_negate(this.underlying);
  }

  pad_left(digits) {
    const diplomat_out = wasm.ICU4XFixedDecimal_pad_left(this.underlying, digits);
  }

  truncate_left(magnitude) {
    const diplomat_out = wasm.ICU4XFixedDecimal_truncate_left(this.underlying, magnitude);
  }

  pad_right(negative_magnitude) {
    const diplomat_out = wasm.ICU4XFixedDecimal_pad_right(this.underlying, negative_magnitude);
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

  static try_new(locale, provider, options) {
    const diplomat_ICU4XFixedDecimalFormatOptions_extracted_grouping_strategy = options["grouping_strategy"];
    const diplomat_ICU4XFixedDecimalFormatOptions_extracted_sign_display = options["sign_display"];
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      const result_tag = {};
      diplomat_alloc_destroy_registry.register(result_tag, {
        ptr: diplomat_receive_buffer,
        size: 5,
        align: 4,
      });
      wasm.ICU4XFixedDecimalFormat_try_new(diplomat_receive_buffer, locale.underlying, provider.underlying, ICU4XFixedDecimalGroupingStrategy_js_to_rust[diplomat_ICU4XFixedDecimalFormatOptions_extracted_grouping_strategy], ICU4XFixedDecimalSignDisplay_js_to_rust[diplomat_ICU4XFixedDecimalFormatOptions_extracted_sign_display]);
      const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
      if (is_ok) {
        const ok_value = (() => {
          const out = new ICU4XFixedDecimalFormat((new Uint32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]);
          out.owner = result_tag;
          return out;
        })();
        return ok_value;
      } else {
        const throw_value = {};
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    return diplomat_out;
  }

  static try_new_from_decimal_symbols_v1(data_struct, options) {
    const diplomat_ICU4XFixedDecimalFormatOptions_extracted_grouping_strategy = options["grouping_strategy"];
    const diplomat_ICU4XFixedDecimalFormatOptions_extracted_sign_display = options["sign_display"];
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      const result_tag = {};
      diplomat_alloc_destroy_registry.register(result_tag, {
        ptr: diplomat_receive_buffer,
        size: 5,
        align: 4,
      });
      wasm.ICU4XFixedDecimalFormat_try_new_from_decimal_symbols_v1(diplomat_receive_buffer, data_struct.underlying, ICU4XFixedDecimalGroupingStrategy_js_to_rust[diplomat_ICU4XFixedDecimalFormatOptions_extracted_grouping_strategy], ICU4XFixedDecimalSignDisplay_js_to_rust[diplomat_ICU4XFixedDecimalFormatOptions_extracted_sign_display]);
      const is_ok = (new Uint8Array(wasm.memory.buffer, diplomat_receive_buffer + 4, 1))[0] == 1;
      if (is_ok) {
        const ok_value = (() => {
          const out = new ICU4XFixedDecimalFormat((new Uint32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]);
          out.owner = result_tag;
          return out;
        })();
        return ok_value;
      } else {
        const throw_value = {};
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    return diplomat_out;
  }

  format(value) {
    const diplomat_out = diplomatRuntime.withWriteable(wasm, (writeable) => {
      return (() => {
        const is_ok = wasm.ICU4XFixedDecimalFormat_format(this.underlying, value.underlying, writeable) == 1;
        if (!is_ok) {
          throw new diplomatRuntime.FFIError({});
        }
      })();
    });
    return diplomat_out;
  }
}

const ICU4XFixedDecimalFormatOptions_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XFixedDecimalFormatOptions_destroy(underlying);
});

export class ICU4XFixedDecimalFormatOptions {
  constructor(underlying) {
    this.underlying = underlying;
  }

  static default() {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(8, 4);
      wasm.ICU4XFixedDecimalFormatOptions_default(diplomat_receive_buffer);
      const out = new ICU4XFixedDecimalFormatOptions(diplomat_receive_buffer);
      diplomat_alloc_destroy_registry.register(out, {
        ptr: out.underlying,
        size: 8,
        align: 4,
      });
      return out;
    })();
    return diplomat_out;
  }

  get grouping_strategy() {
    return ICU4XFixedDecimalGroupingStrategy_rust_to_js[(new Int32Array(wasm.memory.buffer, this.underlying + 0, 1))[0]];
  }

  get sign_display() {
    return ICU4XFixedDecimalSignDisplay_rust_to_js[(new Int32Array(wasm.memory.buffer, this.underlying + 4, 1))[0]];
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

const ICU4XFixedDecimalRoundingMode_js_to_rust = {
  "Truncate": 0,
  "HalfExpand": 1,
};
const ICU4XFixedDecimalRoundingMode_rust_to_js = {
  0: "Truncate",
  1: "HalfExpand",
};

const ICU4XFixedDecimalSignDisplay_js_to_rust = {
  "Auto": 0,
  "Never": 1,
  "Always": 2,
  "ExceptZero": 3,
  "Negative": 4,
};
const ICU4XFixedDecimalSignDisplay_rust_to_js = {
  0: "Auto",
  1: "Never",
  2: "Always",
  3: "ExceptZero",
  4: "Negative",
};

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
        const throw_value = {};
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
        const throw_value = {};
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
          const throw_value = ICU4XLocaleError_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
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
          const throw_value = ICU4XLocaleError_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
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
          const throw_value = ICU4XLocaleError_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
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
        const throw_value = ICU4XLocaleError_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
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
          const throw_value = ICU4XLocaleError_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
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
        const throw_value = ICU4XLocaleError_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
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
          const throw_value = ICU4XLocaleError_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
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
        const throw_value = ICU4XLocaleError_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
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
          const throw_value = ICU4XLocaleError_rust_to_js[(new Int32Array(wasm.memory.buffer, diplomat_receive_buffer, 1))[0]];
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
      const option_value = wasm.ICU4XLocaleCanonicalizer_create(provider.underlying)
      if (option_value !== 0) {
        const inhabited_value = (() => {
          const out = (() => {
            const out = new ICU4XLocaleCanonicalizer(option_value);
            out.owner = null;
            return out;
          })();
          ICU4XLocaleCanonicalizer_box_destroy_registry.register(out, out.underlying)
          return out;
        })();
        return inhabited_value;
      } else {
        return null;
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

const ICU4XLocaleError_js_to_rust = {
  "Undefined": 0,
  "Error": 1,
};
const ICU4XLocaleError_rust_to_js = {
  0: "Undefined",
  1: "Error",
};

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
      wasm.ICU4XPluralOperands_create(diplomat_receive_buffer, s_diplomat_ptr, s_diplomat_bytes.length);
      const out = new ICU4XCreatePluralOperandsResult(diplomat_receive_buffer);
      diplomat_alloc_destroy_registry.register(out, {
        ptr: out.underlying,
        size: 37,
        align: 8,
      });
      return out;
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
      wasm.ICU4XPluralRules_try_new_cardinal(diplomat_receive_buffer, locale.underlying, provider.underlying);
      const out = new ICU4XCreatePluralRulesResult(diplomat_receive_buffer);
      if (out.rules.underlying !== 0) {
        const out_rules_value = out.rules;
        ICU4XPluralRules_box_destroy_registry.register(out_rules_value, out_rules_value.underlying);
        Object.defineProperty(out, "rules", { value: out_rules_value });
      } else {
        Object.defineProperty(out, "rules", { value: null });
      }
      diplomat_alloc_destroy_registry.register(out, {
        ptr: out.underlying,
        size: 5,
        align: 4,
      });
      return out;
    })();
    return diplomat_out;
  }

  static try_new_ordinal(locale, provider) {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XPluralRules_try_new_ordinal(diplomat_receive_buffer, locale.underlying, provider.underlying);
      const out = new ICU4XCreatePluralRulesResult(diplomat_receive_buffer);
      if (out.rules.underlying !== 0) {
        const out_rules_value = out.rules;
        ICU4XPluralRules_box_destroy_registry.register(out_rules_value, out_rules_value.underlying);
        Object.defineProperty(out, "rules", { value: out_rules_value });
      } else {
        Object.defineProperty(out, "rules", { value: null });
      }
      diplomat_alloc_destroy_registry.register(out, {
        ptr: out.underlying,
        size: 5,
        align: 4,
      });
      return out;
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
