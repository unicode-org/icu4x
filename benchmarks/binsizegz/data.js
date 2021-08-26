window.BENCHMARK_DATA = {
  "lastUpdate": 1630000991689,
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
        "date": 1629943720586,
        "tool": "ndjson",
        "benches": [
          {
            "name": "borrowed_pattern+opt.wasm.gz",
            "value": 5990,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unicode_bmp_blocks_selector+opt.wasm.gz",
            "value": 5806,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "syntatically_canonicalize_locales+opt.wasm.gz",
            "value": 12396,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_hash_map+opt.wasm.gz",
            "value": 4699,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "filter_langids+opt.wasm.gz",
            "value": 5815,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "derive+opt.wasm.gz",
            "value": 3768,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "zv_serde+opt.wasm.gz",
            "value": 222,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unread_emails+opt.wasm.gz",
            "value": 2919,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_lite_map+opt.wasm.gz",
            "value": 3229,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "work_log+opt.wasm.gz",
            "value": 2893,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "simple+opt.wasm.gz",
            "value": 119489,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "elevator_floors+opt.wasm.gz",
            "value": 2924,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "permyriad+opt.wasm.gz",
            "value": 3561,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "owned_pattern+opt.wasm.gz",
            "value": 7079,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "tui+opt.wasm.gz",
            "value": 2888,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable_message+opt.wasm.gz",
            "value": 3984,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "code_line_diff+opt.wasm.gz",
            "value": 2924,
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
          "id": "3d668128cfd7f4f04406950f779120c1c4b3d1a9",
          "message": "Remove obsolete ToOwned impls for [SerdeSe/Erased]DataStruct (#1011)",
          "timestamp": "2021-08-25T23:30:22-05:00",
          "tree_id": "4c4920fdf7d06cfec9348867524d80285a58f397",
          "url": "https://github.com/unicode-org/icu4x/commit/3d668128cfd7f4f04406950f779120c1c4b3d1a9"
        },
        "date": 1629952456604,
        "tool": "ndjson",
        "benches": [
          {
            "name": "borrowed_pattern+opt.wasm.gz",
            "value": 5990,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unicode_bmp_blocks_selector+opt.wasm.gz",
            "value": 5806,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "syntatically_canonicalize_locales+opt.wasm.gz",
            "value": 12396,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_hash_map+opt.wasm.gz",
            "value": 4699,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "filter_langids+opt.wasm.gz",
            "value": 5815,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "derive+opt.wasm.gz",
            "value": 3768,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "zv_serde+opt.wasm.gz",
            "value": 222,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unread_emails+opt.wasm.gz",
            "value": 2919,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_lite_map+opt.wasm.gz",
            "value": 3229,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "work_log+opt.wasm.gz",
            "value": 2893,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "simple+opt.wasm.gz",
            "value": 119489,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "elevator_floors+opt.wasm.gz",
            "value": 2924,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "permyriad+opt.wasm.gz",
            "value": 3561,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "owned_pattern+opt.wasm.gz",
            "value": 7079,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "tui+opt.wasm.gz",
            "value": 2888,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable_message+opt.wasm.gz",
            "value": 3984,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "code_line_diff+opt.wasm.gz",
            "value": 2924,
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
          "id": "7136f99745d3ac01ecc4baebd66a2dbbdbc9a97e",
          "message": "Remove ErasedDataStruct::clone_into_box() (#1013)",
          "timestamp": "2021-08-26T01:23:09-05:00",
          "tree_id": "209a5bae0de7793c4308206163a1f7bb44ff09f6",
          "url": "https://github.com/unicode-org/icu4x/commit/7136f99745d3ac01ecc4baebd66a2dbbdbc9a97e"
        },
        "date": 1629959274135,
        "tool": "ndjson",
        "benches": [
          {
            "name": "borrowed_pattern+opt.wasm.gz",
            "value": 5990,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unicode_bmp_blocks_selector+opt.wasm.gz",
            "value": 5806,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "syntatically_canonicalize_locales+opt.wasm.gz",
            "value": 12396,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_hash_map+opt.wasm.gz",
            "value": 4699,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "filter_langids+opt.wasm.gz",
            "value": 5815,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "derive+opt.wasm.gz",
            "value": 3768,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "zv_serde+opt.wasm.gz",
            "value": 222,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unread_emails+opt.wasm.gz",
            "value": 2919,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_lite_map+opt.wasm.gz",
            "value": 3229,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "work_log+opt.wasm.gz",
            "value": 2893,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "simple+opt.wasm.gz",
            "value": 119489,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "elevator_floors+opt.wasm.gz",
            "value": 2924,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "permyriad+opt.wasm.gz",
            "value": 3561,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "owned_pattern+opt.wasm.gz",
            "value": 7079,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "tui+opt.wasm.gz",
            "value": 2888,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable_message+opt.wasm.gz",
            "value": 3984,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "code_line_diff+opt.wasm.gz",
            "value": 2924,
            "unit": "bytes",
            "biggerIsBetter": false
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
        "date": 1629978902567,
        "tool": "ndjson",
        "benches": [
          {
            "name": "borrowed_pattern+opt.wasm.gz",
            "value": 5990,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unicode_bmp_blocks_selector+opt.wasm.gz",
            "value": 5806,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "syntatically_canonicalize_locales+opt.wasm.gz",
            "value": 12924,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_hash_map+opt.wasm.gz",
            "value": 4699,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "filter_langids+opt.wasm.gz",
            "value": 5815,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "derive+opt.wasm.gz",
            "value": 3768,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "zv_serde+opt.wasm.gz",
            "value": 222,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unread_emails+opt.wasm.gz",
            "value": 2919,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_lite_map+opt.wasm.gz",
            "value": 3229,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "work_log+opt.wasm.gz",
            "value": 2893,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "simple+opt.wasm.gz",
            "value": 119489,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "elevator_floors+opt.wasm.gz",
            "value": 2924,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "permyriad+opt.wasm.gz",
            "value": 3561,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "owned_pattern+opt.wasm.gz",
            "value": 7079,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "tui+opt.wasm.gz",
            "value": 2888,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable_message+opt.wasm.gz",
            "value": 3984,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "code_line_diff+opt.wasm.gz",
            "value": 2924,
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
          "id": "9bcfb461c75ea93697647079b4a7025f12bbcfbe",
          "message": "Update pinned nightly for memory benchmarks and coverage (#1015)",
          "timestamp": "2021-08-26T08:20:17-07:00",
          "tree_id": "7936878bf9317b1c5183a45b1abd5884012ac071",
          "url": "https://github.com/unicode-org/icu4x/commit/9bcfb461c75ea93697647079b4a7025f12bbcfbe"
        },
        "date": 1629991441655,
        "tool": "ndjson",
        "benches": [
          {
            "name": "borrowed_pattern+opt.wasm.gz",
            "value": 5990,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unicode_bmp_blocks_selector+opt.wasm.gz",
            "value": 5806,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "syntatically_canonicalize_locales+opt.wasm.gz",
            "value": 12924,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_hash_map+opt.wasm.gz",
            "value": 4699,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "filter_langids+opt.wasm.gz",
            "value": 5815,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "derive+opt.wasm.gz",
            "value": 3768,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "zv_serde+opt.wasm.gz",
            "value": 222,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unread_emails+opt.wasm.gz",
            "value": 2919,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_lite_map+opt.wasm.gz",
            "value": 3229,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "work_log+opt.wasm.gz",
            "value": 2893,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "simple+opt.wasm.gz",
            "value": 119489,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "elevator_floors+opt.wasm.gz",
            "value": 2924,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "permyriad+opt.wasm.gz",
            "value": 3561,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "owned_pattern+opt.wasm.gz",
            "value": 7079,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "tui+opt.wasm.gz",
            "value": 2888,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable_message+opt.wasm.gz",
            "value": 3984,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "code_line_diff+opt.wasm.gz",
            "value": 2924,
            "unit": "bytes",
            "biggerIsBetter": false
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
        "date": 1630000988468,
        "tool": "ndjson",
        "benches": [
          {
            "name": "borrowed_pattern+opt.wasm.gz",
            "value": 5990,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unicode_bmp_blocks_selector+opt.wasm.gz",
            "value": 5806,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "syntatically_canonicalize_locales+opt.wasm.gz",
            "value": 12924,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_hash_map+opt.wasm.gz",
            "value": 4699,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "filter_langids+opt.wasm.gz",
            "value": 5815,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "derive+opt.wasm.gz",
            "value": 3768,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "zv_serde+opt.wasm.gz",
            "value": 222,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "unread_emails+opt.wasm.gz",
            "value": 2919,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "language_names_lite_map+opt.wasm.gz",
            "value": 3229,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "work_log+opt.wasm.gz",
            "value": 2893,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "simple+opt.wasm.gz",
            "value": 119489,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "elevator_floors+opt.wasm.gz",
            "value": 2924,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "permyriad+opt.wasm.gz",
            "value": 3561,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "owned_pattern+opt.wasm.gz",
            "value": 7079,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "tui+opt.wasm.gz",
            "value": 2888,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable_message+opt.wasm.gz",
            "value": 3984,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "code_line_diff+opt.wasm.gz",
            "value": 2924,
            "unit": "bytes",
            "biggerIsBetter": false
          }
        ]
      }
    ]
  }
}