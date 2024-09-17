import { ComposingNormalizer } from "icu4x"
import { DataProvider } from "icu4x"
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
