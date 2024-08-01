import { ComposingNormalizer } from "./js/ComposingNormalizer.mjs"
import { DataProvider } from "./js/DataProvider.mjs"
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
