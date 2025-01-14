import { Locale } from "icu4x"
import { TitlecaseMapper } from "icu4x"
import { TitlecaseOptions } from "icu4x"
export function titlecaseSegment(s, name, leading_adjustment, trailing_case) {
    return (function (...args) { return args[0].titlecaseSegment(...args.slice(1)) }).apply(
        null,
        [
            (function (...args) { return new TitlecaseMapper(...args) } ).apply(
                null,
                [
                ]
            ),
            s,
            Locale.fromString.apply(
                null,
                [
                    name
                ]
            ),
            (function (...args) {
                return TitlecaseOptions.fromFields({
                    leadingAdjustment: args[0],
                    trailingCase: args[1]});
            }).apply(
                null,
                [
                    leading_adjustment,
                    trailing_case
                ]
            )
        ]
    );
}
