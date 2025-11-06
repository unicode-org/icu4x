// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import { icu } from "./index.mjs";

export default {
	"LocaleCanonicalizer.canonicalize": {
		func: (locale) => {
			let canonicalizer = new icu.LocaleCanonicalizer();

			locale = icu.Locale.fromString(locale);

			canonicalizer.canonicalize(locale);

			return locale.toString();
		},
		expr: (locale) => `let locale = icu.Locale.fromString(${locale});\nnew icu.LocaleCanonicalizer().canonicalize(locale);\nlocale`,
		funcName: "LocaleCanonicalizer.canonicalize",
		parameters: [
			{
                name: "locale_name",
                type: "string",
                typeUse: "string"
			}
		]
	}
};
