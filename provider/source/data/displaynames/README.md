# Filtered CLDR Misc Data (`coverageByXPath`)

The JSON files in this directory (`coverageByXPath.json` and `coverageByXPath/*.json`) contain CLDR coverage targets by XPath for various locales.

To minimize repository size and compilation overhead, these source JSON files are filtered down to only retain XPaths located under `//ldml/localeDisplayNames`.

***These files are intended to be deleted as soon as the coverage data are added to CLDR JSON.***

## Re-generating these files

These files were generated from a snapshot of https://github.com/unicode-org/cldr/pull/5909.

## Filtering with `jq`

To filter down the JSON files using [`jq`](https://jqlang.github.io/jq/), run the following shell loop:

```bash
for file in coverageByXPath.json coverageByXPath/*.json; do
  jq '.coverageByXPath[][] |= map(select(startswith("//ldml/localeDisplayNames")))' "$file" > "$file.tmp" && mv "$file.tmp" "$file"
done
```

### Explanation of the `jq` expression

- `.coverageByXPath[][]`: Recursively targets all XPath array elements inside `coverageByXPath.<locale>.<tier>`.
- `map(select(startswith("//ldml/localeDisplayNames")))`: Filters each array to keep only XPaths starting with `//ldml/localeDisplayNames`.
