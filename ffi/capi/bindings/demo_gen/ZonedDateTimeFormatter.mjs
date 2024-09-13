import { Calendar } from "icu4x"
import { CustomTimeZone } from "icu4x"
import { DataProvider } from "icu4x"
import { DateTime } from "icu4x"
import { IsoDateTime } from "icu4x"
import { Locale } from "icu4x"
import { ZonedDateTimeFormatter } from "icu4x"
export function formatDatetimeWithCustomTimeZone() {
    var terminusArgs = arguments;
    return (function (...args) { return args[0].formatDatetimeWithCustomTimeZone(...args.slice(1)) }).apply(
        null,
        [
            ZonedDateTimeFormatter.createWithLength.apply(
                null,
                [
                    DataProvider.compiled.apply(
                        null,
                        [
                        ]
                    ),
                    Locale.fromString.apply(
                        null,
                        [
                            terminusArgs[0]
                        ]
                    ),
                    terminusArgs[1]
                ]
            ),
            DateTime.fromIsoInCalendar.apply(
                null,
                [
                    terminusArgs[2],
                    terminusArgs[3],
                    terminusArgs[4],
                    terminusArgs[5],
                    terminusArgs[6],
                    terminusArgs[7],
                    terminusArgs[8],
                    Calendar.createForLocale.apply(
                        null,
                        [
                            DataProvider.compiled.apply(
                                null,
                                [
                                ]
                            ),
                            Locale.fromString.apply(
                                null,
                                [
                                    terminusArgs[9]
                                ]
                            )
                        ]
                    )
                ]
            ),
            CustomTimeZone.fromString.apply(
                null,
                [
                    terminusArgs[10]
                ]
            )
        ]
    );
}
export function formatIsoDatetimeWithCustomTimeZone() {
    var terminusArgs = arguments;
    return (function (...args) { return args[0].formatIsoDatetimeWithCustomTimeZone(...args.slice(1)) }).apply(
        null,
        [
            ZonedDateTimeFormatter.createWithLength.apply(
                null,
                [
                    DataProvider.compiled.apply(
                        null,
                        [
                        ]
                    ),
                    Locale.fromString.apply(
                        null,
                        [
                            terminusArgs[0]
                        ]
                    ),
                    terminusArgs[1]
                ]
            ),
            IsoDateTime.create.apply(
                null,
                [
                    terminusArgs[2],
                    terminusArgs[3],
                    terminusArgs[4],
                    terminusArgs[5],
                    terminusArgs[6],
                    terminusArgs[7],
                    terminusArgs[8]
                ]
            ),
            CustomTimeZone.fromString.apply(
                null,
                [
                    terminusArgs[9]
                ]
            )
        ]
    );
}
