import { Calendar } from "icu4x"
import { Date } from "icu4x"
import { IsoDate } from "icu4x"
import { Locale } from "icu4x"
import { Time } from "icu4x"
import { TimeZoneInfo } from "icu4x"
import { ZonedDateTimeFormatter } from "icu4x"
export function formatZonedDatetime(name, length, year, month, day, name_1, hour, minute, second, nanosecond, bcp47Id, offsetSeconds, dst) {
    return (function (...args) { return args[0].formatZonedDatetime(...args.slice(1)) }).apply(
        null,
        [
            ZonedDateTimeFormatter.createWithLength.apply(
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
            Date.fromIsoInCalendar.apply(
                null,
                [
                    year,
                    month,
                    day,
                    Calendar.createForLocale.apply(
                        null,
                        [
                            Locale.fromString.apply(
                                null,
                                [
                                    name_1
                                ]
                            )
                        ]
                    )
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
            ),
            (function (...args) { return new TimeZoneInfo(...args) } ).apply(
                null,
                [
                    bcp47Id,
                    offsetSeconds,
                    dst
                ]
            )
        ]
    );
}
export function formatZonedIsoDatetime(name, length, year, month, day, hour, minute, second, nanosecond, bcp47Id, offsetSeconds, dst) {
    return (function (...args) { return args[0].formatZonedIsoDatetime(...args.slice(1)) }).apply(
        null,
        [
            ZonedDateTimeFormatter.createWithLength.apply(
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
            ),
            (function (...args) { return new TimeZoneInfo(...args) } ).apply(
                null,
                [
                    bcp47Id,
                    offsetSeconds,
                    dst
                ]
            )
        ]
    );
}
