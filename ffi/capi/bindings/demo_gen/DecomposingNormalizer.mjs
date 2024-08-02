import { DataProvider } from "../lib/DataProvider.mjs"
import { DecomposingNormalizer } from "../lib/DecomposingNormalizer.mjs"
export function normalize() {
    var terminusArgs = arguments;
    return (function (...args) { return args[0].normalize(...args.slice(1)) }).apply(
        null,
        [
            DecomposingNormalizer.createNfd.apply(
                null,
                [
                    DataProvider.compiled.apply(
                        null,
                        [
                        ]
                    )
                ]
            ),
            terminusArgs[0]
        ]
    );
}
