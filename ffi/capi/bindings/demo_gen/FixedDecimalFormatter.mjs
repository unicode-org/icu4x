import { DataProvider } from "./DataProvider.mjs"
import { FixedDecimal } from "./FixedDecimal.mjs"
import { FixedDecimalFormatter } from "./FixedDecimalFormatter.mjs"
import { Locale } from "./Locale.mjs"
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
