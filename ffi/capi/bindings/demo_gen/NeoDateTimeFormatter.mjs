import { DateTimeFieldSetBuilder } from "icu4x"
import { IsoDate } from "icu4x"
import { Locale } from "icu4x"
import { NeoDateTimeFormatter } from "icu4x"
import { Time } from "icu4x"
export function formatIso(name, length, date_fields, time_precision, zone_style, alignment, year_style, year, month, day, hour, minute, second, nanosecond) {
    return (function (...args) { return args[0].formatIso(...args.slice(1)) }).apply(
        null,
        [
            NeoDateTimeFormatter.createFromBuilder.apply(
                null,
                [
                    Locale.fromString.apply(
                        null,
                        [
                            name
                        ]
                    ),
                    (function (...args) {
                        return DateTimeFieldSetBuilder.fromFields({
                            length: args[0],
                            dateFields: args[1],
                            timePrecision: args[2],
                            zoneStyle: args[3],
                            alignment: args[4],
                            yearStyle: args[5]});
                    }).apply(
                        null,
                        [
                            length,
                            date_fields,
                            time_precision,
                            zone_style,
                            alignment,
                            year_style
                        ]
                    )
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
