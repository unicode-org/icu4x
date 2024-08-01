import { Calendar } from "./js/Calendar.mjs"
import { CustomTimeZone } from "./js/CustomTimeZone.mjs"
import { DataProvider } from "./js/DataProvider.mjs"
import { DateTime } from "./js/DateTime.mjs"
import { IsoDateTime } from "./js/IsoDateTime.mjs"
import { Locale } from "./js/Locale.mjs"
import { ZonedDateTimeFormatter } from "./js/ZonedDateTimeFormatter.mjs"
export function formatDatetimeWithCustomTimeZone() {
    var terminusArgs = arguments;
    return (function (...args) { return args[0].formatDatetimeWithCustomTimeZone(...args.slice(1)) }).apply(
        null,
        [
            ZonedDateTimeFormatter.createWithLengths.apply(
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
                    terminusArgs[1],
                    terminusArgs[2]
                ]
            ),
            DateTime.fromIsoInCalendar.apply(
                null,
                [
                    terminusArgs[3],
                    terminusArgs[4],
                    terminusArgs[5],
                    terminusArgs[6],
                    terminusArgs[7],
                    terminusArgs[8],
                    terminusArgs[9],
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
                                    terminusArgs[10]
                                ]
                            )
                        ]
                    )
                ]
            ),
            CustomTimeZone.fromString.apply(
                null,
                [
                    terminusArgs[11]
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
            ZonedDateTimeFormatter.createWithLengths.apply(
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
                    terminusArgs[1],
                    terminusArgs[2]
                ]
            ),
            IsoDateTime.create.apply(
                null,
                [
                    terminusArgs[3],
                    terminusArgs[4],
                    terminusArgs[5],
                    terminusArgs[6],
                    terminusArgs[7],
                    terminusArgs[8],
                    terminusArgs[9]
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
