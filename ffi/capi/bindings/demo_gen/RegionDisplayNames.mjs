import { DataProvider } from "../lib/DataProvider.mjs"
import { Locale } from "../lib/Locale.mjs"
import { RegionDisplayNames } from "../lib/RegionDisplayNames.mjs"
export function of() {
    var terminusArgs = arguments;
    return (function (...args) { return args[0].of(...args.slice(1)) }).apply(
        null,
        [
            RegionDisplayNames.create.apply(
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
                    )
                ]
            ),
            terminusArgs[1]
        ]
    );
}
