import { Locale } from "icu4x"
import { Time } from "icu4x"
import { TimeFormatter } from "icu4x"
export function formatTime(name, length, hour, minute, second, nanosecond) {
    return (function (...args) { return args[0].formatTime(...args.slice(1)) }).apply(
        null,
        [
            TimeFormatter.createWithLength.apply(
                null,
                [
                    Locale.fromString.apply(
                        null,
                        [
                            name
                        ]
                    ),
                    length
                ]
            ),
            (function (...args) { return new Time(...args) } ).apply(
                null,
                [
                    hour,
                    minute,
                    second,
                    nanosecond
                ]
            )
        ]
    );
}
