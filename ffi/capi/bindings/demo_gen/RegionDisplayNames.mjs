import { DataProvider } from "icu4x"
import { Locale } from "icu4x"
import { RegionDisplayNames } from "icu4x"
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
