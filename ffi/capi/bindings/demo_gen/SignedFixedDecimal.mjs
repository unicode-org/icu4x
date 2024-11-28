import { SignedFixedDecimal } from "icu4x"
export function toString(f, magnitude) {
    return (function (...args) { return args[0].toString(...args.slice(1)) }).apply(
        null,
        [
            SignedFixedDecimal.fromNumberWithLowerMagnitude.apply(
                null,
                [
                    f,
                    magnitude
                ]
            )
        ]
    );
}
