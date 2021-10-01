import test from 'ava';
import { promises as fsPromises } from 'fs';

import { ICU4XFixedDecimal, ICU4XLocale, ICU4XDataProvider, ICU4XFixedDecimalFormat, ICU4XFixedDecimalFormatOptions } from "../lib/api.mjs"

const BLOB_PATH = "../../../../provider/testdata/data/testdata.postcard";

test("use create_from_byte_slice to format a simple decimal", async t => {
  const locale = ICU4XLocale.create("bn");
  const nodeBuffer = await fsPromises.readFile(BLOB_PATH);
  const bytes = new Uint8Array(nodeBuffer.buffer, nodeBuffer.byteOffset, nodeBuffer.length);
  const result = ICU4XDataProvider.create_from_byte_slice(bytes);
  t.assert(result.success);
  const format = ICU4XFixedDecimalFormat.try_new(locale, result.provider, ICU4XFixedDecimalFormatOptions.default()).fdf;

  const decimal = ICU4XFixedDecimal.create(1234);
  decimal.multiply_pow10(-2);

  t.is(format.format(decimal), "১২.৩৪");
});

test("fail to create from invalid buffer", t => {
  const arrayBuffer = new ArrayBuffer(8);
  const bytes = new Uint8Array(arrayBuffer);
  // Fill the buffer with junk data
  for (let i = 0; i < bytes.length; i++) {
    bytes[i] = i;
  }
  const result = ICU4XDataProvider.create_from_byte_slice(bytes);
  t.assert(!result.success);
});
