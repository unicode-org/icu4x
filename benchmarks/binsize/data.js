window.BENCHMARK_DATA = {
  "lastUpdate": 1629134522076,
  "repoUrl": "https://github.com/unicode-org/icu4x",
  "entries": {
    "Benchmark": [
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
        "date": 1627580011233,
        "tool": "ndjson",
        "benches": [
          {
            "name": "derive+opt",
            "value": 9386,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "filter_langids+opt",
            "value": 16510,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unread_emails+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "permyriad+opt",
            "value": 9118,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable_message+opt",
            "value": 9662,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "owned_pattern+opt",
            "value": 16377,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_hash_map+opt",
            "value": 9728,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "zv_serde+opt",
            "value": 236,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "code_line_diff+opt",
            "value": 13315,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "simple+opt",
            "value": 611299,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_lite_map+opt",
            "value": 7299,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "tui+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "borrowed_pattern+opt",
            "value": 13426,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "elevator_floors+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "work_log+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unicode_bmp_blocks_selector+opt",
            "value": 12750,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "syntatically_canonicalize_locales+opt",
            "value": 34103,
            "unit": "bytes",
            "biggerIsBetter": false
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
        "date": 1627580182893,
        "tool": "ndjson",
        "benches": [
          {
            "name": "derive+opt",
            "value": 9386,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "filter_langids+opt",
            "value": 16510,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unread_emails+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "permyriad+opt",
            "value": 9118,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable_message+opt",
            "value": 9662,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "owned_pattern+opt",
            "value": 16377,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_hash_map+opt",
            "value": 9728,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "zv_serde+opt",
            "value": 236,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "code_line_diff+opt",
            "value": 13315,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "simple+opt",
            "value": 611299,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_lite_map+opt",
            "value": 7299,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "tui+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "borrowed_pattern+opt",
            "value": 13426,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "elevator_floors+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "work_log+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unicode_bmp_blocks_selector+opt",
            "value": 12750,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "syntatically_canonicalize_locales+opt",
            "value": 34103,
            "unit": "bytes",
            "biggerIsBetter": false
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
        "date": 1627582880214,
        "tool": "ndjson",
        "benches": [
          {
            "name": "derive+opt",
            "value": 9386,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "filter_langids+opt",
            "value": 16510,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unread_emails+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "permyriad+opt",
            "value": 9118,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable_message+opt",
            "value": 9662,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "owned_pattern+opt",
            "value": 16377,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_hash_map+opt",
            "value": 9728,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "zv_serde+opt",
            "value": 236,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "code_line_diff+opt",
            "value": 13315,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "simple+opt",
            "value": 611299,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_lite_map+opt",
            "value": 7299,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "tui+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "borrowed_pattern+opt",
            "value": 13426,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "elevator_floors+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "work_log+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unicode_bmp_blocks_selector+opt",
            "value": 12750,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "syntatically_canonicalize_locales+opt",
            "value": 34103,
            "unit": "bytes",
            "biggerIsBetter": false
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
        "date": 1627596367482,
        "tool": "ndjson",
        "benches": [
          {
            "name": "derive+opt",
            "value": 9386,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "filter_langids+opt",
            "value": 16510,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unread_emails+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "permyriad+opt",
            "value": 9118,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable_message+opt",
            "value": 9662,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "owned_pattern+opt",
            "value": 16377,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_hash_map+opt",
            "value": 9728,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "zv_serde+opt",
            "value": 236,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "code_line_diff+opt",
            "value": 13315,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "simple+opt",
            "value": 611299,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_lite_map+opt",
            "value": 7299,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "tui+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "borrowed_pattern+opt",
            "value": 13426,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "elevator_floors+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "work_log+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unicode_bmp_blocks_selector+opt",
            "value": 12750,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "syntatically_canonicalize_locales+opt",
            "value": 34103,
            "unit": "bytes",
            "biggerIsBetter": false
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
        "date": 1627598177670,
        "tool": "ndjson",
        "benches": [
          {
            "name": "derive+opt",
            "value": 9386,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "filter_langids+opt",
            "value": 16510,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unread_emails+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "permyriad+opt",
            "value": 9118,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable_message+opt",
            "value": 9662,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "owned_pattern+opt",
            "value": 16377,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_hash_map+opt",
            "value": 9728,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "zv_serde+opt",
            "value": 236,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "code_line_diff+opt",
            "value": 13315,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "simple+opt",
            "value": 611299,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_lite_map+opt",
            "value": 7299,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "tui+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "borrowed_pattern+opt",
            "value": 13426,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "elevator_floors+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "work_log+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unicode_bmp_blocks_selector+opt",
            "value": 12750,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "syntatically_canonicalize_locales+opt",
            "value": 34103,
            "unit": "bytes",
            "biggerIsBetter": false
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
        "date": 1627603511917,
        "tool": "ndjson",
        "benches": [
          {
            "name": "derive+opt",
            "value": 9386,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "filter_langids+opt",
            "value": 16510,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unread_emails+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "permyriad+opt",
            "value": 9118,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable_message+opt",
            "value": 9662,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "owned_pattern+opt",
            "value": 16377,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_hash_map+opt",
            "value": 9728,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "zv_serde+opt",
            "value": 236,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "code_line_diff+opt",
            "value": 13315,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "simple+opt",
            "value": 611299,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_lite_map+opt",
            "value": 7299,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "tui+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "borrowed_pattern+opt",
            "value": 13426,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "elevator_floors+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "work_log+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unicode_bmp_blocks_selector+opt",
            "value": 12750,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "syntatically_canonicalize_locales+opt",
            "value": 34103,
            "unit": "bytes",
            "biggerIsBetter": false
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
        "date": 1627926431425,
        "tool": "ndjson",
        "benches": [
          {
            "name": "derive+opt",
            "value": 9386,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "filter_langids+opt",
            "value": 15470,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unread_emails+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "permyriad+opt",
            "value": 9118,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable_message+opt",
            "value": 9662,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "owned_pattern+opt",
            "value": 16377,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_hash_map+opt",
            "value": 9728,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "zv_serde+opt",
            "value": 236,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "code_line_diff+opt",
            "value": 13315,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "simple+opt",
            "value": 611299,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_lite_map+opt",
            "value": 7299,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "tui+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "borrowed_pattern+opt",
            "value": 13426,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "elevator_floors+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "work_log+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unicode_bmp_blocks_selector+opt",
            "value": 12750,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "syntatically_canonicalize_locales+opt",
            "value": 34103,
            "unit": "bytes",
            "biggerIsBetter": false
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
        "date": 1627929039005,
        "tool": "ndjson",
        "benches": [
          {
            "name": "derive+opt",
            "value": 9386,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "filter_langids+opt",
            "value": 15470,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unread_emails+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "permyriad+opt",
            "value": 9118,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable_message+opt",
            "value": 9662,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "owned_pattern+opt",
            "value": 16377,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_hash_map+opt",
            "value": 9728,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "zv_serde+opt",
            "value": 236,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "code_line_diff+opt",
            "value": 13315,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "simple+opt",
            "value": 611299,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_lite_map+opt",
            "value": 7299,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "tui+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "borrowed_pattern+opt",
            "value": 13426,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "elevator_floors+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "work_log+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unicode_bmp_blocks_selector+opt",
            "value": 12750,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "syntatically_canonicalize_locales+opt",
            "value": 34103,
            "unit": "bytes",
            "biggerIsBetter": false
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
        "date": 1628007692310,
        "tool": "ndjson",
        "benches": [
          {
            "name": "derive+opt",
            "value": 9386,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "filter_langids+opt",
            "value": 15470,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unread_emails+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "permyriad+opt",
            "value": 9118,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable_message+opt",
            "value": 9662,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "owned_pattern+opt",
            "value": 16377,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_hash_map+opt",
            "value": 9728,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "zv_serde+opt",
            "value": 236,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "code_line_diff+opt",
            "value": 13315,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "simple+opt",
            "value": 611299,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_lite_map+opt",
            "value": 7299,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "tui+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "borrowed_pattern+opt",
            "value": 13426,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "elevator_floors+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "work_log+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unicode_bmp_blocks_selector+opt",
            "value": 12750,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "syntatically_canonicalize_locales+opt",
            "value": 34103,
            "unit": "bytes",
            "biggerIsBetter": false
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
        "date": 1628025269119,
        "tool": "ndjson",
        "benches": [
          {
            "name": "derive+opt",
            "value": 9386,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "filter_langids+opt",
            "value": 15470,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unread_emails+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "permyriad+opt",
            "value": 9118,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable_message+opt",
            "value": 9662,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "owned_pattern+opt",
            "value": 16377,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_hash_map+opt",
            "value": 9728,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "zv_serde+opt",
            "value": 236,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "code_line_diff+opt",
            "value": 13315,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "simple+opt",
            "value": 611299,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_lite_map+opt",
            "value": 7299,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "tui+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "borrowed_pattern+opt",
            "value": 13426,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "elevator_floors+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "work_log+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unicode_bmp_blocks_selector+opt",
            "value": 12750,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "syntatically_canonicalize_locales+opt",
            "value": 34103,
            "unit": "bytes",
            "biggerIsBetter": false
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
        "date": 1628147424026,
        "tool": "ndjson",
        "benches": [
          {
            "name": "derive+opt",
            "value": 9386,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "filter_langids+opt",
            "value": 15470,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unread_emails+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "permyriad+opt",
            "value": 9118,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable_message+opt",
            "value": 9662,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "owned_pattern+opt",
            "value": 16377,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_hash_map+opt",
            "value": 9728,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "zv_serde+opt",
            "value": 236,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "code_line_diff+opt",
            "value": 13315,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "simple+opt",
            "value": 611299,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_lite_map+opt",
            "value": 7299,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "tui+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "borrowed_pattern+opt",
            "value": 13426,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "elevator_floors+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "work_log+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unicode_bmp_blocks_selector+opt",
            "value": 12750,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "syntatically_canonicalize_locales+opt",
            "value": 34103,
            "unit": "bytes",
            "biggerIsBetter": false
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
        "date": 1628205697716,
        "tool": "ndjson",
        "benches": [
          {
            "name": "derive+opt",
            "value": 9386,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "filter_langids+opt",
            "value": 15470,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unread_emails+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "permyriad+opt",
            "value": 9118,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable_message+opt",
            "value": 9662,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "owned_pattern+opt",
            "value": 16377,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_hash_map+opt",
            "value": 9728,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "zv_serde+opt",
            "value": 236,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "code_line_diff+opt",
            "value": 13315,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "simple+opt",
            "value": 611299,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_lite_map+opt",
            "value": 7299,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "tui+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "borrowed_pattern+opt",
            "value": 13426,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "elevator_floors+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "work_log+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unicode_bmp_blocks_selector+opt",
            "value": 12750,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "syntatically_canonicalize_locales+opt",
            "value": 34103,
            "unit": "bytes",
            "biggerIsBetter": false
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
        "date": 1628209692849,
        "tool": "ndjson",
        "benches": [
          {
            "name": "derive+opt",
            "value": 9386,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "filter_langids+opt",
            "value": 15470,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unread_emails+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "permyriad+opt",
            "value": 9118,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable_message+opt",
            "value": 9662,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "owned_pattern+opt",
            "value": 16377,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_hash_map+opt",
            "value": 9728,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "zv_serde+opt",
            "value": 236,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "code_line_diff+opt",
            "value": 13315,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "simple+opt",
            "value": 611299,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_lite_map+opt",
            "value": 7299,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "tui+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "borrowed_pattern+opt",
            "value": 13426,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "elevator_floors+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "work_log+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unicode_bmp_blocks_selector+opt",
            "value": 12750,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "syntatically_canonicalize_locales+opt",
            "value": 34103,
            "unit": "bytes",
            "biggerIsBetter": false
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
        "date": 1628271452009,
        "tool": "ndjson",
        "benches": [
          {
            "name": "derive+opt",
            "value": 9386,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "filter_langids+opt",
            "value": 15470,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unread_emails+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "permyriad+opt",
            "value": 9118,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable_message+opt",
            "value": 9662,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "owned_pattern+opt",
            "value": 16377,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_hash_map+opt",
            "value": 9728,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "zv_serde+opt",
            "value": 236,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "code_line_diff+opt",
            "value": 13315,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "simple+opt",
            "value": 611299,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_lite_map+opt",
            "value": 7299,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "tui+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "borrowed_pattern+opt",
            "value": 13426,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "elevator_floors+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "work_log+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unicode_bmp_blocks_selector+opt",
            "value": 12750,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "syntatically_canonicalize_locales+opt",
            "value": 34103,
            "unit": "bytes",
            "biggerIsBetter": false
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
        "date": 1628283944167,
        "tool": "ndjson",
        "benches": [
          {
            "name": "derive+opt",
            "value": 9386,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "filter_langids+opt",
            "value": 15470,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unread_emails+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "permyriad+opt",
            "value": 9118,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable_message+opt",
            "value": 9662,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "owned_pattern+opt",
            "value": 16377,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_hash_map+opt",
            "value": 9728,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "zv_serde+opt",
            "value": 236,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "code_line_diff+opt",
            "value": 13315,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "simple+opt",
            "value": 611299,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_lite_map+opt",
            "value": 7299,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "tui+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "borrowed_pattern+opt",
            "value": 13426,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "elevator_floors+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "work_log+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unicode_bmp_blocks_selector+opt",
            "value": 12750,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "syntatically_canonicalize_locales+opt",
            "value": 34103,
            "unit": "bytes",
            "biggerIsBetter": false
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
        "date": 1628303156746,
        "tool": "ndjson",
        "benches": [
          {
            "name": "derive+opt",
            "value": 9386,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "filter_langids+opt",
            "value": 15470,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unread_emails+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "permyriad+opt",
            "value": 9118,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable_message+opt",
            "value": 9662,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "owned_pattern+opt",
            "value": 16377,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_hash_map+opt",
            "value": 9728,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "zv_serde+opt",
            "value": 236,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "code_line_diff+opt",
            "value": 13315,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "simple+opt",
            "value": 611299,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_lite_map+opt",
            "value": 7299,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "tui+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "borrowed_pattern+opt",
            "value": 13426,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "elevator_floors+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "work_log+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unicode_bmp_blocks_selector+opt",
            "value": 12750,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "syntatically_canonicalize_locales+opt",
            "value": 34103,
            "unit": "bytes",
            "biggerIsBetter": false
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
          "id": "f0977050566fcac7d8b8427feba50eed91f9d1a5",
          "message": "Use cargo-all-features' new denylist feature to improve our CI time (#942)\n\n* Update cargo-all-features version\r\n\r\n* Stop including bench feature in cargo-all-features\r\n\r\n* Deny more features for capi crate",
          "timestamp": "2021-08-09T10:17:29-07:00",
          "tree_id": "c83289bcca60a0624f03ef3382fb9124e7b72393",
          "url": "https://github.com/unicode-org/icu4x/commit/f0977050566fcac7d8b8427feba50eed91f9d1a5"
        },
        "date": 1628529784470,
        "tool": "ndjson",
        "benches": [
          {
            "name": "derive+opt",
            "value": 9386,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "filter_langids+opt",
            "value": 15470,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unread_emails+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "permyriad+opt",
            "value": 9118,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable_message+opt",
            "value": 9662,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "owned_pattern+opt",
            "value": 16377,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_hash_map+opt",
            "value": 9728,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "zv_serde+opt",
            "value": 236,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "code_line_diff+opt",
            "value": 13315,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "simple+opt",
            "value": 611299,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_lite_map+opt",
            "value": 7299,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "tui+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "borrowed_pattern+opt",
            "value": 13426,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "elevator_floors+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "work_log+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unicode_bmp_blocks_selector+opt",
            "value": 12750,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "syntatically_canonicalize_locales+opt",
            "value": 34103,
            "unit": "bytes",
            "biggerIsBetter": false
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
        "date": 1628640299976,
        "tool": "ndjson",
        "benches": [
          {
            "name": "derive+opt",
            "value": 9386,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "filter_langids+opt",
            "value": 15470,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unread_emails+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "permyriad+opt",
            "value": 9118,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable_message+opt",
            "value": 9662,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "owned_pattern+opt",
            "value": 16377,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_hash_map+opt",
            "value": 9728,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "zv_serde+opt",
            "value": 236,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "code_line_diff+opt",
            "value": 13315,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "simple+opt",
            "value": 611299,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_lite_map+opt",
            "value": 7299,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "tui+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "borrowed_pattern+opt",
            "value": 13426,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "elevator_floors+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "work_log+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unicode_bmp_blocks_selector+opt",
            "value": 12750,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "syntatically_canonicalize_locales+opt",
            "value": 34103,
            "unit": "bytes",
            "biggerIsBetter": false
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
        "date": 1628698107928,
        "tool": "ndjson",
        "benches": [
          {
            "name": "derive+opt",
            "value": 9386,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "filter_langids+opt",
            "value": 15470,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unread_emails+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "permyriad+opt",
            "value": 9118,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable_message+opt",
            "value": 9662,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "owned_pattern+opt",
            "value": 16377,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_hash_map+opt",
            "value": 9728,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "zv_serde+opt",
            "value": 236,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "code_line_diff+opt",
            "value": 13315,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "simple+opt",
            "value": 611948,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_lite_map+opt",
            "value": 7299,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "tui+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "borrowed_pattern+opt",
            "value": 13426,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "elevator_floors+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "work_log+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unicode_bmp_blocks_selector+opt",
            "value": 12750,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "syntatically_canonicalize_locales+opt",
            "value": 34103,
            "unit": "bytes",
            "biggerIsBetter": false
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
        "date": 1628724004564,
        "tool": "ndjson",
        "benches": [
          {
            "name": "derive+opt",
            "value": 9386,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "filter_langids+opt",
            "value": 15470,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unread_emails+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "permyriad+opt",
            "value": 9118,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable_message+opt",
            "value": 9662,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "owned_pattern+opt",
            "value": 16377,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_hash_map+opt",
            "value": 9728,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "zv_serde+opt",
            "value": 236,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "code_line_diff+opt",
            "value": 13315,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "simple+opt",
            "value": 611948,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_lite_map+opt",
            "value": 7299,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "tui+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "borrowed_pattern+opt",
            "value": 13426,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "elevator_floors+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "work_log+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unicode_bmp_blocks_selector+opt",
            "value": 12750,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "syntatically_canonicalize_locales+opt",
            "value": 34103,
            "unit": "bytes",
            "biggerIsBetter": false
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
        "date": 1628796281789,
        "tool": "ndjson",
        "benches": [
          {
            "name": "derive+opt",
            "value": 9386,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "filter_langids+opt",
            "value": 15470,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unread_emails+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "permyriad+opt",
            "value": 9118,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable_message+opt",
            "value": 9662,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "owned_pattern+opt",
            "value": 16377,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_hash_map+opt",
            "value": 9728,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "zv_serde+opt",
            "value": 236,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "code_line_diff+opt",
            "value": 13315,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "simple+opt",
            "value": 611948,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_lite_map+opt",
            "value": 7299,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "tui+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "borrowed_pattern+opt",
            "value": 13426,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "elevator_floors+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "work_log+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unicode_bmp_blocks_selector+opt",
            "value": 12750,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "syntatically_canonicalize_locales+opt",
            "value": 34103,
            "unit": "bytes",
            "biggerIsBetter": false
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
        "date": 1629134517749,
        "tool": "ndjson",
        "benches": [
          {
            "name": "derive+opt",
            "value": 9386,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "filter_langids+opt",
            "value": 15470,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unread_emails+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "permyriad+opt",
            "value": 9118,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable_message+opt",
            "value": 9662,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "owned_pattern+opt",
            "value": 16377,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_hash_map+opt",
            "value": 9728,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "zv_serde+opt",
            "value": 236,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "code_line_diff+opt",
            "value": 13315,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "simple+opt",
            "value": 611948,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_lite_map+opt",
            "value": 7299,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "tui+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "borrowed_pattern+opt",
            "value": 13426,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "elevator_floors+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "work_log+opt",
            "value": 13303,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unicode_bmp_blocks_selector+opt",
            "value": 12750,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "syntatically_canonicalize_locales+opt",
            "value": 34103,
            "unit": "bytes",
            "biggerIsBetter": false
          }
        ]
      }
    ]
  }
}