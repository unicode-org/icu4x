import wasm from "./wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
const diplomat_alloc_destroy_registry = new FinalizationRegistry(obj => {
  wasm.diplomat_free(obj["ptr"], obj["size"]);
});

const ICU4XCanonicalizationResult_js_to_rust = {
  "Modified": 0,
  "Unmodified": 1,
};
const ICU4XCanonicalizationResult_rust_to_js = {
  0: "Modified",
  1: "Unmodified",
};

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
      return out;
    })();
  }

  get success() {
    return (new Uint8Array(wasm.memory.buffer, this.underlying + 24, 1))[0] == 1;
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
    let path_diplomat_ptr = wasm.diplomat_alloc(path_diplomat_bytes.length);
    let path_diplomat_buf = new Uint8Array(wasm.memory.buffer, path_diplomat_ptr, path_diplomat_bytes.length);
    path_diplomat_buf.set(path_diplomat_bytes, 0);
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5);
      wasm.ICU4XDataProvider_create_fs(diplomat_receive_buffer, path_diplomat_ptr, path_diplomat_bytes.length);
      const out = new ICU4XCreateDataProviderResult(diplomat_receive_buffer);
      const out_provider_value = out.provider;
      ICU4XDataProvider_box_destroy_registry.register(out_provider_value, out_provider_value.underlying);
      Object.defineProperty(out, "provider", { value: out_provider_value });
      diplomat_alloc_destroy_registry.register(out, {
        ptr: out.underlying,
        size: 5
      });
      return out;
    })();
    wasm.diplomat_free(path_diplomat_ptr, path_diplomat_bytes.length);
    return diplomat_out;
  }

  static create_static() {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5);
      wasm.ICU4XDataProvider_create_static(diplomat_receive_buffer);
      const out = new ICU4XCreateDataProviderResult(diplomat_receive_buffer);
      const out_provider_value = out.provider;
      ICU4XDataProvider_box_destroy_registry.register(out_provider_value, out_provider_value.underlying);
      Object.defineProperty(out, "provider", { value: out_provider_value });
      diplomat_alloc_destroy_registry.register(out, {
        ptr: out.underlying,
        size: 5
      });
      return out;
    })();
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
        return out;
      })();
      ICU4XFixedDecimal_box_destroy_registry.register(out, out.underlying)
      return out;
    })();
    return diplomat_out;
  }

  static create_fromstr(v) {
    let v_diplomat_bytes = (new TextEncoder()).encode(v);
    let v_diplomat_ptr = wasm.diplomat_alloc(v_diplomat_bytes.length);
    let v_diplomat_buf = new Uint8Array(wasm.memory.buffer, v_diplomat_ptr, v_diplomat_bytes.length);
    v_diplomat_buf.set(v_diplomat_bytes, 0);
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5);
      wasm.ICU4XFixedDecimal_create_fromstr(diplomat_receive_buffer, v_diplomat_ptr, v_diplomat_bytes.length);
      const out = new ICU4XCreateFixedDecimalResult(diplomat_receive_buffer);
      const out_fd_value = out.fd;
      ICU4XFixedDecimal_box_destroy_registry.register(out_fd_value, out_fd_value.underlying);
      Object.defineProperty(out, "fd", { value: out_fd_value });
      diplomat_alloc_destroy_registry.register(out, {
        ptr: out.underlying,
        size: 5
      });
      return out;
    })();
    wasm.diplomat_free(v_diplomat_ptr, v_diplomat_bytes.length);
    return diplomat_out;
  }

  multiply_pow10(power) {
    const diplomat_out = wasm.ICU4XFixedDecimal_multiply_pow10(this.underlying, power);
    return diplomat_out;
  }

  negate() {
    wasm.ICU4XFixedDecimal_negate(this.underlying)
  }

  to_string() {
    const diplomat_out = diplomatRuntime.withWriteable(wasm, (writeable) => wasm.ICU4XFixedDecimal_to_string(this.underlying, writeable));
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
      const diplomat_receive_buffer = wasm.diplomat_alloc(5);
      wasm.ICU4XFixedDecimalFormat_try_new(diplomat_receive_buffer, locale.underlying, provider.underlying, ICU4XFixedDecimalGroupingStrategy_js_to_rust[diplomat_ICU4XFixedDecimalFormatOptions_extracted_grouping_strategy], ICU4XFixedDecimalSignDisplay_js_to_rust[diplomat_ICU4XFixedDecimalFormatOptions_extracted_sign_display]);
      const out = new ICU4XFixedDecimalFormatResult(diplomat_receive_buffer);
      const out_fdf_value = out.fdf;
      ICU4XFixedDecimalFormat_box_destroy_registry.register(out_fdf_value, out_fdf_value.underlying);
      Object.defineProperty(out, "fdf", { value: out_fdf_value });
      diplomat_alloc_destroy_registry.register(out, {
        ptr: out.underlying,
        size: 5
      });
      return out;
    })();
    return diplomat_out;
  }

  format_write(value) {
    const diplomat_out = diplomatRuntime.withWriteable(wasm, (writeable) => wasm.ICU4XFixedDecimalFormat_format_write(this.underlying, value.underlying, writeable));
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
      const diplomat_receive_buffer = wasm.diplomat_alloc(8);
      wasm.ICU4XFixedDecimalFormatOptions_default(diplomat_receive_buffer);
      const out = new ICU4XFixedDecimalFormatOptions(diplomat_receive_buffer);
      diplomat_alloc_destroy_registry.register(out, {
        ptr: out.underlying,
        size: 8
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

const ICU4XFixedDecimalFormatResult_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XFixedDecimalFormatResult_destroy(underlying);
});

export class ICU4XFixedDecimalFormatResult {
  constructor(underlying) {
    this.underlying = underlying;
  }

  get fdf() {
    return (() => {
      const out = new ICU4XFixedDecimalFormat((new Uint32Array(wasm.memory.buffer, this.underlying + 0, 1))[0]);
      return out;
    })();
  }

  get success() {
    return (new Uint8Array(wasm.memory.buffer, this.underlying + 4, 1))[0] == 1;
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

const ICU4XLocale_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XLocale_destroy(underlying);
});

export class ICU4XLocale {
  constructor(underlying) {
    this.underlying = underlying;
  }

  static create(name) {
    let name_diplomat_bytes = (new TextEncoder()).encode(name);
    let name_diplomat_ptr = wasm.diplomat_alloc(name_diplomat_bytes.length);
    let name_diplomat_buf = new Uint8Array(wasm.memory.buffer, name_diplomat_ptr, name_diplomat_bytes.length);
    name_diplomat_buf.set(name_diplomat_bytes, 0);
    const diplomat_out = (() => {
      const out = (() => {
        const out = new ICU4XLocale(wasm.ICU4XLocale_create(name_diplomat_ptr, name_diplomat_bytes.length));
        return out;
      })();
      ICU4XLocale_box_destroy_registry.register(out, out.underlying)
      return out;
    })();
    wasm.diplomat_free(name_diplomat_ptr, name_diplomat_bytes.length);
    return diplomat_out;
  }

  clone() {
    const diplomat_out = (() => {
      const out = (() => {
        const out = new ICU4XLocale(wasm.ICU4XLocale_clone(this.underlying));
        return out;
      })();
      ICU4XLocale_box_destroy_registry.register(out, out.underlying)
      return out;
    })();
    return diplomat_out;
  }

  basename() {
    const diplomat_out = diplomatRuntime.withWriteable(wasm, (writeable) => wasm.ICU4XLocale_basename(this.underlying, writeable));
    return diplomat_out;
  }

  get_unicode_extension(bytes) {
    let bytes_diplomat_bytes = (new TextEncoder()).encode(bytes);
    let bytes_diplomat_ptr = wasm.diplomat_alloc(bytes_diplomat_bytes.length);
    let bytes_diplomat_buf = new Uint8Array(wasm.memory.buffer, bytes_diplomat_ptr, bytes_diplomat_bytes.length);
    bytes_diplomat_buf.set(bytes_diplomat_bytes, 0);
    const diplomat_out = diplomatRuntime.withWriteable(wasm, (writeable) => wasm.ICU4XLocale_get_unicode_extension(this.underlying, bytes_diplomat_ptr, bytes_diplomat_bytes.length, writeable));
    wasm.diplomat_free(bytes_diplomat_ptr, bytes_diplomat_bytes.length);
    return diplomat_out;
  }

  language() {
    const diplomat_out = diplomatRuntime.withWriteable(wasm, (writeable) => wasm.ICU4XLocale_language(this.underlying, writeable));
    return diplomat_out;
  }

  region() {
    const diplomat_out = diplomatRuntime.withWriteable(wasm, (writeable) => wasm.ICU4XLocale_region(this.underlying, writeable));
    return diplomat_out;
  }

  script() {
    const diplomat_out = diplomatRuntime.withWriteable(wasm, (writeable) => wasm.ICU4XLocale_script(this.underlying, writeable));
    return diplomat_out;
  }

  tostring() {
    const diplomat_out = diplomatRuntime.withWriteable(wasm, (writeable) => wasm.ICU4XLocale_tostring(this.underlying, writeable));
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
      const out = (() => {
        const out = new ICU4XLocaleCanonicalizer(wasm.ICU4XLocaleCanonicalizer_create(provider.underlying));
        return out;
      })();
      ICU4XLocaleCanonicalizer_box_destroy_registry.register(out, out.underlying)
      return out;
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

const ICU4XLocaleResult_js_to_rust = {
  "Ok": 0,
  "Undefined": 1,
  "Error": 2,
};
const ICU4XLocaleResult_rust_to_js = {
  0: "Ok",
  1: "Undefined",
  2: "Error",
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
    let s_diplomat_ptr = wasm.diplomat_alloc(s_diplomat_bytes.length);
    let s_diplomat_buf = new Uint8Array(wasm.memory.buffer, s_diplomat_ptr, s_diplomat_bytes.length);
    s_diplomat_buf.set(s_diplomat_bytes, 0);
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(25);
      wasm.ICU4XPluralOperands_create(diplomat_receive_buffer, s_diplomat_ptr, s_diplomat_bytes.length);
      const out = new ICU4XCreatePluralOperandsResult(diplomat_receive_buffer);
      diplomat_alloc_destroy_registry.register(out, {
        ptr: out.underlying,
        size: 25
      });
      return out;
    })();
    wasm.diplomat_free(s_diplomat_ptr, s_diplomat_bytes.length);
    return diplomat_out;
  }

  get i() {
    return (new BigUint64Array(wasm.memory.buffer, this.underlying + 0, 1))[0];
  }

  get v() {
    return (new Uint32Array(wasm.memory.buffer, this.underlying + 4, 1))[0];
  }

  get w() {
    return (new Uint32Array(wasm.memory.buffer, this.underlying + 8, 1))[0];
  }

  get f() {
    return (new BigUint64Array(wasm.memory.buffer, this.underlying + 12, 1))[0];
  }

  get t() {
    return (new BigUint64Array(wasm.memory.buffer, this.underlying + 16, 1))[0];
  }

  get c() {
    return (new Uint32Array(wasm.memory.buffer, this.underlying + 20, 1))[0];
  }
}

