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
