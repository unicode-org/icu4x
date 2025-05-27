// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import { lib } from "./index.mjs";

export default {
	"LocaleCanonicalizer.canonicalize": {
		func: (locale) => {
			let canonicalizer = new lib.LocaleCanonicalizer();

			locale = lib.Locale.fromString(locale);

			canonicalizer.canonicalize(locale);

			return locale.toString();
		},
		funcName: "LocaleCanonicalizer.canonicalize",
		parameters: [
			{
                name: "Locale:Name",
                type: "string",
                typeUse: "string"
			}
		]
	}
};
