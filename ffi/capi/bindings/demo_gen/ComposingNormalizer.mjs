import { ComposingNormalizer } from "../lib/ComposingNormalizer.mjs"
import { DataProvider } from "../lib/DataProvider.mjs"
export function normalize() {
    var terminusArgs = arguments;
    return (function (...args) { return args[0].normalize(...args.slice(1)) }).apply(
        null,
        [
            ComposingNormalizer.createNfc.apply(
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
