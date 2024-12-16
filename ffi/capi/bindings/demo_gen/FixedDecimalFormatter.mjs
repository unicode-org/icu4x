import { FixedDecimalFormatter } from "icu4x"
import { Locale } from "icu4x"
import { SignedFixedDecimal } from "icu4x"
export function format(name, groupingStrategy, f, magnitude) {
    return (function (...args) { return args[0].format(...args.slice(1)) }).apply(
        null,
        [
            FixedDecimalFormatter.createWithGroupingStrategy.apply(
                null,
                [
                    Locale.fromString.apply(
                        null,
                        [
                            name
                        ]
                    ),
                    groupingStrategy
                ]
            ),
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
export function numberingSystem(name, groupingStrategy) {
    return (function (...args) { return args[0].numberingSystem(...args.slice(1)) }).apply(
        null,
        [
            FixedDecimalFormatter.createWithGroupingStrategy.apply(
                null,
                [
                    Locale.fromString.apply(
                        null,
                        [
                            name
                        ]
                    ),
                    groupingStrategy
                ]
            )
        ]
    );
}
