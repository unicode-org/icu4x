window.BENCHMARK_DATA = {
  "lastUpdate": 1627580013782,
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
      }
    ]
  }
}