# composition_freq

Tool for computing the relative frequencies of precomposed characters.

The tool wants three arguments:

1. Statistic file: path to local checkout of https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-core/supplemental/territoryInfo.json
2. Corpus directory: path to local checkout of https://github.com/unicode-org/test-corpora/tree/main/gutenberg/Carroll-11/out/google/txt
3. Output .json file.

The output .json file goes to provider/source/data/nfc_statistics.json
