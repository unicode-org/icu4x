import { CustomTimeZone } from "./js/CustomTimeZone.mjs"
import { DataProvider } from "./js/DataProvider.mjs"
import { Locale } from "./js/Locale.mjs"
import { TimeZoneFormatter } from "./js/TimeZoneFormatter.mjs"
export function formatCustomTimeZone() {
    var terminusArgs = arguments;
    return (function (...args) { return args[0].formatCustomTimeZone(...args.slice(1)) }).apply(
        null,
        [
            TimeZoneFormatter.createWithLocalizedGmtFallback.apply(
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
            TimeZoneFormatter.createWithLocalizedGmtFallback.apply(
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
