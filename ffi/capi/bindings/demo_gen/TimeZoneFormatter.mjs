import { CustomTimeZone } from "icu4x"
import { DataProvider } from "icu4x"
import { Locale } from "icu4x"
import { TimeZoneFormatter } from "icu4x"
export function formatCustomTimeZone() {
    var terminusArgs = arguments;
    return (function (...args) { return args[0].formatCustomTimeZone(...args.slice(1)) }).apply(
        null,
        [
            TimeZoneFormatter.createWithLocalizedOffsetFallback.apply(
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
                    )
                ]
            ),
            CustomTimeZone.fromString.apply(
                null,
                [
                    terminusArgs[1]
                ]
            )
        ]
    );
}
export function formatCustomTimeZoneNoFallback() {
    var terminusArgs = arguments;
    return (function (...args) { return args[0].formatCustomTimeZoneNoFallback(...args.slice(1)) }).apply(
        null,
        [
            TimeZoneFormatter.createWithLocalizedOffsetFallback.apply(
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
                    )
                ]
            ),
            CustomTimeZone.fromString.apply(
                null,
                [
                    terminusArgs[1]
                ]
            )
        ]
    );
}
