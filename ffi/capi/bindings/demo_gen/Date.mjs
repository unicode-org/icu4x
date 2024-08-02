import { Calendar } from "./Calendar.mjs"
import { DataProvider } from "./DataProvider.mjs"
import { Date } from "./Date.mjs"
import { Locale } from "./Locale.mjs"
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
