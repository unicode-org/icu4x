import { DataProvider } from "../lib/DataProvider.mjs"
import { FixedDecimal } from "../lib/FixedDecimal.mjs"
import { FixedDecimalFormatter } from "../lib/FixedDecimalFormatter.mjs"
import { Locale } from "../lib/Locale.mjs"
export function format() {
    var terminusArgs = arguments;
    return (function (...args) { return args[0].format(...args.slice(1)) }).apply(
        null,
        [
            FixedDecimalFormatter.createWithGroupingStrategy.apply(
                null,
                [
                    DataProvider.compiled.apply(
                        null,
                        [
                        ]
                    ),
                    Locale.fromString.apply(
                        null,
                        [
                            terminusArgs[0]
                        ]
                    ),
                    terminusArgs[1]
                ]
            ),
            FixedDecimal.fromDoubleWithIntegerPrecision.apply(
                null,
                [
                    terminusArgs[2]
                ]
            )
        ]
    );
}
