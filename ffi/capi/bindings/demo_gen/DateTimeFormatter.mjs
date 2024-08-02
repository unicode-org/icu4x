import { Calendar } from "./Calendar.mjs"
import { DataProvider } from "./DataProvider.mjs"
import { DateTime } from "./DateTime.mjs"
import { DateTimeFormatter } from "./DateTimeFormatter.mjs"
import { IsoDateTime } from "./IsoDateTime.mjs"
import { Locale } from "./Locale.mjs"
export function formatDatetime() {
    var terminusArgs = arguments;
    return (function (...args) { return args[0].formatDatetime(...args.slice(1)) }).apply(
        null,
        [
            DateTimeFormatter.createWithLengths.apply(
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
            )
        ]
    );
}
export function formatIsoDatetime() {
    var terminusArgs = arguments;
    return (function (...args) { return args[0].formatIsoDatetime(...args.slice(1)) }).apply(
        null,
        [
            DateTimeFormatter.createWithLengths.apply(
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
            )
        ]
    );
}
