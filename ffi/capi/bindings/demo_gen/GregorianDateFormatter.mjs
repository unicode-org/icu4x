import { GregorianDateFormatter } from "icu4x"
import { IsoDate } from "icu4x"
import { Locale } from "icu4x"
export function formatIso(name, length, year, month, day) {
    return (function (...args) { return args[0].formatIso(...args.slice(1)) }).apply(
        null,
        [
            GregorianDateFormatter.createWithLength.apply(
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
            (function (...args) { return new IsoDate(...args) } ).apply(
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
