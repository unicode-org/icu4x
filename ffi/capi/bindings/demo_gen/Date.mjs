import { Calendar } from "icu4x"
import { DataProvider } from "icu4x"
import { Date } from "icu4x"
import { Locale } from "icu4x"
export function monthCode() {
    var terminusArgs = arguments;
    return (function (...args) { return args[0].monthCode }).apply(
        null,
        [
            Date.fromIsoInCalendar.apply(
                null,
                [
                    terminusArgs[0],
                    terminusArgs[1],
                    terminusArgs[2],
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
                                    terminusArgs[3]
                                ]
                            )
                        ]
                    )
                ]
            )
        ]
    );
}
export function era() {
    var terminusArgs = arguments;
    return (function (...args) { return args[0].era }).apply(
        null,
        [
            Date.fromIsoInCalendar.apply(
                null,
                [
                    terminusArgs[0],
                    terminusArgs[1],
                    terminusArgs[2],
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
                                    terminusArgs[3]
                                ]
                            )
                        ]
                    )
                ]
            )
        ]
    );
}
