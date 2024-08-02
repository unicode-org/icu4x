import { Calendar } from "./Calendar.mjs"
import { CustomTimeZone } from "./CustomTimeZone.mjs"
import { DataProvider } from "./DataProvider.mjs"
import { DateTime } from "./DateTime.mjs"
import { IsoDateTime } from "./IsoDateTime.mjs"
import { Locale } from "./Locale.mjs"
import { ZonedDateTimeFormatter } from "./ZonedDateTimeFormatter.mjs"
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
