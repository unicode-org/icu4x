import { DataProvider } from "../lib/DataProvider.mjs"
import { GregorianDateFormatter } from "../lib/GregorianDateFormatter.mjs"
import { IsoDate } from "../lib/IsoDate.mjs"
import { IsoDateTime } from "../lib/IsoDateTime.mjs"
import { Locale } from "../lib/Locale.mjs"
export function formatIsoDate() {
    var terminusArgs = arguments;
    return (function (...args) { return args[0].formatIsoDate(...args.slice(1)) }).apply(
        null,
        [
            GregorianDateFormatter.createWithLength.apply(
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
            IsoDate.create.apply(
                null,
                [
                    terminusArgs[2],
                    terminusArgs[3],
                    terminusArgs[4]
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
            GregorianDateFormatter.createWithLength.apply(
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
            )
        ]
    );
}
