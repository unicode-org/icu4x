window.BENCHMARK_DATA = {
  "lastUpdate": 1629991652023,
  "repoUrl": "https://github.com/unicode-org/icu4x",
  "entries": {
    "Rust Benchmark": [
      {
        "commit": {
          "author": {
            "email": "58569820+nordzilla@users.noreply.github.com",
            "name": "Erik Nordin",
            "username": "nordzilla"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "fbecd88af0c31538e290889c919bdc29620bc402",
          "message": "Implement Time Zone Format (#598)\n\n* Implement Time Zone Formatting\r\n\r\n- Add time-zone input functionality.\r\n- Add zoned datetime functionality.\r\n- Add zoned datetime format functionality.\r\n- Add fixtures tests for zoned datetime format.\r\n- Add benchmarks for zoned datetime format.\r\n- Update examples to use zoned datetime format.\r\n\r\n* Clean Up Localized GMT Offset Formatting\r\n\r\n- Separate the positive/negative offsets in the data provider.\r\n- Use the localized hour formats to produce GMT offset formats.\r\n\r\n* Use CLDR hour-format unconditionally\r\n\r\nThe UTS-35 spec and the CLDR-json formatting have a conflict\r\naround localized GMT formats with regard to zero-padding.\r\n\r\nRead more about our conflict-resolution decisions here:\r\nhttps://docs.google.com/document/d/16GAqaDRS6hzL8jNYjus5MglSevGBflISM-BrIS7bd4A/edit?usp=sharing\r\n\r\n* Add Test For Long/Short Specific Non-Location Formats\r\n\r\n* Refactor how time-zone resource keys are loaded into formatter\r\n\r\n- Constructing a formatter now produces an iterator of required\r\n  resource keys from the pattern, and the keys are loaded accordingly.\r\n\r\n* Refactor zone symbol length matching\r\n\r\n* Implement Exemplar City\r\n\r\n* Add time-zone pattern tests\r\n\r\n* Implement ISO-8601 Time Zone Formats\r\n\r\n* Implement Generic Non-Location Format\r\n\r\n* Implement Generic Location Format\r\n\r\n* Replace todo! with real issue\r\n\r\n* Separate the three main formats into individual files\r\n\r\n* Add test that constructing DateTimeFormat with zones is err\r\n\r\n* Remove unneeded DateTimeError::UnexpectedSymbol\r\n\r\n* Document ISO-8601 format\r\n\r\n* Fix typo in TimeZoneInput documentation\r\n\r\n* Add Underflow error type\r\n\r\n* Reduce time-zone formatting methods to pub(super)\r\n\r\n* Document time-zone formatting helpers\r\n\r\n* Add examples of ISO formats\r\n\r\n* Fix typo\r\n\r\n* Remove debug assertions in favor of const fn.\r\n\r\n- The functions are prviate, and the invariant is maintained by all\r\n  callers within the relevant file. There is no need for assertions.\r\n- We can't have bolth until panicking in const contexts is stabilized.\r\n\r\n* Remove unneeded support for U+2212 (minus sign)\r\n\r\n* Add some newlines for readability\r\n\r\n* Clarify ISO-8106 examples\r\n\r\n* Rename `style` -> `length` after rebase\r\n\r\n* Add missing line at end of file\r\n\r\n* Fix typo in documentation\r\n\r\n* Move DateTimeFormat construction test to `tests` dir\r\n\r\n- This fixes the CI error of testing without default features.\r\n\r\n* Regenerate skeleton test data with time zones\r\n\r\n* Clarify skeleton comments about generating test data\r\n\r\n* Return FieldTooLong error instead of panicking\r\n\r\n- Respond to Zibi's feedback about panicking on field to long by\r\n  returning an error instead.\r\n- Remove the invalid symbol macro.\r\n\r\n* Respond to sffc's review feedback\r\n\r\n- Add TODOs to replace strings with TinyStr\r\n- Remove `country_code()` time-zone input function\r\n- Rename IsoSeconds::None -> IsoSeconds::Never\r\n- Move GmtOffset code to `date.rs`\r\n- Have MockZonedDateTime use MockDateTime for relevant input traits.\r\n- Fix typo \"destionation\" -> \"destination\"\r\n- Load TimeZone resources in-line in `try_new()`\r\n\r\n* Make TimeZoneFormat `pub(super)`\r\n\r\nThis type will be `pub(super)` temporarily until we have a good way\r\nto publicly determine a pattern with which to construct the type.\r\nTrack this issue in #622\r\n\r\n* Move ISO8601 format to timezone.rs\r\n\r\nISO8601 formatting functions now belong to TimeZoneFormat, rather\r\nthan to GmtOffset itself.\r\n\r\n* Rename 's to 'l for `format()` functions.\r\n\r\n* Add the word \"offset\" to gmt offset formatting functions\r\n\r\n* Add documentation to ZonedDateTimeFormat\r\n\r\n* Add documentation for `MockTimeZone` and `MockZonedDateTime`\r\n\r\n* Implement Field trait for TimeZone fields\r\n\r\n* Fix typo\r\n\r\n* Fallback to TextOrNumeric::Text for default pattern matching\r\n\r\nAfter consulting with gregtatum who is implementing the skeleton\r\nmatching algorithms, we determined that Text is a reasonable\r\ndefault fallback.\r\n\r\n* Change TimeZoneFormat formatting functions to write directly\r\n\r\nTimeZoneFormat functions now write directly instaed of returning\r\na string.\r\n\r\n* Rename `FieldTooLong` to `FieldLengthInvalid`\r\n\r\n* Rename other TooLong errors to InvalidLength\r\n\r\n* Run rustfmt\r\n\r\n* Remove clone from Time Zone data structs map access\r\n\r\nThis used to be necessary when the format functions returned a string,\r\nbecause some strings were owned, and others were references,\r\nbut now that the format functions write directly to a buffer,\r\nwe can deal with references as needed and just write them.",
          "timestamp": "2021-04-12T23:13:28-07:00",
          "tree_id": "f24e9f38f06b923e07fe2fcbc39ac96046ecc395",
          "url": "https://github.com/unicode-org/icu4x/commit/fbecd88af0c31538e290889c919bdc29620bc402"
        },
        "date": 1618294850980,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1406537,
            "range": "± 71805",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2656127,
            "range": "± 132874",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "elango@google.com",
            "name": "Elango",
            "username": "echeran"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "076a9194e1d28f1cbbaad6b9ae75969404d54e59",
          "message": "Add comment discouraging use of uniset::props (#627)",
          "timestamp": "2021-04-13T01:19:59-05:00",
          "tree_id": "23a139523c994f11ba25715f8f6f4424a9868110",
          "url": "https://github.com/unicode-org/icu4x/commit/076a9194e1d28f1cbbaad6b9ae75969404d54e59"
        },
        "date": 1618295235058,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1167736,
            "range": "± 55302",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2233875,
            "range": "± 94976",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dminor@mozilla.com",
            "name": "Dan Minor",
            "username": "dminor"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "23c599e2352a8fe8f549053ff26567008a41ed7b",
          "message": "Use cargo-readme to generate README.md files (#601)\n\nUse cargo-readme to generate README.md files",
          "timestamp": "2021-04-13T07:04:58-04:00",
          "tree_id": "bf5602ebe0b0ffd3427543e8f8ced7e64e1b0551",
          "url": "https://github.com/unicode-org/icu4x/commit/23c599e2352a8fe8f549053ff26567008a41ed7b"
        },
        "date": 1618312320038,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1370685,
            "range": "± 39594",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2600149,
            "range": "± 85138",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "gregtatum@users.noreply.github.com",
            "name": "Greg Tatum",
            "username": "gregtatum"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "9fae0980aa20781a223cc0647a7d9d06cdf607b2",
          "message": "Fix the CI because of a components::Bag and Time Zone conflict (#639)",
          "timestamp": "2021-04-13T11:40:58-05:00",
          "tree_id": "98d433bad078190e2310a7d142c15366d82a060b",
          "url": "https://github.com/unicode-org/icu4x/commit/9fae0980aa20781a223cc0647a7d9d06cdf607b2"
        },
        "date": 1618332446590,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1329492,
            "range": "± 89433",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2529285,
            "range": "± 143941",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "zibi@braniecki.net",
            "name": "Zibi Braniecki",
            "username": "zbraniecki"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "180d4e5a43780ab078b462c00dc8f0328b5cc00b",
          "message": "Optimize performance of LocaleCanonicalizer::maximize. (#504)\n\n* Add From<Subtag> for TinyStr.\r\n\r\n* Optimize performance of LocaleCanonicalizer by storing resources in custom data structures.",
          "timestamp": "2021-04-13T10:05:36-07:00",
          "tree_id": "a5a6b61f4fd777a23b2707a291d281b1845fba71",
          "url": "https://github.com/unicode-org/icu4x/commit/180d4e5a43780ab078b462c00dc8f0328b5cc00b"
        },
        "date": 1618333963266,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1398298,
            "range": "± 42270",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2641332,
            "range": "± 103140",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c6d1401e9b965cb457c74c4776ffe8c833f003a7",
          "message": "Updating to CLDR JSON 39.0.0-BETA2 (#640)",
          "timestamp": "2021-04-13T13:19:44-05:00",
          "tree_id": "c600d8d68f585239bff14ac35f1f20125449c25a",
          "url": "https://github.com/unicode-org/icu4x/commit/c6d1401e9b965cb457c74c4776ffe8c833f003a7"
        },
        "date": 1618338416386,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1388980,
            "range": "± 48363",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2630808,
            "range": "± 108513",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "zibi@braniecki.net",
            "name": "Zibi Braniecki",
            "username": "zbraniecki"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "6d99323dc945a09af01d1147477584b469b6fe4b",
          "message": "Mute clippy warnings in a few places, apply the suggestions in remaining cases. (#642)\n\n* Mute clippy warnings in a few places, apply the suggestions in remaining cases.\r\n\r\n* Fix clippy fmt\r\n\r\n* Separate line after license header\r\n\r\n* Remove accidentally added diffs",
          "timestamp": "2021-04-13T11:30:56-07:00",
          "tree_id": "b3ec6302e8f8bfcb8fea56433fbc548249ed7289",
          "url": "https://github.com/unicode-org/icu4x/commit/6d99323dc945a09af01d1147477584b469b6fe4b"
        },
        "date": 1618339058033,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1248950,
            "range": "± 59923",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2394222,
            "range": "± 90697",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "58569820+nordzilla@users.noreply.github.com",
            "name": "Erik Nordin",
            "username": "nordzilla"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "496e0759cf7f6810e248544a6cf964bbbf34425d",
          "message": "Update ICU Component Documentation Examples (#616)\n\n* Update documentation for components/datetime\r\n\r\n* Update documentation for components/locid\r\n\r\n* Update documentation for components/plurals\r\n\r\n* Update documentation for components/uniset\r\n\r\n* Alphabetize Cargo.toml changes\r\n\r\n* Use cargo-readme to generate README files",
          "timestamp": "2021-04-14T12:54:13-07:00",
          "tree_id": "87ef342899e93da76627c071c21f8ad306fa5838",
          "url": "https://github.com/unicode-org/icu4x/commit/496e0759cf7f6810e248544a6cf964bbbf34425d"
        },
        "date": 1618430468998,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1292071,
            "range": "± 16837",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2508766,
            "range": "± 20028",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "zibi@braniecki.net",
            "name": "Zibi Braniecki",
            "username": "zbraniecki"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "a3d0a257cb7588fe912bd8a114e4ebb3fbf41280",
          "message": "Update tinystr to 0.4.5 for perf wins on locid and locale_canonicalizer. (#646)\n\n* Update tinystr to 0.4.4 for perf wins on locid and locale_canonicalizer.\r\n\r\n* Update to tinystr 0.4.5",
          "timestamp": "2021-04-14T21:32:52-07:00",
          "tree_id": "8efe3c114253eafafff86f7ed6df39ab8064ac25",
          "url": "https://github.com/unicode-org/icu4x/commit/a3d0a257cb7588fe912bd8a114e4ebb3fbf41280"
        },
        "date": 1618461584160,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1157115,
            "range": "± 71148",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2277208,
            "range": "± 243521",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "nmihai_2000@yahoo.com",
            "name": "Mihai Nita",
            "username": "mihnita"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c4abd70d98c9240cb9df99453c077b073ee8559a",
          "message": "First iteration of horizontal fallback doc (#629)",
          "timestamp": "2021-04-15T12:06:10-07:00",
          "tree_id": "cea7507f81a6033eb38767a71c6121b36e84db42",
          "url": "https://github.com/unicode-org/icu4x/commit/c4abd70d98c9240cb9df99453c077b073ee8559a"
        },
        "date": 1618513989504,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1309339,
            "range": "± 10099",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2512325,
            "range": "± 8060",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "0b328f8a4cff123fc48416c7a03d0f96cc9c2ab0",
          "message": "Adding ZeroVec, zero-copy vector abstraction over a byte buffer (#647)",
          "timestamp": "2021-04-15T16:41:00-05:00",
          "tree_id": "f3eadaebeff876cfbe022497a6417c1756993eec",
          "url": "https://github.com/unicode-org/icu4x/commit/0b328f8a4cff123fc48416c7a03d0f96cc9c2ab0"
        },
        "date": 1618523268172,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1300822,
            "range": "± 6109",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2502632,
            "range": "± 4209",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "d6e5e2d96bcf62d27d07f0ffe3acff8b82e44e08",
          "message": "Adding Valgrind build task (#631)",
          "timestamp": "2021-04-16T00:56:17-05:00",
          "tree_id": "3dfdb297dd2df7f83d2283f739da22a8d7eeb60d",
          "url": "https://github.com/unicode-org/icu4x/commit/d6e5e2d96bcf62d27d07f0ffe3acff8b82e44e08"
        },
        "date": 1618552995694,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1347025,
            "range": "± 64335",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2522869,
            "range": "± 75806",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c9590b53ef98322b903308ff5b4c57d621222e35",
          "message": "Small cleanups to prepare for data source abstraction (#649)",
          "timestamp": "2021-04-16T19:24:01-05:00",
          "tree_id": "f303e309927a4c804d80979b3bd94c308745d80d",
          "url": "https://github.com/unicode-org/icu4x/commit/c9590b53ef98322b903308ff5b4c57d621222e35"
        },
        "date": 1618619501066,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1654577,
            "range": "± 121502",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 3141427,
            "range": "± 250270",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "manishsmail@gmail.com",
            "name": "Manish Goregaokar",
            "username": "Manishearth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "1479e8caacd2004fd43c127d74a649318e7edded",
          "message": "Add VarZeroVec (#653)\n\n* Add VarZeroVec\r\n\r\n* Move varzerovec into toplevel module\r\n\r\n* Add VarULE docs, remove inline(always)\r\n\r\n* SliceComponents::new() -> SliceComponents::try_from_bytes()\r\n\r\n* Switch from u64 to u32 in VarZeroVec\r\n\r\n* Address zibi's comments\r\n\r\n* Use checked_add",
          "timestamp": "2021-04-16T18:14:42-07:00",
          "tree_id": "a2145111f0d4917468d67952cfef709ae12a18be",
          "url": "https://github.com/unicode-org/icu4x/commit/1479e8caacd2004fd43c127d74a649318e7edded"
        },
        "date": 1618622501122,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1462075,
            "range": "± 3127",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2501586,
            "range": "± 5429",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "58569820+nordzilla@users.noreply.github.com",
            "name": "Erik Nordin",
            "username": "nordzilla"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "179438a061f04e221a2f269980fe0b0762734e66",
          "message": "Clean up terminology and documentation (#654)\n\n* Unify all # Examples section headers\r\n\r\n* Unify references to `datetime` as a single word\r\n\r\n* Unify references to `time zone` as a two words\r\n\r\n* Clean up docs in components/capi\r\n\r\n* Clean up docs in components/datetime\r\n\r\n* Clean up docs in components/decimal\r\n\r\n* Clean up docs in components/ecma402\r\n\r\n* Clean up docs in components/locale_canonicalizer\r\n\r\n* Clean up docs in components/locid\r\n\r\n* Clean up docs in components/plurals\r\n\r\n* Run rustfmt\r\n\r\n* Regenerate README files\r\n\r\n* Clean up docs in components/provider\r\n\r\n* Clean up docs in components/provider_cldr\r\n\r\n* Clean up docs in components/provider_fs\r\n\r\n* Clean up docs in comopnents/uniset\r\n\r\n* Ensure that all linked Rust types have backticks\r\n\r\n* Grep for any links that I missed\r\n\r\n* Reslove all doc warnings\r\n\r\nRuns cargo doc --document-private-items --all-features\r\nand ensures that there are no warnings.\r\n\r\n* Regenerate README files",
          "timestamp": "2021-04-16T18:40:54-07:00",
          "tree_id": "aa95cbacf03f5ad3da422f865aecf34db6a72ab5",
          "url": "https://github.com/unicode-org/icu4x/commit/179438a061f04e221a2f269980fe0b0762734e66"
        },
        "date": 1618624039562,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1187694,
            "range": "± 60160",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2192510,
            "range": "± 111538",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "58569820+nordzilla@users.noreply.github.com",
            "name": "Erik Nordin",
            "username": "nordzilla"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "a46f3336d0a2f0c9c9193fc2e0893c8834e30fcf",
          "message": "Remove .DS_Store files and add them to the .gitignore (#655)",
          "timestamp": "2021-04-16T19:03:34-07:00",
          "tree_id": "1b73a4ca319bae725e66ef371f86f6b667f77836",
          "url": "https://github.com/unicode-org/icu4x/commit/a46f3336d0a2f0c9c9193fc2e0893c8834e30fcf"
        },
        "date": 1618625402997,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1254604,
            "range": "± 23487",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2450961,
            "range": "± 40822",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "d6612af59d103dd49c5f6e2992568edce651c239",
          "message": "Move PpucdDataProvider to experimental (#648)",
          "timestamp": "2021-04-17T00:54:08-05:00",
          "tree_id": "5e22c847305181f89ad70830703fa82f2db1385f",
          "url": "https://github.com/unicode-org/icu4x/commit/d6612af59d103dd49c5f6e2992568edce651c239"
        },
        "date": 1618639247573,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1357298,
            "range": "± 79435",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2627528,
            "range": "± 152104",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "cbf7945f62fff01547db32baf9712810b41dd17d",
          "message": "ZeroVec: More docs and code cleanup (#658)",
          "timestamp": "2021-04-17T10:40:53-05:00",
          "tree_id": "4353bff83b35f3b60736824975689aeee2ff9aa0",
          "url": "https://github.com/unicode-org/icu4x/commit/cbf7945f62fff01547db32baf9712810b41dd17d"
        },
        "date": 1618674481309,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1419840,
            "range": "± 47198",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2717551,
            "range": "± 113117",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "zibi@braniecki.net",
            "name": "Zibi Braniecki",
            "username": "zbraniecki"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ca1d3e3910adc48bdb534df94276ffc1874dcafc",
          "message": "Add icu_pattern util (#581)\n\n* Add icu_pattern util\r\n\r\n* Add PatternIterator trait\r\n\r\n* Apply initial feedback: add allow_quoted_literals, support double quotes, and Vec<E> for simple replacements\r\n\r\n* Add missing license header\r\n\r\n* Switch Interpolator key to be an associated type.\r\n\r\n* Add missing license header\r\n\r\n* Add pre-parsed bench and fix Vec<E> replacements\r\n\r\n* Switch to take &[] in Interpolator.\r\n\r\n* Add examples\r\n\r\n* Add license headers\r\n\r\n* Fix docs\r\n\r\n* Switch Literals to Cow\r\n\r\n* Add ParserOptions\r\n\r\n* Add Example prefix to the main example per Gregs feedback\r\n\r\n* Introduce InterpolatorKind and switch Interpolator to return references.\r\n\r\n* Introduce better ergonomics to wrap Parser/Interpolator under Pattern.\r\n\r\n* Slightly clean up the docs and remove the Cow<str> bound on E\r\n\r\n* Clean up docs a bit more\r\n\r\n* Apply feedback\r\n\r\n* Add docs and extend test coverage\r\n\r\n* Remove strenous manual Eq\r\n\r\n* Generate README.md",
          "timestamp": "2021-04-17T09:53:36-07:00",
          "tree_id": "23bf61670dc60d1223c88c778e833c766f5a8ef3",
          "url": "https://github.com/unicode-org/icu4x/commit/ca1d3e3910adc48bdb534df94276ffc1874dcafc"
        },
        "date": 1618678837347,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1304038,
            "range": "± 25051",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2472666,
            "range": "± 25823",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "manishsmail@gmail.com",
            "name": "Manish Goregaokar",
            "username": "Manishearth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "6be34fd43a9ab2d068009105b3027c52c50367bc",
          "message": "Add metadata fields to icu_benchmark_macros (#656)",
          "timestamp": "2021-04-18T07:18:34-07:00",
          "tree_id": "b41c77151f2f359f92b7f05ef822865c839d5c71",
          "url": "https://github.com/unicode-org/icu4x/commit/6be34fd43a9ab2d068009105b3027c52c50367bc"
        },
        "date": 1618755938772,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1253296,
            "range": "± 42223",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2363498,
            "range": "± 74371",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b58ed80e99cf20e1e2bf48875462910cffadfeee",
          "message": "Cleanups to icu, icu_decimal, and zerovec (#650)\n\n- Adds FixedDecimalFormat to the icu crate\r\n- Documents cargo features used in ICU4X\r\n- Adds benchmarks for VarZeroVec\r\n- Documents benchmark results in zerovec docs",
          "timestamp": "2021-04-19T12:41:43-05:00",
          "tree_id": "9a59f644ca27f0e281b7e6def4114186379c7103",
          "url": "https://github.com/unicode-org/icu4x/commit/b58ed80e99cf20e1e2bf48875462910cffadfeee"
        },
        "date": 1618854489817,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1241605,
            "range": "± 39629",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2391233,
            "range": "± 78092",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "zibi@braniecki.net",
            "name": "Zibi Braniecki",
            "username": "zbraniecki"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "a502f58f0c3529d6aa4bf7fed1f43973f2b4818a",
          "message": "Run `cargo outdated` and update deps (#660)\n\n* cargo outdated\r\n\r\n* Attempt to try to fix rand in test\r\n\r\n* Try to make cargo clippy happy\r\n\r\n* Update BIES test for new rng version\r\n\r\nCo-authored-by: Shane F. Carr <shane@unicode.org>",
          "timestamp": "2021-04-19T13:54:41-07:00",
          "tree_id": "9f08a6f3173a6ad547cb00daa8ca6ed914170b7f",
          "url": "https://github.com/unicode-org/icu4x/commit/a502f58f0c3529d6aa4bf7fed1f43973f2b4818a"
        },
        "date": 1618866063078,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1261060,
            "range": "± 123079",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2271850,
            "range": "± 197655",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "zibi@braniecki.net",
            "name": "Zibi Braniecki",
            "username": "zbraniecki"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "938f943154dc8574a9d6e70c130cea2ee05ed527",
          "message": "Auto and manually apply selected nursery clippies. (#661)",
          "timestamp": "2021-04-20T13:43:52-07:00",
          "tree_id": "32b06d6e7705aafb49f1e9e9a9a4b6840faf451f",
          "url": "https://github.com/unicode-org/icu4x/commit/938f943154dc8574a9d6e70c130cea2ee05ed527"
        },
        "date": 1618951815527,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1298683,
            "range": "± 26399",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2481648,
            "range": "± 19939",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "manishsmail@gmail.com",
            "name": "Manish Goregaokar",
            "username": "Manishearth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "7d78c20bd171cbdf15fd9026bac4e6b2997baa41",
          "message": "Add VarZeroVec::into_owned() (#666)",
          "timestamp": "2021-04-20T19:44:52-07:00",
          "tree_id": "270fd9cdd7909625fe6cbd8305f20bd4df88069f",
          "url": "https://github.com/unicode-org/icu4x/commit/7d78c20bd171cbdf15fd9026bac4e6b2997baa41"
        },
        "date": 1618973523931,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1468500,
            "range": "± 97638",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2772652,
            "range": "± 130586",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "gregtatum@users.noreply.github.com",
            "name": "Greg Tatum",
            "username": "gregtatum"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "6daf09323a50001967fcabd778d96aa9176f3cc5",
          "message": "Make it so that unsupported skeleton fields only emit a warning (#670)",
          "timestamp": "2021-04-22T12:38:53-05:00",
          "tree_id": "68d8c45c4df3696e814f2666ec28c859ec5aabda",
          "url": "https://github.com/unicode-org/icu4x/commit/6daf09323a50001967fcabd778d96aa9176f3cc5"
        },
        "date": 1619113536753,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1292332,
            "range": "± 98524",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2550841,
            "range": "± 159343",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "manishsmail@gmail.com",
            "name": "Manish Goregaokar",
            "username": "Manishearth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "74eb46d37209c0a4200826eaa0322a33bf2462cf",
          "message": "Add ZeroMap (#668)\n\n* Add scaffolding traits for ZeroMap\r\n\r\n* Add ZeroMap methods\r\n\r\n* Add remove(), try_append*)\r\n\r\n* Some basic methods\r\n\r\n* Copy over LiteMap tests\r\n\r\n* Add SerializeType\r\n\r\n* Add iterators\r\n\r\n* Add serde\r\n\r\n* Add docs\r\n\r\n* Use individual macros\r\n\r\n* Add sized-ULE helper functions\r\n\r\n* Make make_mut pub(crate)\r\n\r\n* Error on out-of-order ZeroMap deserializations\r\n\r\n* litemap -> zeromap\r\n\r\n* get_as_ser -> with_ser\r\n\r\n* Replace cmp_two_gets with is_ascending\r\n\r\n* contains_key on needletype\r\n\r\n* Fix comment\r\n\r\n* Add 'static bound to ULE\r\n\r\n* Fix fmt",
          "timestamp": "2021-04-23T10:40:15-07:00",
          "tree_id": "19623df58ab2c09cce9e08d01a97d64642f6956a",
          "url": "https://github.com/unicode-org/icu4x/commit/74eb46d37209c0a4200826eaa0322a33bf2462cf"
        },
        "date": 1619200083104,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1189497,
            "range": "± 44143",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2320598,
            "range": "± 63731",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "manishsmail@gmail.com",
            "name": "Manish Goregaokar",
            "username": "Manishearth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "6030ec10e49651651b5145ede735e9b2a8a9b306",
          "message": "Update CONTRIBUTING.md with review dismiss rules and Conventional Comments (#678)\n\n* Update CONTRIBUTING.md with review dismiss rules and Conventional Comments\r\n\r\n* Mention manual reviewers",
          "timestamp": "2021-04-23T12:13:33-07:00",
          "tree_id": "28a2c66575130c374086e93c08064822949db5af",
          "url": "https://github.com/unicode-org/icu4x/commit/6030ec10e49651651b5145ede735e9b2a8a9b306"
        },
        "date": 1619205601693,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 937072,
            "range": "± 14313",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1799197,
            "range": "± 24081",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "zibi@braniecki.net",
            "name": "Zibi Braniecki",
            "username": "zbraniecki"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "62d541647835d9c8af7cfd5110e8e3973f1105d1",
          "message": "1.0 PRD and Roadmap documents (#665)\n\n* 1.0 PRD and Roadmap documents\r\n\r\n* Apply feedback round 1\r\n\r\n* Apply feedback\r\n\r\n* Apply feedback",
          "timestamp": "2021-04-26T11:28:53-07:00",
          "tree_id": "9a53d1f021355ddb628677d07b4bfb51ded0c9dc",
          "url": "https://github.com/unicode-org/icu4x/commit/62d541647835d9c8af7cfd5110e8e3973f1105d1"
        },
        "date": 1619462142334,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1397160,
            "range": "± 44961",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2400634,
            "range": "± 80742",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "gregtatum@users.noreply.github.com",
            "name": "Greg Tatum",
            "username": "gregtatum"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "04ec2da09a759409019103e3aa7ca65337811cd1",
          "message": "Change the components bag to default to None (#675)",
          "timestamp": "2021-04-26T14:04:45-05:00",
          "tree_id": "57980f5a294511c1540dd4902a0e0a38e1a8d586",
          "url": "https://github.com/unicode-org/icu4x/commit/04ec2da09a759409019103e3aa7ca65337811cd1"
        },
        "date": 1619464332243,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1462538,
            "range": "± 65132",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2753173,
            "range": "± 145230",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "gregtatum@users.noreply.github.com",
            "name": "Greg Tatum",
            "username": "gregtatum"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "9015f7820c614afd288567efb2cd80b1eccd2a2e",
          "message": "Update failing generated-readme-check error message (#641)",
          "timestamp": "2021-04-26T17:07:01-05:00",
          "tree_id": "ad42378f451284a1a1ac7b05308b91f7e8c43b5d",
          "url": "https://github.com/unicode-org/icu4x/commit/9015f7820c614afd288567efb2cd80b1eccd2a2e"
        },
        "date": 1619475246154,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1300993,
            "range": "± 59595",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2513558,
            "range": "± 145992",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "zibi@braniecki.net",
            "name": "Zibi Braniecki",
            "username": "zbraniecki"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b937cfadf78a1f6f4223d6269c25629adef98144",
          "message": "Update Tutorial to 0.2. (#684)\n\n* Update Tutorial to 0.2.\r\n\r\n* Apply feedback",
          "timestamp": "2021-04-27T23:57:53-07:00",
          "tree_id": "1dd98cb6a244623c5e74c9627b1d4096267ff0e8",
          "url": "https://github.com/unicode-org/icu4x/commit/b937cfadf78a1f6f4223d6269c25629adef98144"
        },
        "date": 1619593463454,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1353547,
            "range": "± 25863",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2606832,
            "range": "± 51816",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "fc6315e0c35b57db3fc1b35d80325d320a94a71b",
          "message": "Make docs for provider module consistent (#683)",
          "timestamp": "2021-04-28T11:07:08-05:00",
          "tree_id": "75e31b0bdd06a8a3a7d9e596f5913d1aea38e659",
          "url": "https://github.com/unicode-org/icu4x/commit/fc6315e0c35b57db3fc1b35d80325d320a94a71b"
        },
        "date": 1619626462317,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1367960,
            "range": "± 51763",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2598032,
            "range": "± 75119",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dminor@mozilla.com",
            "name": "Dan Minor",
            "username": "dminor"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "e78289331a514adbce008364653dce2fed9ddced",
          "message": "Fixes to locale canonicalizer docs (#688)\n\n* Fixes to locale canonicalizer docs\r\n\r\n* Address review feedback\r\n\r\n* More review feedback\r\n\r\n* Split expect onto following line",
          "timestamp": "2021-04-28T11:00:46-07:00",
          "tree_id": "753f6ecb603059b65471682f71256d9b01c2a5b4",
          "url": "https://github.com/unicode-org/icu4x/commit/e78289331a514adbce008364653dce2fed9ddced"
        },
        "date": 1619633261222,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1296870,
            "range": "± 15307",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2490380,
            "range": "± 23166",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "gregtatum@users.noreply.github.com",
            "name": "Greg Tatum",
            "username": "gregtatum"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "750dc07d0ee12a2f9d19aa435a4eeb42631f3a79",
          "message": "Hide skeleton docs (#689)",
          "timestamp": "2021-04-28T22:23:15-07:00",
          "tree_id": "a21d21f1f85918ed237cd61bc5c6b5864a45362a",
          "url": "https://github.com/unicode-org/icu4x/commit/750dc07d0ee12a2f9d19aa435a4eeb42631f3a79"
        },
        "date": 1619674249399,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1464399,
            "range": "± 101561",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2731460,
            "range": "± 158245",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "zibi@braniecki.net",
            "name": "Zibi Braniecki",
            "username": "zbraniecki"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "dc4accf2da2eed54d80ba57910cedf1de86cf2d8",
          "message": "icu4x 0.2 (#687)\n\n* icu4x 0.2\r\n\r\n* Release will be on the 28th\r\n\r\n* Update to 29th",
          "timestamp": "2021-04-29T09:12:26-07:00",
          "tree_id": "774c341513a82647d9a467863c5c1d50b0db1456",
          "url": "https://github.com/unicode-org/icu4x/commit/dc4accf2da2eed54d80ba57910cedf1de86cf2d8"
        },
        "date": 1619713134651,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1074469,
            "range": "± 28845",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2074973,
            "range": "± 1725",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "zibi@braniecki.net",
            "name": "Zibi Braniecki",
            "username": "zbraniecki"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "d530470ecafed49dcb41896e48e7f1d9112416b4",
          "message": "Add a 0.2 release blog post (#679)\n\n* Add a 0.2 release blog post\r\n\r\n* Update docs/posts/20210427_ICU4X_02_Release.md\r\n\r\nCo-authored-by: Dan Minor <dminor@mozilla.com>\r\n\r\n* Update docs/posts/20210427_ICU4X_02_Release.md\r\n\r\nCo-authored-by: Dan Minor <dminor@mozilla.com>\r\n\r\n* Update docs/posts/20210427_ICU4X_02_Release.md\r\n\r\nCo-authored-by: Dan Minor <dminor@mozilla.com>\r\n\r\n* Update docs/posts/20210427_ICU4X_02_Release.md\r\n\r\nCo-authored-by: Dan Minor <dminor@mozilla.com>\r\n\r\n* Update docs/posts/20210427_ICU4X_02_Release.md\r\n\r\nCo-authored-by: Dan Minor <dminor@mozilla.com>\r\n\r\n* Update docs/posts/20210427_ICU4X_02_Release.md\r\n\r\nCo-authored-by: Dan Minor <dminor@mozilla.com>\r\n\r\n* Apply feedback\r\n\r\n* Apply feedback\r\n\r\n* Add links to the release post\r\n\r\nCo-authored-by: Dan Minor <dminor@mozilla.com>",
          "timestamp": "2021-04-29T09:24:15-07:00",
          "tree_id": "8f0947c7bab7be88803a58d89a0d2df36e657fe5",
          "url": "https://github.com/unicode-org/icu4x/commit/d530470ecafed49dcb41896e48e7f1d9112416b4"
        },
        "date": 1619713917774,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1339753,
            "range": "± 51995",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2548170,
            "range": "± 114168",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "zibi@braniecki.net",
            "name": "Zibi Braniecki",
            "username": "zbraniecki"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "3b6d7784c2a8d7a7a3eb02878a342a1b21709ac0",
          "message": "Add ownership command for crates.io (#690)",
          "timestamp": "2021-04-29T09:39:07-07:00",
          "tree_id": "d01b120141270d38b6149b3ac18b656d83325b5b",
          "url": "https://github.com/unicode-org/icu4x/commit/3b6d7784c2a8d7a7a3eb02878a342a1b21709ac0"
        },
        "date": 1619714800600,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1209606,
            "range": "± 66919",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2272875,
            "range": "± 126710",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "zibi@braniecki.net",
            "name": "Zibi Braniecki",
            "username": "zbraniecki"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "377ba0e6c90280d1dfa6973fce643c439b021c3b",
          "message": "Add benches to includes (#692)",
          "timestamp": "2021-04-29T11:13:48-07:00",
          "tree_id": "bac686a8b23c22d990facc3d850b58bfc0429962",
          "url": "https://github.com/unicode-org/icu4x/commit/377ba0e6c90280d1dfa6973fce643c439b021c3b"
        },
        "date": 1619720386087,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 955243,
            "range": "± 49407",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1838102,
            "range": "± 17929",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "zibi@braniecki.net",
            "name": "Zibi Braniecki",
            "username": "zbraniecki"
          },
          "committer": {
            "email": "zibi@braniecki.net",
            "name": "Zibi Braniecki",
            "username": "zbraniecki"
          },
          "distinct": true,
          "id": "20fc4c5a5936e8d344c67236541911b4913ef6f0",
          "message": "Add description to locale_canonicalizer",
          "timestamp": "2021-04-29T11:17:03-07:00",
          "tree_id": "1237ac82239a5a30d3e9eb81459105f8d6466119",
          "url": "https://github.com/unicode-org/icu4x/commit/20fc4c5a5936e8d344c67236541911b4913ef6f0"
        },
        "date": 1619720628294,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1200178,
            "range": "± 97752",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2395761,
            "range": "± 205107",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "zibi@braniecki.net",
            "name": "Zibi Braniecki",
            "username": "zbraniecki"
          },
          "committer": {
            "email": "zibi@braniecki.net",
            "name": "Zibi Braniecki",
            "username": "zbraniecki"
          },
          "distinct": true,
          "id": "8e93c88897cdd086c49524ef05c7f383a885d804",
          "message": "Add description to decimal",
          "timestamp": "2021-04-29T11:30:16-07:00",
          "tree_id": "a163e1859f11737ea095d198c3ad7bbc22fec00b",
          "url": "https://github.com/unicode-org/icu4x/commit/8e93c88897cdd086c49524ef05c7f383a885d804"
        },
        "date": 1619721449852,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1298580,
            "range": "± 3429",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2480048,
            "range": "± 6313",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "zibi@braniecki.net",
            "name": "Zibi Braniecki",
            "username": "zbraniecki"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "3a458106e62befb111b54fd62014b5b712fc2e8c",
          "message": "Update README.md to 0.2 (#691)",
          "timestamp": "2021-04-29T13:11:07-07:00",
          "tree_id": "4399d22e228d2353940543901bd575f1890e658f",
          "url": "https://github.com/unicode-org/icu4x/commit/3a458106e62befb111b54fd62014b5b712fc2e8c"
        },
        "date": 1619727472332,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1185750,
            "range": "± 53842",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2420249,
            "range": "± 119657",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "zibi@braniecki.net",
            "name": "Zibi Braniecki",
            "username": "zbraniecki"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "478c4a901faca10195edce477b583ed31f8fdd1f",
          "message": "Apply feedback on 0.2 blog post from Mark",
          "timestamp": "2021-04-29T15:40:30-07:00",
          "tree_id": "7547d7a22a86db6021925770049cef13a4159efb",
          "url": "https://github.com/unicode-org/icu4x/commit/478c4a901faca10195edce477b583ed31f8fdd1f"
        },
        "date": 1619736460602,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1320469,
            "range": "± 72296",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2499077,
            "range": "± 93259",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "manishsmail@gmail.com",
            "name": "Manish Goregaokar",
            "username": "Manishearth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "2ff1a07ff1608be5755a40abf96d076b60673195",
          "message": "Add FFI for FixedDecimalFormat (#680)\n\n* Add CAPI bindings for fixed_decimal\r\n\r\n* Add decimals, macro for c-enums\r\n\r\n* Add ICU4XCustomWriteable\r\n\r\n* Add FixedDecimalFormat::format()\r\n\r\n* Add c headers\r\n\r\n* Add fixeddecimal example\r\n\r\n* more comments on custom_writeable\r\n\r\n* Update components/capi/src/custom_writeable.rs\r\n\r\nCo-authored-by: Shane F. Carr <shane@unicode.org>\r\n\r\n* Address review comments\r\n\r\n* Update components/capi/src/custom_writeable.rs\r\n\r\nCo-authored-by: Shane F. Carr <shane@unicode.org>\r\n\r\n* Writeable docs\r\n\r\n* Improve docs\r\n\r\n* add license\r\n\r\n* more review fixes\r\n\r\n* pass ICU4XCustomWriteable to its callbacks\r\n\r\n* Add FixedDecimal::multiply_pow10()\r\n\r\n* Update components/capi/src/custom_writeable.rs\r\n\r\nCo-authored-by: Shane F. Carr <shane@unicode.org>\r\n\r\n* More review fixes\r\n\r\n* add license\r\n\r\n* Lint fixes\r\n\r\n* Use references in FFI function\r\n\r\n* ICU4XCustomWriteable -> ICU4XWriteable\r\n\r\n* Add negate()\r\n\r\n* Fix label in test\r\n\r\n* fmt\r\n\r\nCo-authored-by: Shane F. Carr <shane@unicode.org>",
          "timestamp": "2021-05-04T15:14:51-07:00",
          "tree_id": "5485b797e54f9621abb77f211d2868f3693483e5",
          "url": "https://github.com/unicode-org/icu4x/commit/2ff1a07ff1608be5755a40abf96d076b60673195"
        },
        "date": 1620166871475,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1233133,
            "range": "± 66743",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2386923,
            "range": "± 132793",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ee4fce450e4164456f45d1e92d8f5e6c79c91d03",
          "message": "Updating to CLDR 39.0.0 (#695)",
          "timestamp": "2021-05-04T23:06:09-05:00",
          "tree_id": "81533fc1eb0f006e6923b4f6b7c0b8b425199eae",
          "url": "https://github.com/unicode-org/icu4x/commit/ee4fce450e4164456f45d1e92d8f5e6c79c91d03"
        },
        "date": 1620187972124,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1166544,
            "range": "± 88908",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2208546,
            "range": "± 151877",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "zibi@braniecki.net",
            "name": "Zibi Braniecki",
            "username": "zbraniecki"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c6582ccddf87d49a44e2d4e54604543365c76bff",
          "message": "Switch to use thiserror for errors (#697)\n\n* Switch to use thiserror for errors\r\n\r\n* Update components/datetime/src/error.rs\r\n\r\nCo-authored-by: Shane F. Carr <sffc@google.com>\r\n\r\n* Switch to from, since transparent and source are a no go together.\r\n\r\nCo-authored-by: Shane F. Carr <sffc@google.com>",
          "timestamp": "2021-05-04T22:51:18-07:00",
          "tree_id": "04cd7b21c0788e88cb41cda04f4795f74a2edc6a",
          "url": "https://github.com/unicode-org/icu4x/commit/c6582ccddf87d49a44e2d4e54604543365c76bff"
        },
        "date": 1620194310513,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1343236,
            "range": "± 71491",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2695251,
            "range": "± 134661",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "d0f4879f5ab94dca5494925c0a9fd0c44eb30f2a",
          "message": "Discussion about correctness, performance, and style (#701)",
          "timestamp": "2021-05-05T14:37:36-05:00",
          "tree_id": "ab5169a90ccd68e8463d034a4b324f2f5ab3326c",
          "url": "https://github.com/unicode-org/icu4x/commit/d0f4879f5ab94dca5494925c0a9fd0c44eb30f2a"
        },
        "date": 1620243915527,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1572154,
            "range": "± 91566",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2971351,
            "range": "± 119629",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "3085cc6dbe3018a4029c82f10b0f6452fde949fd",
          "message": "Add GitHub workflow for documentation preview (#694)",
          "timestamp": "2021-05-07T12:30:38-05:00",
          "tree_id": "c22f52d0d9c1e4495fb9dc3cf4a52d0205745e69",
          "url": "https://github.com/unicode-org/icu4x/commit/3085cc6dbe3018a4029c82f10b0f6452fde949fd"
        },
        "date": 1620409019745,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1332429,
            "range": "± 79698",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2581171,
            "range": "± 127157",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "elango@google.com",
            "name": "Elango",
            "username": "echeran"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "1fa93b46cf574d5b61ecb9128e9a5292d3549375",
          "message": "Add PR template, including reminder for Conventional Comments (#706)",
          "timestamp": "2021-05-07T10:34:48-07:00",
          "tree_id": "77e4b32e1ab092625600987a947dc96bef105d7b",
          "url": "https://github.com/unicode-org/icu4x/commit/1fa93b46cf574d5b61ecb9128e9a5292d3549375"
        },
        "date": 1620409328439,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1423375,
            "range": "± 135123",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2726297,
            "range": "± 446678",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "8a91a7c33f5bb56f52c198161a4a4c3a117583a9",
          "message": "Require GCP key to build docs preview (#708)",
          "timestamp": "2021-05-08T13:19:59-05:00",
          "tree_id": "0595fdc51c3a25f889e671b09e13dd8facc63db6",
          "url": "https://github.com/unicode-org/icu4x/commit/8a91a7c33f5bb56f52c198161a4a4c3a117583a9"
        },
        "date": 1620498386077,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1370411,
            "range": "± 57211",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2353615,
            "range": "± 77393",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "manishsmail@gmail.com",
            "name": "Manish Goregaokar",
            "username": "Manishearth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "82ace5ec626d683e1ec3a5783d2a8c9125aada2b",
          "message": "Add yoke crate (#705)\n\n* Add yoke crate\n\n* Fix lint errors\n\n* Fix lint again\n\n* Clarifications for Miguel\n\n* Address shane's comments\n\n* Swap Yokeable::new()\n\n* more fmt\n\n* mention lifetime\n\n* Add Cartable for references\n\n* Remove unwrapping Cart impl on Option, move to having more attach methods\n\n* Reorganize yoke code\n\n* Use ptr::read instead of transmute_copy\n\n* Use StableDeref instead of Cart\n\n* Update utils/yoke/src/yoke.rs\n\nCo-authored-by: Shane F. Carr <shane@unicode.org>\n\n* Add more Clone impls\n\n* Move zerovec yoke impls to zerovec\n\n* lint\n\nCo-authored-by: Shane F. Carr <shane@unicode.org>",
          "timestamp": "2021-05-10T20:42:22-07:00",
          "tree_id": "e02ecb0625f89ebddbf45f9d4c6a4dd04c56fb79",
          "url": "https://github.com/unicode-org/icu4x/commit/82ace5ec626d683e1ec3a5783d2a8c9125aada2b"
        },
        "date": 1620704931001,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1454694,
            "range": "± 72220",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2755929,
            "range": "± 132999",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "manishsmail@gmail.com",
            "name": "Manish Goregaokar",
            "username": "Manishearth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "51e1e72b8a6a4d2df0221e8f617e3d9a6819b815",
          "message": "Fix keyword in Cargo.toml (#714)\n\n* Fix keyword in Cargo.toml\r\n\r\n* Update zerovec crate",
          "timestamp": "2021-05-11T10:19:50-07:00",
          "tree_id": "87f47eb5a6b8194bcf66add1ca291a2fd29b58ba",
          "url": "https://github.com/unicode-org/icu4x/commit/51e1e72b8a6a4d2df0221e8f617e3d9a6819b815"
        },
        "date": 1620754019553,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1403714,
            "range": "± 31574",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2680672,
            "range": "± 51852",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "manishsmail@gmail.com",
            "name": "Manish Goregaokar",
            "username": "Manishearth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "edd710df243c1d7d40660600d9ba2c6cbf7d7153",
          "message": "Add docs about optional features to zerovec crate, enable on docs.rs (#715)\n\n* Add docs about optional features to zerovec crate, enable on docs.rs\r\n\r\n* Fix whitespace in yokeable docs",
          "timestamp": "2021-05-11T12:35:26-07:00",
          "tree_id": "b9138b4b6baa4fd3d7837466905243697898a07f",
          "url": "https://github.com/unicode-org/icu4x/commit/edd710df243c1d7d40660600d9ba2c6cbf7d7153"
        },
        "date": 1620762137578,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1422993,
            "range": "± 88997",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2689934,
            "range": "± 102926",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "9f897fb14dc1ed8093e152819427b98cdeddc657",
          "message": "Re-organize ICU4X data generation tools (#704)\n\n- Adds new crate under `tools/datagen`\r\n- Moves `icu4x-cldr-export` into that crate, renamed as `icu4x-datagen`\r\n- Replaces `icu4x-gen-testdata` with a new tool `icu4x-testdata-download`\r\n- Adds commands in Makefile.toml to invoke these tools",
          "timestamp": "2021-05-12T14:55:57-05:00",
          "tree_id": "b1614a6467b59fab9c68620bae91ffd7d63d6c08",
          "url": "https://github.com/unicode-org/icu4x/commit/9f897fb14dc1ed8093e152819427b98cdeddc657"
        },
        "date": 1620849736471,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1099897,
            "range": "± 3128",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2094689,
            "range": "± 2742",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "elango@google.com",
            "name": "Elango",
            "username": "echeran"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "2b8ce7ebaf43e01c473317339cbcb2dbfa0f56fe",
          "message": "Exclude CI jobs from fork runs that are intended for upstream merges (#672)",
          "timestamp": "2021-05-13T10:15:06-07:00",
          "tree_id": "ec4b73fe682ebe6198e017f608e06d2e4e26a030",
          "url": "https://github.com/unicode-org/icu4x/commit/2b8ce7ebaf43e01c473317339cbcb2dbfa0f56fe"
        },
        "date": 1620926517341,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1333754,
            "range": "± 91308",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2429292,
            "range": "± 39553",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "579f8253d595763908a0eb2ae63b1f18f941e844",
          "message": "Build docs previews in forks with valid key (#709)",
          "timestamp": "2021-05-13T12:17:51-05:00",
          "tree_id": "685a41233951b588f3312408c4a0bf26eb4da01b",
          "url": "https://github.com/unicode-org/icu4x/commit/579f8253d595763908a0eb2ae63b1f18f941e844"
        },
        "date": 1620926692376,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1268109,
            "range": "± 63823",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2376360,
            "range": "± 129358",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "manishsmail@gmail.com",
            "name": "Manish Goregaokar",
            "username": "Manishearth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "0eb7f7bb9bfd6ec45e01258f44bdfdc15d6e5f6a",
          "message": "Fix clippy in zerovec (#719)",
          "timestamp": "2021-05-13T11:58:35-07:00",
          "tree_id": "95a7fef0c65da10068a00f7237da3bf750386fde",
          "url": "https://github.com/unicode-org/icu4x/commit/0eb7f7bb9bfd6ec45e01258f44bdfdc15d6e5f6a"
        },
        "date": 1620932709672,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1290805,
            "range": "± 5379",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2476177,
            "range": "± 32320",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "a8599e4e3f972b3120887252d290ea6e750cbf60",
          "message": "Rename package directories in repo (#721)",
          "timestamp": "2021-05-13T16:00:29-07:00",
          "tree_id": "803a86066a5a41a378a5b2ec01d4968d40b75133",
          "url": "https://github.com/unicode-org/icu4x/commit/a8599e4e3f972b3120887252d290ea6e750cbf60"
        },
        "date": 1620947190707,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1083244,
            "range": "± 31557",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2031596,
            "range": "± 79393",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "0400ad1de102a0774618859ebee214d799b65e40",
          "message": "Refactoring and generalizing impl_dyn_provider! (#723)",
          "timestamp": "2021-05-14T15:41:26-05:00",
          "tree_id": "4a428a443677c4b025c94ba58465e75ecb3865dd",
          "url": "https://github.com/unicode-org/icu4x/commit/0400ad1de102a0774618859ebee214d799b65e40"
        },
        "date": 1621025280774,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1390969,
            "range": "± 35182",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2635012,
            "range": "± 72018",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "1f4a9505f21a6d5c9bb4833e0cf3fe969f734c54",
          "message": "Touch-ups to Cargo.toml files and features (#722)",
          "timestamp": "2021-05-14T18:45:29-05:00",
          "tree_id": "d29be99abbf6b97a6acf945ab3d3feeb0f128558",
          "url": "https://github.com/unicode-org/icu4x/commit/1f4a9505f21a6d5c9bb4833e0cf3fe969f734c54"
        },
        "date": 1621036349642,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1318343,
            "range": "± 62897",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2531404,
            "range": "± 123375",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "manishsmail@gmail.com",
            "name": "Manish Goregaokar",
            "username": "Manishearth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "d68636ac9e319f7ee1c784e8bb83b6313f3f6ae5",
          "message": "Add rust versions policy (#726)\n\n* Add rust versions policy\r\n\r\n* +readme",
          "timestamp": "2021-05-19T14:06:04-07:00",
          "tree_id": "474950468811fce0efd1875b66b313675f033cde",
          "url": "https://github.com/unicode-org/icu4x/commit/d68636ac9e319f7ee1c784e8bb83b6313f3f6ae5"
        },
        "date": 1621458819790,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1290827,
            "range": "± 20613",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2422863,
            "range": "± 31104",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "bacbc93be0130f80f74f1d58bab6c3316041ec14",
          "message": "Document release readiness and experimental code (#700)",
          "timestamp": "2021-05-20T19:05:05-05:00",
          "tree_id": "0ccb405934215df26681b871f5204c949b4099db",
          "url": "https://github.com/unicode-org/icu4x/commit/bacbc93be0130f80f74f1d58bab6c3316041ec14"
        },
        "date": 1621555913902,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1252606,
            "range": "± 42877",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2382710,
            "range": "± 29215",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "elango@google.com",
            "name": "Elango",
            "username": "echeran"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "48c47db44cace4a608b627f6065de1dd39aac166",
          "message": "Make clippy warnings report as errors and update clippy args (#712)",
          "timestamp": "2021-05-20T17:21:57-07:00",
          "tree_id": "e16af60d9b2284d5943f4f01b26b3a156f27aa36",
          "url": "https://github.com/unicode-org/icu4x/commit/48c47db44cace4a608b627f6065de1dd39aac166"
        },
        "date": 1621556926931,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1296272,
            "range": "± 15213",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2483958,
            "range": "± 5716",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5fa630f86b31880cee7000793467d5ab9fd5abc6",
          "message": "Store DataPayload instead of Cow throughout ICU4X (#729)",
          "timestamp": "2021-05-21T01:02:44-05:00",
          "tree_id": "8f998589d9317d31432f0563cfba4936addb3260",
          "url": "https://github.com/unicode-org/icu4x/commit/5fa630f86b31880cee7000793467d5ab9fd5abc6"
        },
        "date": 1621577311108,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1182890,
            "range": "± 77303",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2205112,
            "range": "± 142832",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ca7cf884d164da7472741e81291309f06d80b484",
          "message": "Fix cargo make wasm (#728)",
          "timestamp": "2021-05-20T23:36:22-07:00",
          "tree_id": "954421693b34500a4fa26fb746d4e34ae2977c1b",
          "url": "https://github.com/unicode-org/icu4x/commit/ca7cf884d164da7472741e81291309f06d80b484"
        },
        "date": 1621579440256,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1596589,
            "range": "± 127145",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 3081314,
            "range": "± 268539",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "eb112c397b37a57a551f0c972b9f7220a5177738",
          "message": "Make cow field of DataPayload crate-private",
          "timestamp": "2021-05-24T11:27:59-05:00",
          "tree_id": "2d08a0ca7e40b2f2ae643a70b4eb5e5efe1b0370",
          "url": "https://github.com/unicode-org/icu4x/commit/eb112c397b37a57a551f0c972b9f7220a5177738"
        },
        "date": 1621874094380,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1225169,
            "range": "± 22302",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2409793,
            "range": "± 36815",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "aethanyc@gmail.com",
            "name": "Ting-Yu Lin",
            "username": "aethanyc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "91e2fd60b5c1bba6439a798a18758c6327df2785",
          "message": "Properly initialize ICU4XPluralOperands struct in the capi example (#733)\n\nOther fields in ICU4XPluralOperands are meant to be zero.\r\n\r\nFixed #732.",
          "timestamp": "2021-05-24T12:16:25-07:00",
          "tree_id": "e3fa3af7129e97fd067eec6a5b87c6d2bf102247",
          "url": "https://github.com/unicode-org/icu4x/commit/91e2fd60b5c1bba6439a798a18758c6327df2785"
        },
        "date": 1621884193329,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1315488,
            "range": "± 69694",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2426804,
            "range": "± 20437",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dminor@mozilla.com",
            "name": "Dan Minor",
            "username": "dminor"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "65863887250dc86947386299ad64210f478a36b7",
          "message": "Add include to capi (#738)",
          "timestamp": "2021-05-27T09:22:29-07:00",
          "tree_id": "2be80e2fc4f62b1a15d6f52bb1a59cde322d0abf",
          "url": "https://github.com/unicode-org/icu4x/commit/65863887250dc86947386299ad64210f478a36b7"
        },
        "date": 1622132935402,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1262906,
            "range": "± 27390",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2404919,
            "range": "± 56714",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "958ee68fa4f88fd740bb02927674871f1565dba2",
          "message": "Replace DataPayload Deref with .get() throughout ICU4X (#739)",
          "timestamp": "2021-05-27T20:08:24-05:00",
          "tree_id": "12c3ff9d23b6122cfc28cf0d798f22b89dc01341",
          "url": "https://github.com/unicode-org/icu4x/commit/958ee68fa4f88fd740bb02927674871f1565dba2"
        },
        "date": 1622164468007,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1048235,
            "range": "± 54132",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1997695,
            "range": "± 77222",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "aethanyc@gmail.com",
            "name": "Ting-Yu Lin",
            "username": "aethanyc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "83e10131e36e5cc264f3ec800cdeff6c94b2cf71",
          "message": "Add a line breaker prototype by importing uax14_rs (#717)\n\nThis commit is adapted from uax14_rs's master branch on commit\r\nb9607c520d798effba67a3fb2a0fb0995543b59e with necessary fix to pass\r\nicu4x's CI.",
          "timestamp": "2021-06-01T14:11:47-07:00",
          "tree_id": "5faa37604d9c4d60058f6f237af10d111f75e424",
          "url": "https://github.com/unicode-org/icu4x/commit/83e10131e36e5cc264f3ec800cdeff6c94b2cf71"
        },
        "date": 1622582280237,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1072143,
            "range": "± 22638",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2065075,
            "range": "± 7136",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "shadaj@users.noreply.github.com",
            "name": "Shadaj Laddad",
            "username": "shadaj"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "a7d087c9831e9135acb235e4fc46fd6c3596452d",
          "message": "Initial build infrastructure and example for WASM FFI (#748)\n\n* [WIP] Prototype using ICU4X from JS through WASM\r\n\r\n* Support converting a fixed decimal to a string in WASM\r\n\r\n* Prototype high-level API\r\n\r\n* Add example of returning struct from WASM to JS\r\n\r\n* Directly create views of WASM memory\r\n\r\n* Set up build for separate dev and release WASM modes\r\n\r\n* Clean up example\r\n\r\n* Add docs for new FFI functions\r\n\r\n* Run cargo fmt and address clippy warnings\r\n\r\n* Update ffi/capi/src/fixed_decimal.rs\r\n\r\nCo-authored-by: Manish Goregaokar <manishsmail@gmail.com>\r\n\r\n* Address feedback\r\n\r\n* Fix lint and clippy\r\n\r\n* Cleanup docs\r\n\r\nCo-authored-by: Manish Goregaokar <manishsmail@gmail.com>",
          "timestamp": "2021-06-02T15:10:32-07:00",
          "tree_id": "f7a839085ccffcf7fa439de47807025a7bae00d2",
          "url": "https://github.com/unicode-org/icu4x/commit/a7d087c9831e9135acb235e4fc46fd6c3596452d"
        },
        "date": 1622672212841,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1274330,
            "range": "± 9598",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2424461,
            "range": "± 23857",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "58569820+nordzilla@users.noreply.github.com",
            "name": "Erik Nordin",
            "username": "nordzilla"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c9b8b626f5b8ef58448e6b695a4473ce2b172168",
          "message": "Use TinyStr8 for time-zone variant identifiers (#750)\n\n* Use TinyStr8 for time-zone variants\r\n\r\n- Update `TimeZoneFormatsV1.region_format_variants` from `str` to\r\n  `TinyStr8`.\r\n- Update `MetaZoneSpecificNamesV1` key form `str` to `TinyStr8`.\r\n- Update `map_access` macro to take key types of `str` or `TinyStr8`.\r\n\r\nThe `TinyStr8` impl seems to serialize the same as the `str` impl (for\r\nJSON), so no changes are expected in the test data.\r\n\r\nI did run `cargo make testdata` to be certain.\r\n\r\n* Fix clippy warnings\r\n\r\n* Respond to feedback from zbraniecki",
          "timestamp": "2021-06-02T15:34:58-07:00",
          "tree_id": "2f4b513adc953db1f28f22cc37878638268cdbb5",
          "url": "https://github.com/unicode-org/icu4x/commit/c9b8b626f5b8ef58448e6b695a4473ce2b172168"
        },
        "date": 1622673676916,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1401939,
            "range": "± 35700",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2597358,
            "range": "± 92055",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "manishsmail@gmail.com",
            "name": "Manish Goregaokar",
            "username": "Manishearth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "823c497bc9854ceae5f20d46890117d76aea9af3",
          "message": "Add quick-and-dirty static data provider for WASM testing (#759)\n\n* Add quick-and-dirty static data provider for WASM testing\r\n\r\n* small amount of docs\r\n\r\n* Review fixes\r\n\r\n* ci fixes\r\n\r\n* more ci",
          "timestamp": "2021-06-03T17:20:50-07:00",
          "tree_id": "015200164d516fabb81310fcfb970afce5188874",
          "url": "https://github.com/unicode-org/icu4x/commit/823c497bc9854ceae5f20d46890117d76aea9af3"
        },
        "date": 1622766411954,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1190997,
            "range": "± 64039",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2243698,
            "range": "± 144191",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "dc8e5416de0febccbb9c09971bb52111c45f022e",
          "message": "Migrate DataPayload to Yoke (#745)\n\n- Adds new DataMarker trait and migrates all data structs to it",
          "timestamp": "2021-06-04T15:41:37-05:00",
          "tree_id": "e94e9f15867212d4092e72ef18064582bab92166",
          "url": "https://github.com/unicode-org/icu4x/commit/dc8e5416de0febccbb9c09971bb52111c45f022e"
        },
        "date": 1622839639024,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1188461,
            "range": "± 7006",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2292527,
            "range": "± 4109",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "manishsmail@gmail.com",
            "name": "Manish Goregaokar",
            "username": "Manishearth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "9ec4ae2608e9095313acb56ca404c793581d7ba4",
          "message": "Add Yoke::new_always_owned (#762)\n\n* Add Yoke::new_always_owned\r\n\r\n* lint",
          "timestamp": "2021-06-04T13:46:19-07:00",
          "tree_id": "e1b00211baeb42f529189e9bb64420f16069b399",
          "url": "https://github.com/unicode-org/icu4x/commit/9ec4ae2608e9095313acb56ca404c793581d7ba4"
        },
        "date": 1622839928199,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1178966,
            "range": "± 2687",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2279564,
            "range": "± 3503",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "manishsmail@gmail.com",
            "name": "Manish Goregaokar",
            "username": "Manishearth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "3420be93fb29c6adf7c21e17f1c2eae8beaf51aa",
          "message": "Switch DataPayload::Owned over to using Yoke::new_always_owned (#765)\n\n* Switch DataPayload::Owned over to using Yoke::new_always_owned\n\n* lint",
          "timestamp": "2021-06-04T15:54:08-07:00",
          "tree_id": "6519e2a4b76a53771d6cb784715bd2ff0164265f",
          "url": "https://github.com/unicode-org/icu4x/commit/3420be93fb29c6adf7c21e17f1c2eae8beaf51aa"
        },
        "date": 1622847619762,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1492184,
            "range": "± 129918",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2794438,
            "range": "± 286046",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dminor@mozilla.com",
            "name": "Dan Minor",
            "username": "dminor"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ffd520f415d7cb58d88401641b9f55c421c5d845",
          "message": "Add canonicalize method to LocaleCanonicalizer (#747)\n\nAdd canonicalize method to LocaleCanonicalizer",
          "timestamp": "2021-06-07T15:13:17-04:00",
          "tree_id": "b264324a40acf3466d7a0d9d07e1c4e8f87dfe6d",
          "url": "https://github.com/unicode-org/icu4x/commit/ffd520f415d7cb58d88401641b9f55c421c5d845"
        },
        "date": 1623093622083,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1420610,
            "range": "± 98922",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2802957,
            "range": "± 206568",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dminor@mozilla.com",
            "name": "Dan Minor",
            "username": "dminor"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "6796b23271037d413a6b8706262cb68a9a7a3f76",
          "message": "Add icu4x_locale_tostring and example for locale (#764)\n\n* Fix some trailing whitespace\r\n\r\n* Add icu4x_locale_tostring and example for locale\r\n\r\nThe example will be expanded to demonstrate locale canonicalization as\r\nwell.\r\n\r\nPartial fix for #757.",
          "timestamp": "2021-06-07T23:12:03-07:00",
          "tree_id": "fce991c25e938c96f3015c3ec4fddf22d96bcfa8",
          "url": "https://github.com/unicode-org/icu4x/commit/6796b23271037d413a6b8706262cb68a9a7a3f76"
        },
        "date": 1623133136286,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1410720,
            "range": "± 71697",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2793549,
            "range": "± 192266",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "shadaj@users.noreply.github.com",
            "name": "Shadaj Laddad",
            "username": "shadaj"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "25b82a196e7f5cb77a951509e67233f48388064b",
          "message": "Add logic for parsing structs into JS values with RTTI definitions (#755)\n\n* Setup RTTI and add example\r\n\r\n* Move WASM example under FFI folder\r\n\r\n* Run cargo fmt\r\n\r\n* Set up simple WASM tests\r\n\r\n* Set up CI for WASM tests\r\n\r\n* Install the nightly toolchain\r\n\r\n* Add step to install WASM tools\r\n\r\n* Install NPM dependencies before running tests\r\n\r\n* Fix indentation in package.json\r\n\r\n* Address feedback\r\n\r\n* Run cargo fmt\r\n\r\n* Remove multiply_pow10 detailed error and backport RTTI fixes\r\n\r\n* Throw an error instead of returning a boolean",
          "timestamp": "2021-06-08T13:59:35-07:00",
          "tree_id": "1d8ebfc547bd5e35b7c44de276c3a09c8a927b48",
          "url": "https://github.com/unicode-org/icu4x/commit/25b82a196e7f5cb77a951509e67233f48388064b"
        },
        "date": 1623186325246,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1067325,
            "range": "± 64007",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2265057,
            "range": "± 125894",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "manishsmail@gmail.com",
            "name": "Manish Goregaokar",
            "username": "Manishearth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "2a47704e5e3d8fd276de2a530feceb43e74d5a61",
          "message": "Switch static data provider to using bincode to reduce heap footprint (#775)",
          "timestamp": "2021-06-08T17:51:51-05:00",
          "tree_id": "c0f9f660e5e9b44caa1eaadcb6c7d34874d59c42",
          "url": "https://github.com/unicode-org/icu4x/commit/2a47704e5e3d8fd276de2a530feceb43e74d5a61"
        },
        "date": 1623193064229,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1175489,
            "range": "± 15967",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2268473,
            "range": "± 4115",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "zibi@braniecki.net",
            "name": "Zibi Braniecki",
            "username": "zbraniecki"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "500e458fbb8d5f0295f5f2baf3b7af10f937cd21",
          "message": "Clarify `top level` in testdata README (#773)\n\n* Clarify `top level` in testdata README\r\n\r\n* Match lib.rs with README",
          "timestamp": "2021-06-08T17:24:07-07:00",
          "tree_id": "53c70b71af41aff3dba5f5dfb90519354efef8a7",
          "url": "https://github.com/unicode-org/icu4x/commit/500e458fbb8d5f0295f5f2baf3b7af10f937cd21"
        },
        "date": 1623198641111,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1468725,
            "range": "± 4357",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2840788,
            "range": "± 4329",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "manishsmail@gmail.com",
            "name": "Manish Goregaokar",
            "username": "Manishearth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "fe95051ddc98e77147ff3d48e2fe763ee059167e",
          "message": "Add basic C++ API for PluralRules and FixedDecimal (#768)\n\n* Add basic C++ bindings for PluralRules\r\n\r\n* review comments\r\n\r\n* Add FixedDecimalFormat\r\n\r\n* + fixeddecimal test\r\n\r\n* improve organization of makefile\r\n\r\n* + const noexcept\r\n\r\n* Add Locale::ToString\r\n\r\n* include locale tests\r\n\r\n* // namespace icu4x\r\n\r\n* using\r\n\r\n* writeable_utils.h\r\n\r\n* WriteableFromString\r\n\r\n* deindent namespaces\r\n\r\n* rm inline\r\n\r\n* run clang-format\r\n\r\n* invert error checks\r\n\r\n* include utility\r\n\r\n* rename fixed decimal ctor argument\r\n\r\n* Add comments about fat pointers",
          "timestamp": "2021-06-09T10:14:34-07:00",
          "tree_id": "8c2610adf7f08b22a1b7427d5e72405e906abfd2",
          "url": "https://github.com/unicode-org/icu4x/commit/fe95051ddc98e77147ff3d48e2fe763ee059167e"
        },
        "date": 1623259260069,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1387153,
            "range": "± 12500",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2706129,
            "range": "± 41138",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "shadaj@users.noreply.github.com",
            "name": "Shadaj Laddad",
            "username": "shadaj"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "da5bfbfdfa8dc7946530ec3ec97b365e1ab627c7",
          "message": "Add support for Locale, StaticDataProvider, and FixedDecimalFormat to the WASM FFI (#770)",
          "timestamp": "2021-06-09T18:04:39-05:00",
          "tree_id": "5461df7510ffed138a424199b10721105ad8eb7c",
          "url": "https://github.com/unicode-org/icu4x/commit/da5bfbfdfa8dc7946530ec3ec97b365e1ab627c7"
        },
        "date": 1623280279436,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1407467,
            "range": "± 2981",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2746239,
            "range": "± 7191",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "manishsmail@gmail.com",
            "name": "Manish Goregaokar",
            "username": "Manishearth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "d8af2a93578bfa44027f540b38d92b9435e52957",
          "message": "Fix links in docs (#786)\n\n* Fix links in docs\r\n\r\n* fix fixeddecimal",
          "timestamp": "2021-06-10T06:49:50-07:00",
          "tree_id": "47494dd555f1ff2454c97d4a7f0dc8f23a432ed0",
          "url": "https://github.com/unicode-org/icu4x/commit/d8af2a93578bfa44027f540b38d92b9435e52957"
        },
        "date": 1623333374392,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1484791,
            "range": "± 60599",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2765662,
            "range": "± 22387",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "aethanyc@gmail.com",
            "name": "Ting-Yu Lin",
            "username": "aethanyc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "fb95cfe3d62d718fdb02e5209fd514394e6958b9",
          "message": "Simplify get_linebreak_property_utf32_with_rule() (#777)\n\n* Remove unused ja_zh parameter\r\n\r\n* Simplify get_linebreak_property_utf32_with_rule()\r\n\r\nThis shouldn't change the behavior.",
          "timestamp": "2021-06-10T07:25:00-07:00",
          "tree_id": "65781f28f016f77f32aaf2733581ad2a6f7149f0",
          "url": "https://github.com/unicode-org/icu4x/commit/fb95cfe3d62d718fdb02e5209fd514394e6958b9"
        },
        "date": 1623335690700,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1360367,
            "range": "± 15903",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2656711,
            "range": "± 20571",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "manishsmail@gmail.com",
            "name": "Manish Goregaokar",
            "username": "Manishearth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "17385b2bafdc5d38a91d8460415ff31914b212ff",
          "message": "Refactor Makefile.toml and use it as the source of truth for (almost) all CI (#783)\n\n* Split Makefile into smaller files\r\n\r\n* Add a tidy task for non-fmt/lint checks\r\n\r\n* Move test-ffi into tests.toml\r\n\r\n* Move all CI jobs to Makefile.toml, refactor Makefile.toml\r\n\r\n* build -> check\r\n\r\n* install cargo make in ci\r\n\r\n* Fix error\r\n\r\n* fix CONTRIBUTING\r\n\r\n* tyop\r\n\r\n* Fix makefile\r\n\r\n* fix\r\n\r\n* fix wasm, dirs\r\n\r\n* switch to duckscript\r\n\r\n* Cache cargo-make\r\n\r\n* fix duckscript\r\n\r\n* Fix syntax\r\n\r\n* cache cargo-readme too\r\n\r\n* better action name\r\n\r\n* don't double-install\r\n\r\n* improve cargo tidy\r\n\r\n* include exes\r\n\r\n* syntax\r\n\r\n* fix npm duckscript\r\n\r\n* rm tidy-minus-fmt",
          "timestamp": "2021-06-10T07:23:44-07:00",
          "tree_id": "dfcb211beda43212488ec05ae8b23eb2111dca91",
          "url": "https://github.com/unicode-org/icu4x/commit/17385b2bafdc5d38a91d8460415ff31914b212ff"
        },
        "date": 1623335698995,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1317660,
            "range": "± 33111",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2515017,
            "range": "± 54901",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "61b7083ff2681fc7cdc847859b63b088e23228e9",
          "message": "Add sections on zero-copy and exotic types to style guide (#699)\n\n* Add sections on zero-copy and exotic types to style guide\r\n\r\n* Review feedback",
          "timestamp": "2021-06-10T07:26:08-07:00",
          "tree_id": "a695f509967dfc58e9dc7e16568f900ac65259d7",
          "url": "https://github.com/unicode-org/icu4x/commit/61b7083ff2681fc7cdc847859b63b088e23228e9"
        },
        "date": 1623335823837,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1512223,
            "range": "± 65210",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2800430,
            "range": "± 108695",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dminor@mozilla.com",
            "name": "Dan Minor",
            "username": "dminor"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "acf388649a30f8a2c3d39fa764a36c10996c7482",
          "message": "Add C FFI for LocaleCanonicalizer (#772)\n\n* Specify lifetimes separately for LocaleCanonicalizer\r\n\r\n* Add LocaleCanonicalizer FFI\r\n\r\n* Address review feedback",
          "timestamp": "2021-06-10T13:31:17-04:00",
          "tree_id": "e54207872ab373372038a84ad95639d46cd65abb",
          "url": "https://github.com/unicode-org/icu4x/commit/acf388649a30f8a2c3d39fa764a36c10996c7482"
        },
        "date": 1623346642992,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1156576,
            "range": "± 28981",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2227077,
            "range": "± 2318",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "zibi@braniecki.net",
            "name": "Zibi Braniecki",
            "username": "zbraniecki"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "feb1add946d05c98b8e6d57c4d5bca7311ddd6bd",
          "message": "Split DateTimeFormat keys (#774)",
          "timestamp": "2021-06-11T12:28:30-07:00",
          "tree_id": "1e8c5b30d302d2132f7a3e98b935e24a1cceeae9",
          "url": "https://github.com/unicode-org/icu4x/commit/feb1add946d05c98b8e6d57c4d5bca7311ddd6bd"
        },
        "date": 1623440108268,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1399606,
            "range": "± 51232",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2610431,
            "range": "± 76403",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "58569820+nordzilla@users.noreply.github.com",
            "name": "Erik Nordin",
            "username": "nordzilla"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "299db8ffe4b7328727a36a78630a900bc33cd3dd",
          "message": "Add PluralRules::categories() function. (#789)\n\n* Add PluralRules::categories() function.\r\n\r\nThe categories() function returns an iterator over each\r\nPluralCategory supported by a PluralRules object that\r\nhas been created with a given LanguageIdentifier and\r\nPluralRuleType.\r\n\r\nThe categories are returned in alphabetical order.\r\nThis is what is expected by browsers, and the same order\r\nthat ICU4C returns.\r\n\r\n* Satisfy clippy\r\n\r\n* Respond to feedback from zbraniecki\r\n\r\n* Small code cleanup\r\n\r\n* Make PluralRules JSON test data lowercase\r\n\r\n* Update categories() example\r\n\r\n* Update categories() doc test to use test data",
          "timestamp": "2021-06-14T11:23:31-07:00",
          "tree_id": "812995efc4ba0fdddbdc69ad93d6a24e3a5d8dd9",
          "url": "https://github.com/unicode-org/icu4x/commit/299db8ffe4b7328727a36a78630a900bc33cd3dd"
        },
        "date": 1623695681814,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1325781,
            "range": "± 104873",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2421031,
            "range": "± 191696",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "a0d4cf41951316c7f1c8733e20e8577fcd7e3f51",
          "message": "Change link to good first issues in CONTRIBUTING.md (#793)",
          "timestamp": "2021-06-15T14:20:49-05:00",
          "tree_id": "eddac0795580cbeb18250e822aff0474ff5e7bd9",
          "url": "https://github.com/unicode-org/icu4x/commit/a0d4cf41951316c7f1c8733e20e8577fcd7e3f51"
        },
        "date": 1623785270116,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1377078,
            "range": "± 52406",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2705896,
            "range": "± 157933",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "58569820+nordzilla@users.noreply.github.com",
            "name": "Erik Nordin",
            "username": "nordzilla"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "d4afff68a0314f0ef6535460740a459fbef9f0c3",
          "message": "Expose PluralOperands::from_str() over capi FFI (#795)\n\n* Expose PluralOperands::from_str() over capi\r\n\r\n- Add new struct for ICU4XCreatePluralOperandsResult\r\n- Add new function for icu4x_plural_operands_create()\r\n\r\n* Clean up From trait impls\r\n\r\n* Remove irrelevant todo comment\r\n\r\n* Remove From impl for ICU4XCreatePluralOperandsResult\r\n\r\n* Be explicit about failure case (not using Default)\r\n\r\n* Re-add check for cat1's category",
          "timestamp": "2021-06-15T14:07:25-07:00",
          "tree_id": "1d8f261dafa39fdd3c228ca15fd4d5e2bb8f3fe9",
          "url": "https://github.com/unicode-org/icu4x/commit/d4afff68a0314f0ef6535460740a459fbef9f0c3"
        },
        "date": 1623791653880,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 934692,
            "range": "± 2715",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1776848,
            "range": "± 9290",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "58569820+nordzilla@users.noreply.github.com",
            "name": "Erik Nordin",
            "username": "nordzilla"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "1766db71feddc10e1d84b4c66ff40b8cce4c91c4",
          "message": "Expose PluralRules::categories() over capi FFI (#796)\n\n* Expose PluralRules::categories() over capi FFI\r\n\r\n- Add new struct ICU4XPluralCategories to hold whether each category has\r\n  rules or not for this PluralRules object.\r\n- Add new function icu4x_plural_rules_categories()\r\n\r\n* Remove FromIterator impl since its used only once",
          "timestamp": "2021-06-15T16:26:01-07:00",
          "tree_id": "c7813a383f94c7e03a691d7de98c64aed9e84918",
          "url": "https://github.com/unicode-org/icu4x/commit/1766db71feddc10e1d84b4c66ff40b8cce4c91c4"
        },
        "date": 1623799945479,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1230835,
            "range": "± 18745",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2352814,
            "range": "± 44969",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "58569820+nordzilla@users.noreply.github.com",
            "name": "Erik Nordin",
            "username": "nordzilla"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "6d4e81287751cc93c5155dd8304efd8b6257230f",
          "message": "Fix bug evaluating NotEq on RangeList (#808)",
          "timestamp": "2021-06-16T16:17:03-07:00",
          "tree_id": "f47c78c172c8fc58575633ab80dce753ae3b45fc",
          "url": "https://github.com/unicode-org/icu4x/commit/6d4e81287751cc93c5155dd8304efd8b6257230f"
        },
        "date": 1623885738392,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 921421,
            "range": "± 15919",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2003027,
            "range": "± 3288",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "shadaj@users.noreply.github.com",
            "name": "Shadaj Laddad",
            "username": "shadaj"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "3885aae4eca500de67ba0a7887a1e1795111fe96",
          "message": "Add benchmarks for the WASM FFI layer (#784)\n\n* Add benchmarks for the WASM FFI layer\r\n\r\nAlso fixes FixedDecimalFormat to consider the options parameter\r\n\r\n* Add NPM task for running all benchmarks",
          "timestamp": "2021-06-17T10:15:07-07:00",
          "tree_id": "81c73e6ec4936f00595d9bbeaab4232fad02a3e6",
          "url": "https://github.com/unicode-org/icu4x/commit/3885aae4eca500de67ba0a7887a1e1795111fe96"
        },
        "date": 1623950473934,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1272255,
            "range": "± 10906",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2382420,
            "range": "± 12908",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "mgaudet+github@ualberta.ca",
            "name": "Matthew Gaudet",
            "username": "mgaudet"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "a663155efc52965cda788be1960b28881308b3f5",
          "message": "Make FsDataProvider data generation command lines copy-paste-executable (#799)",
          "timestamp": "2021-06-17T12:16:42-05:00",
          "tree_id": "f5aa6211c98b0421632847dd47f98feab225055d",
          "url": "https://github.com/unicode-org/icu4x/commit/a663155efc52965cda788be1960b28881308b3f5"
        },
        "date": 1623950579366,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1208318,
            "range": "± 32970",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2251428,
            "range": "± 78835",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "manishsmail@gmail.com",
            "name": "Manish Goregaokar",
            "username": "Manishearth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "7636296239e52b8573bf21a1bad2e3993fbf3f00",
          "message": "Add Sphinx docs for C++ (#806)\n\n* Set up Sphinx doc dir\r\n\r\n* Update license check for RST comments\r\n\r\n* Add basic docs for C++ FFI\r\n\r\n* Add cppdocs CI run\r\n\r\n* Address review comments\r\n\r\n* Fix `cargo make tidy`",
          "timestamp": "2021-06-17T10:22:59-07:00",
          "tree_id": "3f78b4b509bcbb51590204f7308dce1280a4861b",
          "url": "https://github.com/unicode-org/icu4x/commit/7636296239e52b8573bf21a1bad2e3993fbf3f00"
        },
        "date": 1623950963411,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1412479,
            "range": "± 143059",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2650489,
            "range": "± 148134",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "zibi@braniecki.net",
            "name": "Zibi Braniecki",
            "username": "zbraniecki"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c5135af6babd7cdbea02f0971ef8656fb6d68b0b",
          "message": "Only load symbols data if pattern requires them. (#791)",
          "timestamp": "2021-06-17T17:10:44-07:00",
          "tree_id": "43a8f51fcb27b486b23f278c6ee37a0fda3f2777",
          "url": "https://github.com/unicode-org/icu4x/commit/c5135af6babd7cdbea02f0971ef8656fb6d68b0b"
        },
        "date": 1623975438065,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 960707,
            "range": "± 10872",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1998973,
            "range": "± 5892",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "a7a965078db95a90c936b5274d299be7be4f8c42",
          "message": "Minor cleanup to format_to_parts.md",
          "timestamp": "2021-06-18T14:50:30-05:00",
          "tree_id": "e5fc6e35dfc28b3be3f8cdd4c104a9943b88872d",
          "url": "https://github.com/unicode-org/icu4x/commit/a7a965078db95a90c936b5274d299be7be4f8c42"
        },
        "date": 1624046252725,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1141639,
            "range": "± 474162",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2249292,
            "range": "± 98730",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "shadaj@users.noreply.github.com",
            "name": "Shadaj Laddad",
            "username": "shadaj"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "bac781dee970092ae7b953e08ae9f9c69a267524",
          "message": "Bring back cargo make wasm-examples (#815)",
          "timestamp": "2021-06-18T19:31:35-05:00",
          "tree_id": "6ee5339adc9563932ad408c3822aeabee9e9fb6d",
          "url": "https://github.com/unicode-org/icu4x/commit/bac781dee970092ae7b953e08ae9f9c69a267524"
        },
        "date": 1624063095291,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1057083,
            "range": "± 68096",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2157622,
            "range": "± 88930",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "elango@google.com",
            "name": "Elango",
            "username": "echeran"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b56441cc06ca09210d7859e7628f06ac7bb7336c",
          "message": "Unicode property struct cleanup (#677)",
          "timestamp": "2021-06-20T22:00:43-07:00",
          "tree_id": "2ab718445c0fafc287e2574c0c55304086a7f87b",
          "url": "https://github.com/unicode-org/icu4x/commit/b56441cc06ca09210d7859e7628f06ac7bb7336c"
        },
        "date": 1624252075872,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 1105360,
            "range": "± 60342",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 2244067,
            "range": "± 74480",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "58569820+nordzilla@users.noreply.github.com",
            "name": "Erik Nordin",
            "username": "nordzilla"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "80c70663dc7110db9a5eb274c310c6f832675f7a",
          "message": "Add bincode example to icu4x-datagen readme (#819)\n\n* Add bincode example to icu4x-datagen readme\r\n\r\n* Update changes in main.rs to generate README\r\n\r\n* Reorder flags\r\n\r\n* Re-run the generation",
          "timestamp": "2021-06-22T11:52:52-07:00",
          "tree_id": "4697cb4f854f18279386ddf130ad7c3e5534b94d",
          "url": "https://github.com/unicode-org/icu4x/commit/80c70663dc7110db9a5eb274c310c6f832675f7a"
        },
        "date": 1624388415056,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 803953,
            "range": "± 6915",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1671919,
            "range": "± 7680",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "2b7ca3bd55486daafbcf579d05243773e006ec89",
          "message": "Add RcBuf variant to DataPayload (#816)\n\n- Adds yoke::trait_hack and other Yoke improvements",
          "timestamp": "2021-06-23T17:04:37-05:00",
          "tree_id": "ce53818d579ce2051ed7fc406c7e8d9f9159a1ac",
          "url": "https://github.com/unicode-org/icu4x/commit/2b7ca3bd55486daafbcf579d05243773e006ec89"
        },
        "date": 1624486291865,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 689305,
            "range": "± 24207",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1394337,
            "range": "± 72296",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "7ea67288c78dfe7ef56bfb702e2b6452bb13c17a",
          "message": "Migrate HelloWorldV1 and DecimalSymbolsV1 to `serde(borrow)` (#820)",
          "timestamp": "2021-06-24T16:26:08-05:00",
          "tree_id": "db67d7073cefa906a67aa139f0171e63edb7df3a",
          "url": "https://github.com/unicode-org/icu4x/commit/7ea67288c78dfe7ef56bfb702e2b6452bb13c17a"
        },
        "date": 1624570384455,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 632193,
            "range": "± 26371",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1289466,
            "range": "± 55485",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "6b79658c0e00dd22a7bb03fa4f0509c4567134d9",
          "message": "Fix cargo quick (#826)",
          "timestamp": "2021-06-24T21:27:31-05:00",
          "tree_id": "670b05ab8d4efc93589ef7021f2098240833b75a",
          "url": "https://github.com/unicode-org/icu4x/commit/6b79658c0e00dd22a7bb03fa4f0509c4567134d9"
        },
        "date": 1624588422007,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 636107,
            "range": "± 11156",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1312118,
            "range": "± 20437",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dminor@mozilla.com",
            "name": "Dan Minor",
            "username": "dminor"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "3c91d59f9051ed0a871343228b52034f22244e63",
          "message": "Add subtag accessors to Locale C FFI (#803)\n\n* Add subtag accessors to Locale C FFI\r\n\r\nFixes #757.",
          "timestamp": "2021-06-25T07:34:36-04:00",
          "tree_id": "f7d011669ce607ec9b33f9157630a5bb6a46c9a0",
          "url": "https://github.com/unicode-org/icu4x/commit/3c91d59f9051ed0a871343228b52034f22244e63"
        },
        "date": 1624621240412,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 606126,
            "range": "± 11291",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1292781,
            "range": "± 17538",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dminor@mozilla.com",
            "name": "Dan Minor",
            "username": "dminor"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "bd69410c5313bc1c476c88c806743a8c3dac1dc6",
          "message": "Add icu4x_fixed_decimal_create_fromstr (#822)\n\n* Add icu4x_fixed_decimal_create_fromstr",
          "timestamp": "2021-06-25T12:20:47-04:00",
          "tree_id": "31e7c09fb93398d054657f21bd83fa7fcf3fca70",
          "url": "https://github.com/unicode-org/icu4x/commit/bd69410c5313bc1c476c88c806743a8c3dac1dc6"
        },
        "date": 1624638431783,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 658379,
            "range": "± 31344",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1296431,
            "range": "± 50819",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "58569820+nordzilla@users.noreply.github.com",
            "name": "Erik Nordin",
            "username": "nordzilla"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "085c8166f01a963b811d752ac41945a49956eb2f",
          "message": "Rename Yokeable::with_mut() to Yokeable::transform_mut() (#828)",
          "timestamp": "2021-06-25T14:23:29-07:00",
          "tree_id": "ccc9701f32a00bcb8a5c38fe3ffbb1b9ae9a5fd2",
          "url": "https://github.com/unicode-org/icu4x/commit/085c8166f01a963b811d752ac41945a49956eb2f"
        },
        "date": 1624656622528,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 683339,
            "range": "± 44148",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1360879,
            "range": "± 83567",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "manishsmail@gmail.com",
            "name": "Manish Goregaokar",
            "username": "Manishearth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "67bd340dd7cb6e1a958ceb36f8f1d4e73c63742e",
          "message": "Add initial calendars crate (#827)\n\n* Add initial calendars crate\r\n\r\n* rm serde\r\n\r\n* review fixes\r\n\r\n* fmt\r\n\r\n* Move iso constructors to Date\r\n\r\n* review fixes\r\n\r\n* rm where\r\n\r\n* scope Debug bound\r\n\r\n* construct_unchecked -> from_raw\r\n\r\n* fmt\r\n\r\n* add Default\r\n\r\n* fix clippy\r\n\r\n* +errors",
          "timestamp": "2021-06-26T10:05:01-07:00",
          "tree_id": "28a78ebc36f4ad188d362de61ea4eb1171992d62",
          "url": "https://github.com/unicode-org/icu4x/commit/67bd340dd7cb6e1a958ceb36f8f1d4e73c63742e"
        },
        "date": 1624727482673,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 583189,
            "range": "± 17561",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1222029,
            "range": "± 26593",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "manishsmail@gmail.com",
            "name": "Manish Goregaokar",
            "username": "Manishearth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "0d428d309b58300a577def53d257337df853f053",
          "message": "Add projection utilities to Yoke (#833)\n\n* typo fix\r\n\r\n* Introduce CloneableCart\r\n\r\n* Add projection utilities\r\n\r\n* use a phantom lifetime\r\n\r\n* use const for safety docs\r\n\r\n* more safety comment\r\n\r\n* Improve safety comment\r\n\r\n* improve comment about compiler bugs\r\n\r\n* fix parentheses\r\n\r\n* Make project() moving\r\n\r\n* safety comment\r\n\r\n* Make project() completely moving\r\n\r\n* Update all Yokeable impls with transform_move\r\n\r\n* Add borrowing projects again\r\n\r\n* fmt\r\n\r\n* Bump yoke version\r\n\r\n* Temporarily ignore Yoke::project doctest",
          "timestamp": "2021-07-02T09:12:54-07:00",
          "tree_id": "08c93419a46c67379be45e15a6cf1fb498d082bb",
          "url": "https://github.com/unicode-org/icu4x/commit/0d428d309b58300a577def53d257337df853f053"
        },
        "date": 1625242872191,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 712861,
            "range": "± 19156",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1411665,
            "range": "± 37669",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c5bebc5d17ed4e77173cdae45e663b2e52d41c41",
          "message": "Add icu_provider_blob crate with StaticDataProvider (#835)",
          "timestamp": "2021-07-04T20:42:00-05:00",
          "tree_id": "fc3336c469387750144e4aeb40183f0e959eea6e",
          "url": "https://github.com/unicode-org/icu4x/commit/c5bebc5d17ed4e77173cdae45e663b2e52d41c41"
        },
        "date": 1625449707830,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 664572,
            "range": "± 43552",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1389276,
            "range": "± 145112",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "aethanyc@gmail.com",
            "name": "Ting-Yu Lin",
            "username": "aethanyc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b9acfc49c983106924f0e201ccccf3453a05b73c",
          "message": "Remove the binary of the pluralrules example (#850)\n\nIt was checked-in accidentally in #712.",
          "timestamp": "2021-07-06T15:58:12-07:00",
          "tree_id": "d41584dd4a106d70812036ad475d213348c47a5d",
          "url": "https://github.com/unicode-org/icu4x/commit/b9acfc49c983106924f0e201ccccf3453a05b73c"
        },
        "date": 1625612667266,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 656312,
            "range": "± 16350",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1362334,
            "range": "± 52054",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "manishsmail@gmail.com",
            "name": "Manish Goregaokar",
            "username": "Manishearth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "56c71b72ce26499abf258e877fb7bfe61bc274c4",
          "message": "Add custom derive for Yokeable and ZCF (#847)\n\n* Add yoke-derive\r\n\r\n* Use yoke derive in icu4x\r\n\r\n* Satisfy clippy\r\n\r\n* Improve ZeroMap yoke impl, add zerovec derive tests\r\n\r\n* CloningZCF -> cloning_zcf\r\n\r\n* unsafe_impl_data_marker_with_lifetime -> impl_data_marker_with_lifetime\r\n\r\n* Add ZCF impls for ZV\r\n\r\n* fix sentence fragment\r\n\r\n* Remove extra yoke deps\r\n\r\n* fix bounds formatting\r\n\r\n* rm yoke::\r\n\r\n* Fix test failure\r\n\r\n* Add #[data_struct] custom derive\r\n\r\n* Use #[data_struct] everywhere\r\n\r\n* UnicodePropertyMarker -> UnicodePropertyV1Marker\r\n\r\n* Fix generated docs",
          "timestamp": "2021-07-13T11:20:21-07:00",
          "tree_id": "6226fde1918ed61005f3993daaf12cfb5d9de17a",
          "url": "https://github.com/unicode-org/icu4x/commit/56c71b72ce26499abf258e877fb7bfe61bc274c4"
        },
        "date": 1626201021589,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 591599,
            "range": "± 20063",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1241040,
            "range": "± 41830",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "manishsmail@gmail.com",
            "name": "Manish Goregaokar",
            "username": "Manishearth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b7d35d1e2085c8429da3266044bac15dfab49ce4",
          "message": "Clean up zerovec::samples so that it is not available outside of tests (#857)\n\n* Clean up zerovec::samples\r\n\r\n* fix unused warning",
          "timestamp": "2021-07-13T11:20:35-07:00",
          "tree_id": "8a07e78f5daa972c0a7494b71f74ab978fae8240",
          "url": "https://github.com/unicode-org/icu4x/commit/b7d35d1e2085c8429da3266044bac15dfab49ce4"
        },
        "date": 1626201135385,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 618126,
            "range": "± 31290",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1234850,
            "range": "± 69578",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "manishsmail@gmail.com",
            "name": "Manish Goregaokar",
            "username": "Manishearth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "cce8f224c263fb579d3f5ff02e6589e3aa9d9717",
          "message": "Bump yoke to 0.2.1 (#858)\n\n* Bump yoke to 0.2.1\r\n\r\n* lockfile",
          "timestamp": "2021-07-13T15:06:24-07:00",
          "tree_id": "7f0d12216417b3a7cb1740c6e812e91414debfd5",
          "url": "https://github.com/unicode-org/icu4x/commit/cce8f224c263fb579d3f5ff02e6589e3aa9d9717"
        },
        "date": 1626214357787,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 642626,
            "range": "± 41831",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1304272,
            "range": "± 76785",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "manishsmail@gmail.com",
            "name": "Manish Goregaokar",
            "username": "Manishearth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "56b5cf828609510cf667cbe3ae2437ba722a358c",
          "message": "Replace thiserror with displaydoc (#863)\n\n* thiserror -> displaydoc\r\n\r\nfastmod \"thiserror = .*$\" \"displaydoc = { version = \\\"0.2.3\\\", default-features = false }\"\r\n\r\n* thiserror::Error -> displaydoc::Display\r\n\r\nfastmod thiserror::Error displaydoc::Display\r\n\r\n* fastmod \"#\\[error\" \"#[displaydoc\"\r\n\r\n* derive(Error) -> derive(Display)\r\n\r\nfastmod \"derive\\(Error\" \"derive(Display\"\r\n\r\n* Replace #[from] with manual impls\r\n\r\n* #[displaydoc(transparent) -> #[displaydoc(\"{0}\")]\r\n\r\n* Remove #[source]\r\n\r\n* Add explicit Error impls\r\n\r\nRun `find . -name *.rs | xargs -I{} gawk -f replace.awk -i inplace {}`\r\n\r\nwith\r\n\r\n```awk\r\nBEGIN { a = 0; enum = \"\" }\r\n\r\n/#\\[derive\\(Display/ {\r\n    a = 1\r\n}\r\n\r\nmatch($0, /enum ([a-zA-Z]+)/, arr) {\r\n    enum = arr[1]\r\n}\r\n\r\n{ print }\r\n\r\n/^}/ && a == 1 {\r\n    print \"\\nimpl std::error::Error for \" enum \" {}\\n\"\r\n    a = 0\r\n}\r\n```\r\n\r\n* Fix generic Error impls\r\n\r\n* Fix displaydoc panic\r\n\r\n* struct not enum\r\n\r\n* Fixup from impl\r\n\r\n* rustfmt\r\n\r\n* fix cfg\r\n\r\n* fix cfg",
          "timestamp": "2021-07-16T16:49:17-07:00",
          "tree_id": "998f2cd394040aa4a2cc7538e7cc9e9216bf52dc",
          "url": "https://github.com/unicode-org/icu4x/commit/56b5cf828609510cf667cbe3ae2437ba722a358c"
        },
        "date": 1626479776658,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 746933,
            "range": "± 25800",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1554844,
            "range": "± 62021",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "e30e7c3f491feb84f30214bc0b1dc751b191e9dc",
          "message": "Re-name data categories (#864)",
          "timestamp": "2021-07-16T19:03:30-05:00",
          "tree_id": "73d71da841c6ef23db46c68bef42c70fdb478815",
          "url": "https://github.com/unicode-org/icu4x/commit/e30e7c3f491feb84f30214bc0b1dc751b191e9dc"
        },
        "date": 1626480623692,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 729569,
            "range": "± 29483",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1460126,
            "range": "± 98878",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "aethanyc@gmail.com",
            "name": "Ting-Yu Lin",
            "username": "aethanyc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "aca8858ffa29cf0721647d59e52863c059b45b3c",
          "message": "Cleanup line breaker data generation and rule table accessing APIs (#851)\n\n* Simplify some APIs accessing line break rule table\r\n\r\n* Simply the setup when parsing property files\r\n\r\n* Simplify the parsing logic of EastAsianWidth.txt\r\n\r\nNote: match() checks for a match at the beginning of the line, so we\r\ndon't need to skip lines start with \"#\".\r\n\r\nThis is a refactor and shouldn't change the behavior.\r\n\r\n* Simplify the parsing logic of LineBreak.txt\r\n\r\nUnified the logic setting line breaking class. Before this patch,\r\n\"CP_EA\" tweak was only set when we parsed a range of codepoints, but we\r\nreally should also apply it when we parsed a single codepoint, although\r\ncurrently the path doesn't do anything with current EastAsianWidth.txt\r\ndata.\r\n\r\nThis is a refactor and shouldn't change the behavior.\r\n\r\n* Update segmenter README\r\n\r\nThe cleanup in previous commits uses \"Assignment expressions\" [1], which\r\nwas added in Python 3.8 (released in Oct 2019), so update the README\r\naccordingly.\r\n\r\n[1] https://docs.python.org/release/3.8.0/whatsnew/3.8.html#assignment-expressions",
          "timestamp": "2021-07-19T07:22:12-07:00",
          "tree_id": "958c1557a868defcf8d8bacde6cac8c779feadb7",
          "url": "https://github.com/unicode-org/icu4x/commit/aca8858ffa29cf0721647d59e52863c059b45b3c"
        },
        "date": 1626704951239,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 631396,
            "range": "± 12310",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1312258,
            "range": "± 15344",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "manishsmail@gmail.com",
            "name": "Manish Goregaokar",
            "username": "Manishearth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "a8377791ffc70c7c6f3aa7f6aa27023e0e59aaca",
          "message": "Progress towards no_std (#865)\n\n* hardcode thumb\r\n\r\n* Fix writeable\r\n\r\n* Fix stable_deref_trait dep\r\n\r\n* no-default-features on tinystr\r\n\r\n* Update tinystr\r\n\r\n* Use serde/serde-json with alloc feature\r\n\r\n* Move to newer resolver\r\n\r\n* Bump tinystr\r\n\r\n* Bump tinystr again\r\n\r\n* fix optional=true after rebase\r\n\r\n* Add alloc feature to yoke\r\n\r\n* fmt\r\n\r\n* Move provider_fs to std for now\r\n\r\n* rm config.toml changes\r\n\r\n* not test in yoke\r\n\r\n* fix tinystr\r\n\r\n* fix features",
          "timestamp": "2021-07-19T11:29:45-07:00",
          "tree_id": "c063ff96e6ce96c0258946e3f6a69b833ab42d55",
          "url": "https://github.com/unicode-org/icu4x/commit/a8377791ffc70c7c6f3aa7f6aa27023e0e59aaca"
        },
        "date": 1626719787391,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 625077,
            "range": "± 30477",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1266338,
            "range": "± 19276",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "manishsmail@gmail.com",
            "name": "Manish Goregaokar",
            "username": "Manishearth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "e5f7a5e5c737ee6e3ba0c53e789d131d3eb277a8",
          "message": "Remove use of `dyn Error` in library code in favor of Strings (#867)\n\n* Remove std::error::Error from provider\r\n\r\n* Remove std::error::Error from fs data provider\r\n\r\n* Pull in serde std only when exporting\r\n\r\n* fmt",
          "timestamp": "2021-07-19T17:07:01-07:00",
          "tree_id": "332ef8ced9691040c640d1d282f255d16ce794ed",
          "url": "https://github.com/unicode-org/icu4x/commit/e5f7a5e5c737ee6e3ba0c53e789d131d3eb277a8"
        },
        "date": 1626740024058,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 590633,
            "range": "± 48447",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1133012,
            "range": "± 97644",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "manishsmail@gmail.com",
            "name": "Manish Goregaokar",
            "username": "Manishearth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ff56a70f588b63f134714956a8eb8e1ee37a99d3",
          "message": "Move locid and fixed_decimal over to no_std (#868)\n\n* Fix fixed-decimal for no_std\r\n\r\n* Move locid to no_std\r\n\r\n* Fix unused error\r\n\r\n* remove displaydoc/std",
          "timestamp": "2021-07-20T17:00:20-07:00",
          "tree_id": "12046357894214fed8784d275b2d9e6ecac3e9f5",
          "url": "https://github.com/unicode-org/icu4x/commit/ff56a70f588b63f134714956a8eb8e1ee37a99d3"
        },
        "date": 1626826024724,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 647210,
            "range": "± 23533",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1394035,
            "range": "± 103353",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "manishsmail@gmail.com",
            "name": "Manish Goregaokar",
            "username": "Manishearth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "09d5aef92cd37de4e51c53e0b1eb03abb1ce8543",
          "message": "Move provider/core to no_std (#870)\n\n* add std feature\n\n* fix yoke derive\n\n* autogen\n\n* fixup errors\n\n* move litemap to no_std\n\n* Move helloworld to litemap\n\n* cargo fix\n\n* fix datagen\n\n* more fmt\n\n* Bump util version numbers\n\n* Update readme/lockfile\n\n* fixup doctests\n\n* more doctest fix\n\n* rm litemap serde",
          "timestamp": "2021-07-20T18:43:44-07:00",
          "tree_id": "af578e52ca0d0a8e53333d4db21c0459b5afa69d",
          "url": "https://github.com/unicode-org/icu4x/commit/09d5aef92cd37de4e51c53e0b1eb03abb1ce8543"
        },
        "date": 1626832199505,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 597352,
            "range": "± 35818",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1188678,
            "range": "± 62947",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "gregtatum@users.noreply.github.com",
            "name": "Greg Tatum",
            "username": "gregtatum"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "4052c2ec46d082142f12c09e680d42fae6d62a82",
          "message": "Modify patterns from a skeleton match to have the correct width (#832)",
          "timestamp": "2021-07-21T16:36:33-05:00",
          "tree_id": "77d6dcabe60d44573d2531361ee356e61b6761d8",
          "url": "https://github.com/unicode-org/icu4x/commit/4052c2ec46d082142f12c09e680d42fae6d62a82"
        },
        "date": 1626903774127,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 473893,
            "range": "± 8553",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1102338,
            "range": "± 3137",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "832a4055c0a3228c14d8ca122405c135ba8a72e4",
          "message": "Update roadmap.md (#875)",
          "timestamp": "2021-07-21T17:00:20-05:00",
          "tree_id": "844376f9087003ddaf175d4c8f85ae7e3c3befe6",
          "url": "https://github.com/unicode-org/icu4x/commit/832a4055c0a3228c14d8ca122405c135ba8a72e4"
        },
        "date": 1626905194580,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 567142,
            "range": "± 27762",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1137723,
            "range": "± 58444",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "manishsmail@gmail.com",
            "name": "Manish Goregaokar",
            "username": "Manishearth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "6a7650eef7b24d2ac7f539d7b1d86249d036e316",
          "message": "Move blob data provider to postcard; support no_std (#878)\n\n* Move blob provider to postcard\r\n\r\n* Move testdata.bincode to postcard\r\n\r\n* Support no_std in blob data provider\r\n\r\n* rm build.rs",
          "timestamp": "2021-07-21T15:10:46-07:00",
          "tree_id": "d4a6b9d19d494947d542b3642dcafdc3e80527af",
          "url": "https://github.com/unicode-org/icu4x/commit/6a7650eef7b24d2ac7f539d7b1d86249d036e316"
        },
        "date": 1626905857420,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 729242,
            "range": "± 44085",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1477864,
            "range": "± 56032",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "manishsmail@gmail.com",
            "name": "Manish Goregaokar",
            "username": "Manishearth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "77e8269d25798b7a4963bee1a0b639659bfe2d57",
          "message": "Move locale_canonicalizer and StaticDataProvider to no_std, use SDP correctly in capi (#880)\n\n* make locale_canonicalizer no_std\r\n\r\n* Remove fs provider dep from capi when there is no OS\r\n\r\n* Move testdata to no_std\r\n\r\n* fmt\r\n\r\n* include sdp in test\r\n\r\n* fixup import",
          "timestamp": "2021-07-22T09:19:40-07:00",
          "tree_id": "1ad244828aae34b87a37b4124230d3df8bcfbc67",
          "url": "https://github.com/unicode-org/icu4x/commit/77e8269d25798b7a4963bee1a0b639659bfe2d57"
        },
        "date": 1626971167184,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 532108,
            "range": "± 4150",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1104850,
            "range": "± 3489",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "elango@google.com",
            "name": "Elango",
            "username": "echeran"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ed677ed1baf0e2181edb8dc2a58ac10e3da8387b",
          "message": "Add iterator of inversion list ranges to UnicodeSet (#839)",
          "timestamp": "2021-07-22T10:20:51-07:00",
          "tree_id": "3f2497ac364fefc2adb8df6cde79043822cf49e7",
          "url": "https://github.com/unicode-org/icu4x/commit/ed677ed1baf0e2181edb8dc2a58ac10e3da8387b"
        },
        "date": 1626974829705,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 530410,
            "range": "± 11155",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1105948,
            "range": "± 5212",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "9acd1e99b12cc2c3eac2338fbe07f0059686b6e7",
          "message": "Fix name of Bidi_M (#884)",
          "timestamp": "2021-07-22T12:29:58-05:00",
          "tree_id": "8d221a68e13801f21723f33ecff92e92651865ff",
          "url": "https://github.com/unicode-org/icu4x/commit/9acd1e99b12cc2c3eac2338fbe07f0059686b6e7"
        },
        "date": 1626975380215,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 597196,
            "range": "± 39878",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1253360,
            "range": "± 88694",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "manishsmail@gmail.com",
            "name": "Manish Goregaokar",
            "username": "Manishearth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b76cb7961957945b2a1576d5c8f1ad25eb280279",
          "message": "Move icu_plurals and icu_decimal to no_std (#888)\n\n* no_std in pluralrules\r\n\r\n* autoreplace\r\n\r\n* fix errors\r\n\r\n* powi\r\n\r\n* Fix icu_plurals use in ffi/ecma402\r\n\r\n* fmt\r\n\r\n* add no_std to fixeddecimal\r\n\r\n* autoreplace\r\n\r\n* fmt\r\n\r\n* fix error\r\n\r\n* require std feature for pluralrules test",
          "timestamp": "2021-07-22T10:29:28-07:00",
          "tree_id": "c205d36691fc577a1b260990fefb3b167d2ff49b",
          "url": "https://github.com/unicode-org/icu4x/commit/b76cb7961957945b2a1576d5c8f1ad25eb280279"
        },
        "date": 1626975402778,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 652315,
            "range": "± 6145",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1358179,
            "range": "± 1981",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "manishsmail@gmail.com",
            "name": "Manish Goregaokar",
            "username": "Manishearth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "83418f8c43949ef19e89956ecac33618ae91ae67",
          "message": "Fix FFI tests (#889)\n\n* Fix duckscript\n\n* Fix return types for cpp locale\n\n* Split out ffi job\n\n* Remove print\n\n* Add exit_on_error\n\n* Refactor ffi tests to separate module",
          "timestamp": "2021-07-22T12:02:49-07:00",
          "tree_id": "6fefee009d44ce57dcdf0de021728ac39d4a9895",
          "url": "https://github.com/unicode-org/icu4x/commit/83418f8c43949ef19e89956ecac33618ae91ae67"
        },
        "date": 1626980961852,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 596536,
            "range": "± 18655",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1270414,
            "range": "± 47121",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "elango@google.com",
            "name": "Elango",
            "username": "echeran"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "6df5ffc44246d68586ccf888b13a761a365a30c7",
          "message": "Set explicit stable values for enum discriminants that match ICU4C (#698)",
          "timestamp": "2021-07-22T13:28:13-07:00",
          "tree_id": "3730e22970b9b9ddc0ac5cf7e8d5cd8395923789",
          "url": "https://github.com/unicode-org/icu4x/commit/6df5ffc44246d68586ccf888b13a761a365a30c7"
        },
        "date": 1626986105645,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 691788,
            "range": "± 21127",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1435857,
            "range": "± 70968",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "manishsmail@gmail.com",
            "name": "Manish Goregaokar",
            "username": "Manishearth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "9a25738b476593d40353e19a2dd0b2c5f9ab81a6",
          "message": "Handle no_std for icu_capi (#890)\n\n* capi nostd\r\n\r\n* replace\r\n\r\n* fmt\r\n\r\n* fix warning\r\n\r\n* Add freertos glue\r\n\r\n* fix warning\r\n\r\n* Add tests for cortex\r\n\r\n* Fix wasm glue\r\n\r\n* fix attr\r\n\r\n* fmt\r\n\r\n* update test-cortex\r\n\r\n* fix actions\r\n\r\n* link to #891",
          "timestamp": "2021-07-22T14:28:42-07:00",
          "tree_id": "b2fbc849d3ba824f44f91de9233d4f1579d7e28f",
          "url": "https://github.com/unicode-org/icu4x/commit/9a25738b476593d40353e19a2dd0b2c5f9ab81a6"
        },
        "date": 1626989732843,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 647051,
            "range": "± 22970",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1338002,
            "range": "± 38074",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "gregtatum@users.noreply.github.com",
            "name": "Greg Tatum",
            "username": "gregtatum"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "2fb8ea7943b3344555191bfdf0fbc5cfb13f7c82",
          "message": "Correctly apply the hour cycle in the components::Bag (#846)",
          "timestamp": "2021-07-23T08:42:21-05:00",
          "tree_id": "ebd677c67f6136cb15ff12dd7a871e9934bd3080",
          "url": "https://github.com/unicode-org/icu4x/commit/2fb8ea7943b3344555191bfdf0fbc5cfb13f7c82"
        },
        "date": 1627048141274,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 747730,
            "range": "± 29244",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1512860,
            "range": "± 52450",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "49ece439952c04a2a5e200c63fc696b176aa79e2",
          "message": "Add working Clone impl for Yoke (#894)\n\nCo-authored-by: Manish Goregaokar <manishsmail@gmail.com>",
          "timestamp": "2021-07-23T15:46:29-05:00",
          "tree_id": "dfba54fc9335730c157139a9fff228b1ec35ff2b",
          "url": "https://github.com/unicode-org/icu4x/commit/49ece439952c04a2a5e200c63fc696b176aa79e2"
        },
        "date": 1627073580746,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 548615,
            "range": "± 26037",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1143520,
            "range": "± 71060",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f5bc1660a2efa1c88f6ab3ee7a6e68af92498ffd",
          "message": "Implement working Clone and PartialEq on DataPayload (#895)",
          "timestamp": "2021-07-23T16:14:52-05:00",
          "tree_id": "1d70b92ebb4a6063fc0ae09e2b27a283eda5fa93",
          "url": "https://github.com/unicode-org/icu4x/commit/f5bc1660a2efa1c88f6ab3ee7a6e68af92498ffd"
        },
        "date": 1627075302941,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 630656,
            "range": "± 6808",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1318821,
            "range": "± 7613",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "9d82dabc1dbc2b41e7467b769d736af02b738227",
          "message": "Make `cargo quick` quicker (#897)",
          "timestamp": "2021-07-25T12:13:22-05:00",
          "tree_id": "aad69299b0283402d2e72e41f6ac028b6504324c",
          "url": "https://github.com/unicode-org/icu4x/commit/9d82dabc1dbc2b41e7467b769d736af02b738227"
        },
        "date": 1627233580814,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 531074,
            "range": "± 5089",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1092970,
            "range": "± 2765",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "774ee840a9630b225496caf797badb6bb6e38094",
          "message": "Improve Yoke Clone docs, saying it is not generally expensive (#899)",
          "timestamp": "2021-07-25T12:13:44-05:00",
          "tree_id": "8420afcc00ca207da960205fe6dc79d8b3dc38c1",
          "url": "https://github.com/unicode-org/icu4x/commit/774ee840a9630b225496caf797badb6bb6e38094"
        },
        "date": 1627233621093,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 622536,
            "range": "± 6773",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1293011,
            "range": "± 12164",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "7ed372f52c29b3740a885f48fdfc00a795f13112",
          "message": "Remove DataPayload::from_borrowed (#898)",
          "timestamp": "2021-07-26T12:49:28-05:00",
          "tree_id": "8f987676e0441709baccc8064395c3e7dcefc885",
          "url": "https://github.com/unicode-org/icu4x/commit/7ed372f52c29b3740a885f48fdfc00a795f13112"
        },
        "date": 1627322122218,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 466379,
            "range": "± 3238",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 969295,
            "range": "± 2267",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "4b48cf79996c997658606e30503ad46f4c586003",
          "message": "Change FsDataProvider to 'static (#902)",
          "timestamp": "2021-07-26T17:56:18-05:00",
          "tree_id": "a468d7cb22248c65686416aa7fe1ebc82dfa08e3",
          "url": "https://github.com/unicode-org/icu4x/commit/4b48cf79996c997658606e30503ad46f4c586003"
        },
        "date": 1627340600402,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 625052,
            "range": "± 5214",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1290929,
            "range": "± 11301",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "e97c8c3b3d9c129bbf63ffc8162080dd01931ee1",
          "message": "Remove second lifetime parameter from DataProvider (#904)",
          "timestamp": "2021-07-27T15:02:37-05:00",
          "tree_id": "fd8ed04e3dfdb5aac06301f67a34be462d8f07d6",
          "url": "https://github.com/unicode-org/icu4x/commit/e97c8c3b3d9c129bbf63ffc8162080dd01931ee1"
        },
        "date": 1627416542048,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 535730,
            "range": "± 5781",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1106412,
            "range": "± 6235",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "becafac79b327f18e8f1721683de638e828f2419",
          "message": "Improve docs on DataProvider lifetimes (#915)",
          "timestamp": "2021-07-27T17:39:28-05:00",
          "tree_id": "d44cf184c25ca2457c67a931b558426e59bfad1f",
          "url": "https://github.com/unicode-org/icu4x/commit/becafac79b327f18e8f1721683de638e828f2419"
        },
        "date": 1627425923031,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 467923,
            "range": "± 5574",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 969420,
            "range": "± 9492",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5970335819dd63caf5653ca76529bd9e9e90650e",
          "message": "Fix FFI lifetimes for DataProvider (#914)",
          "timestamp": "2021-07-27T17:39:49-05:00",
          "tree_id": "5e873125b3716f751c3ccd3acf1a7c938dd499af",
          "url": "https://github.com/unicode-org/icu4x/commit/5970335819dd63caf5653ca76529bd9e9e90650e"
        },
        "date": 1627426012172,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 641241,
            "range": "± 52588",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1255496,
            "range": "± 74537",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "4ea601271e0cb3122b1ed00c28f0ef4cd9a366a6",
          "message": "Add zv_serde example, plus improvements to Makefile.toml (#901)",
          "timestamp": "2021-07-27T17:42:13-05:00",
          "tree_id": "0dbdf8a7cb2b0ea1137bd37e7ab9c26a490abc7e",
          "url": "https://github.com/unicode-org/icu4x/commit/4ea601271e0cb3122b1ed00c28f0ef4cd9a366a6"
        },
        "date": 1627426161162,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 729845,
            "range": "± 30416",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1475594,
            "range": "± 50866",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "manishsmail@gmail.com",
            "name": "Manish Goregaokar",
            "username": "Manishearth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "cd4b7c536c0b573aa68510bc8e1776fa78b1c0fd",
          "message": "Move the `icu` crate to no_std (#911)\n\n* Move uniset to no_std\r\n\r\n* Autoreplace\r\n\r\n* Migrate datetime to no_std\r\n\r\n* Autoreplace\r\n\r\n* fix errors\r\n\r\n* fix import\r\n\r\n* cargo fix\r\n\r\n* Move icu core crate to no-std\r\n\r\n* Add task for testing all of icu4x builds on nostd\r\n\r\n* fix import\r\n\r\n* fix warnings",
          "timestamp": "2021-07-27T16:38:02-07:00",
          "tree_id": "60f8120c2300a934d50144fcc57353e4f63f2d5c",
          "url": "https://github.com/unicode-org/icu4x/commit/cd4b7c536c0b573aa68510bc8e1776fa78b1c0fd"
        },
        "date": 1627429453611,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 531584,
            "range": "± 4849",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1104412,
            "range": "± 5241",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "gregtatum@users.noreply.github.com",
            "name": "Greg Tatum",
            "username": "gregtatum"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "3d49bb1e03c399ebb5927fac6678ea8518841905",
          "message": "Teach length::Bag how to switch hour cycles (#840)",
          "timestamp": "2021-07-28T15:22:16-05:00",
          "tree_id": "2a213c1f287155b7212a110cb12b7ad4333c5f70",
          "url": "https://github.com/unicode-org/icu4x/commit/3d49bb1e03c399ebb5927fac6678ea8518841905"
        },
        "date": 1627504080864,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 543650,
            "range": "± 4393",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1130803,
            "range": "± 25796",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "41129501+gnrunge@users.noreply.github.com",
            "name": "Norbert Runge",
            "username": "gnrunge"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "8d99c160c4581dc247127c328ced20eda809fc43",
          "message": "Binary size benchmarking: Rust script to measure size of the ICU4X examples compiled into wasm binaries (#871)\n\nSet up GHA to build wasm binaries, measure file sizes, push results into benchmark dashboard .\r\n\r\nResolves ticket #140.\r\n\r\nCo-authored-by: Greg Tatum <gregtatum@users.noreply.github.com>\r\n\r\nCo-authored-by: Greg Tatum <gregtatum@users.noreply.github.com>",
          "timestamp": "2021-07-29T12:27:49-05:00",
          "tree_id": "9e0744b9ce8c059ab9d583a862264748644a9a25",
          "url": "https://github.com/unicode-org/icu4x/commit/8d99c160c4581dc247127c328ced20eda809fc43"
        },
        "date": 1627580066884,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 677506,
            "range": "± 44951",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1381556,
            "range": "± 80451",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "cd5b0d278ce455c13dfbd29a4a84694aa9f026fc",
          "message": "Document certain DataProvider impls that return `'static` (#916)\n\nReverts 4b48cf79996c997658606e30503ad46f4c586003",
          "timestamp": "2021-07-29T12:29:28-05:00",
          "tree_id": "5d8a17a8ec99b48a6c6fefc01cbbd1ee9c37eec7",
          "url": "https://github.com/unicode-org/icu4x/commit/cd5b0d278ce455c13dfbd29a4a84694aa9f026fc"
        },
        "date": 1627580186427,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 721491,
            "range": "± 30530",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1451902,
            "range": "± 78250",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "871b9869064b1c056dd7ee13128a33158762bfaa",
          "message": "Improve docs in PluralRules and FixedDecimal (#886)",
          "timestamp": "2021-07-29T13:14:47-05:00",
          "tree_id": "ed5feea1dc9c049874e3d44751986631ccd6d33e",
          "url": "https://github.com/unicode-org/icu4x/commit/871b9869064b1c056dd7ee13128a33158762bfaa"
        },
        "date": 1627582938282,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 736569,
            "range": "± 27803",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1501415,
            "range": "± 152942",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "0b38c6f03d4c182d6dd899e300b8e1b207b74895",
          "message": "Rename data errors to MissingResource (#893)",
          "timestamp": "2021-07-29T16:59:43-05:00",
          "tree_id": "fe3cc86ca1fb9881c8571afffd60f5b2d058b6c6",
          "url": "https://github.com/unicode-org/icu4x/commit/0b38c6f03d4c182d6dd899e300b8e1b207b74895"
        },
        "date": 1627596454374,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 805757,
            "range": "± 56921",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1687911,
            "range": "± 128151",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "288aab2ca18012798d51f0454b4ba36f178b815d",
          "message": "Update CHANGELOG for 0.3",
          "timestamp": "2021-07-29T17:29:47-05:00",
          "tree_id": "8b5972ebb2af9f6f13c26d11d46b8c175689a3c4",
          "url": "https://github.com/unicode-org/icu4x/commit/288aab2ca18012798d51f0454b4ba36f178b815d"
        },
        "date": 1627598219383,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 622873,
            "range": "± 12241",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1318072,
            "range": "± 13621",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "shane@unicode.org",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "shane@unicode.org",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "distinct": true,
          "id": "d3a5153c6da23e434a05ddfae441d2ecc39f1d5c",
          "message": "Bump Yoke to 0.2.3",
          "timestamp": "2021-07-29T18:53:06-05:00",
          "tree_id": "1188c87119763cbcb4f1fcb9241477b585b128fd",
          "url": "https://github.com/unicode-org/icu4x/commit/d3a5153c6da23e434a05ddfae441d2ecc39f1d5c"
        },
        "date": 1627603590321,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 599109,
            "range": "± 17353",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1254377,
            "range": "± 36689",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "manishsmail@gmail.com",
            "name": "Manish Goregaokar",
            "username": "Manishearth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "d12a5c53366b650f27e1bad8bec9791f432df4ad",
          "message": "Fix clippy warnings arising from Rust version upgrade (#923)",
          "timestamp": "2021-08-02T12:40:47-05:00",
          "tree_id": "72f8596d3fd41a8f668b6130084b302f95b67524",
          "url": "https://github.com/unicode-org/icu4x/commit/d12a5c53366b650f27e1bad8bec9791f432df4ad"
        },
        "date": 1627926455825,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 704799,
            "range": "± 43930",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1466267,
            "range": "± 52723",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "shadaj@users.noreply.github.com",
            "name": "Shadaj Laddad",
            "username": "shadaj"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c70c9dd5fdb13d45fc16388c78a6bd3e759430c4",
          "message": "Migrate C/C++/JS APIs to Diplomat (#900)\n\n* Migrate capi to use Diplomat and update examples\r\n\r\n* Migrate C++ and JS to Diplomat API\r\n\r\n* Fix cargo fmt and clippy\r\n\r\n* Fix wasm-test-release\r\n\r\n* Bump Diplomat to branch with no_std runtime\r\n\r\n* Bump Diplomat to disable no_std on WASM\r\n\r\n* Switch to Diplomat main\r\n\r\n* Bump Diplomat\r\n\r\n* Bring back result types\r\n\r\n* Split up header files\r\n\r\n* Update lockfile for latest Diplomat\r\n\r\n* Update headers to drop module paths\r\n\r\n* Set up CI to run Diplomat\r\n\r\n* Add build step that checks if Diplomat bindings are up-to-date\r\n\r\n* Only diff ffi\r\n\r\n* Update Diplomat rev",
          "timestamp": "2021-08-02T11:24:46-07:00",
          "tree_id": "c5436509ef4fc6563790834f21a515dad0d784fa",
          "url": "https://github.com/unicode-org/icu4x/commit/c70c9dd5fdb13d45fc16388c78a6bd3e759430c4"
        },
        "date": 1627929061109,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 612353,
            "range": "± 8982",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1307463,
            "range": "± 5926",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "elango@google.com",
            "name": "Elango",
            "username": "echeran"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "455d57aed9ecec54467412c1d34b731702a163a5",
          "message": "Port ICU4C code point trie (#711)\n\nCo-authored-by: Shane F. Carr <shane@unicode.org>\r\n\r\nCo-authored-by: Manish Goregaokar <manishsmail@gmail.com>\r\nCo-authored-by: Shane F. Carr <shane@unicode.org>",
          "timestamp": "2021-08-03T09:14:32-07:00",
          "tree_id": "e477ec2415c0b0ec31988c86179ddbbae3a04739",
          "url": "https://github.com/unicode-org/icu4x/commit/455d57aed9ecec54467412c1d34b731702a163a5"
        },
        "date": 1628007681770,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 633451,
            "range": "± 9038",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1346962,
            "range": "± 3489",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "manishsmail@gmail.com",
            "name": "Manish Goregaokar",
            "username": "Manishearth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ae65821858b61560d984c28e2c2153ae39e087e2",
          "message": "Regen FFI with new diplomat (#932)\n\n* Update diplomat\r\n\r\n* Regen FFI\r\n\r\n* Update test with to_writeable\r\n\r\n* Rename format_write() to format()",
          "timestamp": "2021-08-03T14:07:14-07:00",
          "tree_id": "e766adf2562ccc0373d1875d8ee76d1be4431ce5",
          "url": "https://github.com/unicode-org/icu4x/commit/ae65821858b61560d984c28e2c2153ae39e087e2"
        },
        "date": 1628025174530,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 455095,
            "range": "± 5822",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 972901,
            "range": "± 3819",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "066edd4c0d0328c6d2fcc039db3ff474c97815d3",
          "message": "Add missing `#[serde(borrow)]` (#930)",
          "timestamp": "2021-08-05T02:03:51-05:00",
          "tree_id": "1b8a2140edb1b7600109d54e782b11810fc5bfb1",
          "url": "https://github.com/unicode-org/icu4x/commit/066edd4c0d0328c6d2fcc039db3ff474c97815d3"
        },
        "date": 1628147411433,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 607098,
            "range": "± 8367",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1289100,
            "range": "± 14825",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "manishsmail@gmail.com",
            "name": "Manish Goregaokar",
            "username": "Manishearth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "2c2f611bb6e4d604bdd40fa4d12687272bae0533",
          "message": "Support --keys in datagen (#938)\n\n* Add --keys support to datagen\r\n\r\n* use writeable",
          "timestamp": "2021-08-05T16:16:01-07:00",
          "tree_id": "db6cccf4ec2f7d50743f21f53201e23fb720b8db",
          "url": "https://github.com/unicode-org/icu4x/commit/2c2f611bb6e4d604bdd40fa4d12687272bae0533"
        },
        "date": 1628205760863,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 686022,
            "range": "± 21041",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1429266,
            "range": "± 65035",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "manishsmail@gmail.com",
            "name": "Manish Goregaokar",
            "username": "Manishearth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b412c7696eb43ad10864f468a41041edf28c6ee2",
          "message": "Call destructor for locale (#937)",
          "timestamp": "2021-08-05T17:21:26-07:00",
          "tree_id": "9375203a41643aafe4d8d6cbe65f0589a25562cb",
          "url": "https://github.com/unicode-org/icu4x/commit/b412c7696eb43ad10864f468a41041edf28c6ee2"
        },
        "date": 1628209673696,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 656908,
            "range": "± 31589",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1374345,
            "range": "± 66259",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "iireland@mozilla.com",
            "name": "iainireland",
            "username": "iainireland"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "d838bb5a9446fd70951b4b3157d52fe6d92fe0c8",
          "message": "Minimal uprops provider (#885)\n\n* Add binary uprops data needed for irregexp to testdata\r\n\r\n* Implement BinaryPropertiesDataProvider\r\n\r\n* Add export support for BinaryPropertiesDataProvider\r\n\r\n* Add license to uprops testdata\r\n\r\n* Address review feedback\r\n\r\n* Remove second lifetime parameter from BinaryPropertiesDataProvider\r\n\r\n* Fix newline for cargo fmt\r\n\r\n* Remove unnecessary cargo-all-features from Cargo.toml\r\n\r\n* Address review feedback\r\n\r\n* Bump rust toolchain version to 1.52 for cargo-make\r\n\r\n* rust-toolchain was already bumped\r\n\r\n* Update uprops Cargo.toml for 0.3 release\r\n\r\n* Update uprops version to 0.3\r\n\r\n* Update Cargo.lock\r\n\r\nCo-authored-by: Iain Ireland <iain.i.ireland@gmail.com>",
          "timestamp": "2021-08-06T10:31:01-07:00",
          "tree_id": "085a5f87894f0c9aa7802ad3c0b02fafe8d23027",
          "url": "https://github.com/unicode-org/icu4x/commit/d838bb5a9446fd70951b4b3157d52fe6d92fe0c8"
        },
        "date": 1628271450214,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 539006,
            "range": "± 31844",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1105601,
            "range": "± 53820",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "manishsmail@gmail.com",
            "name": "Manish Goregaokar",
            "username": "Manishearth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "a1b5ada1142196384e18a01d9abca87f2bc5f942",
          "message": "Add feature flags to CAPI; update WearOS build steps (#939)\n\n* Generate smaller testdata\r\n\r\n* Add smaller_static feature to testdata\r\n\r\n* Add features to capi\r\n\r\n* Update build command for cortex\r\n\r\n* fix ci\r\n\r\n* fix ci\r\n\r\n* skip optional\r\n\r\n* fix target_os\r\n\r\n* Remove feature slicing\r\n\r\n* Only use smaller_static in capi\r\n\r\n* nit",
          "timestamp": "2021-08-06T13:58:58-07:00",
          "tree_id": "566c1ec91b00bb8a3b542c76939aebfa00997047",
          "url": "https://github.com/unicode-org/icu4x/commit/a1b5ada1142196384e18a01d9abca87f2bc5f942"
        },
        "date": 1628283905244,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 522801,
            "range": "± 6412",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1090581,
            "range": "± 17579",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "manishsmail@gmail.com",
            "name": "Manish Goregaokar",
            "username": "Manishearth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "220f4590e91bebe1aa71a1c60b78fe6c4e68abec",
          "message": "Make icu_capi always no_std (#941)",
          "timestamp": "2021-08-06T19:18:20-07:00",
          "tree_id": "e9bd413f14e3089b569be6ea09df4c7831afb914",
          "url": "https://github.com/unicode-org/icu4x/commit/220f4590e91bebe1aa71a1c60b78fe6c4e68abec"
        },
        "date": 1628303052633,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 513593,
            "range": "± 7164",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1099601,
            "range": "± 11188",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c5f62dfc5f15c88581d69279634b6087da42768c",
          "message": "Data struct tutorial (#929)",
          "timestamp": "2021-08-10T18:58:13-05:00",
          "tree_id": "8f65327cdddac0173da8ea1fbdfc6aded27a76ac",
          "url": "https://github.com/unicode-org/icu4x/commit/c5f62dfc5f15c88581d69279634b6087da42768c"
        },
        "date": 1628640292108,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 525664,
            "range": "± 2116",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1100576,
            "range": "± 2041",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "9e7ca843719c5ccf7570e5f6cef0fbebfbb84003",
          "message": "Check that testdata is up-to-date in CI (#947)\n\n* Remoes CLDR download from `cargo make testdata` and updates docs\r\n* Adds workaround for serde_json line ending bug",
          "timestamp": "2021-08-11T11:02:08-05:00",
          "tree_id": "d79f9b80c81810b6f27116d93709cfb8fb1c3b64",
          "url": "https://github.com/unicode-org/icu4x/commit/9e7ca843719c5ccf7570e5f6cef0fbebfbb84003"
        },
        "date": 1628698107734,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 524202,
            "range": "± 2106",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 1094479,
            "range": "± 1818",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "07f659080d9beb013402570c6b18791a0ecd247f",
          "message": "Change benches to use static data provider (#892)",
          "timestamp": "2021-08-11T18:13:32-05:00",
          "tree_id": "22ec80426360506800cb47c6e7e5e55d705ac7d7",
          "url": "https://github.com/unicode-org/icu4x/commit/07f659080d9beb013402570c6b18791a0ecd247f"
        },
        "date": 1628724031080,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 354420,
            "range": "± 35127",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 638626,
            "range": "± 40968",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f52429e0ca3505eee821bd57218c20a80013283a",
          "message": "Add design doc explaining phases of data provider information (#498)",
          "timestamp": "2021-08-12T14:18:08-05:00",
          "tree_id": "e0077c8d2fa7aa2e7c0b5aa896ee0419cad0132b",
          "url": "https://github.com/unicode-org/icu4x/commit/f52429e0ca3505eee821bd57218c20a80013283a"
        },
        "date": 1628796305464,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 356426,
            "range": "± 18039",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 681676,
            "range": "± 14568",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "aethanyc@gmail.com",
            "name": "Ting-Yu Lin",
            "username": "aethanyc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "acfb8f4151f978edcb731a029ce22793830ff24a",
          "message": "Improve documentation for line breaker's public interfaces (#950)",
          "timestamp": "2021-08-16T10:14:59-07:00",
          "tree_id": "8bee46fd0c9a9c95d5130ee62058b96354d778ce",
          "url": "https://github.com/unicode-org/icu4x/commit/acfb8f4151f978edcb731a029ce22793830ff24a"
        },
        "date": 1629134515837,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 320225,
            "range": "± 37145",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 637785,
            "range": "± 47305",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "iireland@mozilla.com",
            "name": "iainireland",
            "username": "iainireland"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "6a964cb21137a18405e9adaadfe4e5352b479be5",
          "message": "Uniset cleanup (#956)\n\n* Use where clauses to make fn signatures more readable\r\n\r\n* Support groups of general categories\r\n\r\n* Remove clippy and rustfmt exceptions\r\n\r\n* Clean up enum_props::Script\r\n\r\n* Fix incorrect key for gc=Surrogate\r\n\r\n* Implement get_script_val_set\r\n\r\n* Add UnicodeSet::get_range\r\n\r\n* Add UnicodeSetBuilder::add_range_u32\r\n\r\n* Address review comments\r\n\r\n* Update test\r\n\r\nCo-authored-by: Iain Ireland <iain.i.ireland@gmail.com>",
          "timestamp": "2021-08-17T13:28:46-07:00",
          "tree_id": "dc428cf324c85a3b1142766e62ea23b64953cf78",
          "url": "https://github.com/unicode-org/icu4x/commit/6a964cb21137a18405e9adaadfe4e5352b479be5"
        },
        "date": 1629232506518,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 358157,
            "range": "± 6855",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 688557,
            "range": "± 13930",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "2edf576106fe276e2e811e8d2869070729af800d",
          "message": "Add \"x86tiny\" feature to icu_capi (#957)",
          "timestamp": "2021-08-17T15:58:43-05:00",
          "tree_id": "51351e73881d1851ab4434ff7dd516c7837ead09",
          "url": "https://github.com/unicode-org/icu4x/commit/2edf576106fe276e2e811e8d2869070729af800d"
        },
        "date": 1629234347782,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 393973,
            "range": "± 51608",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 689975,
            "range": "± 35108",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "aethanyc@gmail.com",
            "name": "Ting-Yu Lin",
            "username": "aethanyc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "300b83a1ff5608fd9e8eadca6f9805c9155c3174",
          "message": "Minor cleanup to line breaker (#949)\n\n* Move line break test from lib.rs into line_breaker.rs\r\n\r\nTo consolidate the tests in line_breaker.rs and keep the lib.rs simple.\r\n\r\n* Rename break_rule to line_break_rule\r\n\r\nThis gives a better symmetry with `word_break_rule` next to it.\r\n\r\n* Remove ja_zh argument for LineBreakIteratorLatin1::new_with_break_rule()\r\n\r\nLatin1 cannot encode Chinese or Japanese characters, so it makes more\r\nsense to remove this argument.",
          "timestamp": "2021-08-17T16:48:20-07:00",
          "tree_id": "d7cb30473cb3b4e6ab222de57e994337f8ac6706",
          "url": "https://github.com/unicode-org/icu4x/commit/300b83a1ff5608fd9e8eadca6f9805c9155c3174"
        },
        "date": 1629244456403,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 309143,
            "range": "± 16076",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 594834,
            "range": "± 47448",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "manishsmail@gmail.com",
            "name": "Manish Goregaokar",
            "username": "Manishearth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "305643d72bbb6a1b9fd09e0aedad46b32bc938d8",
          "message": "Update diplomat; improve diplomat makefiles (#959)\n\n* Update Diplomat\r\n\r\n* Include cpp docs\r\n\r\n* Consistently name diplomat tasks, add diplomat-gen task\r\n\r\n* Have Diplomat clean up before regenerating\r\n\r\n* fix",
          "timestamp": "2021-08-17T23:10:04-07:00",
          "tree_id": "2165f97f6ac06ea6123100bc1fe529d83994cc07",
          "url": "https://github.com/unicode-org/icu4x/commit/305643d72bbb6a1b9fd09e0aedad46b32bc938d8"
        },
        "date": 1629267398354,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 378985,
            "range": "± 22881",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 774105,
            "range": "± 27022",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "245eb95ee01b2cabdea611a2f1b8436b4ad7890b",
          "message": "Update Diplomat and add `pub` to all exported functions (#963)",
          "timestamp": "2021-08-17T23:36:13-07:00",
          "tree_id": "02b981699dd728b559fcc90e4e47b7d1f049da58",
          "url": "https://github.com/unicode-org/icu4x/commit/245eb95ee01b2cabdea611a2f1b8436b4ad7890b"
        },
        "date": 1629268960464,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 355618,
            "range": "± 25918",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 676587,
            "range": "± 38526",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "d320b0439d7cc48d2b9ac579245f43a16c439705",
          "message": "Add size-optimized FFI functions for Locale and DataProvider (#962)",
          "timestamp": "2021-08-18T13:35:47-05:00",
          "tree_id": "4a9f5d7fa3ad807ff877c351757d8fc6e5333859",
          "url": "https://github.com/unicode-org/icu4x/commit/d320b0439d7cc48d2b9ac579245f43a16c439705"
        },
        "date": 1629312158660,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 406887,
            "range": "± 49851",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 782147,
            "range": "± 61484",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dminor@mozilla.com",
            "name": "Dan Minor",
            "username": "dminor"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "803af54cdb0d263fe372912a0647fa8a0e50a247",
          "message": "Package LICENSE with each component (#790)\n\n* Package LICENSE with each component\r\n\r\n* Address review feedback\r\n\r\n* Add LICENSE for new components",
          "timestamp": "2021-08-18T15:40:00-04:00",
          "tree_id": "f7e848173bcdb9a02dbf33b6c39302dabd82fbc8",
          "url": "https://github.com/unicode-org/icu4x/commit/803af54cdb0d263fe372912a0647fa8a0e50a247"
        },
        "date": 1629316024000,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 365001,
            "range": "± 25630",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 729571,
            "range": "± 14653",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dminor@mozilla.com",
            "name": "Dan Minor",
            "username": "dminor"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "cdab7f1771ff47093e006d8309e137b5ce1097df",
          "message": "Ignore duplicate keywords and attributes (#966)\n\nFrom https://tc39.es/ecma402/#sec-canonicalizeunicodelocaleid, we should\r\nonly keep the first keyword for a given key and the first instance of any\r\nattribute defined in the input.",
          "timestamp": "2021-08-19T09:02:10-04:00",
          "tree_id": "db46eb09517856ec3d326b95924bf62e65d34f89",
          "url": "https://github.com/unicode-org/icu4x/commit/cdab7f1771ff47093e006d8309e137b5ce1097df"
        },
        "date": 1629378546374,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 370344,
            "range": "± 2510",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 689594,
            "range": "± 6296",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dminor@mozilla.com",
            "name": "Dan Minor",
            "username": "dminor"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ccfb206002f52cd363291d0070a27084e167bc22",
          "message": "Reject duplicate variants (#965)\n\nWe currently silently drop duplicate variants, but according to BCP47,\r\nthis is not valid:\r\n\r\n   5.  The same variant subtag MUST NOT be used more than once within a\r\n       language tag.\r\n\r\n       *  For example, the tag \"de-DE-1901-1901\" is not valid.\r\n\r\nThis changes the code to reject duplicate variants.",
          "timestamp": "2021-08-19T09:02:44-04:00",
          "tree_id": "da4965acfa3d71be73ba32313b11ada7c6c2c66e",
          "url": "https://github.com/unicode-org/icu4x/commit/ccfb206002f52cd363291d0070a27084e167bc22"
        },
        "date": 1629378566382,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 369415,
            "range": "± 6749",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 701204,
            "range": "± 10175",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dminor@mozilla.com",
            "name": "Dan Minor",
            "username": "dminor"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "9b41685af03e5cbc8ddd97348ae19719ace30753",
          "message": "Reject empty unicode extensions (#970)\n\nFrom https://www.unicode.org/reports/tr35/#Unicode_locale_identifier,\r\nunicode extensions must include at least one attribute or at least one\r\nkeyword.",
          "timestamp": "2021-08-20T09:04:27-04:00",
          "tree_id": "1becf955de38f8291f81982f6322c5a11a1328f9",
          "url": "https://github.com/unicode-org/icu4x/commit/9b41685af03e5cbc8ddd97348ae19719ace30753"
        },
        "date": 1629465100325,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 374499,
            "range": "± 37255",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 709608,
            "range": "± 44340",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "manishsmail@gmail.com",
            "name": "Manish Goregaokar",
            "username": "Manishearth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b66b237d22db389c8333c558937e66e16d12db34",
          "message": "Load Diplomat and twiggy from cache; update diplomat (#974)\n\n* Update diplomat\r\n\r\n* Add make tasks for updating diplomat\r\n\r\n* Load Diplomat from cache\r\n\r\n* Load twiggy from cache\r\n\r\n* No need to sync anymore\r\n\r\n* indent\r\n\r\n* Include tool name in cache key name",
          "timestamp": "2021-08-24T13:55:41-07:00",
          "tree_id": "7ba0519a6e4776039c31421f98948c41043ab754",
          "url": "https://github.com/unicode-org/icu4x/commit/b66b237d22db389c8333c558937e66e16d12db34"
        },
        "date": 1629839263995,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 370778,
            "range": "± 1050",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 686163,
            "range": "± 3257",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "manishsmail@gmail.com",
            "name": "Manish Goregaokar",
            "username": "Manishearth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "19c05d9785c2ab7eafc25f4e204948b2394320dc",
          "message": "Add cpp-to-wasm test (#968)\n\n* Add cpp-to-wasm test\r\n\r\n* Run wasm test on CI\r\n\r\n* Fix CI\r\n\r\n* Add host test to test-ffi\r\n\r\n* pin emsdk\r\n\r\n* Pin emscripten test to older nightly too",
          "timestamp": "2021-08-24T22:31:57-07:00",
          "tree_id": "fa49db00c5b98cbb54c04cecdb3249d13dc265e0",
          "url": "https://github.com/unicode-org/icu4x/commit/19c05d9785c2ab7eafc25f4e204948b2394320dc"
        },
        "date": 1629869934288,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 418422,
            "range": "± 19714",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 728405,
            "range": "± 36855",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "41129501+gnrunge@users.noreply.github.com",
            "name": "Norbert Runge",
            "username": "gnrunge"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "569d0c0347d650a4cc6b9f45492067a6edf428c3",
          "message": "Enhance binary size benchmark: monitor size of the gzip'd wasm (#980)\n\nexecutables as well.\r\n\r\nResolves ticket #912.",
          "timestamp": "2021-08-25T16:28:30-07:00",
          "tree_id": "d77852153e52c38b11070795b2ec786b074f728a",
          "url": "https://github.com/unicode-org/icu4x/commit/569d0c0347d650a4cc6b9f45492067a6edf428c3"
        },
        "date": 1629934565872,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 379302,
            "range": "± 13890",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 694444,
            "range": "± 33322",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "manishsmail@gmail.com",
            "name": "Manish Goregaokar",
            "username": "Manishearth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "a807a847bff2e8608d73a9881db0376a406e5125",
          "message": "Update pinned nightly (#1010)\n\n* Update pinned nightly rustc to nightly-2021-08-20\r\n\r\n* Install prerelease twiggy from git\r\n\r\n* fix indent\r\n\r\n* set hash as variable\r\n\r\n* fix indent more\r\n\r\n* Install newer wasm-opt on wasm task",
          "timestamp": "2021-08-25T19:01:38-07:00",
          "tree_id": "cdd0442822805569a7642e2d4949409d40d61015",
          "url": "https://github.com/unicode-org/icu4x/commit/a807a847bff2e8608d73a9881db0376a406e5125"
        },
        "date": 1629943728914,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 355195,
            "range": "± 34126",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 662361,
            "range": "± 36481",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "3d668128cfd7f4f04406950f779120c1c4b3d1a9",
          "message": "Remove obsolete ToOwned impls for [SerdeSe/Erased]DataStruct (#1011)",
          "timestamp": "2021-08-25T23:30:22-05:00",
          "tree_id": "4c4920fdf7d06cfec9348867524d80285a58f397",
          "url": "https://github.com/unicode-org/icu4x/commit/3d668128cfd7f4f04406950f779120c1c4b3d1a9"
        },
        "date": 1629952578863,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 276840,
            "range": "± 745",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 511329,
            "range": "± 723",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "7136f99745d3ac01ecc4baebd66a2dbbdbc9a97e",
          "message": "Remove ErasedDataStruct::clone_into_box() (#1013)",
          "timestamp": "2021-08-26T01:23:09-05:00",
          "tree_id": "209a5bae0de7793c4308206163a1f7bb44ff09f6",
          "url": "https://github.com/unicode-org/icu4x/commit/7136f99745d3ac01ecc4baebd66a2dbbdbc9a97e"
        },
        "date": 1629959400209,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 372719,
            "range": "± 5864",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 681070,
            "range": "± 9821",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dminor@mozilla.com",
            "name": "Dan Minor",
            "username": "dminor"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c6ad8b6403b4cda34e7773d5493ffc8d5161edbc",
          "message": "Support `other` extensions (#976)\n\n* Support `other` extensions\r\n\r\nFor test262 compliance, we need to be able to parse and write `other`\r\nextensions.\r\n\r\n* Add missing key.rs file\r\n\r\n* Run fmt and clippy\r\n\r\n* Apply review feedback",
          "timestamp": "2021-08-26T07:51:19-04:00",
          "tree_id": "6a3950abcaa5b691cd95a904be7a3ece9a4bf153",
          "url": "https://github.com/unicode-org/icu4x/commit/c6ad8b6403b4cda34e7773d5493ffc8d5161edbc"
        },
        "date": 1629979091514,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 305798,
            "range": "± 563",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 566683,
            "range": "± 2649",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sffc@google.com",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "9bcfb461c75ea93697647079b4a7025f12bbcfbe",
          "message": "Update pinned nightly for memory benchmarks and coverage (#1015)",
          "timestamp": "2021-08-26T08:20:17-07:00",
          "tree_id": "7936878bf9317b1c5183a45b1abd5884012ac071",
          "url": "https://github.com/unicode-org/icu4x/commit/9bcfb461c75ea93697647079b4a7025f12bbcfbe"
        },
        "date": 1629991649024,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 387877,
            "range": "± 13105",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 705323,
            "range": "± 32320",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}