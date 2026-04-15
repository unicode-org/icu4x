# Changelog generation tool


This needs `gh` installed and set up in the ICU4X repo. You may need to run `gh auth login` and `gh repo set-default unicode-org/icu4x`.


First `cargo run -p changelog -- fetch-github --revs <revset> --json /path/to/changelog.json`, giving it an changelog.json to write to, and a git revset to fetch PR info for (e.g. `<hash>..main`). If you wish for it to update data in an existing output.json, pass `--update`. This flag can be used to fix up formatting issues / missing changelog entries that you find without having to refetch data for all of the PRs (which takes a while).

Once you have a `changelog.json`, run `cargo run -p changelog -- make-changelog --json /path/to/changelog.json`. This will produce a nice report with the following sections:

 - "Crates": All of the changelog entries collected per crate. Be sure to go through these, and reorder if necessary
 - "PRs with additional notes": Some PRs have additional notes that were not prefaced by `cratename: foo` or in a bullet point. These might be irrelevant, but they may also give you additional information pertinent to the changelog, e.g. "\<some other PR\> should also be mentioned in the changelog entry"
 - "no changelog found": PRs without changelog entries. Ideally go add them and rerun `fetch-github` with `--update` and just the relevant rev.
 - "Potentially misformatted": Some things the tool might have messed up on. Double check the changelog entries for these in particular
 - "N/A": PRs marked Changelog: N/A. Spot check, and potentially include in the changelog update PR so others can spot check.
 
 