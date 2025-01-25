import { IsoDate } from "icu4x"
import { Locale } from "icu4x"
import { NeoDateTimeFormatter } from "icu4x"
import { Time } from "icu4x"
export function formatIso(name, length, timePrecision, alignment, year, month, day, hour, minute, second, nanosecond) {
    return (function (...args) { return args[0].formatIso(...args.slice(1)) }).apply(
        null,
        [
            NeoDateTimeFormatter.createMdt.apply(
                null,
                [
                    Locale.fromString.apply(
                        null,
                        [
                            name
                        ]
                    ),
                    length,
                    timePrecision,
                    alignment
                ]
            ),
            (function (...args) { return new IsoDate(...args) } ).apply(
                null,
                [
                    year,
                    month,
                    day
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
