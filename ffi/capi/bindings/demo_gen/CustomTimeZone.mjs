import { CustomTimeZone } from "icu4x"
export function timeZoneId() {
    var terminusArgs = arguments;
    return (function (...args) { return args[0].timeZoneId }).apply(
        null,
        [
            CustomTimeZone.fromString.apply(
                null,
                [
                    terminusArgs[0]
                ]
            )
        ]
    );
}
export function metazoneId() {
    var terminusArgs = arguments;
    return (function (...args) { return args[0].metazoneId }).apply(
        null,
        [
            CustomTimeZone.fromString.apply(
                null,
                [
                    terminusArgs[0]
                ]
            )
        ]
    );
}
