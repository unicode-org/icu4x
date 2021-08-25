window.BENCHMARK_DATA = {
  "lastUpdate": 1629934343440,
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
        "date": 1629934341127,
        "tool": "ndjson",
        "benches": [
          {
            "name": "borrowed_pattern+opt.wasm.gz",
            "value": 5986,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unicode_bmp_blocks_selector+opt.wasm.gz",
            "value": 5735,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "syntatically_canonicalize_locales+opt.wasm.gz",
            "value": 13336,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_hash_map+opt.wasm.gz",
            "value": 4590,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "filter_langids+opt.wasm.gz",
            "value": 7022,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "derive+opt.wasm.gz",
            "value": 3728,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "zv_serde+opt.wasm.gz",
            "value": 221,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unread_emails+opt.wasm.gz",
            "value": 6925,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_lite_map+opt.wasm.gz",
            "value": 3277,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "work_log+opt.wasm.gz",
            "value": 6919,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "simple+opt.wasm.gz",
            "value": 120146,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "elevator_floors+opt.wasm.gz",
            "value": 6924,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "permyriad+opt.wasm.gz",
            "value": 4091,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "owned_pattern+opt.wasm.gz",
            "value": 7123,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "tui+opt.wasm.gz",
            "value": 6914,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable_message+opt.wasm.gz",
            "value": 3946,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "code_line_diff+opt.wasm.gz",
            "value": 6932,
            "unit": "bytes",
            "biggerIsBetter": false
          }
        ]
      }
    ]
  }
}