// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import * as path from "path";
import * as url from "url";

export const TOP_DIR = path.resolve(path.join(path.dirname(url.fileURLToPath(import.meta.url)), "../../../../"));

export const WASM_PATH = path.resolve(path.join(TOP_DIR, "target/wasm32-unknown-unknown/release/icu_capi_cdylib.wasm"));

export const TESTDATA_POSTCARD_PATH = path.resolve(path.join(TOP_DIR, "provider/testdata/data/testdata.postcard"));
