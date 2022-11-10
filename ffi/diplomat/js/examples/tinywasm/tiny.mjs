import {
    ICU4XLocale,
    ICU4XDataProvider,
    ICU4XFixedDecimalFormatter,
    ICU4XFixedDecimal
} from "./lib/index.js"
import { readFileSync } from "fs";

const data = readFileSync("icu4x_data.postcard");

async function main() {
    const locale = ICU4XLocale.create_from_string("bn");
    const provider = ICU4XDataProvider.create_from_byte_slice(data);
  
    const format = ICU4XFixedDecimalFormatter.create_with_grouping_strategy(provider, locale, "Auto");
  
    const decimal = ICU4XFixedDecimal.create_from_i32(1000007);
    decimal.multiply_pow10(-2);
  
    const result = format.format(decimal);
    console.log("Output is ", result);
}

main();
