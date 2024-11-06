import { SignedFixedDecimal } from "icu4x"
export function toString() {
    var terminusArgs = arguments;
    return (function (...args) { return args[0].toString(...args.slice(1)) }).apply(
        null,
        [
            SignedFixedDecimal.fromNumberWithLowerMagnitude.apply(
                null,
                [
                    terminusArgs[0],
                    terminusArgs[1]
                ]
            )
        ]
    );
}
