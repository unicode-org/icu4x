import { Calendar } from "icu4x"
import { Date } from "icu4x"
import { DateFormatter } from "icu4x"
import { IsoDate } from "icu4x"
import { Locale } from "icu4x"
import { Time } from "icu4x"
export function formatDate(name, length, year, month, day, name_1) {
    return (function (...args) { return args[0].formatDate(...args.slice(1)) }).apply(
        null,
        [
            DateFormatter.createWithLength.apply(
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
            )
        ]
    );
}
export function formatIsoDate(name, length, year, month, day) {
    return (function (...args) { return args[0].formatIsoDate(...args.slice(1)) }).apply(
        null,
        [
            DateFormatter.createWithLength.apply(
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
            )
        ]
    );
}
export function formatDatetime(name, length, year, month, day, name_1, hour, minute, second, nanosecond) {
    return (function (...args) { return args[0].formatDatetime(...args.slice(1)) }).apply(
        null,
        [
            DateFormatter.createWithLength.apply(
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
            )
        ]
    );
}
export function formatIsoDatetime(name, length, year, month, day, hour, minute, second, nanosecond) {
    return (function (...args) { return args[0].formatIsoDatetime(...args.slice(1)) }).apply(
        null,
        [
            DateFormatter.createWithLength.apply(
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
            )
        ]
    );
}
