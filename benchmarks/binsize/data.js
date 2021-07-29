window.BENCHMARK_DATA = {
  "lastUpdate": 1627598183353,
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
      }
    ]
  }
}