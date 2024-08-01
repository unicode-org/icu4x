import { DataProvider } from "./js/DataProvider.mjs"
import { ListFormatter } from "./js/ListFormatter.mjs"
import { Locale } from "./js/Locale.mjs"
export function format() {
    var terminusArgs = arguments;
    return (function (...args) { return args[0].format(...args.slice(1)) }).apply(
        null,
        [
            ListFormatter.createAndWithLength.apply(
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
            terminusArgs[2]
        ]
    );
}
