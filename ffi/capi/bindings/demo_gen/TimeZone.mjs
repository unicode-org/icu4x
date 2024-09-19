import { TimeZone } from "icu4x"
export function bcp47Id() {
    var terminusArgs = arguments;
    return (function (...args) { return args[0].bcp47Id }).apply(
        null,
        [
            TimeZone.fromString.apply(
                null,
                [
                    terminusArgs[0]
                ]
            )
        ]
    );
}
