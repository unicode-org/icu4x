import icu4x from "./icu4x.mjs"
import * as rtti from "./rtti.mjs"

function withEncodedString(str, fn) {
  let bytes = (new TextEncoder()).encode(str);

  let ptr = icu4x.icu4x_alloc(bytes.length);
  const buf = new Uint8Array(icu4x.memory.buffer, ptr, bytes.length);
  buf.set(bytes, 0);

  try {
    return fn(ptr, buf.length);
  } finally {
    icu4x.icu4x_free(ptr, buf.length);
  }
}

function readString(ptr, len) {
  const buf = new Uint8Array(icu4x.memory.buffer, ptr, len);
  return (new TextDecoder("utf-8")).decode(buf)
}

const fixedDecimalRegistry = new FinalizationRegistry(ptr => {
  icu4x.icu4x_fixed_decimal_destroy(ptr);
});

export class FixedDecimal {
  constructor(magnitude) {
    this.underlying = icu4x.icu4x_fixed_decimal_create(magnitude);    
    fixedDecimalRegistry.register(this, this.underlying);
  }

  multiply_pow10(pow) {
    if (icu4x.icu4x_fixed_decimal_multiply_pow10(this.underlying, pow) != 1) {
      throw new Error("Failed to multiply_pow10 the decimal");
    }
  }

  negate() {
    icu4x.icu4x_fixed_decimal_negate(this.underlying);
  }

  write_to(writable) {
    icu4x.icu4x_fixed_decimal_write_to(this.underlying, writable.underlying);
  }
}

const bufferWritableRegistry = new FinalizationRegistry(ptr => {
  icu4x.icu4x_buffer_writeable_destroy(ptr);
});

export class BufferWritable {
  constructor() {
    this.underlying = icu4x.icu4x_buffer_writeable_create(0);    
    bufferWritableRegistry.register(this, this.underlying);
  }

  getString() {
    const outStringPtr = icu4x.icu4x_buffer_writeable_get_bytes(this.underlying);
    const outStringLen = icu4x.icu4x_buffer_writeable_len(this.underlying);
    return readString(outStringPtr, outStringLen);
  }
}

const LocaleRegistry = new FinalizationRegistry(ptr => {
  icu4x.icu4x_locale_destroy(ptr);
});

export class Locale {
  constructor(name) {
    withEncodedString(name, (encoded, len) => {
      this.underlying = icu4x.icu4x_locale_create(encoded, len);
    });
    LocaleRegistry.register(this, this.underlying);
  }
}

const StaticDataProviderRegistry = new FinalizationRegistry(ptr => {
  const dataProviderView = new Uint32Array(icu4x.memory.buffer, ptr, 2);
  icu4x.icu4x_data_provider_destroy(dataProviderView[0], dataProviderView[1]); // structs are expanded into multiple params
  icu4x.icu4x_free(ptr);
});

const ICU4XCreateDataProviderResult = {
  parse: rtti.StructType({
    success: [ 8, rtti.ScalarType.bool ]
  }),
  size: 12
};

export class StaticDataProvider {
  constructor() {
    const receiveBuffer = icu4x.icu4x_alloc(ICU4XCreateDataProviderResult.size);
    icu4x.icu4x_static_data_provider_create(receiveBuffer);
    const resultParsed = ICU4XCreateDataProviderResult.parse(icu4x.memory.buffer, receiveBuffer);
    if (resultParsed.success) {
      this.underlying = receiveBuffer; // pointer to where the actual data provider is
      StaticDataProviderRegistry.register(this, this.underlying);
    } else {
      throw new Error("Failed to create a static data provider");
    }
  }
}

const FixedDecimalFormatRegistry = new FinalizationRegistry(ptr => {
  icu4x.icu4x_fixed_decimal_format_destroy(ptr);
});

const ICU4XCreateFixedDecimalFormatResult = {
  parse: rtti.StructType({
    fdf: [ 0, rtti.ScalarType.u32 ],
    success: [ 4, rtti.ScalarType.bool ]
  }),
  size: 8
};

export class FixedDecimalFormat {
  constructor(locale, provider, options) {
    const receiveBuffer = icu4x.icu4x_alloc(ICU4XCreateFixedDecimalFormatResult.size);
    icu4x.icu4x_fixed_decimal_format_create(receiveBuffer, locale.underlying, provider.underlying, 0, 0);
    const resultParsed = ICU4XCreateFixedDecimalFormatResult.parse(icu4x.memory.buffer, receiveBuffer);
    icu4x.icu4x_free(receiveBuffer);

    if (resultParsed.success) {
      this.underlying = resultParsed.fdf;
      FixedDecimalFormatRegistry.register(this, this.underlying);
    } else {
      throw new Error("Failed to create a fixed decimal format");
    }
  }

  write(decimal, writable) {
    if (!icu4x.icu4x_fixed_decimal_format_write(this.underlying, decimal.underlying, writable.underlying)) {
      throw new Error("Writing fixed decimal failed");
    }
  }
}
