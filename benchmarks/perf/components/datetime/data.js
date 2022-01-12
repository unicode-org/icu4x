window.BENCHMARK_DATA = {
  "lastUpdate": 1641951819378,
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
          "id": "0bd9e98b0bce1cc6724f596e91f51332fd266b28",
          "message": "Add LICENSE to Cargo.toml include section (#1016)\n\nWe need to actually add LICENSE to the Cargo.toml include\r\nsection in order for it to be vendored properly.",
          "timestamp": "2021-08-26T13:58:26-04:00",
          "tree_id": "e6d9d779bae57b5480cbd008a90e0a95a503e757",
          "url": "https://github.com/unicode-org/icu4x/commit/0bd9e98b0bce1cc6724f596e91f51332fd266b28"
        },
        "date": 1630001083577,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 255228,
            "range": "± 4274",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 470260,
            "range": "± 5727",
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
          "id": "9858aac7b581729580809cac02f556ee827a4c9e",
          "message": "Fix a typo in the destination directory path of binsize benchmark data. (#1021)\n\nPart of resolution of ticket# 1019.",
          "timestamp": "2021-08-26T11:38:07-07:00",
          "tree_id": "f193f5530a8ad0a8f63115321fd6b456c2b715dc",
          "url": "https://github.com/unicode-org/icu4x/commit/9858aac7b581729580809cac02f556ee827a4c9e"
        },
        "date": 1630003438873,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 303796,
            "range": "± 429",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 566848,
            "range": "± 961",
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
          "id": "9cd784ed0a39196644cea04525b3f8669e1b4902",
          "message": "Incorporate codesize reduction work into wearos build (#1017)",
          "timestamp": "2021-08-26T13:01:27-07:00",
          "tree_id": "c2576a301b98a5b56fb8c3061063f9b7d4615cf5",
          "url": "https://github.com/unicode-org/icu4x/commit/9cd784ed0a39196644cea04525b3f8669e1b4902"
        },
        "date": 1630008484382,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 302858,
            "range": "± 4878",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 560207,
            "range": "± 1265",
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
          "id": "ee3da0d8310e6adafea99f713893c5e61ed81533",
          "message": "Add EqULE trait to make ZeroVec::from_aligned more efficient (#1012)\n\n* Bumps ZeroVec to 0.3.0 since this is a breaking change (new fn name)",
          "timestamp": "2021-08-26T15:54:06-05:00",
          "tree_id": "516599893c6f434c786758ca445656cffe69ac75",
          "url": "https://github.com/unicode-org/icu4x/commit/ee3da0d8310e6adafea99f713893c5e61ed81533"
        },
        "date": 1630011665316,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 356235,
            "range": "± 38178",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 674881,
            "range": "± 59850",
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
          "id": "3c019c9c1ce1476a6e41325e22fbce1447dab127",
          "message": "Add LiteMap benchmark binaries for Bincode, Postcard and tests for Rkyv (#1014)\n\n* Bump minor version number of LiteMap for new methods",
          "timestamp": "2021-08-27T13:06:22-05:00",
          "tree_id": "8dafae9d6a0ceebd64a76ee406ee4505667acbd9",
          "url": "https://github.com/unicode-org/icu4x/commit/3c019c9c1ce1476a6e41325e22fbce1447dab127"
        },
        "date": 1630087947460,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 300613,
            "range": "± 956",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 559432,
            "range": "± 4754",
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
          "id": "36524d6f40f45a18e5afade7c1010adcd4659ab2",
          "message": "Fix UnicodeSetBuilder::complement for empty sets (#961)\n\n* Fix UnicodeSetBuilder::complement for empty sets\r\n\r\n* Cargo fmt\r\n\r\nCo-authored-by: Iain Ireland <iain.i.ireland@gmail.com>",
          "timestamp": "2021-08-30T12:08:58-07:00",
          "tree_id": "97f4c6cea5cc5156477600529327695ef6822995",
          "url": "https://github.com/unicode-org/icu4x/commit/36524d6f40f45a18e5afade7c1010adcd4659ab2"
        },
        "date": 1630350934383,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 302747,
            "range": "± 543",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 564582,
            "range": "± 3407",
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
          "id": "22d4384959540d71da32831a3f8e62901aa60551",
          "message": "Add missing docs warnings and suppressions to the components directory (#955)",
          "timestamp": "2021-08-31T10:10:59-05:00",
          "tree_id": "ef9246121450628cf3059625d62e0fd962cc809b",
          "url": "https://github.com/unicode-org/icu4x/commit/22d4384959540d71da32831a3f8e62901aa60551"
        },
        "date": 1630423502507,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 336732,
            "range": "± 34083",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 630808,
            "range": "± 52647",
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
          "id": "26b1be1d27a533ae9254c38c76573929fcf14491",
          "message": "Lint missing docs in icu_datetime (#725)",
          "timestamp": "2021-08-31T10:23:54-05:00",
          "tree_id": "fcfbb60de2b4549617d9271414663d2820394abb",
          "url": "https://github.com/unicode-org/icu4x/commit/26b1be1d27a533ae9254c38c76573929fcf14491"
        },
        "date": 1630423866267,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 377684,
            "range": "± 10882",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 690884,
            "range": "± 28957",
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
          "id": "a305081cde7d46598c5a2acf8c8681d237c54dfc",
          "message": "Add documentation for LocaleCanonicalizer (#1031)\n\n* Add documentation for LocaleCanonicalizer\r\n\r\n* Address review feedback\r\n\r\n* Update generated readme",
          "timestamp": "2021-09-01T13:13:55-04:00",
          "tree_id": "7ae03d964d0280e872b26ac0813ce7cdb1ec3808",
          "url": "https://github.com/unicode-org/icu4x/commit/a305081cde7d46598c5a2acf8c8681d237c54dfc"
        },
        "date": 1630516869325,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 332518,
            "range": "± 43107",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 661219,
            "range": "± 46672",
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
          "id": "35db5543a9009d8eb394d25bc18962e588bd4978",
          "message": "Update diplomat (const APIs in C++) (#1035)\n\n* Update diplomat\r\n\r\n* Update and regen diplomat",
          "timestamp": "2021-09-02T09:35:21-07:00",
          "tree_id": "cfdd7d9fb1e8d4b42d816f253d0049a4f4ac99c6",
          "url": "https://github.com/unicode-org/icu4x/commit/35db5543a9009d8eb394d25bc18962e588bd4978"
        },
        "date": 1630600958043,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 400617,
            "range": "± 21497",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 706188,
            "range": "± 25811",
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
          "id": "cee2fc64dbd39f58039e427eae4fc6f988822162",
          "message": "Disable Codecov comments on PRs (#1023)",
          "timestamp": "2021-09-02T16:32:03-07:00",
          "tree_id": "149d4d5b5d08017ae0643ece3805ee268b24105a",
          "url": "https://github.com/unicode-org/icu4x/commit/cee2fc64dbd39f58039e427eae4fc6f988822162"
        },
        "date": 1630625895585,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_overview",
            "value": 303682,
            "range": "± 731",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 565161,
            "range": "± 7335",
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
          "id": "5f34ab9faa7a0a640118361d44831fdf462bf007",
          "message": "Add a DateTimeFormat components bag benchmark (#1037)",
          "timestamp": "2021-09-07T10:42:46-05:00",
          "tree_id": "397e07a278e5895f3c0e3e7829ffa0617e85a585",
          "url": "https://github.com/unicode-org/icu4x/commit/5f34ab9faa7a0a640118361d44831fdf462bf007"
        },
        "date": 1631029819347,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 325458,
            "range": "± 7898",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1712786,
            "range": "± 58525",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 643793,
            "range": "± 30526",
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
          "id": "bccef3f99cb816738e05f1e07e0a6ebf2bab305d",
          "message": "Add initial support for timezones in component::Bag (#845)",
          "timestamp": "2021-09-07T13:24:51-05:00",
          "tree_id": "4f46975bb7cd5de8a833f0dff0fbf88b258a2677",
          "url": "https://github.com/unicode-org/icu4x/commit/bccef3f99cb816738e05f1e07e0a6ebf2bab305d"
        },
        "date": 1631039544391,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 362228,
            "range": "± 18203",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1724224,
            "range": "± 73589",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 663153,
            "range": "± 46459",
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
          "id": "9cf6367ccef3c741c559c9ce04ec1ab08c22a3bb",
          "message": "Move datetime types into calendars crate (#1038)\n\n* Move experimental/calendar to components/calendar\r\n\r\n* Move calendar crate to no_std\r\n\r\n* Import DateTimeError type\r\n\r\n* Migrate datetime types to icu_calendar crate\r\n\r\n* fix test imports\r\n\r\n* fmt",
          "timestamp": "2021-09-08T09:43:36-07:00",
          "tree_id": "7615c5f781ceb540bed30b02ee3ce84db2b39e37",
          "url": "https://github.com/unicode-org/icu4x/commit/9cf6367ccef3c741c559c9ce04ec1ab08c22a3bb"
        },
        "date": 1631119841191,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 273718,
            "range": "± 8239",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1560997,
            "range": "± 6730",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 504376,
            "range": "± 492",
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
          "id": "8e66c5ea835b43a6a46e9b994b31b6d92f224aed",
          "message": "Improve safety of SerdeSeDataStructWrap (#1040)",
          "timestamp": "2021-09-09T13:58:59-05:00",
          "tree_id": "cda8af44ed85c96b7a5f746befed84dc89b660cd",
          "url": "https://github.com/unicode-org/icu4x/commit/8e66c5ea835b43a6a46e9b994b31b6d92f224aed"
        },
        "date": 1631214316645,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 272932,
            "range": "± 502",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1556118,
            "range": "± 11405",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 510291,
            "range": "± 2683",
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
          "id": "61c5aa3a716da2580aaeecb45d35df0fa39801ca",
          "message": "Document icu_calendars (#1039)\n\n* Document icu_calendars\r\n\r\n* DateDurationUnit\r\n\r\n* Add DateDuration example\r\n\r\n* fmt",
          "timestamp": "2021-09-09T14:22:01-07:00",
          "tree_id": "2f45ce46cc8c48bca8c75b6e46e01f1d941b0bc8",
          "url": "https://github.com/unicode-org/icu4x/commit/61c5aa3a716da2580aaeecb45d35df0fa39801ca"
        },
        "date": 1631222959735,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 306760,
            "range": "± 555",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1591777,
            "range": "± 1882",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 575781,
            "range": "± 556",
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
          "id": "369215a1184e85d524bcd121edd769a6ccf0dfdb",
          "message": "Add [As]VarULE impls for Vec<T>/[T] (#1042)\n\n* Clarify safety of from_byte_slice_unchecked\n\n* Add from_byte_slice_unchecked to ULE\n\n* Add VarULE impl for Vec<T>\n\n* Add example for VarULE vec\n\n* Make ULE and VarULE unsafe traits\n\nSee https://twitter.com/ManishEarth/status/1436387041114157057 , in\ngeneral people think traits should be `unsafe` even if their only\nimplementor-side safety invaraints are in `unsafe` fns\n\n* Add impl for ZeroVec<'static>\n\n* add test",
          "timestamp": "2021-09-10T15:19:55-07:00",
          "tree_id": "740b9ad066ce03d53feadd7db7cf2a14edef7e4d",
          "url": "https://github.com/unicode-org/icu4x/commit/369215a1184e85d524bcd121edd769a6ccf0dfdb"
        },
        "date": 1631312856708,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 369435,
            "range": "± 2613",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1888013,
            "range": "± 9724",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 687606,
            "range": "± 5787",
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
          "id": "a252023697c67309f4118e35c2b1923f99a88356",
          "message": "Add IsCovariant trait (#1041)",
          "timestamp": "2021-09-10T19:54:19-05:00",
          "tree_id": "39fd66f8b1fdde87279693fe1a8cefe92921c1c6",
          "url": "https://github.com/unicode-org/icu4x/commit/a252023697c67309f4118e35c2b1923f99a88356"
        },
        "date": 1631322131178,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 368577,
            "range": "± 28737",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1849359,
            "range": "± 123150",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 736262,
            "range": "± 39692",
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
          "id": "54d3b85100eb10aaed26f5f40e44a4cca5996f1b",
          "message": "Derive Yokeable for ZeroMap (#1046)\n\n* WIP: Asking Questions\r\n\r\n* Make it work\r\n\r\n* Clean up comments\r\n\r\n* Update documentation\r\n\r\nCo-authored-by: Manish Goregaokar <manishsmail@gmail.com>",
          "timestamp": "2021-09-13T16:27:52-07:00",
          "tree_id": "f893d7bd949e0236f4cae7f224b5bdc456fa2d9f",
          "url": "https://github.com/unicode-org/icu4x/commit/54d3b85100eb10aaed26f5f40e44a4cca5996f1b"
        },
        "date": 1631576110872,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 373395,
            "range": "± 18836",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1878475,
            "range": "± 59658",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 685551,
            "range": "± 27899",
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
          "id": "d5a9f48d2b533825fbccb94077fd504a5f34041c",
          "message": "Add binary enumerated properties data provider; update uprop toml files (#1047)\n\n* Add EnumeratedPropertiesDataProvider\r\n\r\n* Implement IterableDataProviderCore\r\n\r\n* Implement unified PropertyDataProvider\r\n\r\n* Re-run cargo make generate-readmes\r\n\r\n* Update uprops data to use new icuwriteuprops data format\r\n\r\n* Refactor expand_groupings\r\n\r\nCo-authored-by: Iain Ireland <iain.i.ireland@gmail.com>",
          "timestamp": "2021-09-14T10:21:10-07:00",
          "tree_id": "e3dfa72898eb38ee3354b272e6b0d5f803394de7",
          "url": "https://github.com/unicode-org/icu4x/commit/d5a9f48d2b533825fbccb94077fd504a5f34041c"
        },
        "date": 1631640490997,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 308955,
            "range": "± 1380",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1561323,
            "range": "± 2057",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 571563,
            "range": "± 3519",
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
          "id": "0719bef4e5e77ebc0dfe5f8cade17707baa81f7e",
          "message": "Add provider_json and provider_bincode features (#1049)\n\n- Migrates manifest to serde-json-core\r\n- Renames bincode to provider_bincode\r\n- Fails early when syntax is not supported",
          "timestamp": "2021-09-14T14:20:55-05:00",
          "tree_id": "89bb880f2dce4e5076374c935f9845d80c20547f",
          "url": "https://github.com/unicode-org/icu4x/commit/0719bef4e5e77ebc0dfe5f8cade17707baa81f7e"
        },
        "date": 1631647693832,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 306159,
            "range": "± 14290",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1544224,
            "range": "± 86116",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 605675,
            "range": "± 44850",
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
          "id": "630b25eefe50fad986919fe604ed40f81b39ad6d",
          "message": "Add basic setter implementations to the ICU4XLocale FFI (#1055)",
          "timestamp": "2021-09-16T09:18:50-05:00",
          "tree_id": "60d51cf91d6c826177d1d5b209efa6c7e4baea3d",
          "url": "https://github.com/unicode-org/icu4x/commit/630b25eefe50fad986919fe604ed40f81b39ad6d"
        },
        "date": 1631802422786,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 375652,
            "range": "± 16865",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1899783,
            "range": "± 47404",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 700007,
            "range": "± 28138",
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
          "id": "a40b7bd39520ba58eb95bd211f539eeb22a3a842",
          "message": "Update the READMEs for FFI (#1054)",
          "timestamp": "2021-09-16T12:14:56-05:00",
          "tree_id": "ac0704750c7f4f7a3316c1037c156a64c9762ddf",
          "url": "https://github.com/unicode-org/icu4x/commit/a40b7bd39520ba58eb95bd211f539eeb22a3a842"
        },
        "date": 1631812971742,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 367943,
            "range": "± 17326",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1781812,
            "range": "± 81151",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 669888,
            "range": "± 25827",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "software@lfcode.ca",
            "name": "Jade",
            "username": "lf-"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5039bf90501011338aacb875d20f90ac116bce1b",
          "message": "Change \"us\" to μs in zerovec documentation (#1062)",
          "timestamp": "2021-09-16T14:22:10-07:00",
          "tree_id": "dc022abe4381792d19a0bd7b0992195b5831d346",
          "url": "https://github.com/unicode-org/icu4x/commit/5039bf90501011338aacb875d20f90ac116bce1b"
        },
        "date": 1631827774654,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 367325,
            "range": "± 3411",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1854331,
            "range": "± 15883",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 681924,
            "range": "± 10418",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "10595307+mildgravitas@users.noreply.github.com",
            "name": "mildgravitas",
            "username": "mildgravitas"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5b1bf8820b3c3759c34c418c3abeab0b8e58c015",
          "message": "Misc cleanup done for #488 (#1059)\n\n* Only compute DateTimeFormatsV1 once when building DatePatternsV1.\r\n\r\n* Remove unnecessary get_mut() in parse_placeholders.\r\n\r\n* Cleanup redundant test fixture: combine-datetime was forked from date-time 5 months ago but the test still referred to the old file.\r\n\r\n* Add a basic skeleton parsing test.\r\n\r\n* Update test_components_combine_datetime's comment to the correct path.",
          "timestamp": "2021-09-17T10:33:59-07:00",
          "tree_id": "5fad7650df25e12df2edcacf33108788e5d4e973",
          "url": "https://github.com/unicode-org/icu4x/commit/5b1bf8820b3c3759c34c418c3abeab0b8e58c015"
        },
        "date": 1631900510259,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 369443,
            "range": "± 7944",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1878235,
            "range": "± 64926",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 700766,
            "range": "± 17105",
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
          "id": "f19a64b4662d56c2588a592685e87c1e799c7f2a",
          "message": "Add data size benchmark: monitor size of file (#1051)\n\nprovider/testdata/data/testdata.postcard.\r\n\r\nRemove erroneous second print-out of results.\r\n\r\nMoves directory datasize/ with the output data under 'benchmarks/' directory,\r\nfrom 'benchmarks/binsize/'.",
          "timestamp": "2021-09-20T10:25:55-07:00",
          "tree_id": "470c8072eebfc55a2fad062841a147b4b49fdae1",
          "url": "https://github.com/unicode-org/icu4x/commit/f19a64b4662d56c2588a592685e87c1e799c7f2a"
        },
        "date": 1632159208163,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 382446,
            "range": "± 16317",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1911339,
            "range": "± 81627",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 709738,
            "range": "± 23418",
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
          "id": "04cfbd2684f1eaa0439ffe3e394fd496e6160a85",
          "message": "Remove provider_ppucd (#1069)",
          "timestamp": "2021-09-20T13:55:31-07:00",
          "tree_id": "c24ef6012e2105fa1ccf4a0cf686aa6ee105e542",
          "url": "https://github.com/unicode-org/icu4x/commit/04cfbd2684f1eaa0439ffe3e394fd496e6160a85"
        },
        "date": 1632171762295,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 358450,
            "range": "± 26296",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 2135864,
            "range": "± 126050",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 690520,
            "range": "± 52966",
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
          "id": "adf02273511847c7b3422a0916e05d8ac7c236ec",
          "message": "Add DataPayload::map_project functions (#920)",
          "timestamp": "2021-09-21T13:26:51-05:00",
          "tree_id": "24a3d122e6416385e322c8bfb13b5d2cb24bfacd",
          "url": "https://github.com/unicode-org/icu4x/commit/adf02273511847c7b3422a0916e05d8ac7c236ec"
        },
        "date": 1632249277580,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 330116,
            "range": "± 26639",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1824304,
            "range": "± 88046",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 655785,
            "range": "± 51586",
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
          "id": "007b532720771ef334a02d8c4704db6f689b8493",
          "message": "Restructure FFI folder (#1060)\n\n* Move all diplomat generated stuff into diplomat folder\n\n* Update readme\n\n* Update CI paths\n\n* Update tests\n\n* Add c readme\n\n* Update script\n\n* fix readmes\n\n* fix wasm",
          "timestamp": "2021-09-21T19:42:24-07:00",
          "tree_id": "684b960c5edb80e5e5f92613a3bf00cef648d7f9",
          "url": "https://github.com/unicode-org/icu4x/commit/007b532720771ef334a02d8c4704db6f689b8493"
        },
        "date": 1632278958949,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 310412,
            "range": "± 395",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1575772,
            "range": "± 5518",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 583499,
            "range": "± 565",
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
          "id": "fea0eb7cba1a52e4316b444edff46575f4639c94",
          "message": "More docs for ZeroMap and add Vec<u8> as supported type (#1057)",
          "timestamp": "2021-09-22T00:05:37-05:00",
          "tree_id": "e375ec6c540f24dc62f6fefe1339c992de9efbed",
          "url": "https://github.com/unicode-org/icu4x/commit/fea0eb7cba1a52e4316b444edff46575f4639c94"
        },
        "date": 1632287520923,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 313339,
            "range": "± 510",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1584471,
            "range": "± 1138",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 570850,
            "range": "± 459",
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
          "id": "a342c02c162fd3090fb10f5d4a5496a5f0ba427b",
          "message": "Upgrade default rust-toolchain to 1.56-beta.3 (#1085)",
          "timestamp": "2021-09-22T17:09:49-05:00",
          "tree_id": "18b607f90ad903dce9baa07d7a19adcc44754195",
          "url": "https://github.com/unicode-org/icu4x/commit/a342c02c162fd3090fb10f5d4a5496a5f0ba427b"
        },
        "date": 1632349045743,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 393101,
            "range": "± 32423",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 2023534,
            "range": "± 82321",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 748940,
            "range": "± 29557",
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
          "id": "c6ec54a4d7bb189161c11fd0cb8784a27066e782",
          "message": "Upgrade nightly version to 2021-09-22 (#1088)",
          "timestamp": "2021-09-22T19:31:17-05:00",
          "tree_id": "ec2512ee6bd90179a95e58722b1b4e14e0974b14",
          "url": "https://github.com/unicode-org/icu4x/commit/c6ec54a4d7bb189161c11fd0cb8784a27066e782"
        },
        "date": 1632357519421,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 357366,
            "range": "± 9800",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1792817,
            "range": "± 55038",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 659385,
            "range": "± 17483",
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
          "id": "877707aa532a45ec51c1f2a27140ce632ed6173d",
          "message": "Support for infinitely nesting VarZeroVecs (#1065)\n\n* Add SliceComponents::from_bytes_unchecked\r\n\r\n* Add VZVULE\r\n\r\n* Impl AsVarULE for VarZeroVec\r\n\r\n* Add basic readonly VarZeroVecOwned\r\n\r\n* Add mutation ops\r\n\r\n* Use VZVOwned in VZV\r\n\r\n* Add test\r\n\r\n* fix ci\r\n\r\n* no unsafe on from_byte_slice_unchecked_mut\r\n\r\n* try_from_bytes -> parse_byte_slice\r\n\r\n* Address some review comments\r\n\r\n* get issue number\r\n\r\n* require no padding bytes in VarULE\r\n\r\n* fix compile\r\n\r\n* rename try_from_bytes\r\n\r\n* fix tidy\r\n\r\n* satisfy clippy\r\n\r\n* safety comment",
          "timestamp": "2021-09-22T18:53:38-07:00",
          "tree_id": "8473207dcc7ba2bb216ef7a586e35bdd59e8fc58",
          "url": "https://github.com/unicode-org/icu4x/commit/877707aa532a45ec51c1f2a27140ce632ed6173d"
        },
        "date": 1632362473490,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 442095,
            "range": "± 23379",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 2202645,
            "range": "± 142703",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 780862,
            "range": "± 42829",
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
          "id": "ea4b7d9d7b2363dbcfd5d883e9890fe881ddea45",
          "message": "Adding BlobDataProvider for dynamically loaded data blobs (#1084)\n\n- Adds try_project_cloned_with_capture",
          "timestamp": "2021-09-23T09:54:39-05:00",
          "tree_id": "0215c38db8f7aa7793fc7798353b719114dd4b5f",
          "url": "https://github.com/unicode-org/icu4x/commit/ea4b7d9d7b2363dbcfd5d883e9890fe881ddea45"
        },
        "date": 1632409309143,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 351375,
            "range": "± 11736",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1740249,
            "range": "± 64050",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 631986,
            "range": "± 32905",
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
          "id": "292419806edd3c5a3e1e430f5bc820c296c8ee83",
          "message": "Clean up type fallback in TimeZonesProvider (#1063)\n\n* Clean up type fallback in TimeZonesProvider\r\n\r\nThe old way of handling type fallback in the DataProvider made assumptions\r\nabout the shape of the CLDR data (that I realize now were wrong, in at least one case).\r\n\r\nThis adds an explicit function to handle type fallback exactly as specified by UTS-35,\r\nimproving the readability of the code.\r\n\r\nUpdates the data and test case to fix the prior mistake.",
          "timestamp": "2021-09-23T14:08:46-07:00",
          "tree_id": "4a013e7a2297f7351f548b28a8b0813ac08abd0a",
          "url": "https://github.com/unicode-org/icu4x/commit/292419806edd3c5a3e1e430f5bc820c296c8ee83"
        },
        "date": 1632431731082,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 348741,
            "range": "± 10380",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1739478,
            "range": "± 39842",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 644882,
            "range": "± 18955",
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
          "id": "d821987ada3f86d2fbba8777c81184ba3fe71aa6",
          "message": "Move zerovec to no_std (#1094)\n\nMake zerovec no_std",
          "timestamp": "2021-09-23T14:36:42-07:00",
          "tree_id": "ab06701f9c750f3bb77f77b94a46425ce04801c3",
          "url": "https://github.com/unicode-org/icu4x/commit/d821987ada3f86d2fbba8777c81184ba3fe71aa6"
        },
        "date": 1632433448361,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 343546,
            "range": "± 17233",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1727180,
            "range": "± 98888",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 647177,
            "range": "± 26952",
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
          "id": "6dfdac6ae98bdd2d96d83c13f423a42c7f307b93",
          "message": "Switch DTF field symbols to use macros. (#1093)\n\n* Switch DTF field symbols to use macros.\r\n\r\n* Revert Minute changes",
          "timestamp": "2021-09-23T18:40:36-07:00",
          "tree_id": "a90826366e0d1cec72aaeef4a1c4311b3bda0835",
          "url": "https://github.com/unicode-org/icu4x/commit/6dfdac6ae98bdd2d96d83c13f423a42c7f307b93"
        },
        "date": 1632448091264,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 316004,
            "range": "± 481",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1583979,
            "range": "± 14156",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 572684,
            "range": "± 570",
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
          "id": "b890acff2184b6a8857a5193fd20cbf55b55cd6b",
          "message": "Make DTF PatternItem Copy (#1098)\n\n* Switch Skeletons to work on chars\r\n\r\n* Switch API of field type index operations to be more explicit.\r\n\r\n* Switch PatternItem::Literal to be a char",
          "timestamp": "2021-09-24T16:09:33-07:00",
          "tree_id": "aec04be38b879ace95b7434cb704bd2e92cde362",
          "url": "https://github.com/unicode-org/icu4x/commit/b890acff2184b6a8857a5193fd20cbf55b55cd6b"
        },
        "date": 1632525375683,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 267823,
            "range": "± 1127",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1367744,
            "range": "± 2256",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 518996,
            "range": "± 5305",
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
          "id": "3dd207490113fc19549732cc56f0d2a5ccdc5c36",
          "message": "Introduce GenericPattern (#1101)",
          "timestamp": "2021-09-24T16:29:25-07:00",
          "tree_id": "67fc59cc3d73e7880b3cc553a362d1685165a666",
          "url": "https://github.com/unicode-org/icu4x/commit/3dd207490113fc19549732cc56f0d2a5ccdc5c36"
        },
        "date": 1632526552601,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 266234,
            "range": "± 822",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1371123,
            "range": "± 13209",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 515531,
            "range": "± 1311",
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
          "id": "e7e6feb68c6415c802da0630065f31f216aec4f4",
          "message": "Update diplomat (#1100)",
          "timestamp": "2021-09-24T19:04:48-05:00",
          "tree_id": "3d287c3c8266df80d77053f983e82ff2104b928f",
          "url": "https://github.com/unicode-org/icu4x/commit/e7e6feb68c6415c802da0630065f31f216aec4f4"
        },
        "date": 1632528683794,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 312843,
            "range": "± 13263",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1622814,
            "range": "± 73423",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 626086,
            "range": "± 36308",
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
          "id": "49079e09fa7fb6baa998ed1bcc0949a8174dbbe7",
          "message": "Fix wasm-cpp-emscripten (#1099)",
          "timestamp": "2021-09-24T19:37:04-05:00",
          "tree_id": "51edaf9605a21b5d191b90ccf1836fcf4a8b7c7b",
          "url": "https://github.com/unicode-org/icu4x/commit/49079e09fa7fb6baa998ed1bcc0949a8174dbbe7"
        },
        "date": 1632530611042,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 265833,
            "range": "± 18336",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1328532,
            "range": "± 55801",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 518761,
            "range": "± 41109",
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
          "id": "5a5810de8ea75fe661ef9461735505ebde249b6b",
          "message": "Make it possible to use icu_calendar dates with DateTimeFormat (#1097)",
          "timestamp": "2021-09-25T10:10:24-07:00",
          "tree_id": "83bca2805cc0fb3702207b2d3d96683404e4e8fc",
          "url": "https://github.com/unicode-org/icu4x/commit/5a5810de8ea75fe661ef9461735505ebde249b6b"
        },
        "date": 1632590221336,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 273489,
            "range": "± 18037",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1348411,
            "range": "± 46909",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 539121,
            "range": "± 20340",
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
          "id": "47cea1b4b494bd723a63328fd09e3b72378cd785",
          "message": "Disable default features for postcard (#1096)\n\nCo-authored-by: Iain Ireland <iain.i.ireland@gmail.com>",
          "timestamp": "2021-09-25T22:44:50-07:00",
          "tree_id": "0bcb0186593b76087850351478cc5206987a513e",
          "url": "https://github.com/unicode-org/icu4x/commit/47cea1b4b494bd723a63328fd09e3b72378cd785"
        },
        "date": 1632635470350,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 266217,
            "range": "± 15621",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1310660,
            "range": "± 82233",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 500806,
            "range": "± 26145",
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
          "id": "3c4852e13703f829b31413e58e51090ed8f457d8",
          "message": "Set min version of cargo-make for diplomat-get-rev task (#1064)",
          "timestamp": "2021-09-26T17:27:26-07:00",
          "tree_id": "caac163edc4dcd0b6b7172ab9ace0971c16c1978",
          "url": "https://github.com/unicode-org/icu4x/commit/3c4852e13703f829b31413e58e51090ed8f457d8"
        },
        "date": 1632702854791,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 296651,
            "range": "± 14101",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1587601,
            "range": "± 67213",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 592943,
            "range": "± 21574",
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
          "id": "19e4a15de3a91d9ec74e6942ddbdfcaa63010062",
          "message": "Use ZeroVec in UnicodeSet (#922)",
          "timestamp": "2021-09-27T07:40:16-07:00",
          "tree_id": "eb9a545863c1db55745fbab0dd72b2f307ea246e",
          "url": "https://github.com/unicode-org/icu4x/commit/19e4a15de3a91d9ec74e6942ddbdfcaa63010062"
        },
        "date": 1632753987476,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 273248,
            "range": "± 933",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1399584,
            "range": "± 1849",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 512994,
            "range": "± 784",
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
          "id": "89d9c504c1fa8f444bb7cfd69b9f6b6fe7d7e927",
          "message": "Reorganize DTF pattern and move it to reference module. (#1111)",
          "timestamp": "2021-09-27T07:57:47-07:00",
          "tree_id": "93d602459b30b859a8c2b280a0ab21198e50b3ec",
          "url": "https://github.com/unicode-org/icu4x/commit/89d9c504c1fa8f444bb7cfd69b9f6b6fe7d7e927"
        },
        "date": 1632755067455,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 247677,
            "range": "± 19166",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1296379,
            "range": "± 79932",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 523542,
            "range": "± 35683",
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
          "id": "31497db60513ba6f34b6acf68c30ac0b710d8d55",
          "message": "Use/require byte equality in VarZeroVec (#1103)\n\n* Add equality and same-slice invariants to ULE traits\r\n\r\n* Add assert for same-slice invariant in VarZeroVec\r\n\r\n* use equality guarantee in vzv partialeq impl\r\n\r\n* Don't construct the SliceComponents for get_encoded_slice()\r\n\r\n* review fixes\r\n\r\n* Add default impls of as_byte_slice()\r\n\r\n* Add default impl for ULE::from_byte_slice_unchecked()",
          "timestamp": "2021-09-27T14:22:44-07:00",
          "tree_id": "f7ff2623ebd59b54c77ef005069b03d6ab8c143e",
          "url": "https://github.com/unicode-org/icu4x/commit/31497db60513ba6f34b6acf68c30ac0b710d8d55"
        },
        "date": 1632778159449,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 320699,
            "range": "± 8683",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1637883,
            "range": "± 36828",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 626607,
            "range": "± 14236",
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
          "id": "e7cbcdbd7fe129c8cfc41d374eb262767004dded",
          "message": "Separate out validation function for ULE to be able to derive parse_byte_slice (#1113)\n\n* Separate out validation function for ULE to be able to derive parse_byte_slice\r\n\r\n* Apply reviewers feedback\r\n\r\n* Add same for VarULE and document default impl for parse_byte_slice\r\n\r\n* Improve docs",
          "timestamp": "2021-09-27T17:06:34-07:00",
          "tree_id": "da488d2aa0bbebb65e865a20b928754af0ef4ebe",
          "url": "https://github.com/unicode-org/icu4x/commit/e7cbcdbd7fe129c8cfc41d374eb262767004dded"
        },
        "date": 1632788027204,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 316964,
            "range": "± 559",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1637334,
            "range": "± 7878",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 615403,
            "range": "± 1371",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "m_kato@ga2.so-net.ne.jp",
            "name": "Makoto Kato",
            "username": "makotokato"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5d367e88ef6a21ef055acff920ace6cec87c5b1c",
          "message": " Use Burmese LSTM for Burmese code point (#1045)\n\n* Add Burmese LTSM model.\r\n\r\n* Use Burmese LSTM model when codepoint is Burmese.\r\n\r\n* Fix clippy lints\r\n\r\n* Fix clippy lints\r\n\r\n* Add more codepoint range for Burmese.\r\n\r\n* Use for-loop intead of loop.\r\n\r\n* Add document for LanguageIterator.\r\n\r\n* Make LanguageIterator::new simple.\r\n\r\n* Add comment for Burmese test.\r\n\r\n* Use common function to detect language.\r\n\r\n* Remove TODO from test comments.",
          "timestamp": "2021-09-29T08:03:01+09:00",
          "tree_id": "c29c392138fade7da9a8305baed9dedce301ebdf",
          "url": "https://github.com/unicode-org/icu4x/commit/5d367e88ef6a21ef055acff920ace6cec87c5b1c"
        },
        "date": 1632870555370,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 264587,
            "range": "± 528",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1362070,
            "range": "± 35310",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 519272,
            "range": "± 941",
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
          "id": "30a5f4ad1417d5c1bd2189d139ac09b8726644f2",
          "message": "Introduce DTF runtime Pattern backed by ZV (#1112)\n\n* Introduce DTF runtime Pattern backed by ZV\r\n\r\n* Apply reviewers feedback\r\n\r\n* Make num_enum work with no_std\r\n\r\n* Apply reviewers feedback\r\n\r\n* Append final feedback",
          "timestamp": "2021-09-28T21:16:55-07:00",
          "tree_id": "0f1da621772ceece2c525267ba0490f2dfc2a7d3",
          "url": "https://github.com/unicode-org/icu4x/commit/30a5f4ad1417d5c1bd2189d139ac09b8726644f2"
        },
        "date": 1632889413986,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 265664,
            "range": "± 470",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1374674,
            "range": "± 1417",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 517194,
            "range": "± 880",
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
          "id": "848f20957de134138ff1a27d341105140df7d702",
          "message": "Remove name field from UnicodePropertyV1 (#1125)\n\nCo-authored-by: Iain Ireland <iain.i.ireland@gmail.com>",
          "timestamp": "2021-09-29T16:17:33-07:00",
          "tree_id": "f405cf70750c235e9fe642689195cad33d1d4f6f",
          "url": "https://github.com/unicode-org/icu4x/commit/848f20957de134138ff1a27d341105140df7d702"
        },
        "date": 1632957836765,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 266433,
            "range": "± 678",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1375534,
            "range": "± 1838",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 515906,
            "range": "± 1037",
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
          "id": "47f667eae48d6b8a446a36e1cae01b8eccf66881",
          "message": "Add ZeroVec::iter_mut() (#1128)\n\n* Add ZeroVec::iter_mut()\r\n\r\n* Add ZeroVec::for_each_mut()\r\n\r\n* Expose make_mut, remove iter_mut, expose try_for_each_mut\r\n\r\n* Update utils/zerovec/src/zerovec/mod.rs\r\n\r\nCo-authored-by: Shane F. Carr <shane@unicode.org>\r\n\r\n* apply suggestion\r\n\r\n* make_mut -> to_mut\r\n\r\n* fix\r\n\r\nCo-authored-by: Shane F. Carr <shane@unicode.org>",
          "timestamp": "2021-09-30T14:24:14-07:00",
          "tree_id": "d605158ffdabb5d6a61471cdcba0f8cff72a2da2",
          "url": "https://github.com/unicode-org/icu4x/commit/47f667eae48d6b8a446a36e1cae01b8eccf66881"
        },
        "date": 1633037481651,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 319574,
            "range": "± 14007",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1672652,
            "range": "± 53698",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 640349,
            "range": "± 20989",
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
          "id": "20100345fc28379015107c7b4aa8b3d088bd41c8",
          "message": "Move Display bound on ULE::Error for Serde (#1130)",
          "timestamp": "2021-09-30T18:41:01-05:00",
          "tree_id": "ae42e9daadea8e64133edc94add784ed47a65c9c",
          "url": "https://github.com/unicode-org/icu4x/commit/20100345fc28379015107c7b4aa8b3d088bd41c8"
        },
        "date": 1633045667526,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 331299,
            "range": "± 781",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1710002,
            "range": "± 16359",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 646964,
            "range": "± 1662",
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
          "id": "530279a1c48f8d3f7d18a9cdf4441ff862d00b13",
          "message": "Add BlobDataProvider FFI and async data loading test in JavaScript (#1104)",
          "timestamp": "2021-09-30T19:55:00-05:00",
          "tree_id": "6973996a72974b6eb04db1c3a219a6783962c106",
          "url": "https://github.com/unicode-org/icu4x/commit/530279a1c48f8d3f7d18a9cdf4441ff862d00b13"
        },
        "date": 1633050130802,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 317193,
            "range": "± 1174",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1640414,
            "range": "± 12868",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 618226,
            "range": "± 5984",
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
          "id": "d11c8404177eb0f762cda93d376a772e08f932bd",
          "message": "Move CodePointTrie to components/utils (#1114)",
          "timestamp": "2021-10-01T07:32:54-07:00",
          "tree_id": "7de796383da4b4e00fcceb88c6ed0876b7e20168",
          "url": "https://github.com/unicode-org/icu4x/commit/d11c8404177eb0f762cda93d376a772e08f932bd"
        },
        "date": 1633099210543,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 266674,
            "range": "± 675",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1378504,
            "range": "± 1554",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 524188,
            "range": "± 3825",
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
          "id": "4aaac96425b8a9de9f4566c1c2d2453f5654d5ba",
          "message": "Update property enums to support CodepointTrie (#1089)\n\n* Make GC exhaustive\r\n\r\nGeneralCategory will never be extended\r\nSee https://www.unicode.org/policies/stability_policy.html\r\n\r\n* Add GeneralSubcategory to represent raw GC data\r\n\r\n* Implement AsULE for GeneralSubcategory\r\n\r\n* Cargo fmt\r\n\r\n* refactor\r\n\r\n* Make GC repr(u8)\r\n\r\n* Cargo fmt\r\n\r\n* Convert Script to an identifier\r\n\r\n* Implement AsULE for Script\r\n\r\n* Implement validate_byte_slice instead of parse_byte_slice\r\n\r\n* Impl From<GeneralSubcategory> for GeneralCategory\r\n\r\n* Remove default-implemented ULE methods\r\n\r\n* Add safety comment on GeneralSubcategoryULE impl\r\n\r\nCo-authored-by: Iain Ireland <iain.i.ireland@gmail.com>",
          "timestamp": "2021-10-01T09:51:24-07:00",
          "tree_id": "bd7e6e9c4a851a30a6b8594d1eebf7018aa48c0d",
          "url": "https://github.com/unicode-org/icu4x/commit/4aaac96425b8a9de9f4566c1c2d2453f5654d5ba"
        },
        "date": 1633107462727,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 273098,
            "range": "± 15663",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1362173,
            "range": "± 72104",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 504430,
            "range": "± 25956",
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
          "id": "209ddc7aaf9c78c646da3add611dfabc6f82c1d8",
          "message": "Start splitting gregory provider data (#1137)\n\n* Start splitting gregory provider data\r\n\r\n* Fix imports from symbols for provider",
          "timestamp": "2021-10-01T10:13:11-07:00",
          "tree_id": "a9d0ade0dcb8f3c3d05669dc3fa7e0b1edac2790",
          "url": "https://github.com/unicode-org/icu4x/commit/209ddc7aaf9c78c646da3add611dfabc6f82c1d8"
        },
        "date": 1633108792027,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 283302,
            "range": "± 28600",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1495398,
            "range": "± 108531",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 543900,
            "range": "± 33272",
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
          "id": "e5067da312ef0cd0b31894e82239be70a91f5602",
          "message": "Clean up DTF Skeleton. (#1132)\n\n* Clean up DTF Skeleton.\r\n\r\n* Fix fmt\r\n\r\n* Fix clippy\r\n\r\n* Fix regression in prefer_matched_pattern",
          "timestamp": "2021-10-01T10:18:36-07:00",
          "tree_id": "5d38e239551c13728f04748c2f3734200a7a2c78",
          "url": "https://github.com/unicode-org/icu4x/commit/e5067da312ef0cd0b31894e82239be70a91f5602"
        },
        "date": 1633109121871,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 265953,
            "range": "± 652",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1372528,
            "range": "± 1840",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 521503,
            "range": "± 870",
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
          "id": "bb2f1e279675e374233d260112cb2f12125d2a05",
          "message": "Set default datagen log level to INFO (#1138)\n\n* Update SimpleLogger\r\n\r\n* Set default datagen log level to INFO",
          "timestamp": "2021-10-01T15:32:21-07:00",
          "tree_id": "c94f15127bd233e1c1d5599dc77708926b499f0c",
          "url": "https://github.com/unicode-org/icu4x/commit/bb2f1e279675e374233d260112cb2f12125d2a05"
        },
        "date": 1633127969449,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 315388,
            "range": "± 17694",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1582619,
            "range": "± 65615",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 609666,
            "range": "± 26012",
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
          "id": "d169c30a21eb8a784801ee88a90a741f8da86902",
          "message": "Make datagen only need one -v option (#1141)",
          "timestamp": "2021-10-01T18:51:49-07:00",
          "tree_id": "912ce99f90576952386f5034a5137d53dc9b14d2",
          "url": "https://github.com/unicode-org/icu4x/commit/d169c30a21eb8a784801ee88a90a741f8da86902"
        },
        "date": 1633139925612,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 317624,
            "range": "± 1395",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1635006,
            "range": "± 10621",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 622548,
            "range": "± 2575",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kupiakos@gmail.com",
            "name": "Alyssa Haroldsen",
            "username": "kupiakos"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "e6245c2001931f22c60fd889f9523e6830597d1e",
          "message": "Implement and consolidate VarZeroVec mutation operations (#1127)\n\n* Implement and consolidate VarZeroVec mutation operations\r\n\r\nThis consolidates all bit-shifting mutation operations into a single\r\nfunction, since there's significant shared logic.\r\n\r\nFixes #1080.\r\n\r\nCo-authored-by: Manish Goregaokar <manishsmail@gmail.com>",
          "timestamp": "2021-10-02T00:48:26-07:00",
          "tree_id": "3de484e2b5cf81706a5b2e80bed524f52139229d",
          "url": "https://github.com/unicode-org/icu4x/commit/e6245c2001931f22c60fd889f9523e6830597d1e"
        },
        "date": 1633161294320,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 266360,
            "range": "± 10477",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1375504,
            "range": "± 73541",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 523303,
            "range": "± 21051",
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
          "id": "75b01f9f4939969ab7bb468f452a5c50dfe15fab",
          "message": "Move package.json up one level and add better path resolution (#1143)",
          "timestamp": "2021-10-03T00:25:39-05:00",
          "tree_id": "3f80725bdb49a01ffef0e81beec81e8f4ecffc1a",
          "url": "https://github.com/unicode-org/icu4x/commit/75b01f9f4939969ab7bb468f452a5c50dfe15fab"
        },
        "date": 1633239372577,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 301729,
            "range": "± 14694",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1557816,
            "range": "± 46587",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 606069,
            "range": "± 24270",
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
          "id": "f7f0bc43874d7d0b24f44011498f83ef55c1d0ce",
          "message": "Add try_map_project_[cloned]_with_capture (#1135)",
          "timestamp": "2021-10-03T00:28:14-05:00",
          "tree_id": "21fd95b724da056d4e2cbb3184864e82a70b9815",
          "url": "https://github.com/unicode-org/icu4x/commit/f7f0bc43874d7d0b24f44011498f83ef55c1d0ce"
        },
        "date": 1633239480330,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 278054,
            "range": "± 21469",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1462745,
            "range": "± 127368",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 611298,
            "range": "± 71575",
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
          "id": "2d64d81d4661090c0d446b7bd700e65035840a5e",
          "message": "Update tracking bug for removal of YokeTraitHack (#1134)",
          "timestamp": "2021-10-03T00:29:22-05:00",
          "tree_id": "3b946fe94fb1248550596d343b88e32157cc0a96",
          "url": "https://github.com/unicode-org/icu4x/commit/2d64d81d4661090c0d446b7bd700e65035840a5e"
        },
        "date": 1633239613130,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 345825,
            "range": "± 14127",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1798535,
            "range": "± 87039",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 696528,
            "range": "± 32975",
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
          "id": "7ab53189bd43c03a9baf57a532b7fe714c97a26b",
          "message": "Add runtime GenericPattern to DTF. (#1120)\n\n* Add runtime GenericPattern to DTF.\r\n\r\n* Apply reviewers feedback\r\n\r\n* Apply reviewers feedback\r\n\r\n* Fix comment\r\n\r\n* Make PatternULE repr transparent",
          "timestamp": "2021-10-03T18:28:10-07:00",
          "tree_id": "d07e48c0a56e2e0f03e50b3cfbb72bf98d07a9f7",
          "url": "https://github.com/unicode-org/icu4x/commit/7ab53189bd43c03a9baf57a532b7fe714c97a26b"
        },
        "date": 1633311294399,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 251785,
            "range": "± 12026",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1305780,
            "range": "± 68556",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 520020,
            "range": "± 37957",
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
          "id": "2528fe16843a31c74f91e30d20bffefc64a1d01f",
          "message": "Check for lengths in ULE and revise safety docs (#1121)",
          "timestamp": "2021-10-03T21:06:58-07:00",
          "tree_id": "af79dd21fd558d4af6ea2c1793c3a0d39be72943",
          "url": "https://github.com/unicode-org/icu4x/commit/2528fe16843a31c74f91e30d20bffefc64a1d01f"
        },
        "date": 1633320841874,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 303383,
            "range": "± 38502",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1528348,
            "range": "± 82915",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 593714,
            "range": "± 38349",
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
          "id": "8a8c0ab9dcd24b0e6af8d834bb23dabfb87c6ceb",
          "message": "Remove AsVarULE (#1126)",
          "timestamp": "2021-10-04T09:37:48-07:00",
          "tree_id": "ce1a3085cb8add5f554ee2da3002d2e7cda97244",
          "url": "https://github.com/unicode-org/icu4x/commit/8a8c0ab9dcd24b0e6af8d834bb23dabfb87c6ceb"
        },
        "date": 1633365910202,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 313927,
            "range": "± 29422",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1580879,
            "range": "± 87683",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 587130,
            "range": "± 61065",
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
          "id": "e6582015d32d370e5a673c61dbb46aed727bf194",
          "message": "Add UleError type; use for PlainOldULE and char conversions (#1147)",
          "timestamp": "2021-10-04T13:29:27-07:00",
          "tree_id": "a717317fc89f3bafa82f258e2f07a020b6cffd26",
          "url": "https://github.com/unicode-org/icu4x/commit/e6582015d32d370e5a673c61dbb46aed727bf194"
        },
        "date": 1633379782241,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 273514,
            "range": "± 9021",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1415926,
            "range": "± 30579",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 544345,
            "range": "± 13437",
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
          "id": "9b1462728cb5b7cd6d739a50728400810b2975fb",
          "message": "Add doc discussing enumerations versus identifiers (#1052)",
          "timestamp": "2021-10-04T13:32:45-07:00",
          "tree_id": "0f12ec29da7e5da026d833e661542ec159dbd76e",
          "url": "https://github.com/unicode-org/icu4x/commit/9b1462728cb5b7cd6d739a50728400810b2975fb"
        },
        "date": 1633380043886,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 325706,
            "range": "± 19471",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1712382,
            "range": "± 91704",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 654739,
            "range": "± 68285",
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
          "id": "807116bfedf24c04d6bd7987431eb6b2ddc8434f",
          "message": "Separate SkeletonPatterns into its own key (#1139)\n\n* Separate SkeletonPatterns into its own key\r\n\r\n* Fix test\r\n\r\n* Make cargo clippy happy",
          "timestamp": "2021-10-04T15:54:54-07:00",
          "tree_id": "de502eed1d3bc762828b7b391506edb1a59bb162",
          "url": "https://github.com/unicode-org/icu4x/commit/807116bfedf24c04d6bd7987431eb6b2ddc8434f"
        },
        "date": 1633388524501,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 92659,
            "range": "± 1070",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1421054,
            "range": "± 1495",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 282747,
            "range": "± 465",
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
          "id": "ebff2635aa4a83811d34187eba085f9976ebe56f",
          "message": "Apply last round of feedback on splitting DTF Skeletons.",
          "timestamp": "2021-10-04T15:58:19-07:00",
          "tree_id": "c379d6674a2b2a961ba6245cf4aa845026ebf2a6",
          "url": "https://github.com/unicode-org/icu4x/commit/ebff2635aa4a83811d34187eba085f9976ebe56f"
        },
        "date": 1633388751690,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 122221,
            "range": "± 12347",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1726550,
            "range": "± 59820",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 353769,
            "range": "± 24629",
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
          "id": "5da378032b6a98225436c476938744a6156da8b8",
          "message": "Make clippy happy",
          "timestamp": "2021-10-04T15:59:31-07:00",
          "tree_id": "a827f99d580f3c336dc91b5524b74f7c9d1a9f03",
          "url": "https://github.com/unicode-org/icu4x/commit/5da378032b6a98225436c476938744a6156da8b8"
        },
        "date": 1633388800936,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 125381,
            "range": "± 5715",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1827914,
            "range": "± 115471",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 389241,
            "range": "± 24081",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "robertbastian@users.noreply.github.com",
            "name": "Robert Bastian",
            "username": "robertbastian"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5a7cd0430f4ca3256d4cea75b57af08ae992a54d",
          "message": "Add initial FormattedStringBuilder and ListFormatter to experimental (#1053)",
          "timestamp": "2021-10-06T08:19:09-07:00",
          "tree_id": "be1442c6796b1debe1f1554c63babca0682e2424",
          "url": "https://github.com/unicode-org/icu4x/commit/5a7cd0430f4ca3256d4cea75b57af08ae992a54d"
        },
        "date": 1633533968874,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 96786,
            "range": "± 4757",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1432932,
            "range": "± 57837",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 287604,
            "range": "± 16064",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "samchen61661@gmail.com",
            "name": "samchen",
            "username": "samchen61661"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "a289b6a80fc83817bf6e1ab0323e55a5a8fad7c1",
          "message": "Test fixture data should be easier to repeat with multiple locales (#1152)",
          "timestamp": "2021-10-07T14:09:44-07:00",
          "tree_id": "2b4a22079923449cbcf1eddf01472e7bc978dcae",
          "url": "https://github.com/unicode-org/icu4x/commit/a289b6a80fc83817bf6e1ab0323e55a5a8fad7c1"
        },
        "date": 1633641396755,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 80576,
            "range": "± 1067",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1240849,
            "range": "± 9825",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 248503,
            "range": "± 1115",
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
          "id": "9dfd3d5a50eb6cf6cc557959b6a9ab57bd41d6dd",
          "message": "Add humanized serde serialization to DTF runtime pattern (#1157)\n\n* Add humanized serde serialization to DTF runtime pattern\r\n\r\n* Switch to use ZeroVec serialize/deserialize for binary path\r\n\r\n* Use helper struct\r\n\r\n* Put serde into module",
          "timestamp": "2021-10-08T09:17:06-07:00",
          "tree_id": "3f7a2eddcf6782ea0215a0f4cff730f58b2fb562",
          "url": "https://github.com/unicode-org/icu4x/commit/9dfd3d5a50eb6cf6cc557959b6a9ab57bd41d6dd"
        },
        "date": 1633710244890,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 90306,
            "range": "± 149",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1424234,
            "range": "± 2175",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 277161,
            "range": "± 3146",
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
          "id": "f0332973d5b685ce81b1172b1cf9dd2d121994f5",
          "message": "Add support for generics in derive(Yokeable, ZeroCopyFrom) (#1162)\n\n* Add ability to use generics with derive(Yokeable) for non-lifetime types\n\n* Start testing ZeroCopyFrom on types without lifetime parameters\n\n* Add ability to use derive(ZCF) for non-lifetime types with generics\n\n* Support generics for non-manually-proven Yokeable covariance\n\n* Support type param bounds on derive(Yokeable)\n\n* Add check_type_for_parameters\n\n* Allow generics on derive(ZCF) with lifetimes\n\n* Add tests for yokeability\n\n* Support generics in prove_covariance_manually\n\n* Also make sure ZCF works\n\n* Support non-lifetimed fields better\n\n* Add comments",
          "timestamp": "2021-10-08T10:51:10-07:00",
          "tree_id": "6b5628b11b306939c2e883b260998000dc89f5d3",
          "url": "https://github.com/unicode-org/icu4x/commit/f0332973d5b685ce81b1172b1cf9dd2d121994f5"
        },
        "date": 1633715916022,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 112234,
            "range": "± 12230",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1548581,
            "range": "± 58168",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 314284,
            "range": "± 11928",
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
          "id": "c0c1380a7cf4ab82fe0dd952c86b16606e95e247",
          "message": "Reorganize skeleton and separate runtime (#1156)\n\n* Reorganize skeleton and separate runtime\r\n\r\n* Update based on the design from Pattern serde",
          "timestamp": "2021-10-08T12:04:06-07:00",
          "tree_id": "93bfcd68c3680c4a92255bf18d27fa5947b11a8f",
          "url": "https://github.com/unicode-org/icu4x/commit/c0c1380a7cf4ab82fe0dd952c86b16606e95e247"
        },
        "date": 1633720287235,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 116398,
            "range": "± 6614",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1698073,
            "range": "± 36034",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 331416,
            "range": "± 2189",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "alansliu@gmail.com",
            "name": "Alan Liu",
            "username": "poulsbo"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "317a263fcd60805bc5c4c468eab2513e3087fd33",
          "message": "Add benches for ZeroMap::get, also litemap/HashMap for comparison (#1087)",
          "timestamp": "2021-10-08T16:22:50-07:00",
          "tree_id": "64f429b38d3eef556294ec7e2a7c5e20091ef267",
          "url": "https://github.com/unicode-org/icu4x/commit/317a263fcd60805bc5c4c468eab2513e3087fd33"
        },
        "date": 1633735799345,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 115117,
            "range": "± 3472",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1685714,
            "range": "± 50647",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 323129,
            "range": "± 6229",
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
          "id": "a3ef861aa8531297b04b93104b8e774e0ed0d1f5",
          "message": "Add FromIterator impl for ZeroVec (#1169)\n\n* Add FromIterator impl for ZeroVec\r\n\r\n* fmt",
          "timestamp": "2021-10-12T13:06:11-07:00",
          "tree_id": "4c5d61e4adde2ac1da042e1e25cd3cba1229a627",
          "url": "https://github.com/unicode-org/icu4x/commit/a3ef861aa8531297b04b93104b8e774e0ed0d1f5"
        },
        "date": 1634069621958,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 113635,
            "range": "± 5752",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1544304,
            "range": "± 65863",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 317545,
            "range": "± 18939",
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
          "id": "651f113cdcf85f620b56793f6b5c8b5c865a9a53",
          "message": "Add doc comments to icu_uniset (#1154)\n\n* Add doc comments to icu_uniset\r\n\r\n* More detailed doc comments for UnicodeSetError",
          "timestamp": "2021-10-12T15:02:45-07:00",
          "tree_id": "b0ce9939c014e7921227ed0a22f8cfa2496b280c",
          "url": "https://github.com/unicode-org/icu4x/commit/651f113cdcf85f620b56793f6b5c8b5c865a9a53"
        },
        "date": 1634076598460,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 110276,
            "range": "± 1832",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1678283,
            "range": "± 24244",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 329360,
            "range": "± 5899",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "m_kato@ga2.so-net.ne.jp",
            "name": "Makoto Kato",
            "username": "makotokato"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "9c8afa504fed91f346a256ab216079663208c6bb",
          "message": "Move Latin1Indices and Utf16Indices to its own file (#1165)\n\n* Move *Indices to own file to use other segmenter.\r\n\r\n* Fix per review comment.\r\n\r\n* Add comment for surrogate pair.",
          "timestamp": "2021-10-13T08:45:17+09:00",
          "tree_id": "70e3d274a5ab85bf9be4d4e9e47e0c86ee7285c7",
          "url": "https://github.com/unicode-org/icu4x/commit/9c8afa504fed91f346a256ab216079663208c6bb"
        },
        "date": 1634082774330,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 111866,
            "range": "± 2335",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1761548,
            "range": "± 35783",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 338486,
            "range": "± 9117",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "10595307+mildgravitas@users.noreply.github.com",
            "name": "mildgravitas",
            "username": "mildgravitas"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "42a820bbfd30971156674121e28ee29b7422e5d6",
          "message": "Week of month/year arithmetic & plural patterns for datetime (#918)\n\n* Add test data for the filipino locale.\r\n\r\nOnly fil, hy & ps have different patterns for singular vs plural week of month/year.\r\n\r\n* Add FieldSymbol::Week.\r\n\r\nThis is wired into skeleton parsing but not date formatting.\r\n\r\n* Define plural variants for Pattern.\r\n\r\n* Change SkeletonsV1 to support plural variants for patterns.\r\n\r\n* Plural dateFormatItem variant parsing for CLDR.\r\n\r\n* Arithmetic for computing the week of month/week of year.\r\n\r\n* Add partial week of year support to LocalizedDateTimeInput.\r\n\r\nThe latter is still missing a means to inject calendar information for\r\ne.g. min_days.\r\n\r\n* Propagate PatternPlurals throughout datetime.\r\n\r\nThis has the side effect of cursory week-of-year support since the\r\nplural code cannot be tested without it.\r\n\r\nCalendar integration is needed for full (non-ISO week counting)\r\nweek-of-x support.\r\n\r\n* fix(datetime): benchmark build (missing plural provider).\r\n\r\n* chore: Make parse_plural_category() an associated function of PluralCategory.\r\n\r\n* refactor(datetime): Use serde's kebab-case feature rather than manual names for Week serialization.\r\n\r\n* docs(datetime): document PluralPattern & PatternPlurals methods.\r\n\r\n* refactor: Directly (de)serialize Week for PluralPattern::pivot_week rather than go through FieldSymbol.\r\n\r\n* refactor(datetime): use Either instead of custom iterators for datetime's PatternPlurals iterators.\r\n\r\n* fix(datetime): add context to expect_pattern & expect_pattern_ref's error messages.\r\n\r\n* docs(cldr): explain plural variant parsing.\r\n\r\n* docs(arithmetic): Add docstrings for week_of types.\r\n\r\n* refactor(datetime): use for x in y {} rather than for_each in a few places.\r\n\r\n* perf: In PluralPatterns elide patterns that are duplicates of the 'Other' pattern.\r\n\r\nThis improves the datetime_lengths bench by decreasing deserialization time.\r\n\r\n* refactor(arithmetic): rename utils.rs to arithmetic.rs & add comments\r\n\r\n* refactor(datetime): Use a LiteMap rather than a tuple vec for PatternPlural::variants\r\n\r\n* fix: cleanup clippy linter warnings\r\n\r\n* fix(datetime): use default-features = false for the either dependency\r\n\r\n* refactor(datetime): make select_pattern a method of PatternPlurals.\r\n\r\n* fix(plurals): remove serde renames from PluralCategory.\r\n\r\nThese were added to make json dumps look more like TR35 but are\r\notherwise non-functional.\r\n\r\n* fix(datetime): add custom serialization to PatternPluralsV1 to make json dumps more readable.\r\n\r\n* doces(cldr): use json rather than xml in the example\r\n\r\n* perf(datetime): Normalize singleton PatternPlurals::MultipleVariants into PatternPlurals::SinglePatterns.\r\n\r\n* refactor(datetime): Make PluralPattern::get infaillable & check that PluralPatterns have a Other pattern in PluralPattern::normalize.\r\n\r\n* docs(cldr): use json rather than xml in the example\r\n\r\n* docs(datetime): restore the unintentionally removed docs for TimeZoneName.\r\n\r\n* style(calendar): fix clippy linter error\r\n\r\n* style(datetime): fix clippy linter warnings\r\n\r\n* refactor: remove unused serde renames & explicit enum values.",
          "timestamp": "2021-10-15T07:43:33-07:00",
          "tree_id": "13ba0ee9c979ae2dd518f7d1d69952977c4def28",
          "url": "https://github.com/unicode-org/icu4x/commit/42a820bbfd30971156674121e28ee29b7422e5d6"
        },
        "date": 1634309462895,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 113285,
            "range": "± 3083",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1815342,
            "range": "± 56860",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 328466,
            "range": "± 3379",
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
          "id": "a430f66c32040c3ac1e055620ee3f5c6515ab1e8",
          "message": "Adds UnicodePropertyMapV1 data struct for enumerated properties (#1161)\n\n* Rename TrieTypeEnum to TrieType\r\n\r\nTrieType no longer exists, so we don't need an awkward name for TrieTypeEnum.\r\n\r\n* Implement Yokeable/ZeroCopyFrom for CodePointTrie and data struct\r\n\r\n* Cargo fmt + minor fixes\r\n\r\n* Rebase on yoke-generics\r\n\r\n* Add doc comments\r\n\r\n* Address feedback\r\nFunny how everyone is talking about C9 having an easy route and not GenG lmao.\r\n\r\n* Add additional derives\r\n\r\n* Update comment on DATA_GET_ERROR_VALUE\r\n\r\nCo-authored-by: Shane F. Carr <shane@unicode.org>\r\n\r\n* Cargo fmt\r\n\r\nCo-authored-by: Shane F. Carr <shane@unicode.org>",
          "timestamp": "2021-10-18T14:43:18-07:00",
          "tree_id": "b35ea471e60581d01924e11d6dd7d1f10d642fcc",
          "url": "https://github.com/unicode-org/icu4x/commit/a430f66c32040c3ac1e055620ee3f5c6515ab1e8"
        },
        "date": 1634594002188,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 117478,
            "range": "± 5137",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1763033,
            "range": "± 78028",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 344504,
            "range": "± 16710",
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
          "id": "b09e5ea677f74c241dad129a305a05bc2dc3eead",
          "message": "Add testing section to CONTRIBUTING.md (#1164)\n\n* Add ci-job-clippy\n\n* Add testing section to CONTRIBUTING.md\n\n* node 14\n\n* review fix\n\n* Update CONTRIBUTING.md\n\nCo-authored-by: Shane F. Carr <shane@unicode.org>\n\n* Update CONTRIBUTING.md\n\nCo-authored-by: Shane F. Carr <shane@unicode.org>\n\n* Update CONTRIBUTING.md\n\nCo-authored-by: Shane F. Carr <shane@unicode.org>\n\n* Update Makefile.toml\n\nCo-authored-by: Elango <elango@unicode.org>\n\n* Update CONTRIBUTING.md\n\nCo-authored-by: Elango <elango@unicode.org>\n\nCo-authored-by: Shane F. Carr <shane@unicode.org>\nCo-authored-by: Elango <elango@unicode.org>",
          "timestamp": "2021-10-18T15:16:26-07:00",
          "tree_id": "9932d4ce0b08bd9417f74d638c04eec088480e5f",
          "url": "https://github.com/unicode-org/icu4x/commit/b09e5ea677f74c241dad129a305a05bc2dc3eead"
        },
        "date": 1634595837483,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 115905,
            "range": "± 3862",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1783284,
            "range": "± 76401",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 351005,
            "range": "± 14636",
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
          "id": "1579874c154691fb2232fdd13ef178f833f313d6",
          "message": "Document ZeroVec's variants (#1182)\n\n* Add ZeroVec::borrowed_from_slice()\r\n\r\n* move docs to borrowed\r\n\r\n* review comment\r\n\r\n* Update utils/zerovec/src/zerovec/mod.rs\r\n\r\nCo-authored-by: Shane F. Carr <shane@unicode.org>\r\n\r\n* Update utils/zerovec/src/zerovec/mod.rs\r\n\r\nCo-authored-by: Shane F. Carr <shane@unicode.org>\r\n\r\nCo-authored-by: Shane F. Carr <shane@unicode.org>",
          "timestamp": "2021-10-18T15:44:27-07:00",
          "tree_id": "2dfcaeb172ad4dd7603626c64b2b1227da65558b",
          "url": "https://github.com/unicode-org/icu4x/commit/1579874c154691fb2232fdd13ef178f833f313d6"
        },
        "date": 1634597497594,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 93629,
            "range": "± 9064",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1378579,
            "range": "± 78843",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 275367,
            "range": "± 8301",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "m_kato@ga2.so-net.ne.jp",
            "name": "Makoto Kato",
            "username": "makotokato"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "8a016a0398676d1d7316cab83667a92983f903f7",
          "message": "Cleanup linebreak segmenter specification test to use other segmenters (#1168)\n\n* Cleanup specification test for linebreak since other specification tests will use same test format.\r\n\r\nUAX#29 test data uses same format for UAX#14. So I would like to clean up it\r\nto use other tests for UAX#29.\r\n\r\n* Fix per review comment.",
          "timestamp": "2021-10-19T13:05:29+09:00",
          "tree_id": "58ea476df51df851563e4c8d874194ddd6ccc018",
          "url": "https://github.com/unicode-org/icu4x/commit/8a016a0398676d1d7316cab83667a92983f903f7"
        },
        "date": 1634616702994,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 91076,
            "range": "± 159",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1484886,
            "range": "± 5718",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 277398,
            "range": "± 338",
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
          "id": "dddfb63ad54ad7fc32fea8e728e487415d955caa",
          "message": "Add small comment to ZeroVec::from_iter (#1185)",
          "timestamp": "2021-10-19T00:49:42-07:00",
          "tree_id": "872018bb3b34ce9e8dedd10014cb143fc624f4e7",
          "url": "https://github.com/unicode-org/icu4x/commit/dddfb63ad54ad7fc32fea8e728e487415d955caa"
        },
        "date": 1634630192274,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 109554,
            "range": "± 369",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1770977,
            "range": "± 19709",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 332657,
            "range": "± 941",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "samchen61661@gmail.com",
            "name": "samchen",
            "username": "samchen61661"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c7c9535f18567ff6b32b02c8c3b042fe9aec6680",
          "message": "Likely subtags minimization of und should be en (#1170)",
          "timestamp": "2021-10-19T10:20:00-05:00",
          "tree_id": "8f35470d047227f9e1dd61a99f0763ff54b7bae3",
          "url": "https://github.com/unicode-org/icu4x/commit/c7c9535f18567ff6b32b02c8c3b042fe9aec6680"
        },
        "date": 1634657213160,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 112027,
            "range": "± 4323",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1727844,
            "range": "± 30811",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 323076,
            "range": "± 5277",
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
          "id": "30f0687631686e6b2d0a2dd3796ef34b06d5d08f",
          "message": "Refactor properties to separate crate (#1153)",
          "timestamp": "2021-10-19T17:57:17-07:00",
          "tree_id": "3e662d624ef229e9abd9c6d37a43796578f1861e",
          "url": "https://github.com/unicode-org/icu4x/commit/30f0687631686e6b2d0a2dd3796ef34b06d5d08f"
        },
        "date": 1634691927892,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 112029,
            "range": "± 15524",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1706686,
            "range": "± 106642",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 342349,
            "range": "± 26373",
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
          "id": "82ea03a765305fdf75af1974ba7251562c81a499",
          "message": "Basic impl of EncodeAsVarULE (#1173)",
          "timestamp": "2021-10-20T07:18:12-07:00",
          "tree_id": "dc92259200a2dd9ec9ac4e358390c98fe3832fc6",
          "url": "https://github.com/unicode-org/icu4x/commit/82ea03a765305fdf75af1974ba7251562c81a499"
        },
        "date": 1634739958436,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 118351,
            "range": "± 10564",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1802571,
            "range": "± 151983",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 360175,
            "range": "± 21683",
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
          "id": "81aaa42a07cc1a9342dbdeb8014012f96e542acc",
          "message": "Make ULE types Copy, add PairULE (#1193)\n\n* Add PairULE\r\n\r\n* Make ULE Copy\r\n\r\n* take AsULE values by-copy",
          "timestamp": "2021-10-20T12:57:06-07:00",
          "tree_id": "7b0bc17f0356b6545d9995af465d04c7d8ec89d2",
          "url": "https://github.com/unicode-org/icu4x/commit/81aaa42a07cc1a9342dbdeb8014012f96e542acc"
        },
        "date": 1634760265179,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 114373,
            "range": "± 3432",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1811378,
            "range": "± 52064",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 344904,
            "range": "± 14162",
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
          "id": "2611e3a833eda73f77fdf394c5a932e6821f236c",
          "message": "Bump ZeroVec to 0.4 (#1194)",
          "timestamp": "2021-10-20T13:22:39-07:00",
          "tree_id": "0f0f19f8632fec2e3b253385e435861422bc4109",
          "url": "https://github.com/unicode-org/icu4x/commit/2611e3a833eda73f77fdf394c5a932e6821f236c"
        },
        "date": 1634761773365,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 114014,
            "range": "± 7275",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1745406,
            "range": "± 216651",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 318042,
            "range": "± 21124",
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
          "id": "3ea4f805888821d2ca52d821a868806ae2fa8abb",
          "message": "CodePointTrie data provider  (#1167)",
          "timestamp": "2021-10-20T18:01:02-07:00",
          "tree_id": "5719596148d5f910e042100cf339eee2626add21",
          "url": "https://github.com/unicode-org/icu4x/commit/3ea4f805888821d2ca52d821a868806ae2fa8abb"
        },
        "date": 1634778538159,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 113317,
            "range": "± 4381",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1650645,
            "range": "± 79701",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 337313,
            "range": "± 17347",
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
          "id": "40817d26fc0f58b71d03c5a23d20045c77c09bf9",
          "message": "Tidy up DTF PluralPatterns for ZeroVec runtime data (#1178)",
          "timestamp": "2021-10-20T19:23:18-07:00",
          "tree_id": "d2e92f61a8aed3cef4b2211697a005676c880f5d",
          "url": "https://github.com/unicode-org/icu4x/commit/40817d26fc0f58b71d03c5a23d20045c77c09bf9"
        },
        "date": 1634783445933,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 117948,
            "range": "± 5381",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1875315,
            "range": "± 103420",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 332774,
            "range": "± 22766",
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
          "id": "8aa98be2e3cb650a89d36361ebd516cb9e973b57",
          "message": "Fix clippy warnings in yoke (#1200)",
          "timestamp": "2021-10-21T09:10:36-07:00",
          "tree_id": "da46d6df50c611035cd82c45bba2dc544a9ba192",
          "url": "https://github.com/unicode-org/icu4x/commit/8aa98be2e3cb650a89d36361ebd516cb9e973b57"
        },
        "date": 1634833095475,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 111657,
            "range": "± 4850",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1691721,
            "range": "± 78946",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 343092,
            "range": "± 12724",
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
          "id": "7256f16612e3bdf24e034102d6aab78e40d86440",
          "message": "Update Rust version to 1.56 stable (#1201)\n\n* Remove Rust version lock\r\n\r\n* Readd rust-toolchain",
          "timestamp": "2021-10-21T09:51:49-07:00",
          "tree_id": "29c1a9f8519ccb69bd260a83355a99afd2bf10f2",
          "url": "https://github.com/unicode-org/icu4x/commit/7256f16612e3bdf24e034102d6aab78e40d86440"
        },
        "date": 1634835575201,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 102871,
            "range": "± 4153",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1682426,
            "range": "± 56931",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 311122,
            "range": "± 12799",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "91630072+sapriyag@users.noreply.github.com",
            "name": "sapriyag",
            "username": "sapriyag"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "8e8b1cc83052c14a445487a88b995e680276a332",
          "message": "Updates to roadmap.md (#1140)\n\n* Work in progress to update roadmap.md\r\n\r\n* Update roadmap.md\r\n\r\n* Update roadmap.md\r\n\r\n* Update roadmap.md\r\n\r\n* Update roadmap.md\r\n\r\n* Update roadmap.md\r\n\r\n* Update roadmap.md\r\n\r\n* Updated roadmap.md\r\n\r\nReady for review.\r\n\r\n* Update roadmap.md\r\n\r\n* Update roadmap.md\r\n\r\n* Update roadmap.md\r\n\r\n* Update roadmap.md\r\n\r\nFixed indentation.\r\n\r\n* Update docs/process/roadmap.md\r\n\r\nCo-authored-by: Shane F. Carr <sffc@google.com>\r\n\r\n* Update roadmap.md\r\n\r\n* Update roadmap.md\r\n\r\n* Updated roadmap.md\r\n\r\nFixed indentation.\r\n\r\nCo-authored-by: Shane F. Carr <sffc@google.com>",
          "timestamp": "2021-10-21T17:04:31-07:00",
          "tree_id": "11e792c36f9901c0207d9014bc43ff168c24cb25",
          "url": "https://github.com/unicode-org/icu4x/commit/8e8b1cc83052c14a445487a88b995e680276a332"
        },
        "date": 1634861479325,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 89839,
            "range": "± 3168",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1370845,
            "range": "± 22269",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 283681,
            "range": "± 9143",
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
          "id": "ea6dd3c16de4963493f20fb1f831ecac2d4f27e0",
          "message": "Fix CI (#1207)\n\n* Make diplomat-get-rev fail early on CI\r\n\r\n* Fix loglevel argument ordering\r\n\r\n* include loglevel in the getrev check",
          "timestamp": "2021-10-25T11:46:41-07:00",
          "tree_id": "c57b3490614ddcc9138704321616846704a1046b",
          "url": "https://github.com/unicode-org/icu4x/commit/ea6dd3c16de4963493f20fb1f831ecac2d4f27e0"
        },
        "date": 1635188291341,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 118063,
            "range": "± 3967",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1838841,
            "range": "± 61428",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 365076,
            "range": "± 15725",
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
          "id": "3920ad263ca5d63d1cc206bde3266cf5e013318c",
          "message": "Add more methods to EncodeAsVarULE (#1199)",
          "timestamp": "2021-10-25T11:59:29-07:00",
          "tree_id": "441529dd6dc23bf43b0ca4923d22acf95448bbbd",
          "url": "https://github.com/unicode-org/icu4x/commit/3920ad263ca5d63d1cc206bde3266cf5e013318c"
        },
        "date": 1635188856932,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 117048,
            "range": "± 9528",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1791378,
            "range": "± 67030",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 363082,
            "range": "± 107806",
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
          "id": "b362fd7878841f27a375083dc880103b84a1bd1d",
          "message": "Add enumerated property APIs returning DataPayload of CodePointTrie (#1197)",
          "timestamp": "2021-10-26T07:42:00-07:00",
          "tree_id": "cb38195d0feee1ab2304bfb99e7d6da78eacbd55",
          "url": "https://github.com/unicode-org/icu4x/commit/b362fd7878841f27a375083dc880103b84a1bd1d"
        },
        "date": 1635259787296,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 119757,
            "range": "± 11071",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1806732,
            "range": "± 84313",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 363550,
            "range": "± 20669",
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
          "id": "e85321359b9d66425388bedd7e24875888a1f955",
          "message": "Make Bincode makefile commands equal to JSON and Blob ones (#1211)",
          "timestamp": "2021-10-26T15:58:22-07:00",
          "tree_id": "f47976ab570824a6ce1e3b6e8102ea94785cfb86",
          "url": "https://github.com/unicode-org/icu4x/commit/e85321359b9d66425388bedd7e24875888a1f955"
        },
        "date": 1635289538075,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 113352,
            "range": "± 1426",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1812172,
            "range": "± 13048",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 348157,
            "range": "± 2140",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "m_kato@ga2.so-net.ne.jp",
            "name": "Makoto Kato",
            "username": "makotokato"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "131b84548c7a7a204c91dbfaa8d57b647199c4d5",
          "message": "Implement DataMarker to use DataProvider for LstmData. (#1142)\n\n* Implement DataMarker to use DataProvider for LstmData.\r\n\r\n* Fix cargo wasm-build-release\r\n\r\n* Fix per review comment.\r\n\r\n* Use icu_provider::data_struct.\r\n\r\n* Fix fmt error\r\n\r\n* Use DataPayload for Lstm struct.\r\n\r\n* Fix fmt.\r\n\r\n* Use DataPayload methods instead of receiver.\r\n\r\n* Don't panic key isn't found.",
          "timestamp": "2021-10-27T08:40:02+09:00",
          "tree_id": "07787927fab2faddd9d72fbf0b7e6fbc277bb2b0",
          "url": "https://github.com/unicode-org/icu4x/commit/131b84548c7a7a204c91dbfaa8d57b647199c4d5"
        },
        "date": 1635291981615,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 93321,
            "range": "± 1340",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1557220,
            "range": "± 1997",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 289735,
            "range": "± 2312",
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
          "id": "5e98e933b07cf21248e9b117ddbed0a37533897f",
          "message": "Update Diplomat; pin tests to C++17 (#1220)\n\n* Update diplomat\r\n\r\n* Regen\r\n\r\n* Update tests to pin C++17",
          "timestamp": "2021-10-27T11:41:05-07:00",
          "tree_id": "fe4106a3229ec934d6c8df21508207edca093540",
          "url": "https://github.com/unicode-org/icu4x/commit/5e98e933b07cf21248e9b117ddbed0a37533897f"
        },
        "date": 1635360523478,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 104926,
            "range": "± 6944",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1614705,
            "range": "± 159376",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 310735,
            "range": "± 19862",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "91630072+sapriyag@users.noreply.github.com",
            "name": "sapriyag",
            "username": "sapriyag"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5338437ef9ecc5f83cb92d9fb2c0350989892e2d",
          "message": "Fixed broken link [Data Management] (#1221)\n\nReplaced \r\nhttps://github.com/unicode-org/icu4x/blob/main/docs/data-pipeline.md with \r\nhttps://github.com/unicode-org/icu4x/blob/main/docs/design/data_pipeline.md",
          "timestamp": "2021-10-27T14:57:06-07:00",
          "tree_id": "f2b961629e5d6790a8672139d79ff915bb1828a1",
          "url": "https://github.com/unicode-org/icu4x/commit/5338437ef9ecc5f83cb92d9fb2c0350989892e2d"
        },
        "date": 1635372300004,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 107675,
            "range": "± 7849",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1623672,
            "range": "± 103710",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 325863,
            "range": "± 20868",
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
          "id": "3a63e2d49c1d8c7700289bd60bfe4145a559af41",
          "message": "Make VarZeroVecBorrowed/VarZeroVecOwned public (#1223)\n\n* SliceComponents -> VZVBorrowed\n\n* get_components() -> as_borrowed()\n\n* Remove duplicate method\n\n* make compile again\n\n* components -> borrowed\n\n* Add docs, make VZVBorrowed public\n\n* more docs\n\n* more docs etc\n\n* Make VZVOwned public\n\n* Fixup doc warnings and fmt\n\n* Make VZV a publicly inspectable enum of VZVO/VZVB\n\n* clippy\n\n* move docs\n\n* Update utils/zerovec/src/varzerovec/borrowed.rs\n\nCo-authored-by: Shane F. Carr <shane@unicode.org>\n\n* Update utils/zerovec/src/varzerovec/mod.rs\n\nCo-authored-by: Shane F. Carr <shane@unicode.org>\n\n* Update utils/zerovec/src/varzerovec/mod.rs\n\nCo-authored-by: Shane F. Carr <shane@unicode.org>\n\n* Add examples to docs; add VZVO::push() and VZV::new()\n\nCo-authored-by: Shane F. Carr <shane@unicode.org>",
          "timestamp": "2021-10-27T17:48:58-07:00",
          "tree_id": "b8ae9b8419d77c1ad679cdb53612405634143950",
          "url": "https://github.com/unicode-org/icu4x/commit/3a63e2d49c1d8c7700289bd60bfe4145a559af41"
        },
        "date": 1635382591494,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 112557,
            "range": "± 7278",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1736463,
            "range": "± 78551",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 342918,
            "range": "± 22200",
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
          "id": "26d779ac767f613aace34a2031d9d474b11c913a",
          "message": "Fix crates.io badges no longer loading (#1036) (#1219)\n\n* Fix crates.io badges no longer loading (#1036)\r\n\r\n* Don't try to fix the diplomat link, just the badge",
          "timestamp": "2021-10-28T08:57:56-07:00",
          "tree_id": "95ec2ad4fde73e130f5600e70bac200e43788cac",
          "url": "https://github.com/unicode-org/icu4x/commit/26d779ac767f613aace34a2031d9d474b11c913a"
        },
        "date": 1635437084265,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 81240,
            "range": "± 809",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1324925,
            "range": "± 13810",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 254673,
            "range": "± 1312",
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
          "id": "9aaf98424fed0a14f37ce6f8962229967eb0a77a",
          "message": "Add manual clone impl for VZV (#1230)",
          "timestamp": "2021-10-28T11:19:02-07:00",
          "tree_id": "26d2ba782f7fad66471d105f5d0d382cf7f21ea4",
          "url": "https://github.com/unicode-org/icu4x/commit/9aaf98424fed0a14f37ce6f8962229967eb0a77a"
        },
        "date": 1635445554811,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 93328,
            "range": "± 224",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1513119,
            "range": "± 17583",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 285769,
            "range": "± 323",
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
          "id": "0e5084a8f41ae28fbfc1c27c5a080fab6e62b22e",
          "message": "Properly fold lifetimes in zcf/yoke custom derives (#1231)\n\n* Properly fold lifetimes in zcf/yoke custom derives\n\n* fix test\n\n* fmt",
          "timestamp": "2021-10-28T12:45:47-07:00",
          "tree_id": "da8c4f42decc761d4cdbb6014c8325bb0b9650f0",
          "url": "https://github.com/unicode-org/icu4x/commit/0e5084a8f41ae28fbfc1c27c5a080fab6e62b22e"
        },
        "date": 1635450817125,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 115965,
            "range": "± 8844",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1715702,
            "range": "± 97779",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 357172,
            "range": "± 39371",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "samchen61661@gmail.com",
            "name": "samchen",
            "username": "samchen61661"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "6621c4078b4c2433bda882cd631c3a639369e656",
          "message": "Likely subtags should minimize es-ES-preeuro to es-preeuro (#1189)",
          "timestamp": "2021-10-28T13:52:40-07:00",
          "tree_id": "0366501958c72e6a917e91a78c0e5af51f9e64ce",
          "url": "https://github.com/unicode-org/icu4x/commit/6621c4078b4c2433bda882cd631c3a639369e656"
        },
        "date": 1635454839675,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 123994,
            "range": "± 13284",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1926262,
            "range": "± 100401",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 378314,
            "range": "± 36598",
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
          "id": "40dff703ca88306047c47d1187a8a6880e8c4b58",
          "message": "Switch examples to use static provider to better measure memory impact. (#1241)",
          "timestamp": "2021-10-28T16:29:17-07:00",
          "tree_id": "5d17c95204aa764ea02a0247bb3a4dc7a450637e",
          "url": "https://github.com/unicode-org/icu4x/commit/40dff703ca88306047c47d1187a8a6880e8c4b58"
        },
        "date": 1635464189461,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 118453,
            "range": "± 7378",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1969753,
            "range": "± 51451",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 365247,
            "range": "± 19189",
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
          "id": "7e3fc02556b180a687e0d9fa3566b5bbfdd419cf",
          "message": "Move DTF to runtime patterns (#1198)\n\n* Move DTF to runtime patterns\r\n\r\n* Apply reviewers feedback\r\n\r\n* Apply reviewers feedback\r\n\r\n* Apply feedback\r\n\r\n* Apply feedback\r\n\r\n* Apply feedback",
          "timestamp": "2021-10-28T16:29:48-07:00",
          "tree_id": "f5ec35332ec40c41ff3330d295c66f599455429e",
          "url": "https://github.com/unicode-org/icu4x/commit/7e3fc02556b180a687e0d9fa3566b5bbfdd419cf"
        },
        "date": 1635464221513,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 126902,
            "range": "± 5654",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1404520,
            "range": "± 46920",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 353850,
            "range": "± 29426",
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
          "id": "6fba6e5ecdf959b41a679dab0a2eb76583b5013f",
          "message": "Remove missing docs config flag from ICU crate. (#1242)",
          "timestamp": "2021-10-28T18:41:44-07:00",
          "tree_id": "ea98cf4b1cecb1a48323254c743da230d502139c",
          "url": "https://github.com/unicode-org/icu4x/commit/6fba6e5ecdf959b41a679dab0a2eb76583b5013f"
        },
        "date": 1635472123322,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 100564,
            "range": "± 172",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1162524,
            "range": "± 3995",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 288917,
            "range": "± 2813",
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
          "id": "2313561b1ad423cf30574815f8e285f91f124ffd",
          "message": "Connect properties provider to the icu4x_datagen exporter tool (#1204)",
          "timestamp": "2021-10-29T10:47:08-07:00",
          "tree_id": "136a34b69ef8b1d1ccea8621faf9a511d1788736",
          "url": "https://github.com/unicode-org/icu4x/commit/2313561b1ad423cf30574815f8e285f91f124ffd"
        },
        "date": 1635530090437,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 128929,
            "range": "± 6402",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1471415,
            "range": "± 75027",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 373298,
            "range": "± 22033",
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
          "id": "d1d29c4aa848a5bcf9a81ef1e59d7f7034586552",
          "message": "Skip skeletons with -alt-variant when processing CLDR JSON (#1224)",
          "timestamp": "2021-10-29T11:36:26-07:00",
          "tree_id": "fafab1b15d35fed8839ae3ac416bf808bc48e148",
          "url": "https://github.com/unicode-org/icu4x/commit/d1d29c4aa848a5bcf9a81ef1e59d7f7034586552"
        },
        "date": 1635533009907,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 100519,
            "range": "± 167",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1159935,
            "range": "± 1704",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 291351,
            "range": "± 2166",
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
          "id": "0df5de32e80dd283920d351cc1889d7d0b8c65f2",
          "message": "Update to CLDR 40 (#1216)\n\n- Removes CldrLangID in favor of vanilla LanguageIdentifier",
          "timestamp": "2021-10-29T13:24:28-07:00",
          "tree_id": "8693b87d0fca66745b7cb13906c8b56daee4ceeb",
          "url": "https://github.com/unicode-org/icu4x/commit/0df5de32e80dd283920d351cc1889d7d0b8c65f2"
        },
        "date": 1635540487416,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 111235,
            "range": "± 4344",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1332328,
            "range": "± 50409",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 319762,
            "range": "± 13484",
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
          "id": "bf4b5dfc496f5580ebd8d8d5610b3e4bbacee625",
          "message": "Design doc: locale fallback and negotiation (#1237)\n\n* Design doc: locale fallback and negotiation\r\n\r\n* Design doc: renamed file\r\n\r\n* Update docs/design/locale_fallback_and_negotiation.md\r\n\r\nCo-authored-by: Elango <elango@unicode.org>\r\n\r\n* Update docs/design/locale_fallback_and_negotiation.md\r\n\r\nCo-authored-by: Elango <elango@unicode.org>\r\n\r\n* Update docs/design/locale_fallback_and_negotiation.md\r\n\r\nCo-authored-by: Elango <elango@unicode.org>\r\n\r\n* Update docs/design/locale_fallback_and_negotiation.md\r\n\r\nCo-authored-by: Elango <elango@unicode.org>\r\n\r\n* Update docs/design/locale_fallback_and_negotiation.md\r\n\r\nCo-authored-by: Elango <elango@unicode.org>\r\n\r\n* Update docs/design/locale_fallback_and_negotiation.md\r\n\r\nCo-authored-by: Elango <elango@unicode.org>\r\n\r\n* Update docs/design/locale_fallback_and_negotiation.md\r\n\r\nCo-authored-by: Elango <elango@unicode.org>\r\n\r\n* Fixing a couple of line breaks\r\n\r\n* Update docs/design/locale_fallback_and_negotiation.md\r\n\r\nCo-authored-by: Elango <elango@unicode.org>\r\n\r\n* Update docs/design/locale_fallback_and_negotiation.md\r\n\r\nCo-authored-by: Elango <elango@unicode.org>\r\n\r\n* Update docs/design/locale_fallback_and_negotiation.md\r\n\r\nCo-authored-by: Elango <elango@unicode.org>\r\n\r\n* Apply suggestions from code review\r\n\r\nCo-authored-by: Elango <elango@unicode.org>\r\n\r\n* Removed double quote about optimization\r\n\r\n* Apply suggestions from code review\r\n\r\nCo-authored-by: Elango <elango@unicode.org>\r\n\r\n* Removed TODO, reordered terminology\r\n\r\nCo-authored-by: Elango <elango@unicode.org>",
          "timestamp": "2021-10-29T17:41:55-07:00",
          "tree_id": "0f28c05193029ab529faac34d50067ace0bcae52",
          "url": "https://github.com/unicode-org/icu4x/commit/bf4b5dfc496f5580ebd8d8d5610b3e4bbacee625"
        },
        "date": 1635554938319,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 97310,
            "range": "± 145",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1161122,
            "range": "± 4550",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 286759,
            "range": "± 1848",
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
          "id": "e71aa54a2ed3300ee745ecca6d355af6a90955d3",
          "message": "Speed up icu4x-datagen by pre-loading data in icu_provider_uprops (#1244)",
          "timestamp": "2021-11-01T12:46:13-07:00",
          "tree_id": "41a3e8e12ba95040d79668ed203b83a51df2dced",
          "url": "https://github.com/unicode-org/icu4x/commit/e71aa54a2ed3300ee745ecca6d355af6a90955d3"
        },
        "date": 1635796517969,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 96224,
            "range": "± 201",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1157027,
            "range": "± 1426",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 279491,
            "range": "± 3229",
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
          "id": "9892697f554b36238613348b323aead3cdd75be1",
          "message": "Updating CHANGELOG for ICU4X 0.4",
          "timestamp": "2021-11-01T15:12:03-07:00",
          "tree_id": "bde858ffe0098794cea6f83b4ac95847625df9ce",
          "url": "https://github.com/unicode-org/icu4x/commit/9892697f554b36238613348b323aead3cdd75be1"
        },
        "date": 1635805151581,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 128493,
            "range": "± 13181",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1492920,
            "range": "± 70588",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 384995,
            "range": "± 60035",
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
          "id": "c68d73b35fb91b552f1a4b8394a8290e3588ee27",
          "message": "Document utils release process (#1250)\n\n* Document utils release process\r\n\r\n* fix",
          "timestamp": "2021-11-01T15:21:05-07:00",
          "tree_id": "72f72c9c4e7679f5d612198a9974b2eb7ad84fa7",
          "url": "https://github.com/unicode-org/icu4x/commit/c68d73b35fb91b552f1a4b8394a8290e3588ee27"
        },
        "date": 1635805738124,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 119118,
            "range": "± 5681",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1431982,
            "range": "± 77901",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 341964,
            "range": "± 25172",
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
          "id": "6975267434af46fc4ec70fc7c7459a2bc74c2699",
          "message": "Update utils in preparation for release (#1249)\n\n* Bump writeable to 0.2.1\r\n\r\n* Update dependency on writeable\r\n\r\n* Bump yoke version to 0.3.0\r\n\r\n* Update dependency on yoke\r\n\r\n* Bump zerovec version to 0.4.1\r\n\r\n* Bump icu_pattern to 0.1.1\r\n\r\n* Bump CPT to 0.3.0\r\n\r\n* Update dependency on CPT\r\n\r\n* Bump uniset to 0.4.0\r\n\r\n* Update dependency on uniset",
          "timestamp": "2021-11-01T15:47:18-07:00",
          "tree_id": "4e15d1bb10b5cdf39bf5ecbaeb12d9cff0804b00",
          "url": "https://github.com/unicode-org/icu4x/commit/6975267434af46fc4ec70fc7c7459a2bc74c2699"
        },
        "date": 1635807264679,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 97627,
            "range": "± 4774",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1203167,
            "range": "± 52934",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 284286,
            "range": "± 15379",
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
          "id": "e8b38536c46afca5d9e04dbdfb07b51c9384fccf",
          "message": "Bump yoke-derive in preparation for release (#1253)\n\n* Bump yoke-derive to 0.1.2\r\n\r\n* Bump yoke to 0.3.1\r\n\r\n* Update users of yoke and yoke-derive",
          "timestamp": "2021-11-01T16:25:02-07:00",
          "tree_id": "30a4caba07bf220bd63a0fa5d0b12c51ea0c5038",
          "url": "https://github.com/unicode-org/icu4x/commit/e8b38536c46afca5d9e04dbdfb07b51c9384fccf"
        },
        "date": 1635809576881,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 118434,
            "range": "± 7590",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1487584,
            "range": "± 36071",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 365475,
            "range": "± 18101",
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
          "id": "1b4dd6279b683a248658fd6b9876f8d2ac9e16ef",
          "message": "Update version numbers for ICU4X 0.4 (#1252)",
          "timestamp": "2021-11-01T16:59:18-07:00",
          "tree_id": "12cef258eb3ee945c36b75ba258c79a184470fef",
          "url": "https://github.com/unicode-org/icu4x/commit/1b4dd6279b683a248658fd6b9876f8d2ac9e16ef"
        },
        "date": 1635811832321,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 120622,
            "range": "± 4776",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1431262,
            "range": "± 2600",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 350386,
            "range": "± 961",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "shane@unicode.org",
            "name": "Shane Carr",
            "username": "sffc"
          },
          "committer": {
            "email": "shane@unicode.org",
            "name": "Shane Carr",
            "username": "sffc"
          },
          "distinct": true,
          "id": "dc5ff31e24e1203d719701cb179b3a6f3df3eac0",
          "message": "Add missing dependency version numbers",
          "timestamp": "2021-11-01T17:12:05-07:00",
          "tree_id": "9ca1aab7691fea14624f0f736a25bb34fbafbfbc",
          "url": "https://github.com/unicode-org/icu4x/commit/dc5ff31e24e1203d719701cb179b3a6f3df3eac0"
        },
        "date": 1635813491820,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 130161,
            "range": "± 5723",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1479547,
            "range": "± 81947",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 362460,
            "range": "± 16489",
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
          "id": "0c855b106c6e8b8a66142defc6102885ccc8c7c5",
          "message": "Implement Grapheme_Cluster_Break, Word_Break, and Sentence_Break Unicode properties (#1233)\n\nThe obsolete enum values in GraphemeClusterBreak and WordBreak are added\r\nto retain the compatibility with ICU.\r\n\r\nThe TOML file was obtained from Azure artifact archive built on\r\nunicode-org/icu, commit 2921a81ee4c67459ff455e31c599e7d7a09086ab titled\r\n\"ICU-21811 TZ update 2021a (2021e)\" on maint/maint-70 branch. This\r\ncommit imports TrieType::Small flavor of the uprops files.\r\n\r\nThe json and postcard files are generated via `cargo make testdata`.",
          "timestamp": "2021-11-02T10:01:52-07:00",
          "tree_id": "c6fef0184e27cf1c3e920786d6034e64f16479c8",
          "url": "https://github.com/unicode-org/icu4x/commit/0c855b106c6e8b8a66142defc6102885ccc8c7c5"
        },
        "date": 1635873016615,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 116133,
            "range": "± 1272",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1371558,
            "range": "± 13558",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 337054,
            "range": "± 1529",
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
          "id": "04b902b0e81a6964035a3fbcc796bd0207b11e11",
          "message": "Reorder LineBreakRule's enum values (#1263)\n\nMake their ordering the same as in the spec\r\nhttps://drafts.csswg.org/css-text-3/#line-break-property",
          "timestamp": "2021-11-04T22:34:54-07:00",
          "tree_id": "3f3d149419e058192cbc9675667636c8a27905c8",
          "url": "https://github.com/unicode-org/icu4x/commit/04b902b0e81a6964035a3fbcc796bd0207b11e11"
        },
        "date": 1636090956361,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 120790,
            "range": "± 1582",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1446590,
            "range": "± 2511",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 353084,
            "range": "± 481",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "younies@chromium.org",
            "name": "Younies Mahmoud",
            "username": "younies"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "11288c3272483b12a01e6e021187a728c4854491",
          "message": "add pad and padded to FixedDecimal (#1195)\n\n* add pad and padded to FixedDecimal\r\n\r\n* Make it pad or trunc@\r\n\r\n* Fix edge cases and add more tests\r\n\r\n* add an extensive tests and fix the case which the number is an empty string.\r\n\r\nCo-authored-by: younies <younies>",
          "timestamp": "2021-11-05T09:21:28+01:00",
          "tree_id": "6f5974b5a70cdebc3a97891ec8b358c89e5d1449",
          "url": "https://github.com/unicode-org/icu4x/commit/11288c3272483b12a01e6e021187a728c4854491"
        },
        "date": 1636100902088,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 86641,
            "range": "± 288",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1024058,
            "range": "± 1923",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 248535,
            "range": "± 1587",
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
          "id": "899d8cdd35064b75767d70c845c477cf3e13b849",
          "message": "Fix PairULE validation function (#1266)\n\n* Fix PairULE validate function\r\n\r\n* Add pairule validation test\r\n\r\n* satisfy clippy",
          "timestamp": "2021-11-05T09:57:10-07:00",
          "tree_id": "7d51db9b48b2d36522f8bfd6fa6dcd7e6dcd35dd",
          "url": "https://github.com/unicode-org/icu4x/commit/899d8cdd35064b75767d70c845c477cf3e13b849"
        },
        "date": 1636131924708,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 133463,
            "range": "± 4925",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1565604,
            "range": "± 49186",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 382408,
            "range": "± 19277",
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
          "id": "d18ba05e92871480874c48bc2199e5743ef61141",
          "message": "Update roadmap.md",
          "timestamp": "2021-11-05T10:07:48-07:00",
          "tree_id": "8ca649407d3ae8072ab2aa7c1105eded888809ba",
          "url": "https://github.com/unicode-org/icu4x/commit/d18ba05e92871480874c48bc2199e5743ef61141"
        },
        "date": 1636132470586,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 85533,
            "range": "± 215",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1021529,
            "range": "± 3246",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 249118,
            "range": "± 5169",
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
          "id": "74e74a3bd7e7b00073d4c73d2efa1fcfcc81579d",
          "message": "Handle exponents in FixedDecimal::from_str() (#1265)\n\n* Handle exponents in FixedDecimal::from_str()\r\n\r\n* Clean up leftmost/rightmost digit calculation\r\n\r\n* use filter()/collect()\r\n\r\n* clippy",
          "timestamp": "2021-11-05T10:12:36-07:00",
          "tree_id": "23e854171573c5e2fd85f1c595c6fdb4f47a2d50",
          "url": "https://github.com/unicode-org/icu4x/commit/74e74a3bd7e7b00073d4c73d2efa1fcfcc81579d"
        },
        "date": 1636132831515,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 121923,
            "range": "± 7929",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1461039,
            "range": "± 67077",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 356723,
            "range": "± 24999",
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
          "id": "b51dccd78a474c32742917f5a177f808f612db99",
          "message": "Fix typo in datagen docs (#1222)",
          "timestamp": "2021-11-05T10:42:49-07:00",
          "tree_id": "b40f7d249e19ce0265b83e71c34a7df2d7ac3013",
          "url": "https://github.com/unicode-org/icu4x/commit/b51dccd78a474c32742917f5a177f808f612db99"
        },
        "date": 1636134640144,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 115635,
            "range": "± 4389",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1341299,
            "range": "± 54085",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 321078,
            "range": "± 14519",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "pandusonu@google.com",
            "name": "Gollapudi Vamsi Krishna",
            "username": "pandusonu2"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "027c190e86868ba0acd1dd6be75b32bd4e97855b",
          "message": "Fix maximize function (#1171)\n\n* Fix maximize function\r\n\r\n* fmt\r\n\r\n* clone only required values\r\n\r\n* Use boolean to maintain modification\r\n\r\n* Minor nit fix",
          "timestamp": "2021-11-08T10:11:10-08:00",
          "tree_id": "8f3fdca4a2269614c538745b954ed4f0a099da57",
          "url": "https://github.com/unicode-org/icu4x/commit/027c190e86868ba0acd1dd6be75b32bd4e97855b"
        },
        "date": 1636395553445,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 110712,
            "range": "± 4745",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1310396,
            "range": "± 97434",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 328215,
            "range": "± 14919",
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
          "id": "4fbf81c29ef49cc0c5dc0ccc25c708c68dd128c3",
          "message": "Change anyhow to eyre (#1268)",
          "timestamp": "2021-11-08T12:06:56-08:00",
          "tree_id": "4cf8ab3790b2c7f4387f2d5dde515a8ea875015b",
          "url": "https://github.com/unicode-org/icu4x/commit/4fbf81c29ef49cc0c5dc0ccc25c708c68dd128c3"
        },
        "date": 1636402453693,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 97051,
            "range": "± 272",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1158144,
            "range": "± 1883",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 282341,
            "range": "± 811",
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
          "id": "c88c6fe1a714c872d7ea1893417fbe4ac72bd58c",
          "message": "Add EncodeAsVarULE for ZeroVec (#1274)",
          "timestamp": "2021-11-09T09:06:27-08:00",
          "tree_id": "ada144b053d15e1ef9bcbdf8c5109984a4b77ad5",
          "url": "https://github.com/unicode-org/icu4x/commit/c88c6fe1a714c872d7ea1893417fbe4ac72bd58c"
        },
        "date": 1636478079508,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 127080,
            "range": "± 8716",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1470145,
            "range": "± 110676",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 324137,
            "range": "± 23382",
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
          "id": "615570cc88f9012e78b2bb79900006b9747f0259",
          "message": "Add borrowed-only version of ZeroMap (#1238)",
          "timestamp": "2021-11-09T12:06:53-08:00",
          "tree_id": "f6cd826673762ec45246a9610573c6a71d6bf64a",
          "url": "https://github.com/unicode-org/icu4x/commit/615570cc88f9012e78b2bb79900006b9747f0259"
        },
        "date": 1636488916082,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 123569,
            "range": "± 6730",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1404922,
            "range": "± 81090",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 353258,
            "range": "± 12879",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "robertbastian@users.noreply.github.com",
            "name": "Robert Bastian",
            "username": "robertbastian"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "e12752342b684bf6e19ce1b7157b5bb24090422f",
          "message": "Providers (#1218)",
          "timestamp": "2021-11-09T13:42:54-08:00",
          "tree_id": "001c4681b515333542ba9161e8df8691c3dad9ce",
          "url": "https://github.com/unicode-org/icu4x/commit/e12752342b684bf6e19ce1b7157b5bb24090422f"
        },
        "date": 1636494633815,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 121924,
            "range": "± 4349",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1422623,
            "range": "± 67075",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 350748,
            "range": "± 11434",
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
          "id": "8a99635bf98b44c4cb57e7a2db491c9a88d3d4ad",
          "message": "Add double-to-decimal for FixedDecimal via ryū (#1217)\n\n* Add ryu and new_from_f32/new_from_f64 to FixedDecimal\r\n\r\n* Add C++ test for ryu APIs\r\n\r\n* fmt\r\n\r\n* Remove f32 methods\r\n\r\n* Add rounding routine\r\n\r\n* Add DoublePrecision\r\n\r\n* Add DoublePrecision::Magnitude\r\n\r\n* fixes\r\n\r\n* fix\r\n\r\n* fix\r\n\r\n* fix\r\n\r\n* Fix bounds\r\n\r\n* More testcases with scientific notation\r\n\r\n* Add SignificantDigits\r\n\r\n* comments\r\n\r\n* round_digits -> round_trailing_digits\r\n\r\n* Add rounding modes\r\n\r\n* Update utils/fixed_decimal/src/decimal.rs\r\n\r\nCo-authored-by: Shane F. Carr <shane@unicode.org>\r\n\r\n* +todo\r\n\r\n* review\r\n\r\n* more review\r\n\r\n* Add DoublePrecision::Integer\r\n\r\n* Properly round up\r\n\r\n* fmt\r\n\r\n* Update utils/fixed_decimal/src/decimal.rs\r\n\r\nCo-authored-by: Shane F. Carr <shane@unicode.org>\r\n\r\n* fmt\r\n\r\n* fix .0\r\n\r\n* fixes\r\n\r\n* refactor integer types\r\n\r\n* fix\r\n\r\n* Correctly uphold invariant of not having trailing zeroes\r\n\r\n* fmt\r\n\r\n* Handle invariants\r\n\r\n* clippy\r\n\r\n* don't override upper magnitude; test fix\r\n\r\nCo-authored-by: Shane F. Carr <shane@unicode.org>",
          "timestamp": "2021-11-10T11:23:07-08:00",
          "tree_id": "c546e4179c86a307544001ce0a6ad4706f86293a",
          "url": "https://github.com/unicode-org/icu4x/commit/8a99635bf98b44c4cb57e7a2db491c9a88d3d4ad"
        },
        "date": 1636572686250,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 119539,
            "range": "± 277",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1427928,
            "range": "± 12870",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 348631,
            "range": "± 441",
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
          "id": "0fe7189471f3889ede366fcc9979ff311ed84d45",
          "message": "Move ErasedDataStruct to the owned variant of DataPayload (#1278)",
          "timestamp": "2021-11-10T11:23:19-08:00",
          "tree_id": "19db1c514ffcdbfcbc0e208aede04dc42f408dee",
          "url": "https://github.com/unicode-org/icu4x/commit/0fe7189471f3889ede366fcc9979ff311ed84d45"
        },
        "date": 1636572713341,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 135425,
            "range": "± 6802",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1566498,
            "range": "± 81536",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 390652,
            "range": "± 20341",
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
          "id": "052c398ed85fe63c64b746d2e4c58167d47564f5",
          "message": "Unify handling of TOML in uprops provider (#1254)",
          "timestamp": "2021-11-10T11:32:56-08:00",
          "tree_id": "7b37812cc03345076bcf62d494a78cf7478d1ef3",
          "url": "https://github.com/unicode-org/icu4x/commit/052c398ed85fe63c64b746d2e4c58167d47564f5"
        },
        "date": 1636573249856,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 106282,
            "range": "± 6646",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1241113,
            "range": "± 98247",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 313519,
            "range": "± 26035",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "robertbastian@users.noreply.github.com",
            "name": "Robert Bastian",
            "username": "robertbastian"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "fd2515c6b881d0a47ab7414637d3eaf3787f0a5f",
          "message": "Deduplicating list formatter data (#1276)",
          "timestamp": "2021-11-10T13:07:29-08:00",
          "tree_id": "06957d906059cccc72452b9893dfffe8b72fdefe",
          "url": "https://github.com/unicode-org/icu4x/commit/fd2515c6b881d0a47ab7414637d3eaf3787f0a5f"
        },
        "date": 1636578863345,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 96136,
            "range": "± 4939",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1117374,
            "range": "± 38267",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 283335,
            "range": "± 11328",
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
          "id": "c6857ec1508b0fd74e6d15e20a76220bad211067",
          "message": "Burn DataMarker::Cart down with fire (#1279)\n\n* Add Yoke::replace_cart()\r\n\r\n* Use ErasedCart\r\n\r\n* Replace M::Cart with ErasedCart in RcStruct\r\n\r\n* Remove Cart type from DataMarker\r\n\r\n* Remove 'data from DataMarker",
          "timestamp": "2021-11-10T13:14:00-08:00",
          "tree_id": "6e7c0f3da973b0e47ec23491c260e3c359071dc2",
          "url": "https://github.com/unicode-org/icu4x/commit/c6857ec1508b0fd74e6d15e20a76220bad211067"
        },
        "date": 1636579358118,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 124884,
            "range": "± 6085",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1417164,
            "range": "± 103038",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 349328,
            "range": "± 18314",
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
          "id": "1ed57813d760147f20cea0084de736790561b730",
          "message": "Fixes for icu4x-datagen for Unicode properties (#1285)",
          "timestamp": "2021-11-10T14:00:17-08:00",
          "tree_id": "28b5c0cb4d42dc3d541322dcad37f276b1cb9c92",
          "url": "https://github.com/unicode-org/icu4x/commit/1ed57813d760147f20cea0084de736790561b730"
        },
        "date": 1636582110080,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 125461,
            "range": "± 9123",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1430566,
            "range": "± 43320",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 360736,
            "range": "± 22128",
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
          "id": "92abc789dd46000a3dcfd3d5b46a592f76e834da",
          "message": "Update tutorial to include uprops data (#1286)",
          "timestamp": "2021-11-10T14:30:34-08:00",
          "tree_id": "b376f260011fd1c5a660f2df1511b68d6b11fe81",
          "url": "https://github.com/unicode-org/icu4x/commit/92abc789dd46000a3dcfd3d5b46a592f76e834da"
        },
        "date": 1636583902069,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 123881,
            "range": "± 8864",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1394474,
            "range": "± 72211",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 355822,
            "range": "± 18715",
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
          "id": "fe24e7b9309bb87eb43c3bfde7c74412c818a059",
          "message": "Impl Debug/PartialEq on PairULE (#1287)",
          "timestamp": "2021-11-10T15:54:13-08:00",
          "tree_id": "3437c2cc8d3f14828b25ca72d60e937f90522e18",
          "url": "https://github.com/unicode-org/icu4x/commit/fe24e7b9309bb87eb43c3bfde7c74412c818a059"
        },
        "date": 1636588917150,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 117033,
            "range": "± 3710",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1391830,
            "range": "± 4077",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 339381,
            "range": "± 735",
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
          "id": "771b837e5f30a2a8e89dcde088dd2404a9a6a4bd",
          "message": "Fix up some intra doc links (#1288)",
          "timestamp": "2021-11-10T15:54:29-08:00",
          "tree_id": "fd50229087ec30a6e6668075580ba5bfbd75851d",
          "url": "https://github.com/unicode-org/icu4x/commit/771b837e5f30a2a8e89dcde088dd2404a9a6a4bd"
        },
        "date": 1636588947343,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 110623,
            "range": "± 7777",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1273691,
            "range": "± 64509",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 343513,
            "range": "± 22951",
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
          "id": "b2c93173505d9e307a89fec12083e59a6706b936",
          "message": "Fix diplomat regen (#1289)",
          "timestamp": "2021-11-11T09:56:51-08:00",
          "tree_id": "04163ee3040678ce6fadcf449d4ac9d991bfaf7d",
          "url": "https://github.com/unicode-org/icu4x/commit/b2c93173505d9e307a89fec12083e59a6706b936"
        },
        "date": 1636653835523,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 97605,
            "range": "± 380",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1205516,
            "range": "± 14712",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 286921,
            "range": "± 484",
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
          "id": "89508d93bb5421beaf9a3d09793c0944743cb046",
          "message": "Document ULE alignment guarantee; update all impls with checklists (#1294)",
          "timestamp": "2021-11-12T08:03:17-08:00",
          "tree_id": "4898988d1b364be4c516444d5e353139c36bfae0",
          "url": "https://github.com/unicode-org/icu4x/commit/89508d93bb5421beaf9a3d09793c0944743cb046"
        },
        "date": 1636733444550,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 110884,
            "range": "± 7974",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1366819,
            "range": "± 68439",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 309025,
            "range": "± 18698",
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
          "id": "33d441cc9fdfdb5c86ba21f28eb50753bbe6e6a1",
          "message": "Fix item 5 in the VarULE checklist (#1295)",
          "timestamp": "2021-11-12T17:15:25-08:00",
          "tree_id": "b4278d2f88dfc273a605362bd4adad31578c246b",
          "url": "https://github.com/unicode-org/icu4x/commit/33d441cc9fdfdb5c86ba21f28eb50753bbe6e6a1"
        },
        "date": 1636766620720,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 127381,
            "range": "± 8426",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1401592,
            "range": "± 84724",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 355844,
            "range": "± 20726",
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
          "id": "0845aa18ff40b5f7a0dfad8b7d63ec86b55d3d59",
          "message": "Add CodePointTrie and ZeroVec conversion methods (#1291)",
          "timestamp": "2021-11-12T17:35:22-08:00",
          "tree_id": "27420c27afdad4d6295266a3afa6acc186ed3c4c",
          "url": "https://github.com/unicode-org/icu4x/commit/0845aa18ff40b5f7a0dfad8b7d63ec86b55d3d59"
        },
        "date": 1636767747906,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 110815,
            "range": "± 8451",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1281005,
            "range": "± 89901",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 331825,
            "range": "± 32807",
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
          "id": "e5da8ba9e5d792e836316808b050a015ff902741",
          "message": "Switch PluralRules Data to ZeroVec (#1240)\n\n* Separate runtime and reference rules in PluralRules.\r\n\r\n* Add ZeroVec::borrowed_from_slice()\r\n\r\n* Add RelationULE::as_relation()\r\n\r\n* Add encoding/decoding for andor/plurals/operand\r\n\r\n* fix encode\r\n\r\n* fix from_byte_slice_unchecked\r\n\r\n* slightly better impl\r\n\r\n* Plug it all together\r\n\r\n* Fix tests\r\n\r\n* Apply feedback\r\n\r\n* Add RelationULE doc\r\n\r\n* Apply feedback\r\n\r\n* Add inlines and docs\r\n\r\n* Uncomment tests\r\n\r\n* Apply reviewers feedback\r\n\r\n* Fix tests and apply feedback\r\n\r\n* Revise postcard file\r\n\r\n* Fix provider benchmarks\r\n\r\n* Fix more tests\r\n\r\n* Add rountrip test\r\n\r\n* Fix another test\r\n\r\n* Fix more tests\r\n\r\n* Update postcard\r\n\r\n* Fix readme\r\n\r\n* Regenerate diplomat ffi\r\n\r\n* Fix diplomat example\r\n\r\n* Fix test-cpp\r\n\r\n* Update safety comment\r\n\r\n* Apply feedback\r\n\r\n* No need to guard local type used for debug test. DCE will take care of it.\r\n\r\nCo-authored-by: Manish Goregaokar <manishsmail@gmail.com>",
          "timestamp": "2021-11-15T10:20:37-08:00",
          "tree_id": "c81ae767fafcfefdd0858b50338d7504be2fe9c6",
          "url": "https://github.com/unicode-org/icu4x/commit/e5da8ba9e5d792e836316808b050a015ff902741"
        },
        "date": 1637000866610,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 85415,
            "range": "± 144",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1022384,
            "range": "± 1605",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 253399,
            "range": "± 1318",
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
          "id": "bcba2fbdb6fdfcb8648e3826579255b43a799f32",
          "message": "Implement East_Asian_Width and Line_Break Unicode properties (#1280)\n\nThe numeric value of EastAsianWidth=0x1004 and LineBreak=0x1008 are defined in:\r\nhttps://github.com/unicode-org/icu/blob/d3a56c5ceda272054e7c6bf7e62b4b51367eecf5/icu4c/source/common/unicode/uchar.h#L559-L574\r\n\r\nUEastAsianWidth in ICU4C:\r\nhttps://github.com/unicode-org/icu/blob/d3a56c5ceda272054e7c6bf7e62b4b51367eecf5/icu4c/source/common/unicode/uchar.h#L1905-L1934\r\n\r\nULineBreak enum in ICU4C:\r\nhttps://github.com/unicode-org/icu/blob/d3a56c5ceda272054e7c6bf7e62b4b51367eecf5/icu4c/source/common/unicode/uchar.h#L2361-L2436\r\n\r\nThe TOML file was obtained from Azure artifact archive built on\r\nunicode-org/icu, commit 2921a81ee4c67459ff455e31c599e7d7a09086ab titled\r\n\"ICU-21811 TZ update 2021a (2021e)\" on maint/maint-70 branch. This\r\ncommit imports TrieType::Small flavor of the uprops files.",
          "timestamp": "2021-11-15T11:17:59-08:00",
          "tree_id": "e3efc1ef7541cf3bb9baa1c13b6105f53d56a2b5",
          "url": "https://github.com/unicode-org/icu4x/commit/bcba2fbdb6fdfcb8648e3826579255b43a799f32"
        },
        "date": 1637004312610,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 109748,
            "range": "± 5713",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1279621,
            "range": "± 71041",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 329355,
            "range": "± 16885",
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
          "id": "cfc5fda14a03bf6a053492c0bcc4fa2ae4f36f82",
          "message": "Get rid of the lifetime on DataPayload/DataProvider/DataExporter (#1297)\n\nRemove 'data from DataProvider/DataPayload/DataExport",
          "timestamp": "2021-11-15T12:14:01-08:00",
          "tree_id": "00730afd744adbc03372b4c4aa58235eeb210f07",
          "url": "https://github.com/unicode-org/icu4x/commit/cfc5fda14a03bf6a053492c0bcc4fa2ae4f36f82"
        },
        "date": 1637007689426,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 94793,
            "range": "± 208",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1128518,
            "range": "± 1804",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 275759,
            "range": "± 3993",
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
          "id": "242fa55e995a92da7f6a8d333ca9639a7b9e222c",
          "message": "Initial Properties FFI (#1269)",
          "timestamp": "2021-11-15T13:07:21-08:00",
          "tree_id": "646c5472b4ee5fd28b5ac8bc650afc6a45e0fc49",
          "url": "https://github.com/unicode-org/icu4x/commit/242fa55e995a92da7f6a8d333ca9639a7b9e222c"
        },
        "date": 1637010913080,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 121618,
            "range": "± 5039",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1448900,
            "range": "± 49331",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 354853,
            "range": "± 15273",
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
          "id": "ef6477796157315b35c2537441edbb7fff37cf9c",
          "message": "Tooling fixes (#1298)",
          "timestamp": "2021-11-15T13:30:36-08:00",
          "tree_id": "2ecfba3ac3c89402fb640e8130493263c9bd7eaf",
          "url": "https://github.com/unicode-org/icu4x/commit/ef6477796157315b35c2537441edbb7fff37cf9c"
        },
        "date": 1637012322029,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 133626,
            "range": "± 6717",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1535058,
            "range": "± 52823",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 383262,
            "range": "± 28349",
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
          "id": "cee2b5fa9dc7649e78b29e5e2d67e3e7c33764e3",
          "message": "Remove RcStruct (#1299)\n\n* Use owned data structs in SerdeSeDataStruct\r\n\r\n* Move ErasedCart to Yoke\r\n\r\n* Remove DataPayload::from_partial_owned()\r\n\r\n* Remove RcStruct",
          "timestamp": "2021-11-15T15:26:37-08:00",
          "tree_id": "1c45872111bf72096ebed3c3a6723b48db0f6841",
          "url": "https://github.com/unicode-org/icu4x/commit/cee2b5fa9dc7649e78b29e5e2d67e3e7c33764e3"
        },
        "date": 1637019275289,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 115938,
            "range": "± 5732",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1320066,
            "range": "± 73670",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 332846,
            "range": "± 24923",
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
          "id": "608fbd45586a8f392421b30087c6cd1c2cf902ca",
          "message": "Update data model for non-gregorian calendars (#1300)\n\n* Parametrize cldr_dates() on calendar type\n\n* Refactor date code into common\n\n* Use litemap in CLDR provider\n\n* Start producing locale data in per-calendar subfolders\n\n* Regen testdata\n\n* Update tests\n\n* Use gregorian variant in datetime\n\n* Rename data keys to remove 'gregory'\n\n* Regen testdata\n\n* Rename GREGORY_ keys to not say GREGORY_\n\n* Move gregory -> calendar\n\n* fmt\n\n* gregory -> calendar\n\n* appease clippy\n\n* Add MissingVariant\n\n* gregorian -> gregory\n\n* Regen data\n\n* rm pubself\n\n* Handle other calendars in the JSON\n\n* Rename CLDR field types\n\n* fmt\n\n* MissingVariant -> NeedsVariant",
          "timestamp": "2021-11-16T18:29:12-08:00",
          "tree_id": "da0101cec667cc87db0ea4471ad72ed687e6cfae",
          "url": "https://github.com/unicode-org/icu4x/commit/608fbd45586a8f392421b30087c6cd1c2cf902ca"
        },
        "date": 1637116639842,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 122954,
            "range": "± 5230",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1424393,
            "range": "± 41957",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 361475,
            "range": "± 19128",
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
          "id": "a0a65962002a56f610a9f171f0aee6ed0caeead7",
          "message": "Handle serialization of tuples (etc) in litemaps (#1306)\n\n* Move serde code into separate file\r\n\r\n* Add serde_json to litemap, split into serde_serialize and serde features\r\n\r\n* Handle tuple serialization in LiteMap\r\n\r\n* fmt\r\n\r\n* appease clippy\r\n\r\n* appease clippy\r\n\r\n* fix dep",
          "timestamp": "2021-11-17T11:58:32-08:00",
          "tree_id": "2348028445d58bcc172c3f8e0dbc8225bac1f8c5",
          "url": "https://github.com/unicode-org/icu4x/commit/a0a65962002a56f610a9f171f0aee6ed0caeead7"
        },
        "date": 1637179572939,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 119323,
            "range": "± 5194",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1396031,
            "range": "± 119240",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 347471,
            "range": "± 13583",
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
          "id": "1c296f0cce90c50742d4f820dfa298b06d04f0c3",
          "message": "Add Buddhist calendar calculations (#1305)\n\n* Add Buddhist calendar calculations\n\n* move into const\n\n* Remove intermediate layer of inner date",
          "timestamp": "2021-11-17T13:25:18-08:00",
          "tree_id": "8ea32a66097faf4ee3362c136f2092facffc3e8c",
          "url": "https://github.com/unicode-org/icu4x/commit/1c296f0cce90c50742d4f820dfa298b06d04f0c3"
        },
        "date": 1637184778860,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 106731,
            "range": "± 3934",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1248580,
            "range": "± 32103",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 318284,
            "range": "± 15224",
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
          "id": "f2a5bf5e9dcad52e072e23e3770d5c7decf67a64",
          "message": "Homogenize yoke generic impls to always work with Yokeable, add OwnedYokeable (#1302)",
          "timestamp": "2021-11-17T14:09:09-08:00",
          "tree_id": "ba9ebd58c27b92d3b49c79602137cfd4378f1303",
          "url": "https://github.com/unicode-org/icu4x/commit/f2a5bf5e9dcad52e072e23e3770d5c7decf67a64"
        },
        "date": 1637187448568,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 122379,
            "range": "± 7601",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1427322,
            "range": "± 105304",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 371966,
            "range": "± 32153",
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
          "id": "46e82e78f8a0480334540ac39fdded2b976cc908",
          "message": "Add Buddhist calendar data to testdata (#1308)\n\n* Set up CLDR transformer for buddhist data\r\n\r\n* Download buddhist calendar testdata\r\n\r\n* Add buddhist calendar data to testdata",
          "timestamp": "2021-11-17T16:37:42-08:00",
          "tree_id": "8d22b0a57fa89ec4f77595134ae7a0781e6f5edd",
          "url": "https://github.com/unicode-org/icu4x/commit/46e82e78f8a0480334540ac39fdded2b976cc908"
        },
        "date": 1637196329624,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 116046,
            "range": "± 6757",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1378864,
            "range": "± 52614",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 347957,
            "range": "± 26009",
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
          "id": "6c25706bf3967816438337bc3ca483acd490e1a1",
          "message": "Add StaticDataProvider create_empty (#1314)",
          "timestamp": "2021-11-18T10:52:29-06:00",
          "tree_id": "3437ab39d4f995e177875cc06b55e8609f7583ca",
          "url": "https://github.com/unicode-org/icu4x/commit/6c25706bf3967816438337bc3ca483acd490e1a1"
        },
        "date": 1637254758469,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 97444,
            "range": "± 379",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1164266,
            "range": "± 3280",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 285144,
            "range": "± 685",
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
          "id": "a96c0aa8873370bbba80fb0f041f65945ab30d37",
          "message": "Add missing Serde/Yoke impls to ZeroVec crate (#1328)",
          "timestamp": "2021-11-19T11:11:54-08:00",
          "tree_id": "35314837ec943e81683fa5d5d1601ae0a1b670ff",
          "url": "https://github.com/unicode-org/icu4x/commit/a96c0aa8873370bbba80fb0f041f65945ab30d37"
        },
        "date": 1637349592730,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 114017,
            "range": "± 4097",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1317834,
            "range": "± 42965",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 324379,
            "range": "± 10763",
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
          "id": "c78b611172290d7ab729a985f48879b2f724b63e",
          "message": "Add Default impls for all zerovec types (#1330)",
          "timestamp": "2021-11-19T12:12:57-08:00",
          "tree_id": "94a7417e5e9735ada9ce18c952d9d279e2e825d3",
          "url": "https://github.com/unicode-org/icu4x/commit/c78b611172290d7ab729a985f48879b2f724b63e"
        },
        "date": 1637353184159,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 96606,
            "range": "± 840",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1164532,
            "range": "± 2227",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 282849,
            "range": "± 2647",
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
          "id": "9258d0505eab7165723c6e052c51e26f085cd1fa",
          "message": "Add PartialEq, Debug, Clone impls to ZeroMap (#1332)\n\n* Add PartialEq, Debug, Clone impls to ZeroMap\r\n\r\n* fmt\r\n\r\n* Add ?Sized bounds",
          "timestamp": "2021-11-19T15:14:31-08:00",
          "tree_id": "b796e4cc08c29f67ced0c4544209936495c84873",
          "url": "https://github.com/unicode-org/icu4x/commit/9258d0505eab7165723c6e052c51e26f085cd1fa"
        },
        "date": 1637364142251,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 107151,
            "range": "± 4597",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1265752,
            "range": "± 82604",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 328600,
            "range": "± 41443",
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
          "id": "1c5901fa24fd17605e0534a1c2bcee89d84807e3",
          "message": "Simplify ZeroMapKV (#1334)",
          "timestamp": "2021-11-19T16:52:23-08:00",
          "tree_id": "b32a7f40aa662c53e397928d2433585634b41292",
          "url": "https://github.com/unicode-org/icu4x/commit/1c5901fa24fd17605e0534a1c2bcee89d84807e3"
        },
        "date": 1637369994196,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 95495,
            "range": "± 200",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1142315,
            "range": "± 2273",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 280515,
            "range": "± 3604",
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
          "id": "08f539e48bac34b218a27df29ef18dc62ede132e",
          "message": "Allow cloning_zcf to be applied to individual variants or fields (#1307)\n\n* Move has_cloning_zcf_attr to function\r\n\r\n* Extend cloning_zcf to work on fields as well\r\n\r\n* Add test\r\n\r\n* fmt",
          "timestamp": "2021-11-22T11:38:21-08:00",
          "tree_id": "4aade8279b57f5efda2967fda8fa2e670fe6e8f0",
          "url": "https://github.com/unicode-org/icu4x/commit/08f539e48bac34b218a27df29ef18dc62ede132e"
        },
        "date": 1637610389375,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 126755,
            "range": "± 3957",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1489324,
            "range": "± 51721",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 369726,
            "range": "± 11241",
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
          "id": "ca2dd56cc13ad6c18d66d7b2f4df989acc5a71c5",
          "message": "Bump zerovec to 0.5 (#1336)",
          "timestamp": "2021-11-22T12:31:57-08:00",
          "tree_id": "58002db444f6e981b88b31e01c282e95f58d72f2",
          "url": "https://github.com/unicode-org/icu4x/commit/ca2dd56cc13ad6c18d66d7b2f4df989acc5a71c5"
        },
        "date": 1637613579959,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 103736,
            "range": "± 7578",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1162935,
            "range": "± 54512",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 305035,
            "range": "± 23402",
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
          "id": "9d0137c20b31d2fd222cd13a2257e8d98416b86c",
          "message": "Migrate StaticDataProvider and BlobDataProvider to ZeroMap (#1058)",
          "timestamp": "2021-11-22T23:39:30-08:00",
          "tree_id": "2a458590b63493adabe0d3da0c5bc6b6b9887558",
          "url": "https://github.com/unicode-org/icu4x/commit/9d0137c20b31d2fd222cd13a2257e8d98416b86c"
        },
        "date": 1637653589778,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 96446,
            "range": "± 280",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1169462,
            "range": "± 2274",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 291570,
            "range": "± 1713",
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
          "id": "d683334935645db8a7015ee490b3ee3356e0e058",
          "message": "Move CLDR JSON structs to central location and other refactoring (#1337)",
          "timestamp": "2021-11-23T16:34:25-08:00",
          "tree_id": "510d3c7f53e55c8dfa796ded764603be821ceb2d",
          "url": "https://github.com/unicode-org/icu4x/commit/d683334935645db8a7015ee490b3ee3356e0e058"
        },
        "date": 1637714591241,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 134841,
            "range": "± 8829",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1559063,
            "range": "± 68247",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 399768,
            "range": "± 34053",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "samchen61661@gmail.com",
            "name": "samchen",
            "username": "samchen61661"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "6edc9d418a52661ef7fcefd3b6d99c77e80d10f9",
          "message": "Create TimeZoneFormatConfig enum (#1256)\n\n* Create TimeZoneFormatConfig enum\r\n\r\n* Add tests\r\n\r\n* Create enum TimeZoneFormatKind",
          "timestamp": "2021-11-23T18:33:15-08:00",
          "tree_id": "5af6d889548ed9d7b037f814ffb2231a19b108e6",
          "url": "https://github.com/unicode-org/icu4x/commit/6edc9d418a52661ef7fcefd3b6d99c77e80d10f9"
        },
        "date": 1637721679918,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 128675,
            "range": "± 4340",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1512877,
            "range": "± 61053",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 386629,
            "range": "± 15750",
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
          "id": "919d6f03fc2809ddb5adf7d20addd2d91b2754f1",
          "message": "Extend DateTimeFormat to support other calendars (#1339)",
          "timestamp": "2021-11-24T07:11:30-08:00",
          "tree_id": "1369c739ad1b079dc63553d9efc154e066af61c9",
          "url": "https://github.com/unicode-org/icu4x/commit/919d6f03fc2809ddb5adf7d20addd2d91b2754f1"
        },
        "date": 1637767152506,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 121963,
            "range": "± 7606",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1385163,
            "range": "± 79476",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 324422,
            "range": "± 30545",
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
          "id": "7418270529534848b000a195299802c74e9971ff",
          "message": "Migrate the various map impls in the CLDR transformer to LiteMap (#1341)",
          "timestamp": "2021-11-24T11:07:30-08:00",
          "tree_id": "54277d03057779d7e9bfb95a3db4faaf77f3470d",
          "url": "https://github.com/unicode-org/icu4x/commit/7418270529534848b000a195299802c74e9971ff"
        },
        "date": 1637781312243,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 134426,
            "range": "± 3107",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1584840,
            "range": "± 75535",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 397144,
            "range": "± 18294",
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
          "id": "d7e2d1bbfe836885979d6ad7e2cdd22d3b91edda",
          "message": "Add extend_from_litemap to LiteMap (#1340)",
          "timestamp": "2021-11-24T11:07:03-08:00",
          "tree_id": "b8670c5ee7651cf859eff9c112033d4b06e4de49",
          "url": "https://github.com/unicode-org/icu4x/commit/d7e2d1bbfe836885979d6ad7e2cdd22d3b91edda"
        },
        "date": 1637781322578,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 129409,
            "range": "± 8130",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1465634,
            "range": "± 66364",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 383080,
            "range": "± 27859",
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
          "id": "b42c3594c9c109816ba59f3f7a0a948ebf53f6ad",
          "message": "ZeroVec::clone_from_slice --> ZeroVec::alloc_from_slice (#1343)",
          "timestamp": "2021-11-24T11:07:58-08:00",
          "tree_id": "3fb31ed25169382eb10dad3e42b0b5c0aa439f83",
          "url": "https://github.com/unicode-org/icu4x/commit/b42c3594c9c109816ba59f3f7a0a948ebf53f6ad"
        },
        "date": 1637781325810,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 99140,
            "range": "± 640",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1190911,
            "range": "± 2697",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 288730,
            "range": "± 896",
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
          "id": "92dbabdbf4f81f55492d65dcfde9a003aaa9de24",
          "message": "Remove DataPayloadInner and migrate to a single Yoke type (#1342)",
          "timestamp": "2021-11-24T11:07:48-08:00",
          "tree_id": "16b9957252cb5158b6f9e499a38fe0945886838d",
          "url": "https://github.com/unicode-org/icu4x/commit/92dbabdbf4f81f55492d65dcfde9a003aaa9de24"
        },
        "date": 1637781339819,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 118252,
            "range": "± 824",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1404724,
            "range": "± 2128",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 344212,
            "range": "± 480",
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
          "id": "c7db0802d44dd9c79518a29712f63cbb8b2159d7",
          "message": "Include era symbol information in data (#1344)",
          "timestamp": "2021-11-25T08:52:59-08:00",
          "tree_id": "3c2a3e20ec46888831efca1657021ee707bded8d",
          "url": "https://github.com/unicode-org/icu4x/commit/c7db0802d44dd9c79518a29712f63cbb8b2159d7"
        },
        "date": 1637859637716,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 141277,
            "range": "± 4633",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1551508,
            "range": "± 53802",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 398084,
            "range": "± 28103",
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
          "id": "fd5eb4b373b48870b7ffd4dd19946fdcc46e2cd9",
          "message": "Remove attach_to_option_cart() (#1348)",
          "timestamp": "2021-11-29T17:39:44-08:00",
          "tree_id": "dff9d68f81b1d9c112ea3ed080853f8f8b7f24be",
          "url": "https://github.com/unicode-org/icu4x/commit/fd5eb4b373b48870b7ffd4dd19946fdcc46e2cd9"
        },
        "date": 1638237094825,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 142678,
            "range": "± 9531",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1585178,
            "range": "± 79907",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 391488,
            "range": "± 53206",
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
          "id": "fcf248c8a8ff1c6da7e18800a21ee4c95a3b97ca",
          "message": "Implement era formatting (#1346)\n\n* Add era field parsing\r\n\r\n* Handle eras in components bag\r\n\r\n* Apply era formatting\r\n\r\n* Add tests for buddhist date formatting",
          "timestamp": "2021-11-29T18:18:45-08:00",
          "tree_id": "2bc3073d9d69c2d89d93c86684541854e1c17582",
          "url": "https://github.com/unicode-org/icu4x/commit/fcf248c8a8ff1c6da7e18800a21ee4c95a3b97ca"
        },
        "date": 1638239249602,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 137971,
            "range": "± 8262",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1584062,
            "range": "± 59413",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 393098,
            "range": "± 29632",
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
          "id": "e5a49da1bb9022134ad07b8c754ed1070655d0bb",
          "message": "Update principles.md to reflect current reality (#1335)",
          "timestamp": "2021-11-30T13:59:19-08:00",
          "tree_id": "7d3e972ddc4419775a4309436ea1022a674d3183",
          "url": "https://github.com/unicode-org/icu4x/commit/e5a49da1bb9022134ad07b8c754ed1070655d0bb"
        },
        "date": 1638310051489,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 126257,
            "range": "± 3011",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1550396,
            "range": "± 48068",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 342066,
            "range": "± 12675",
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
          "id": "910cf452e290e040ea087a1e2f39a3a429990117",
          "message": "Add GC predicate functions (#1310)",
          "timestamp": "2021-12-01T09:11:30-08:00",
          "tree_id": "fa63e1087cfbe9d56b457f0808369c4438a64fb2",
          "url": "https://github.com/unicode-org/icu4x/commit/910cf452e290e040ea087a1e2f39a3a429990117"
        },
        "date": 1638379150606,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 140506,
            "range": "± 7130",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1650921,
            "range": "± 83459",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 419263,
            "range": "± 59508",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "pandusonu@google.com",
            "name": "Gollapudi Vamsi Krishna",
            "username": "pandusonu2"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "dc414a8fcaeafe0625c576d90adb8053cdfca7fa",
          "message": "Replace Vec with LiteMap in locale canoicalizer (#1275)",
          "timestamp": "2021-12-01T23:18:39-08:00",
          "tree_id": "afd1597800ce284a07948ddee926a5bc42e22f45",
          "url": "https://github.com/unicode-org/icu4x/commit/dc414a8fcaeafe0625c576d90adb8053cdfca7fa"
        },
        "date": 1638429961583,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 137558,
            "range": "± 7214",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1559078,
            "range": "± 77363",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 348996,
            "range": "± 24019",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "10595307+mildgravitas@users.noreply.github.com",
            "name": "mildgravitas",
            "username": "mildgravitas"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "a0f78c5dcea33368cd89c06332e1b59e335e4de3",
          "message": "Year of week-of-year support (#1206)\n\n* feat(datetime): Add support for week of year (this previously just used the calendar year).\r\n\r\n* feat(datetime): Add year of week-of-year to components::Bag & fix width adjustments for it.\r\n\r\nTo do this I've loosened get_best_available_format_pattern() to match on\r\nFieldSymbol enums but not their data. From the function's greater/lesser\r\nmatching this is apparently what the function tried to do all along. Without\r\nthis Year::NumericWeekOf wouldn't match as CLDR skeletons use 'y' even for\r\npatterns with 'Y'\r\n\r\nThis accessorily improves full & long time_h11_h12/time_h23_h24\r\npatterns: the h11_h12/h23_h24 coercion logic matches adjusted patterns\r\nagainst skeletons & previously 'z' was not matched againts 'v' leading\r\nto the time zone being dropped.\r\n\r\nIf we don't care to expose the week-of year variants in components::Bag\r\n& don't care about coerced time patterns then only\r\nadjust_pattern_field_lengths() need be adjusted.\r\n\r\n* doc(datetime): add examples to the descriptions of datetime::options::components::Year.\r\n\r\n* fix(datetime): remove serde renames & further expand the documentation of datetime::options::components::Year\r\n\r\n* fix(datetime): also update year enum names in benches.\r\n\r\n* fix(datetime): swich datetime::options::components::Year & Week to kebab case for serialization\r\n\r\n* style(datetime): use is_eq instead of == Ordering:::Equal\r\n\r\n* fix(datetime): use kebab-case for all options::components enums\r\n\r\n* tests(datetime): add extra locales to some test cases\r\n\r\n* fix(datetime): Return an error in DateTimeFormat::try_new instead of an empty pattern if there are no matches.",
          "timestamp": "2021-12-04T02:01:23-08:00",
          "tree_id": "db0214ac36483d76bfc51ef378fdda271fc23dbe",
          "url": "https://github.com/unicode-org/icu4x/commit/a0f78c5dcea33368cd89c06332e1b59e335e4de3"
        },
        "date": 1638612569739,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 120412,
            "range": "± 6115",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1378254,
            "range": "± 64703",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 345223,
            "range": "± 26117",
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
          "id": "bffe5bad6d78ee37fed4c7f5b44ed730600b6c17",
          "message": "Install (and cache) grcov before loading overridden coverage toolchain (#1359)",
          "timestamp": "2021-12-06T08:45:05-06:00",
          "tree_id": "4e22e629b9a363169e6808373147c27d00ec111b",
          "url": "https://github.com/unicode-org/icu4x/commit/bffe5bad6d78ee37fed4c7f5b44ed730600b6c17"
        },
        "date": 1638802381631,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 111182,
            "range": "± 7754",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1279475,
            "range": "± 78381",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 329513,
            "range": "± 21756",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "robertbastian@users.noreply.github.com",
            "name": "Robert Bastian",
            "username": "robertbastian"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "bc9c9ffec1e492407a7e23fc9b8d08b12b0ae5c0",
          "message": "Custom fallbacks (#1309)",
          "timestamp": "2021-12-07T03:03:23-08:00",
          "tree_id": "fc4695f8f16e7fc01525bc01b851d0991df11380",
          "url": "https://github.com/unicode-org/icu4x/commit/bc9c9ffec1e492407a7e23fc9b8d08b12b0ae5c0"
        },
        "date": 1638875471034,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 131408,
            "range": "± 358",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1594235,
            "range": "± 71459",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 369717,
            "range": "± 28326",
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
          "id": "3418070e32c024f8f6ce4316d18090507c7a5400",
          "message": "Add ZeroVecULE, make VarULE impl of `[T: ULE]` work on reflexive ULEs (#1357)",
          "timestamp": "2021-12-07T20:32:02-08:00",
          "tree_id": "d376843679d5bf2e8996d02084ac27a51376fa88",
          "url": "https://github.com/unicode-org/icu4x/commit/3418070e32c024f8f6ce4316d18090507c7a5400"
        },
        "date": 1638938337487,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 105494,
            "range": "± 379",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1204132,
            "range": "± 1700",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 298383,
            "range": "± 2284",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "m_kato@ga2.so-net.ne.jp",
            "name": "Makoto Kato",
            "username": "makotokato"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "7d774380d73d8726f84259818884dc7c883cdcb3",
          "message": "Implement word segmenter using rule based segmenter (#1273)\n\n* Implement word segmenter using rule based segmenter\r\n\r\n* Fix cargo make generate-readmes.\r\n\r\n* Update segmenter table generation\r\n\r\n- Use builtin data for SA\r\n- Expand codepoint for property\r\n\r\n* UAX documents should use https URL\r\n\r\n* Change data format to toml.\r\n\r\n* Convert json file to toml fomrat\r\n\r\n* cargo make generate-readmes\r\n\r\n* Fix per review comment",
          "timestamp": "2021-12-08T14:27:42+09:00",
          "tree_id": "d3e79a1bbf16f1d5b5a88da1869b2d08fba62794",
          "url": "https://github.com/unicode-org/icu4x/commit/7d774380d73d8726f84259818884dc7c883cdcb3"
        },
        "date": 1638941788038,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 127772,
            "range": "± 551",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1495803,
            "range": "± 12801",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 362867,
            "range": "± 1043",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "robertbastian@users.noreply.github.com",
            "name": "Robert Bastian",
            "username": "robertbastian"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "fd504b0bf0264c391823eeb86a39a10a83730518",
          "message": "Using a trait for FormattedString (#1345)",
          "timestamp": "2021-12-08T19:31:12+01:00",
          "tree_id": "aeba2241aaa974589048d758bfc7000965f3b911",
          "url": "https://github.com/unicode-org/icu4x/commit/fd504b0bf0264c391823eeb86a39a10a83730518"
        },
        "date": 1638988767125,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 106822,
            "range": "± 757",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1245695,
            "range": "± 1490",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 305321,
            "range": "± 380",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "robertbastian@users.noreply.github.com",
            "name": "Robert Bastian",
            "username": "robertbastian"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "64c3859ae91ab7881c3f8161f0f200daba7438f2",
          "message": "Adding special case data for es and he list formatting (#1365)",
          "timestamp": "2021-12-09T09:30:49+01:00",
          "tree_id": "772c336bd1db24fbee280ff02f49d1bde231ca25",
          "url": "https://github.com/unicode-org/icu4x/commit/64c3859ae91ab7881c3f8161f0f200daba7438f2"
        },
        "date": 1639039182529,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 113660,
            "range": "± 9717",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1248767,
            "range": "± 74337",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 398980,
            "range": "± 26220",
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
          "id": "1d9ddad75f72a378881526e0fc2cd3ac0f37c6e0",
          "message": "Replace SerdeDeDataProvider with BufferProvider (#1369)\n\n- Add buffer_format to DataResponseMetadata\r\n- Migrate fs provider to BufferFormat enum\r\n- impl BufferProvider for FsDataProvider\r\n- Migrate FsDataProvider to use DeserializingProvider for deserialization\r\n- Rename serialization primitives\r\n- Rename features",
          "timestamp": "2021-12-09T11:08:39-06:00",
          "tree_id": "591f615255693682b5ded5400247c27665e1954a",
          "url": "https://github.com/unicode-org/icu4x/commit/1d9ddad75f72a378881526e0fc2cd3ac0f37c6e0"
        },
        "date": 1639070209371,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 126701,
            "range": "± 8687",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1428108,
            "range": "± 71311",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 355650,
            "range": "± 24058",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "robertbastian@users.noreply.github.com",
            "name": "Robert Bastian",
            "username": "robertbastian"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "eae3099d496d66f4fdd27168ff89e5211d0b3bc7",
          "message": "Moving deduplicating_array to its own crate (#1364)",
          "timestamp": "2021-12-09T21:18:56-08:00",
          "tree_id": "2b9b89f49d206e34b9782ded38c5359320adc574",
          "url": "https://github.com/unicode-org/icu4x/commit/eae3099d496d66f4fdd27168ff89e5211d0b3bc7"
        },
        "date": 1639114061763,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 122178,
            "range": "± 1785",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1451485,
            "range": "± 21762",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 355301,
            "range": "± 5226",
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
          "id": "560dd8e57002226ba9f2a7853e0cb7fe195b6b30",
          "message": "Enable test-c-tiny in CI (#1382)",
          "timestamp": "2021-12-10T02:18:30-08:00",
          "tree_id": "2506eff44cbe6dfbc25995cbb512b2beb010105e",
          "url": "https://github.com/unicode-org/icu4x/commit/560dd8e57002226ba9f2a7853e0cb7fe195b6b30"
        },
        "date": 1639132029807,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 122419,
            "range": "± 1546",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1477167,
            "range": "± 19947",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 356684,
            "range": "± 3513",
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
          "id": "0a9aea981ca6e85523600ff191562dc4576d7fb1",
          "message": "Coalesce more impls into the new BufferProvider framework (#1384)",
          "timestamp": "2021-12-10T15:40:26-08:00",
          "tree_id": "52f482af5fe17e274e3c4d374c1efecbfebbf884",
          "url": "https://github.com/unicode-org/icu4x/commit/0a9aea981ca6e85523600ff191562dc4576d7fb1"
        },
        "date": 1639180322251,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 139509,
            "range": "± 12390",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1480067,
            "range": "± 90615",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 375132,
            "range": "± 29328",
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
          "id": "61620c52818565dfec760a93f1435f80590f1c3a",
          "message": "Change break_iterator_impl macro to use a helper trait (#1380)",
          "timestamp": "2021-12-11T15:18:04-08:00",
          "tree_id": "1b6c7144be64da71d9a883bd94ebca8255db8360",
          "url": "https://github.com/unicode-org/icu4x/commit/61620c52818565dfec760a93f1435f80590f1c3a"
        },
        "date": 1639265480816,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 146498,
            "range": "± 9593",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1632429,
            "range": "± 71381",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 419613,
            "range": "± 24791",
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
          "id": "7e1d77e59e3145175d1267905eef35eefecc34b7",
          "message": "Fix typo in Yokeable documentation (#1386)",
          "timestamp": "2021-12-11T16:45:39-08:00",
          "tree_id": "905795728b12e9ff444346a16fe51a3544626d9f",
          "url": "https://github.com/unicode-org/icu4x/commit/7e1d77e59e3145175d1267905eef35eefecc34b7"
        },
        "date": 1639270494769,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 140473,
            "range": "± 5131",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1579730,
            "range": "± 63991",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 381440,
            "range": "± 19853",
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
          "id": "65fef9e14457d6c744f8dca11fcf22461837c838",
          "message": "calendars: fix offset handling around month boundaries (#1352)\n\n* calendars: fix offset handling around month boundaries\n\n* fmt\n\n* clippy",
          "timestamp": "2021-12-14T07:20:31+05:30",
          "tree_id": "3fde4534287b6868ec554389f622cb7bb3776b75",
          "url": "https://github.com/unicode-org/icu4x/commit/65fef9e14457d6c744f8dca11fcf22461837c838"
        },
        "date": 1639447179126,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 148225,
            "range": "± 5541",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1647506,
            "range": "± 43122",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 413994,
            "range": "± 14747",
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
          "id": "290e46c3969d5345cebc4587e33638ed1251dd61",
          "message": "Add (so far unused) Japanese era data (#1375)",
          "timestamp": "2021-12-13T20:38:17-08:00",
          "tree_id": "e32846f140d833dfbdbcaf2664a580ed66f5e0df",
          "url": "https://github.com/unicode-org/icu4x/commit/290e46c3969d5345cebc4587e33638ed1251dd61"
        },
        "date": 1639457170698,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 112385,
            "range": "± 170",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1289511,
            "range": "± 1990",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 314313,
            "range": "± 1359",
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
          "id": "7ae1e6df3178f839840e313b0065dcd1915a151c",
          "message": "Simplify ZeroVec/VarZeroVec error handling, consolidate ULEError type (#1389)\n\n* Replace all ZeroVec errors with ULEError\r\n\r\n* Use ULEError for VarZeroVec\r\n\r\n* Use new ULE impl everywhere",
          "timestamp": "2021-12-13T21:05:07-08:00",
          "tree_id": "ddfe182d74812e7523ff713f0c5be85cb4d287b7",
          "url": "https://github.com/unicode-org/icu4x/commit/7ae1e6df3178f839840e313b0065dcd1915a151c"
        },
        "date": 1639458847406,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 132785,
            "range": "± 16114",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1456285,
            "range": "± 86840",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 349463,
            "range": "± 23802",
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
          "id": "6fac854351da0d609a955603b81853b63a12100f",
          "message": "constrain zcf on T: 'static (#1392)",
          "timestamp": "2021-12-14T10:47:07-05:00",
          "tree_id": "d3b89d890692c00266455ca107cc7d0521bddb02",
          "url": "https://github.com/unicode-org/icu4x/commit/6fac854351da0d609a955603b81853b63a12100f"
        },
        "date": 1639497365662,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 134435,
            "range": "± 1129",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1533191,
            "range": "± 9555",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 369349,
            "range": "± 8005",
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
          "id": "5002e70343983cc08163b3df2da0b48dadc9856f",
          "message": "Add LineBreakSegmenter and rewire code to use it (#1387)",
          "timestamp": "2021-12-14T10:26:06-06:00",
          "tree_id": "6ea782251c3b62bbbe5c845b771213c7e88d0221",
          "url": "https://github.com/unicode-org/icu4x/commit/5002e70343983cc08163b3df2da0b48dadc9856f"
        },
        "date": 1639499671792,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 111529,
            "range": "± 214",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1283605,
            "range": "± 1360",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 305716,
            "range": "± 453",
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
          "id": "782f62334153f30b3f360a364f8e7b8c7918ed7a",
          "message": "Add ZeroSlice and VarZeroSlice, move ZeroVec functionality to ZeroSlice and add Deref (#1371)",
          "timestamp": "2021-12-14T08:37:44-08:00",
          "tree_id": "e6bcf82dcb0a032dd3a6b809ee7bfc5825808b33",
          "url": "https://github.com/unicode-org/icu4x/commit/782f62334153f30b3f360a364f8e7b8c7918ed7a"
        },
        "date": 1639500396406,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 155017,
            "range": "± 3154",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1672782,
            "range": "± 65608",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 417774,
            "range": "± 23132",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "hsivonen@hsivonen.fi",
            "name": "Henri Sivonen",
            "username": "hsivonen"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b081f84c087a0d5b94c1ac2cf6b0bc3c78e9079a",
          "message": "Implement the Canonical_Combining_Class property (#1347)",
          "timestamp": "2021-12-14T18:41:22+02:00",
          "tree_id": "b658d80a737985fc9940e3beed7e54fac241c9b2",
          "url": "https://github.com/unicode-org/icu4x/commit/b081f84c087a0d5b94c1ac2cf6b0bc3c78e9079a"
        },
        "date": 1639500613770,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 112399,
            "range": "± 5782",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1292191,
            "range": "± 57137",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 317311,
            "range": "± 12740",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "robertbastian@users.noreply.github.com",
            "name": "Robert Bastian",
            "username": "robertbastian"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "472900ff5fa1078ed12fa9f2497f1055f4982164",
          "message": "Adding size hints to ListFormatter buffer initialization (#1390)",
          "timestamp": "2021-12-14T20:58:13+01:00",
          "tree_id": "89f0d6abc4d64789200f31d57e3f3b256f10fd67",
          "url": "https://github.com/unicode-org/icu4x/commit/472900ff5fa1078ed12fa9f2497f1055f4982164"
        },
        "date": 1639512370808,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 99538,
            "range": "± 273",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1147170,
            "range": "± 7443",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 276107,
            "range": "± 2324",
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
          "id": "bea61e5d70f41baf52ebe710f57fbe7bab77ccc9",
          "message": "Rename enums for General_Category (#1355)",
          "timestamp": "2021-12-14T13:19:55-08:00",
          "tree_id": "e8ca7cd3fc832646c220e7bdd2fe62dd3edf6f5e",
          "url": "https://github.com/unicode-org/icu4x/commit/bea61e5d70f41baf52ebe710f57fbe7bab77ccc9"
        },
        "date": 1639517313048,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 128507,
            "range": "± 2557",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1472230,
            "range": "± 33671",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 364414,
            "range": "± 8345",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "robertbastian@users.noreply.github.com",
            "name": "Robert Bastian",
            "username": "robertbastian"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f3db25335e27591a3b1d83d1fd0325bc4403958b",
          "message": "Making ListFormatterPattern construction more private (#1398)",
          "timestamp": "2021-12-15T10:21:30+01:00",
          "tree_id": "30473ad60984458f19b0b97d7e4daeff06551689",
          "url": "https://github.com/unicode-org/icu4x/commit/f3db25335e27591a3b1d83d1fd0325bc4403958b"
        },
        "date": 1639560596905,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 107814,
            "range": "± 6054",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1227867,
            "range": "± 69637",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 295605,
            "range": "± 18373",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "robertbastian@users.noreply.github.com",
            "name": "Robert Bastian",
            "username": "robertbastian"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "a4717395673f0a8492f2cf677071302e8f9673f2",
          "message": "Swapping arguments of assert_writeable_eq (#1399)",
          "timestamp": "2021-12-15T19:01:46+05:30",
          "tree_id": "dd75480265fe1df59e621f0dac2550e1ef50412e",
          "url": "https://github.com/unicode-org/icu4x/commit/a4717395673f0a8492f2cf677071302e8f9673f2"
        },
        "date": 1639575670579,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 112214,
            "range": "± 225",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1280274,
            "range": "± 1571",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 311391,
            "range": "± 503",
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
          "id": "5c7c7fd7acebd6826d681e85dde3e8cbf6e04ca7",
          "message": "Add a resolve_components method to DateTimeFormat (#1362)\n\nThis is required for feature compatibility with ECMA-402. The backing\r\nDateTimeFormat implementation needs to be able to return the resolved\r\ncomponents from the given options. These resolved components can differ\r\nbased on locale or the resolution algorithm.",
          "timestamp": "2021-12-15T08:45:20-06:00",
          "tree_id": "ba0847a5059f238781197e2d088998b52956e32f",
          "url": "https://github.com/unicode-org/icu4x/commit/5c7c7fd7acebd6826d681e85dde3e8cbf6e04ca7"
        },
        "date": 1639580014546,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 127581,
            "range": "± 2277",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1467574,
            "range": "± 22113",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 357799,
            "range": "± 7397",
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
          "id": "5217972b3d7eb583317aaef4455bd2f9cd8d91aa",
          "message": "Remove workaround for rustc bug after upstream fix (#1402)",
          "timestamp": "2021-12-15T14:12:06-08:00",
          "tree_id": "ce6eb537cd3daefe7215aeb5a0476801799aff57",
          "url": "https://github.com/unicode-org/icu4x/commit/5217972b3d7eb583317aaef4455bd2f9cd8d91aa"
        },
        "date": 1639606827989,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 148861,
            "range": "± 7226",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1555413,
            "range": "± 86426",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 394623,
            "range": "± 18269",
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
          "id": "3f2a96a06472d61e6c7d2f6e688fb2a9a64e14fd",
          "message": "Fix EncodeAsVarULE for ZeroVec (#1407)",
          "timestamp": "2021-12-16T15:05:42-08:00",
          "tree_id": "947f7f255c540e2363519ab9cc8e4e1e608b7d9b",
          "url": "https://github.com/unicode-org/icu4x/commit/3f2a96a06472d61e6c7d2f6e688fb2a9a64e14fd"
        },
        "date": 1639696791293,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 102486,
            "range": "± 5280",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1154742,
            "range": "± 60201",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 279440,
            "range": "± 16274",
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
          "id": "685622ee653d322b6e824b72d765db2cfd99ef03",
          "message": "Implement data transformer for Script_Extensions map data (#1353)",
          "timestamp": "2021-12-16T15:49:38-08:00",
          "tree_id": "474f7633428f399e7a54d9448fe76c5d94a200f6",
          "url": "https://github.com/unicode-org/icu4x/commit/685622ee653d322b6e824b72d765db2cfd99ef03"
        },
        "date": 1639699149499,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 162344,
            "range": "± 10501",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1746992,
            "range": "± 77552",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 429127,
            "range": "± 22877",
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
          "id": "294c66ff679eaf646d9a0fd91888135b54ee8869",
          "message": "Rename resource key category for properties (#1406)",
          "timestamp": "2021-12-16T15:50:16-08:00",
          "tree_id": "b8c91afc9736a504006dd38c159dae3edf411c99",
          "url": "https://github.com/unicode-org/icu4x/commit/294c66ff679eaf646d9a0fd91888135b54ee8869"
        },
        "date": 1639699180416,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 145966,
            "range": "± 9563",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1565881,
            "range": "± 50700",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 388978,
            "range": "± 18487",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "robertbastian@users.noreply.github.com",
            "name": "Robert Bastian",
            "username": "robertbastian"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "20b82065bec51661c90d070c6809210bb256b5ed",
          "message": "Making writeable::LengthHint a range (#1400)",
          "timestamp": "2021-12-18T00:50:20+01:00",
          "tree_id": "8671ac01c032f277d7c4393b1df3a6545337eda6",
          "url": "https://github.com/unicode-org/icu4x/commit/20b82065bec51661c90d070c6809210bb256b5ed"
        },
        "date": 1639785590055,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 152289,
            "range": "± 7807",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1654916,
            "range": "± 100409",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 408540,
            "range": "± 17185",
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
          "id": "e165d22abc6f636c2e4e9e52097d96357557c1a8",
          "message": "Implement the Japanese calendar (#1394)\n\n* Make stub japanese calendar\n\n* Move japanese era data struct into icu_calendar\n\n* Add LiteMap::get_indexed() and LiteMap::find_index()\n\n* Load era data into japanese calendar\n\n* Add era_for function\n\n* Store era and era start on japanese dates\n\n* Report correct year from japanese calendar\n\n* Fill in day_of_year_info\n\n* Include japanese testdata\n\n* Update cldr transform to handle japanese eras\n\n* Include japanese calendar data\n\n* Add formatting tests\n\n* Add test fixtures for japanese eras\n\n* fmt\n\n* mention zero-copy issue\n\n* clippy\n\n* mention meiji opt",
          "timestamp": "2021-12-18T07:45:24+05:30",
          "tree_id": "2a588b48681453ce1a23613a4e1bbf529cd2fe2e",
          "url": "https://github.com/unicode-org/icu4x/commit/e165d22abc6f636c2e4e9e52097d96357557c1a8"
        },
        "date": 1639794231825,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 134305,
            "range": "± 251",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1543433,
            "range": "± 10765",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 372005,
            "range": "± 4354",
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
          "id": "da07d1d672a0ce53c69bc79e52fc33eddba94728",
          "message": "Improve EncodeAsVarULE (#1385)",
          "timestamp": "2021-12-18T21:49:10-08:00",
          "tree_id": "1fff779fa4b2bb235086d18c2aee5d51fa1cf5a2",
          "url": "https://github.com/unicode-org/icu4x/commit/da07d1d672a0ce53c69bc79e52fc33eddba94728"
        },
        "date": 1639893466659,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 157077,
            "range": "± 7219",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1645857,
            "range": "± 63964",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 415097,
            "range": "± 21844",
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
          "id": "a6799976ee0ba76708393f13efd3466f198bed57",
          "message": "Fail gracefully when building provider_fs on alternate platforms (#1414)",
          "timestamp": "2021-12-19T00:09:31-08:00",
          "tree_id": "07ddb9723cef5b64460a026f4969f479f562f407",
          "url": "https://github.com/unicode-org/icu4x/commit/a6799976ee0ba76708393f13efd3466f198bed57"
        },
        "date": 1639901889637,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 137638,
            "range": "± 6849",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1463139,
            "range": "± 65089",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 368915,
            "range": "± 18937",
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
          "id": "55aeb880fc1e2dd86717227b0e7b918d2e444ec4",
          "message": "Rename PlainOldULE to RawBytesULE (#1413)",
          "timestamp": "2021-12-19T00:09:40-08:00",
          "tree_id": "6ea2fc3a0e5dbba74634602b8a60a02b88db8121",
          "url": "https://github.com/unicode-org/icu4x/commit/55aeb880fc1e2dd86717227b0e7b918d2e444ec4"
        },
        "date": 1639901911888,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 135159,
            "range": "± 14944",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1440107,
            "range": "± 120118",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 357931,
            "range": "± 33097",
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
          "id": "8328551082ea99dda9e32a16ce126991830e81a8",
          "message": "Add precompute benches for VZV, prefix all ZeroVecLike methods with `zvl_` (#1412)\n\n* Add benches for precomputing\r\n* prefix all ZeroVecLike methods with `zvl_`",
          "timestamp": "2021-12-19T00:35:01-08:00",
          "tree_id": "6702de66a80483ab93586c7bb9d8b36bbbebe809",
          "url": "https://github.com/unicode-org/icu4x/commit/8328551082ea99dda9e32a16ce126991830e81a8"
        },
        "date": 1639903418123,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 133251,
            "range": "± 8893",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1591394,
            "range": "± 73915",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 366482,
            "range": "± 92950",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "m_kato@ga2.so-net.ne.jp",
            "name": "Makoto Kato",
            "username": "makotokato"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "e82c1fbbd9827de1db96334edcc21c5a8a67d5c0",
          "message": "Add Grapheme segmenter that is a part of UAX29. (#1388)\n\n* Add Grapheme segmenter that is a part of UAX29.\r\n\r\n* Fix review comment.",
          "timestamp": "2021-12-19T20:42:53+09:00",
          "tree_id": "6443b26a9bbf0b4643465016869233d9df1f19a1",
          "url": "https://github.com/unicode-org/icu4x/commit/e82c1fbbd9827de1db96334edcc21c5a8a67d5c0"
        },
        "date": 1639914754662,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 135222,
            "range": "± 3655",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1536432,
            "range": "± 3777",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 377563,
            "range": "± 3028",
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
          "id": "355487a0e0e42493d55373a569f7651791c8cc50",
          "message": "Change DataRequest to be borrowed in BufferProvider (#1416)",
          "timestamp": "2021-12-19T11:50:56-06:00",
          "tree_id": "7e877899a9a8d801d666bfac53fe908e2de00c82",
          "url": "https://github.com/unicode-org/icu4x/commit/355487a0e0e42493d55373a569f7651791c8cc50"
        },
        "date": 1639936816051,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 135162,
            "range": "± 1245",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1533563,
            "range": "± 7466",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 380008,
            "range": "± 3927",
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
          "id": "988f2464b6a40aa72cfe04acc59fe3c98928f088",
          "message": "Construct VZVs and use as_encoded_bytes instead of the static function (#1411)\n\n* Start migrating clients away from get_serializable_bytes\n\n* fmt\n\n* Rename functions involving VZV bytes\n\n* Update docs\n\n* More fixes\n\n* update bench\n\nCo-authored-by: Manish Goregaokar <manishsmail@gmail.com>",
          "timestamp": "2021-12-20T07:28:41+05:30",
          "tree_id": "74347ebc2561a72e813d4b1f3e8610ed1e846303",
          "url": "https://github.com/unicode-org/icu4x/commit/988f2464b6a40aa72cfe04acc59fe3c98928f088"
        },
        "date": 1639966130279,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 140790,
            "range": "± 8916",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1562346,
            "range": "± 87551",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 390891,
            "range": "± 21528",
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
          "id": "66b20815ed4126800da5183a293e16bbcf634618",
          "message": "Refactor VZV code to center on (and Deref to) VarZeroSlice; make VZVBorrowed private (#1418)",
          "timestamp": "2021-12-20T14:24:57-06:00",
          "tree_id": "13ad1701af5cc287c89f1b737574814d59987732",
          "url": "https://github.com/unicode-org/icu4x/commit/66b20815ed4126800da5183a293e16bbcf634618"
        },
        "date": 1640032499425,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 112519,
            "range": "± 340",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1296321,
            "range": "± 22801",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 317049,
            "range": "± 2388",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "robertbastian@users.noreply.github.com",
            "name": "Robert Bastian",
            "username": "robertbastian"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "e96204ea6b758acc25b7cad2cbbac5ec705c265b",
          "message": "Implementing Writeable for all integers (#1408)",
          "timestamp": "2021-12-20T22:12:52+01:00",
          "tree_id": "3eb10e05d14aa09c84c18b9250c5fb9c6ffc8a7b",
          "url": "https://github.com/unicode-org/icu4x/commit/e96204ea6b758acc25b7cad2cbbac5ec705c265b"
        },
        "date": 1640035332251,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 111442,
            "range": "± 1125",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1301238,
            "range": "± 2407",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 310752,
            "range": "± 534",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "m_kato@ga2.so-net.ne.jp",
            "name": "Makoto Kato",
            "username": "makotokato"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "7769518dc099f1763061aa3be3df86bacffd9cbb",
          "message": "Add missing license block in segmenter. (#1420)",
          "timestamp": "2021-12-20T20:50:44-08:00",
          "tree_id": "ba99f339e8e2a26dc5c7fcbb8a15fdda7af1c44f",
          "url": "https://github.com/unicode-org/icu4x/commit/7769518dc099f1763061aa3be3df86bacffd9cbb"
        },
        "date": 1640062803938,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 156433,
            "range": "± 6278",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1660153,
            "range": "± 42499",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 408169,
            "range": "± 13490",
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
          "id": "0a44cebda56a170654ed009c5d934d4c16d41299",
          "message": "Make icu_segmenter be no_std with lstm feature disabled (#1425)",
          "timestamp": "2021-12-21T09:42:54-08:00",
          "tree_id": "3aa733b235ae7b86dbaf95107c3cc01bfe2ad5d9",
          "url": "https://github.com/unicode-org/icu4x/commit/0a44cebda56a170654ed009c5d934d4c16d41299"
        },
        "date": 1640109105565,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 117557,
            "range": "± 8894",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1280720,
            "range": "± 57856",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 328982,
            "range": "± 36679",
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
          "id": "9e7e21843237dade01c42947fa3e4166a911fcb1",
          "message": "Move BorrowedVariant to ZeroVecLike (#1429)",
          "timestamp": "2021-12-21T12:17:25-08:00",
          "tree_id": "5d544310e3b392ee83d3cb3d7f279a53c744ec42",
          "url": "https://github.com/unicode-org/icu4x/commit/9e7e21843237dade01c42947fa3e4166a911fcb1"
        },
        "date": 1640118352350,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 133125,
            "range": "± 7164",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1463064,
            "range": "± 94873",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 397202,
            "range": "± 26046",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "m_kato@ga2.so-net.ne.jp",
            "name": "Makoto Kato",
            "username": "makotokato"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f4fee4264c86ea324e6cf5202632ca8838d6803b",
          "message": "Add build scripts in Cargo.toml for cargo vendor. (#1424)",
          "timestamp": "2021-12-22T09:32:46+09:00",
          "tree_id": "4a6285a9b07b5dd8bc8e49ffa5135480951aea00",
          "url": "https://github.com/unicode-org/icu4x/commit/f4fee4264c86ea324e6cf5202632ca8838d6803b"
        },
        "date": 1640133674019,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 140992,
            "range": "± 6973",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1479997,
            "range": "± 84944",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 380592,
            "range": "± 21270",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "m_kato@ga2.so-net.ne.jp",
            "name": "Makoto Kato",
            "username": "makotokato"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b7bb53376d5f67130cd79d7585fc24d93c1daf59",
          "message": "Update line break table per Unicode 14.0 (#1419)\n\n* Update link break table per Unicode 14.0\r\n\r\nFix https://github.com/unicode-org/icu4x/issues/1122\r\n\r\n* Fix per review comment.",
          "timestamp": "2021-12-22T19:40:38+09:00",
          "tree_id": "767e14216ac4ca0edfc9dc18efb10dcef776c4cf",
          "url": "https://github.com/unicode-org/icu4x/commit/b7bb53376d5f67130cd79d7585fc24d93c1daf59"
        },
        "date": 1640170156756,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 151340,
            "range": "± 12169",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1736245,
            "range": "± 129599",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 432651,
            "range": "± 27402",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "robertbastian@users.noreply.github.com",
            "name": "Robert Bastian",
            "username": "robertbastian"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5cfbb17701d725fe739afb5164c9a75d565674eb",
          "message": "Rewriting ListFormatter to only use appends (#1395)",
          "timestamp": "2021-12-22T18:13:12+01:00",
          "tree_id": "f3b5d4e6abfb8beea793cd0eed768989808bc5b2",
          "url": "https://github.com/unicode-org/icu4x/commit/5cfbb17701d725fe739afb5164c9a75d565674eb"
        },
        "date": 1640193690110,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 137699,
            "range": "± 6750",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1512852,
            "range": "± 66072",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 373090,
            "range": "± 14070",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "robertbastian@users.noreply.github.com",
            "name": "Robert Bastian",
            "username": "robertbastian"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "7eda44fdb2cef111dc0b1c4abaa769fbc974b7d9",
          "message": "Fuzz testing integer Writeable (#1421)",
          "timestamp": "2021-12-22T18:22:07+01:00",
          "tree_id": "20a0b11d1c1a63d6a27deb96443c73d52f05b236",
          "url": "https://github.com/unicode-org/icu4x/commit/7eda44fdb2cef111dc0b1c4abaa769fbc974b7d9"
        },
        "date": 1640194228140,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 150452,
            "range": "± 10787",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1620871,
            "range": "± 92166",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 417175,
            "range": "± 30394",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "robertbastian@users.noreply.github.com",
            "name": "Robert Bastian",
            "username": "robertbastian"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "0c1f751b6a50b74fbfa367758f9cf89cda55273a",
          "message": "Using new list data invariants (#1434)",
          "timestamp": "2021-12-22T19:00:03+01:00",
          "tree_id": "9aea966e2390f60b4377c37a1f75d3d72412831f",
          "url": "https://github.com/unicode-org/icu4x/commit/0c1f751b6a50b74fbfa367758f9cf89cda55273a"
        },
        "date": 1640196519226,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 113386,
            "range": "± 776",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1318856,
            "range": "± 5627",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 310787,
            "range": "± 1165",
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
          "id": "4f4ba6ec0880f97dc9c3f81720485784da460481",
          "message": "Add various helper functions to zerovec (#1430)\n\n- VZS::binary_search_in_range\r\n- ZV::get_subslice\r\n- RawBytesULE::as_unsigned_int()",
          "timestamp": "2021-12-22T12:06:15-08:00",
          "tree_id": "7370bbd9df32f95e9969690514149a4f80b42554",
          "url": "https://github.com/unicode-org/icu4x/commit/4f4ba6ec0880f97dc9c3f81720485784da460481"
        },
        "date": 1640204052948,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 99096,
            "range": "± 408",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1170034,
            "range": "± 3872",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 275794,
            "range": "± 1531",
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
          "id": "27ee71234c96033975f07f9d442a74b0629aae63",
          "message": "Update nightly to 2021-12-22 (#1442)",
          "timestamp": "2021-12-22T15:36:54-08:00",
          "tree_id": "523f5c7884991079d1343d7e478815f484102315",
          "url": "https://github.com/unicode-org/icu4x/commit/27ee71234c96033975f07f9d442a74b0629aae63"
        },
        "date": 1640216730527,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 122854,
            "range": "± 4176",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1460366,
            "range": "± 36801",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 345159,
            "range": "± 10475",
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
          "id": "1a87cea77e6e25f27c994ea40ee8bd21342b7534",
          "message": "Port UCharsTrie (#1264)\n\n* Add ucharstrie to utils\r\n\r\n* Import uchars_trie\r\n\r\nImport of the UCharsTrie implementation as of:\r\nhttps://github.com/makotokato/dictionary_segmenter/commit/4d79bbc7ad65d1f51f3d51d42996b3bc34cc2ae7\r\n\r\n* Rename uchars_trie to ucharstrie and add to lib.\r\n\r\n* Add get_value method\r\n\r\n* Add months unit test\r\n\r\n* Fold trie into ucharstrie\r\n\r\n* Add UCharsTrieIterator\r\n\r\n* Return value as part of TrieResult\r\n\r\n* Don't truncate input character to u16\r\n\r\n* Use ZeroVec\r\n\r\n* Add additional unit tests and fix bugs\r\n\r\n* Rename to Char16Trie and add documentation\r\n\r\n* Remove offset from API, improve documentation\r\n\r\n* Update generated readme\r\n\r\n* Fix issues found in CI\r\n\r\n* Fixup generated readme\r\n\r\n* Address review feedback\r\n\r\n* Bump ZV version\r\n\r\n* Address some review feedback\r\n\r\n* Move char16trie to experimental\r\n\r\n* Add todo for get panic\r\n\r\n* Use as_ule_slice",
          "timestamp": "2021-12-23T09:25:47-05:00",
          "tree_id": "6c0d7a257d9df1a1d2296654f9c0f2926e19359c",
          "url": "https://github.com/unicode-org/icu4x/commit/1a87cea77e6e25f27c994ea40ee8bd21342b7534"
        },
        "date": 1640270023009,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 111986,
            "range": "± 1008",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1286017,
            "range": "± 1541",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 311310,
            "range": "± 6191",
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
          "id": "d9cd5870dd444ba64ed6671089e1de4c471d1dd9",
          "message": "Remove SmallStr and recommend Cow<str> (#1417)",
          "timestamp": "2021-12-23T13:23:10-08:00",
          "tree_id": "78ad5c6a020995304aa59d662dc7a9dea4a5302a",
          "url": "https://github.com/unicode-org/icu4x/commit/d9cd5870dd444ba64ed6671089e1de4c471d1dd9"
        },
        "date": 1640295120314,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 164121,
            "range": "± 19916",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1733864,
            "range": "± 129036",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 430316,
            "range": "± 38051",
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
          "id": "061017f8c2a1e7463dd4bc27d1c7bb2eb9e17e1f",
          "message": "Fix license header check script (#1423)",
          "timestamp": "2021-12-23T20:46:21-08:00",
          "tree_id": "f19a0c864530708a52542031b65bd7b45b7c39c0",
          "url": "https://github.com/unicode-org/icu4x/commit/061017f8c2a1e7463dd4bc27d1c7bb2eb9e17e1f"
        },
        "date": 1640321654337,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 138675,
            "range": "± 1383",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1618187,
            "range": "± 4942",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 386984,
            "range": "± 2014",
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
          "id": "4bbbabc6d773396b9aaf2bb57f66136b233a3422",
          "message": "Update Diplomat (#1447)\n\n* Bump diplomat\n\n* Fix for new diplomat\n\n* Regen diplomat\n\n* fix tests",
          "timestamp": "2021-12-24T15:52:51-08:00",
          "tree_id": "e1bb9883099ad179ba06bd5fe13bdab7413eda00",
          "url": "https://github.com/unicode-org/icu4x/commit/4bbbabc6d773396b9aaf2bb57f66136b233a3422"
        },
        "date": 1640390458190,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 145808,
            "range": "± 4276",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1626063,
            "range": "± 41652",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 395153,
            "range": "± 6262",
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
          "id": "6b66b28f0f0a3f49a04e26c9b8277c47380f25c7",
          "message": "Add ZeroMap2d, a two-dimensional zero-copy map (#1432)",
          "timestamp": "2021-12-29T00:26:43-06:00",
          "tree_id": "7610c1ff6f6722e8620c8e4027fe046378071172",
          "url": "https://github.com/unicode-org/icu4x/commit/6b66b28f0f0a3f49a04e26c9b8277c47380f25c7"
        },
        "date": 1640759747667,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 153385,
            "range": "± 7375",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1641744,
            "range": "± 60712",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 420118,
            "range": "± 19849",
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
          "id": "fe67c68dcbb6996ac9d68fad4c05946f76a0342d",
          "message": "Add categories to yoke and zerovec (#1451)\n\n* Add categories to yoke and zerovec\n\n* data-structures",
          "timestamp": "2021-12-29T16:45:15-08:00",
          "tree_id": "964a4d62e164f9e759154f7dd9012170e7ac2991",
          "url": "https://github.com/unicode-org/icu4x/commit/fe67c68dcbb6996ac9d68fad4c05946f76a0342d"
        },
        "date": 1640825649281,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 131469,
            "range": "± 2943",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1519364,
            "range": "± 27843",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 368546,
            "range": "± 4683",
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
          "id": "4b8496d31067f23cd9446a3df0eab439908568a5",
          "message": "Re-write DataError to be Copy and use logging (#1449)",
          "timestamp": "2021-12-29T19:20:54-08:00",
          "tree_id": "86a32db4a1ba77fa5086d60842e960a2bc076d68",
          "url": "https://github.com/unicode-org/icu4x/commit/4b8496d31067f23cd9446a3df0eab439908568a5"
        },
        "date": 1640834946851,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 121770,
            "range": "± 8560",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1425737,
            "range": "± 96434",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 325520,
            "range": "± 21916",
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
          "id": "59edff5d06e20fe0c71e972d623e2a67ed80ee9e",
          "message": "Change Writeable::writeable_to_string to return a Cow (#1452)",
          "timestamp": "2021-12-30T12:22:03-08:00",
          "tree_id": "996f55c5023a97041e457910acd3c91f30f40b16",
          "url": "https://github.com/unicode-org/icu4x/commit/59edff5d06e20fe0c71e972d623e2a67ed80ee9e"
        },
        "date": 1640896220544,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 120994,
            "range": "± 7734",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1361816,
            "range": "± 99513",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 337513,
            "range": "± 28015",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "robertbastian@users.noreply.github.com",
            "name": "Robert Bastian",
            "username": "robertbastian"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "714acbd4ed5ce70722a60f102ad693ee48189048",
          "message": "ListFormatter example (#1456)",
          "timestamp": "2021-12-30T23:22:43+01:00",
          "tree_id": "989ecc8c140bb226aa6c7b1cf061a97b20c95fb7",
          "url": "https://github.com/unicode-org/icu4x/commit/714acbd4ed5ce70722a60f102ad693ee48189048"
        },
        "date": 1640903477921,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 124196,
            "range": "± 8812",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1380857,
            "range": "± 55805",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 342613,
            "range": "± 20442",
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
          "id": "7d9f89fcd7df4567e17ddd8c46810b0db287436a",
          "message": "Add EitherProvider and rename IterableDataProviderCore to IterableProvider (#1455)",
          "timestamp": "2021-12-30T18:02:47-08:00",
          "tree_id": "d23c5a53a2948f0032d7293bd0efaf622fe24012",
          "url": "https://github.com/unicode-org/icu4x/commit/7d9f89fcd7df4567e17ddd8c46810b0db287436a"
        },
        "date": 1640916742393,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 143540,
            "range": "± 8194",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1600666,
            "range": "± 55263",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 400200,
            "range": "± 19561",
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
          "id": "11290a0e0ff8a2b909a8d669b43b2ec7df5d3b15",
          "message": "Add back the empty data provider hook in FFI (#1466)",
          "timestamp": "2021-12-31T16:00:55-08:00",
          "tree_id": "414df51f932e56c715bc21748b5b027f1e39b597",
          "url": "https://github.com/unicode-org/icu4x/commit/11290a0e0ff8a2b909a8d669b43b2ec7df5d3b15"
        },
        "date": 1640995749331,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 130752,
            "range": "± 1042",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1535411,
            "range": "± 4899",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 367240,
            "range": "± 3932",
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
          "id": "0f7103311a587e423a2b74bf3f5f411d3769ee0f",
          "message": "Add boilerplate to new ListFormatter example (#1465)",
          "timestamp": "2022-01-03T10:54:55+01:00",
          "tree_id": "de7dc85d4baf39f2b86264191e0a371301a6369b",
          "url": "https://github.com/unicode-org/icu4x/commit/0f7103311a587e423a2b74bf3f5f411d3769ee0f"
        },
        "date": 1641204258999,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 141069,
            "range": "± 9645",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1594405,
            "range": "± 84836",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 403460,
            "range": "± 22430",
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
          "id": "9fdbd62e6de1f46f83248533955905a1bca167ad",
          "message": "Add ForkByKeyProvider, first multi-source data provider (#1463)\n\n* Update HelloWorldProvider and add docs to BufferProvider\r\n* Support BufferProvider in filters",
          "timestamp": "2022-01-03T18:19:25-06:00",
          "tree_id": "3bbea7b75819f81bea66ba9e8da9d85e0440113c",
          "url": "https://github.com/unicode-org/icu4x/commit/9fdbd62e6de1f46f83248533955905a1bca167ad"
        },
        "date": 1641256069255,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 132278,
            "range": "± 11579",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1397645,
            "range": "± 110433",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 343368,
            "range": "± 23415",
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
          "id": "0521917d5d6d75b927fd40bd3462a6275bde94e5",
          "message": "Remove almost empty and unused package-lock.json file from root directory. (#1469)",
          "timestamp": "2022-01-05T17:04:02-08:00",
          "tree_id": "7a0cff1cfa1f315add0e480dc6dc63da3bb4f6e7",
          "url": "https://github.com/unicode-org/icu4x/commit/0521917d5d6d75b927fd40bd3462a6275bde94e5"
        },
        "date": 1641431628752,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 137603,
            "range": "± 14048",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1529693,
            "range": "± 95948",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 334957,
            "range": "± 12652",
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
          "id": "625b0f97a2336cb3eb3782d6ad8e14b8ec162082",
          "message": "Skip node_modules in license header check (#1454)",
          "timestamp": "2022-01-06T12:47:37-08:00",
          "tree_id": "1927edaee3a477f8840d996fae3ea493b7f848ab",
          "url": "https://github.com/unicode-org/icu4x/commit/625b0f97a2336cb3eb3782d6ad8e14b8ec162082"
        },
        "date": 1641502550649,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 129843,
            "range": "± 333",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1534403,
            "range": "± 7158",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 364478,
            "range": "± 1434",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "pandusonu@google.com",
            "name": "Gollapudi Vamsi Krishna",
            "username": "pandusonu2"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "d87058a6a34e16582b6300abe9339b1916384a79",
          "message": "Julian calendar (#1351)\n\n* Julian calendar\n\n* fix logic and add tests\n\n* rustfmt\n\n* clippy\n\n* Minor changes\n\n* Year info\n\n* change by comments\n\n* Explain conversion to iso date\n\n* Add tests for julian to iso conversion\n\n* Add additional test case of March 1st in julian to iso conversion\n\n* Use book calculations for julian iso conversions\n\n* Fix calculations\n\n* clippy\n\n* Add lisp references",
          "timestamp": "2022-01-07T01:11:25-08:00",
          "tree_id": "1b4b21d1d981c56248ce6f465090d75473f55383",
          "url": "https://github.com/unicode-org/icu4x/commit/d87058a6a34e16582b6300abe9339b1916384a79"
        },
        "date": 1641547186365,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 130879,
            "range": "± 7667",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1448889,
            "range": "± 79822",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 360597,
            "range": "± 14532",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "robertbastian@users.noreply.github.com",
            "name": "Robert Bastian",
            "username": "robertbastian"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "794fa7ebeb262b2137cc9235b78fd208d5f54800",
          "message": "Using regex_automata in `ListFormatter` (#1435)",
          "timestamp": "2022-01-07T15:44:42+01:00",
          "tree_id": "fc51dc85aa1d10ec0f69fe114f70af20b6613122",
          "url": "https://github.com/unicode-org/icu4x/commit/794fa7ebeb262b2137cc9235b78fd208d5f54800"
        },
        "date": 1641567268316,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 151663,
            "range": "± 9297",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1693856,
            "range": "± 63128",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 421387,
            "range": "± 20001",
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
          "id": "7e0f1272e9a79643259c35a76fc61535061bcbd7",
          "message": "Update diplomat (#1478)\n\n* Bump diplomat version\r\n\r\n* Regen",
          "timestamp": "2022-01-07T11:11:54-08:00",
          "tree_id": "14e22380c0c60c2c5eb67fdf9f002c6b6b67b2a1",
          "url": "https://github.com/unicode-org/icu4x/commit/7e0f1272e9a79643259c35a76fc61535061bcbd7"
        },
        "date": 1641583194020,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 108497,
            "range": "± 209",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1279662,
            "range": "± 8636",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 305894,
            "range": "± 2685",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "samchen61661@gmail.com",
            "name": "samchen",
            "username": "samchen61661"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "899c90f463a41996758b969fa0f58159c8443452",
          "message": "Fix bug for time-zones provider specific variants (#1405)\n\nFixes a bug in which time-zone data was not being included if \"standard\"/\"daylight\" variants were present, but \"generic\" was not present.",
          "timestamp": "2022-01-07T16:39:47-08:00",
          "tree_id": "7897afdb70c979fdae1ce64fe6838d56abad530c",
          "url": "https://github.com/unicode-org/icu4x/commit/899c90f463a41996758b969fa0f58159c8443452"
        },
        "date": 1641602947802,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 142593,
            "range": "± 9976",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1498825,
            "range": "± 99727",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 476166,
            "range": "± 29471",
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
          "id": "96601c93b793a604608243be81cb10b44dc9f95a",
          "message": "Add FFI for constructing fixed decimals from float (#1483)\n\n* Expose fixed decimal construction functions from f32\r\n\r\n* float -> f64\r\n\r\n* regen\r\n\r\n* update method\r\n\r\n* regen\r\n\r\n* Update names",
          "timestamp": "2022-01-07T16:49:05-08:00",
          "tree_id": "eada95e71c75e562da4f2940079c985972bea299",
          "url": "https://github.com/unicode-org/icu4x/commit/96601c93b793a604608243be81cb10b44dc9f95a"
        },
        "date": 1641603466068,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 147731,
            "range": "± 6237",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1648197,
            "range": "± 65601",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 484666,
            "range": "± 34268",
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
          "id": "66fe4b7cfdd97c71d8dac308576cb14c826da2f4",
          "message": "Write design doc for yoke (#1459)\n\n* Add design doc\r\n\r\n* fix\r\n\r\n* fix\r\n\r\n* Manually apply a few changes\r\n\r\n* More feedback edited directly into the doc\r\n\r\n* Feedback\r\n\r\n* clarify lifetime erasure\r\n\r\n* quotes\r\n\r\n* mention targeting\r\n\r\nCo-authored-by: Shane F. Carr <shane@unicode.org>",
          "timestamp": "2022-01-07T17:11:19-08:00",
          "tree_id": "11b3bc8a3678246dad4e0464e3d635e1d7226519",
          "url": "https://github.com/unicode-org/icu4x/commit/66fe4b7cfdd97c71d8dac308576cb14c826da2f4"
        },
        "date": 1641604822219,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 127171,
            "range": "± 2995",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1418414,
            "range": "± 56398",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 406609,
            "range": "± 19134",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "samchen61661@gmail.com",
            "name": "samchen",
            "username": "samchen61661"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "89ce5f09b461ce302c4c7372e82a482a61007bbb",
          "message": "Retrieve localized metazone info from timeZoneNames.json (#1354)\n\nAdds functionality and data to have local overrides for metazone formatting, such as Europe/London formatting preferring to use \"British Summer Time\" for its \"daylight\" variant, instead of the default GMT metazone formatting.",
          "timestamp": "2022-01-07T18:45:48-08:00",
          "tree_id": "03c6822a228b060905d833db1f231b3de5bd7881",
          "url": "https://github.com/unicode-org/icu4x/commit/89ce5f09b461ce302c4c7372e82a482a61007bbb"
        },
        "date": 1641610503508,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 154373,
            "range": "± 6449",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1707087,
            "range": "± 50776",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 511869,
            "range": "± 20656",
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
          "id": "ef4125d78f71bbe26d3c9f891e93e92d1b227f00",
          "message": "Add EitherCart and rename Yoke ZCF constructors (#1484)\n\n* Centralize ZCF constructors into attach_to_zero_copy_cart",
          "timestamp": "2022-01-08T14:56:43-06:00",
          "tree_id": "6b18a215fceceb911830b792602347d1f15f02a6",
          "url": "https://github.com/unicode-org/icu4x/commit/ef4125d78f71bbe26d3c9f891e93e92d1b227f00"
        },
        "date": 1641675905936,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 127039,
            "range": "± 15621",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1362184,
            "range": "± 159111",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 457184,
            "range": "± 38771",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "robertbastian@users.noreply.github.com",
            "name": "Robert Bastian",
            "username": "robertbastian"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "cf216a8c9140a44ed61ffea6148d1c63927eb0af",
          "message": "Fixing test without --all-features (#1489)",
          "timestamp": "2022-01-10T10:51:25+01:00",
          "tree_id": "41b734a4f2c8f9333335f12ef4bfd79699fb3a52",
          "url": "https://github.com/unicode-org/icu4x/commit/cf216a8c9140a44ed61ffea6148d1c63927eb0af"
        },
        "date": 1641808802412,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 108353,
            "range": "± 233",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1281192,
            "range": "± 6235",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 370615,
            "range": "± 2770",
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
          "id": "f7d03669cfdb5bc7582051bb2aba36df7cc3d382",
          "message": "Update FDF FFI to use DiplomatResult (#1491)\n\n* Update FDF FFI to use DiplomatResult\r\n\r\n* regen",
          "timestamp": "2022-01-11T10:56:56-08:00",
          "tree_id": "86ee59a77a16308573735c2a3e6905169572fd7b",
          "url": "https://github.com/unicode-org/icu4x/commit/f7d03669cfdb5bc7582051bb2aba36df7cc3d382"
        },
        "date": 1641928181185,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 135184,
            "range": "± 9181",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1418786,
            "range": "± 90958",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 430112,
            "range": "± 37293",
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
          "id": "247b113f749ad327c15edb84e925b916ae7f75a1",
          "message": "Add -Ccodegen-units=1 (#1492)",
          "timestamp": "2022-01-11T13:01:09-08:00",
          "tree_id": "b05d99fe80aca8e4279a32c0be4f3f80f1f2b57f",
          "url": "https://github.com/unicode-org/icu4x/commit/247b113f749ad327c15edb84e925b916ae7f75a1"
        },
        "date": 1641935396406,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 141214,
            "range": "± 6810",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1580711,
            "range": "± 84883",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 457807,
            "range": "± 19518",
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
          "id": "8b26c199bcc07cb383ccbe6a41772e6fce714bc4",
          "message": "Make most Yokeable impls inline (#1493)",
          "timestamp": "2022-01-11T13:02:02-08:00",
          "tree_id": "0e2a30c9c0739e991de1733b2e4b7dabcc4a48ce",
          "url": "https://github.com/unicode-org/icu4x/commit/8b26c199bcc07cb383ccbe6a41772e6fce714bc4"
        },
        "date": 1641935439918,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 127885,
            "range": "± 6362",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1486472,
            "range": "± 69275",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 431214,
            "range": "± 24655",
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
          "id": "c6a9402a015fa39b08123ca89ca370986c4a7563",
          "message": "Add API for padding to a minimum number of decimal digits (#1482)\n\n* Add API for padding to a minimum number of decimal digits\r\n\r\n* Add padded_left, truncated_left, padded_right\r\n\r\n* rm print\r\n\r\n* magnitude -> digits\r\n\r\n* tests\r\n\r\n* fix test\r\n\r\n* Update utils/fixed_decimal/src/decimal.rs\r\n\r\nCo-authored-by: Shane F. Carr <shane@unicode.org>\r\n\r\n* Update utils/fixed_decimal/src/decimal.rs\r\n\r\nCo-authored-by: Shane F. Carr <shane@unicode.org>\r\n\r\n* cut\r\n\r\n* Add self -> self methods\r\n\r\n* fmt\r\n\r\n* invariants\r\n\r\n* invariants2\r\n\r\n* fix clippy\r\n\r\nCo-authored-by: Shane F. Carr <shane@unicode.org>",
          "timestamp": "2022-01-11T14:36:15-08:00",
          "tree_id": "510eeca73d373d57d727db867e268e352f53704f",
          "url": "https://github.com/unicode-org/icu4x/commit/c6a9402a015fa39b08123ca89ca370986c4a7563"
        },
        "date": 1641941100621,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 91888,
            "range": "± 177",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1105813,
            "range": "± 1761",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 321197,
            "range": "± 1199",
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
          "id": "07e35af5c204695f6f8d78c49b00b91199c31d6c",
          "message": "Wire DataProvider into LineBreakSegmenter (#1446)",
          "timestamp": "2022-01-11T17:08:46-08:00",
          "tree_id": "beb0d3e564aa3389316d28d686b5aa7fa618451c",
          "url": "https://github.com/unicode-org/icu4x/commit/07e35af5c204695f6f8d78c49b00b91199c31d6c"
        },
        "date": 1641950263948,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 124619,
            "range": "± 10395",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1476836,
            "range": "± 16990",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 425016,
            "range": "± 9918",
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
          "id": "821af5a5cceabf839e76d1c3af9a062ba753a491",
          "message": "Update diplomat (#1498)\n\n* Update Diplomat\n\n* regen\n\n* fix update",
          "timestamp": "2022-01-11T17:35:17-08:00",
          "tree_id": "1182a2a34b89f067621b2e6cadf3b1c835da9a82",
          "url": "https://github.com/unicode-org/icu4x/commit/821af5a5cceabf839e76d1c3af9a062ba753a491"
        },
        "date": 1641951810540,
        "tool": "cargo",
        "benches": [
          {
            "name": "datetime/datetime_lengths",
            "value": 115299,
            "range": "± 9283",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/datetime_components",
            "value": 1331563,
            "range": "± 74957",
            "unit": "ns/iter"
          },
          {
            "name": "datetime/zoned_datetime_overview",
            "value": 393235,
            "range": "± 50411",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}