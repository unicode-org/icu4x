import { Calendar } from "icu4x"
import { Date } from "icu4x"
import { DateFormatter } from "icu4x"
import { DateTime } from "icu4x"
import { IsoDate } from "icu4x"
import { IsoDateTime } from "icu4x"
import { Locale } from "icu4x"
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
            IsoDate.create.apply(
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
export function formatDatetime(name, length, year, month, day, hour, minute, second, nanosecond, name_1) {
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
            DateTime.fromIsoInCalendar.apply(
                null,
                [
                    year,
                    month,
                    day,
                    hour,
                    minute,
                    second,
                    nanosecond,
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
            IsoDateTime.create.apply(
                null,
                [
                    year,
                    month,
                    day,
                    hour,
                    minute,
                    second,
                    nanosecond
                ]
            )
        ]
    );
}
