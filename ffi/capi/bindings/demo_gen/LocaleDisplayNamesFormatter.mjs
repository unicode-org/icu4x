import { DataProvider } from "icu4x"
import { DisplayNamesOptions } from "icu4x"
import { Locale } from "icu4x"
import { LocaleDisplayNamesFormatter } from "icu4x"
export function of() {
    var terminusArgs = arguments;
    return (function (...args) { return args[0].of(...args.slice(1)) }).apply(
        null,
        [
            LocaleDisplayNamesFormatter.create.apply(
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
                    (function (...args) {
                        let out = new DisplayNamesOptions();
                        
                        out.style = args[0];
                        
                        out.fallback = args[1];
                        
                        out.languageDisplay = args[2];
                        
                        return out;
                    }).apply(
                        null,
                        [
                            terminusArgs[1],
                            terminusArgs[2],
                            terminusArgs[3]
                        ]
                    )
                ]
            ),
            Locale.fromString.apply(
                null,
                [
                    terminusArgs[4]
                ]
            )
        ]
    );
}
