import { AnyCalendarKind } from "./AnyCalendarKind.mjs"
export function bcp47() {
    var terminusArgs = arguments;
    return (function (...args) { return args[0].bcp47 }).apply(
        null,
        [
            terminusArgs[0]
        ]
    );
}
