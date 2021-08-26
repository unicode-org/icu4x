window.BENCHMARK_DATA = {
  "lastUpdate": 1629943750557,
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
        "date": 1629943747415,
        "tool": "ndjson",
        "benches": [
          {
            "name": "derive+opt.wasm",
            "value": 8677,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "filter_langids+opt.wasm",
            "value": 12281,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unread_emails+opt.wasm",
            "value": 6577,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "permyriad+opt.wasm",
            "value": 7925,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable_message+opt.wasm",
            "value": 9040,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "owned_pattern+opt.wasm",
            "value": 15992,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_hash_map+opt.wasm",
            "value": 10136,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "zv_serde+opt.wasm",
            "value": 197,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "code_line_diff+opt.wasm",
            "value": 6569,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "simple+opt.wasm",
            "value": 609724,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_lite_map+opt.wasm",
            "value": 7191,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "tui+opt.wasm",
            "value": 6534,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "borrowed_pattern+opt.wasm",
            "value": 13475,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "elevator_floors+opt.wasm",
            "value": 6577,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "work_log+opt.wasm",
            "value": 6534,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unicode_bmp_blocks_selector+opt.wasm",
            "value": 12859,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "syntatically_canonicalize_locales+opt.wasm",
            "value": 29329,
            "unit": "bytes",
            "biggerIsBetter": false
          }
        ]
      }
    ]
  }
}