const ICU4XPluralRuleType_js_to_rust = {
  "Cardinal": 0,
  "Ordinal": 1,
};
const ICU4XPluralRuleType_rust_to_js = {
  0: "Cardinal",
  1: "Ordinal",
};

const ICU4XPluralRules_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XPluralRules_destroy(underlying);
});

export class ICU4XPluralRules {
  constructor(underlying) {
    this.underlying = underlying;
  }

  static create(locale, provider, ty) {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5);
      wasm.ICU4XPluralRules_create(diplomat_receive_buffer, locale.underlying, provider.underlying, ICU4XPluralRuleType_js_to_rust[ty]);
      const out = new ICU4XCreatePluralRulesResult(diplomat_receive_buffer);
      const out_rules_value = out.rules;
      ICU4XPluralRules_box_destroy_registry.register(out_rules_value, out_rules_value.underlying);
      Object.defineProperty(out, "rules", { value: out_rules_value });
      diplomat_alloc_destroy_registry.register(out, {
        ptr: out.underlying,
        size: 5
      });
      return out;
    })();
    return diplomat_out;
  }

  select(op) {
    const diplomat_out = ICU4XPluralCategory_rust_to_js[wasm.ICU4XPluralRules_select(this.underlying, op.underlying)];
    return diplomat_out;
  }

  categories() {
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(6);
      wasm.ICU4XPluralRules_categories(this.underlying, diplomat_receive_buffer);
      const out = new ICU4XPluralCategories(diplomat_receive_buffer);
      diplomat_alloc_destroy_registry.register(out, {
        ptr: out.underlying,
        size: 6
      });
      return out;
    })();
    return diplomat_out;
  }
}
