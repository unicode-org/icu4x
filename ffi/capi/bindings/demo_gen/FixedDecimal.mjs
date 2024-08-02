import { FixedDecimal } from "./FixedDecimal.mjs"
export function toString() {
    var terminusArgs = arguments;
    return (function (...args) { return args[0].toString(...args.slice(1)) }).apply(
        null,
        [
            FixedDecimal.fromDoubleWithIntegerPrecision.apply(
                null,
                [
                    terminusArgs[0]
                ]
            )
        ]
    );
}
