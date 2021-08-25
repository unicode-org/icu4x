window.BENCHMARK_DATA = {
  "lastUpdate": 1629934342065,
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
          "id": "569d0c0347d650a4cc6b9f45492067a6edf428c3",
          "message": "Enhance binary size benchmark: monitor size of the gzip'd wasm (#980)\n\nexecutables as well.\r\n\r\nResolves ticket #912.",
          "timestamp": "2021-08-25T16:28:30-07:00",
          "tree_id": "d77852153e52c38b11070795b2ec786b074f728a",
          "url": "https://github.com/unicode-org/icu4x/commit/569d0c0347d650a4cc6b9f45492067a6edf428c3"
        },
        "date": 1629934339328,
        "tool": "ndjson",
        "benches": [
          {
            "name": "derive+opt.wasm",
            "value": 8468,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "filter_langids+opt.wasm",
            "value": 15403,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unread_emails+opt.wasm",
            "value": 12674,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "permyriad+opt.wasm",
            "value": 9004,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable_message+opt.wasm",
            "value": 8827,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "owned_pattern+opt.wasm",
            "value": 15984,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_hash_map+opt.wasm",
            "value": 9667,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "zv_serde+opt.wasm",
            "value": 196,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "code_line_diff+opt.wasm",
            "value": 12679,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "simple+opt.wasm",
            "value": 610276,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_lite_map+opt.wasm",
            "value": 7235,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "tui+opt.wasm",
            "value": 12674,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "borrowed_pattern+opt.wasm",
            "value": 13037,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "elevator_floors+opt.wasm",
            "value": 12674,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "work_log+opt.wasm",
            "value": 12674,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unicode_bmp_blocks_selector+opt.wasm",
            "value": 12805,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "syntatically_canonicalize_locales+opt.wasm",
            "value": 31966,
            "unit": "bytes",
            "biggerIsBetter": false
          }
        ]
      }
    ]
  }
}