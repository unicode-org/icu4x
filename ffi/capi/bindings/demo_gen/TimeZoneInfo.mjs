import { TimeZoneInfo } from "icu4x"
export function timeZoneId() {
    var terminusArgs = arguments;
    return (function (...args) { return args[0].timeZoneId }).apply(
        null,
        [
            TimeZoneInfo.unknown.apply(
                null,
                [
                ]
            )
        ]
    );
}
export function zoneVariant() {
    var terminusArgs = arguments;
    return (function (...args) { return args[0].zoneVariant }).apply(
        null,
        [
            TimeZoneInfo.unknown.apply(
                null,
                [
                ]
            )
        ]
    );
}
