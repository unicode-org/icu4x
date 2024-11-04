import { TimeZoneInfo } from "icu4x"
export function timeZoneId() {
    var terminusArgs = arguments;
    return (function (...args) { return args[0].timeZoneId }).apply(
        null,
        [
            TimeZoneInfo.fromParts.apply(
                null,
                [
                    terminusArgs[0],
                    terminusArgs[1],
                    terminusArgs[2]
                ]
            )
        ]
    );
}
