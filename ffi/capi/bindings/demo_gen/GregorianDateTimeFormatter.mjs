import { GregorianDateTimeFormatter } from "icu4x"
import { IsoDateTime } from "icu4x"
import { Locale } from "icu4x"
export function formatIsoDatetime(name, length, year, month, day, hour, minute, second, nanosecond) {
    return (function (...args) { return args[0].formatIsoDatetime(...args.slice(1)) }).apply(
        null,
        [
            GregorianDateTimeFormatter.createWithLength.apply(
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
