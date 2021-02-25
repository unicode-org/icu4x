window.BENCHMARK_DATA = {
  "lastUpdate": 1614296625116,
  "repoUrl": "https://github.com/unicode-org/icu4x",
  "entries": {
    "Heap – macos-latest": [
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
          "id": "71deed46c6e36a6283ed83e6af3c01cda7f642de",
          "message": "Add memory benchmarks (#446)\n\n* Add dhat as a dev dependency to instrumented components\r\n\r\n* Update .gitignore for the newly generated files\r\n\r\n/benchmarks store all of the benchmark data.\r\n\r\n* Add a tool to inject dhat instrumentation into a file\r\n\r\nThis is a partial commit of the new tool.\r\n\r\nThe following commit completes the tool by running an example with the\r\ninjected code. dhat-rs must be manually instrumented in rust, as opposed\r\nto other memory tools. The benefit with using this approach is that it\r\nworks on macOS, Windows, and Linux, while Valgrind only works on Linux.\r\n\r\nThe added code takes the strategy of parsing the Rust AST, and injecting\r\nthe proper AST for the new code. The rust analyzer project has a pattern\r\nmatching search and replace, which was a good candidate, but it wasn't\r\nwell documented, and not great for matching the first expression in the\r\nmain function.\r\n\r\n* Finish memory_bench so that it automatically runs examples\r\n\r\nThis commit completes the memory_bench tool. It can be run locally, but\r\nis intended to run in CI. The next commit will add the CI functionality.\r\n\r\n* Add a CI job to collect memory benchmarks\r\n\r\nThis uses a custom fork of the benchmarking tool to add support for\r\nndjson as a tool. This ndjson uses the same format as the internal json\r\nstructure of the benchmarking tool. This allows for fully customizing\r\nthe output of the data, and what information is collected.\r\n\r\n* Create tools directory\r\n\r\n* Add macro support for the memory instrumentation\r\n\r\n* Remove the code injection, and update the benchmark tool\r\n\r\n* Update the workflows\r\n\r\n* Update the Cargo.lock\r\n\r\n* Make the DateTime bags more explicit (#429)\r\n\r\n* Make the DateTime bags more explicit\r\n\r\nI found myself confused on the usage of the bags and the difference\r\nbetween them and the options provided to the date time. I felt that it\r\nwas better to be explicit in the examples, rather than giving more\r\nterse, but potentially misleading examples.\r\n\r\n* Address feedback on adding into() examples\r\n\r\n* Remove use icu_benchmark_macros statements\r\n\r\n* Add mention of the firefox profiler",
          "timestamp": "2021-02-01T16:12:33-06:00",
          "tree_id": "a17d3bec25466f93a0a646280468b702473aa3c1",
          "url": "https://github.com/unicode-org/icu4x/commit/71deed46c6e36a6283ed83e6af3c01cda7f642de"
        },
        "date": 1612218092053,
        "tool": "ndjson",
        "benches": [
          {
            "name": "icu_datetime/work_log – Total Heap Allocations",
            "value": 16173,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_datetime/work_log – Heap at Global Memory Max",
            "value": 9610,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_datetime/work_log – Heap at End of Program Execution",
            "value": 1112,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/syntatically_canonicalize_locales – Total Heap Allocations",
            "value": 514,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/syntatically_canonicalize_locales – Heap at Global Memory Max",
            "value": 305,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/syntatically_canonicalize_locales – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/filter_langids – Total Heap Allocations",
            "value": 1223,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/filter_langids – Heap at Global Memory Max",
            "value": 702,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/filter_langids – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/unread_emails – Total Heap Allocations",
            "value": 12739,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/unread_emails – Heap at Global Memory Max",
            "value": 8984,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/unread_emails – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/elevator_floors – Total Heap Allocations",
            "value": 13797,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/elevator_floors – Heap at Global Memory Max",
            "value": 9069,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/elevator_floors – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_uniset/unicode_bmp_blocks_selector – Total Heap Allocations",
            "value": 976,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_uniset/unicode_bmp_blocks_selector – Heap at Global Memory Max",
            "value": 388,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_uniset/unicode_bmp_blocks_selector – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "fixed_decimal/permyriad – Total Heap Allocations",
            "value": 7,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "fixed_decimal/permyriad – Heap at Global Memory Max",
            "value": 7,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "fixed_decimal/permyriad – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable/writeable_message – Total Heap Allocations",
            "value": 11,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable/writeable_message – Heap at Global Memory Max",
            "value": 11,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable/writeable_message – Heap at End of Program Execution",
            "value": 0,
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
          "id": "e9416c3fa5100f11a621f9c8bd498a7dd3a986db",
          "message": "Fix clippy warnings (#473)",
          "timestamp": "2021-02-01T22:08:40-08:00",
          "tree_id": "8940f02ba87a19483c0027b6a5904a79a40c7a8d",
          "url": "https://github.com/unicode-org/icu4x/commit/e9416c3fa5100f11a621f9c8bd498a7dd3a986db"
        },
        "date": 1612246593976,
        "tool": "ndjson",
        "benches": [
          {
            "name": "icu_datetime/work_log – Total Heap Allocations",
            "value": 16173,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_datetime/work_log – Heap at Global Memory Max",
            "value": 9610,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_datetime/work_log – Heap at End of Program Execution",
            "value": 1112,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/syntatically_canonicalize_locales – Total Heap Allocations",
            "value": 514,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/syntatically_canonicalize_locales – Heap at Global Memory Max",
            "value": 305,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/syntatically_canonicalize_locales – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/filter_langids – Total Heap Allocations",
            "value": 1223,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/filter_langids – Heap at Global Memory Max",
            "value": 702,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/filter_langids – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/unread_emails – Total Heap Allocations",
            "value": 12739,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/unread_emails – Heap at Global Memory Max",
            "value": 8984,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/unread_emails – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/elevator_floors – Total Heap Allocations",
            "value": 13797,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/elevator_floors – Heap at Global Memory Max",
            "value": 9069,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/elevator_floors – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_uniset/unicode_bmp_blocks_selector – Total Heap Allocations",
            "value": 976,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_uniset/unicode_bmp_blocks_selector – Heap at Global Memory Max",
            "value": 388,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_uniset/unicode_bmp_blocks_selector – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "fixed_decimal/permyriad – Total Heap Allocations",
            "value": 7,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "fixed_decimal/permyriad – Heap at Global Memory Max",
            "value": 7,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "fixed_decimal/permyriad – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable/writeable_message – Total Heap Allocations",
            "value": 11,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable/writeable_message – Heap at Global Memory Max",
            "value": 11,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable/writeable_message – Heap at End of Program Execution",
            "value": 0,
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
          "id": "ae89da479dfbbcfc73fbaa977cd3f4ecfd0e4fac",
          "message": "Allow some clippy lints (#474)",
          "timestamp": "2021-02-02T17:40:53-08:00",
          "tree_id": "b24a02430c995fda5f39e5982049523f7037643c",
          "url": "https://github.com/unicode-org/icu4x/commit/ae89da479dfbbcfc73fbaa977cd3f4ecfd0e4fac"
        },
        "date": 1612316990348,
        "tool": "ndjson",
        "benches": [
          {
            "name": "icu_datetime/work_log – Total Heap Allocations",
            "value": 16173,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_datetime/work_log – Heap at Global Memory Max",
            "value": 9610,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_datetime/work_log – Heap at End of Program Execution",
            "value": 1112,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/syntatically_canonicalize_locales – Total Heap Allocations",
            "value": 514,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/syntatically_canonicalize_locales – Heap at Global Memory Max",
            "value": 305,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/syntatically_canonicalize_locales – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/filter_langids – Total Heap Allocations",
            "value": 1223,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/filter_langids – Heap at Global Memory Max",
            "value": 702,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/filter_langids – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/unread_emails – Total Heap Allocations",
            "value": 12739,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/unread_emails – Heap at Global Memory Max",
            "value": 8984,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/unread_emails – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/elevator_floors – Total Heap Allocations",
            "value": 13797,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/elevator_floors – Heap at Global Memory Max",
            "value": 9069,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/elevator_floors – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_uniset/unicode_bmp_blocks_selector – Total Heap Allocations",
            "value": 976,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_uniset/unicode_bmp_blocks_selector – Heap at Global Memory Max",
            "value": 388,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_uniset/unicode_bmp_blocks_selector – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "fixed_decimal/permyriad – Total Heap Allocations",
            "value": 7,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "fixed_decimal/permyriad – Heap at Global Memory Max",
            "value": 7,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "fixed_decimal/permyriad – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable/writeable_message – Total Heap Allocations",
            "value": 11,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable/writeable_message – Heap at Global Memory Max",
            "value": 11,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable/writeable_message – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
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
          "id": "85c3a5a933a3a5b1dd2499f916511b80164a8fc1",
          "message": "Stub out a components test, and use serde serialization for DateTime options (#461)\n\n* Stub out tests and serialization for components::Bag\r\n\r\n* Add serialization for the style::Bag\r\n\r\n* Fix preference bag attribute to use the conditional feature\r\n\r\n* Fix the attributes to properly treat serde as optional\r\n\r\n* Address feedback on freely deriving certain traits",
          "timestamp": "2021-02-08T11:59:06-06:00",
          "tree_id": "aa4dfdf479054ceece40e1ef09ddb1b5137c46a7",
          "url": "https://github.com/unicode-org/icu4x/commit/85c3a5a933a3a5b1dd2499f916511b80164a8fc1"
        },
        "date": 1612807751213,
        "tool": "ndjson",
        "benches": [
          {
            "name": "icu_datetime/work_log – Total Heap Allocations",
            "value": 16173,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_datetime/work_log – Heap at Global Memory Max",
            "value": 9610,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_datetime/work_log – Heap at End of Program Execution",
            "value": 1112,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/syntatically_canonicalize_locales – Total Heap Allocations",
            "value": 514,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/syntatically_canonicalize_locales – Heap at Global Memory Max",
            "value": 305,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/syntatically_canonicalize_locales – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/filter_langids – Total Heap Allocations",
            "value": 1223,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/filter_langids – Heap at Global Memory Max",
            "value": 702,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/filter_langids – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/unread_emails – Total Heap Allocations",
            "value": 12739,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/unread_emails – Heap at Global Memory Max",
            "value": 8984,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/unread_emails – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/elevator_floors – Total Heap Allocations",
            "value": 13797,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/elevator_floors – Heap at Global Memory Max",
            "value": 9069,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/elevator_floors – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_uniset/unicode_bmp_blocks_selector – Total Heap Allocations",
            "value": 976,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_uniset/unicode_bmp_blocks_selector – Heap at Global Memory Max",
            "value": 388,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_uniset/unicode_bmp_blocks_selector – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "fixed_decimal/permyriad – Total Heap Allocations",
            "value": 7,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "fixed_decimal/permyriad – Heap at Global Memory Max",
            "value": 7,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "fixed_decimal/permyriad – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable/writeable_message – Total Heap Allocations",
            "value": 11,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable/writeable_message – Heap at Global Memory Max",
            "value": 11,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable/writeable_message – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
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
          "id": "01d85cc1bd793aecbd189a1826d22ed3ddaba771",
          "message": "Fix deploying memory benchmarks to GitHub pages (#472)",
          "timestamp": "2021-02-08T13:44:52-06:00",
          "tree_id": "2c56364a53a4a7ae440c17ab874dea143b804d24",
          "url": "https://github.com/unicode-org/icu4x/commit/01d85cc1bd793aecbd189a1826d22ed3ddaba771"
        },
        "date": 1612814174020,
        "tool": "ndjson",
        "benches": [
          {
            "name": "icu_datetime/work_log – Total Heap Allocations",
            "value": 16173,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_datetime/work_log – Heap at Global Memory Max",
            "value": 9610,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_datetime/work_log – Heap at End of Program Execution",
            "value": 1112,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/syntatically_canonicalize_locales – Total Heap Allocations",
            "value": 514,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/syntatically_canonicalize_locales – Heap at Global Memory Max",
            "value": 305,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/syntatically_canonicalize_locales – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/filter_langids – Total Heap Allocations",
            "value": 1223,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/filter_langids – Heap at Global Memory Max",
            "value": 702,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/filter_langids – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/unread_emails – Total Heap Allocations",
            "value": 12739,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/unread_emails – Heap at Global Memory Max",
            "value": 8984,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/unread_emails – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/elevator_floors – Total Heap Allocations",
            "value": 13797,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/elevator_floors – Heap at Global Memory Max",
            "value": 9069,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/elevator_floors – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_uniset/unicode_bmp_blocks_selector – Total Heap Allocations",
            "value": 976,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_uniset/unicode_bmp_blocks_selector – Heap at Global Memory Max",
            "value": 388,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_uniset/unicode_bmp_blocks_selector – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "fixed_decimal/permyriad – Total Heap Allocations",
            "value": 7,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "fixed_decimal/permyriad – Heap at Global Memory Max",
            "value": 7,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "fixed_decimal/permyriad – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable/writeable_message – Total Heap Allocations",
            "value": 11,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable/writeable_message – Heap at Global Memory Max",
            "value": 11,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable/writeable_message – Heap at End of Program Execution",
            "value": 0,
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
          "id": "ad895a860fc83a7617a0ab604d19ac36d1380915",
          "message": "Change DateTimeFormat constructor to take Locale instead of LangID (#475)",
          "timestamp": "2021-02-08T17:19:38-06:00",
          "tree_id": "76c5cf0b9b2525cd88649614be13f54c1b8c849c",
          "url": "https://github.com/unicode-org/icu4x/commit/ad895a860fc83a7617a0ab604d19ac36d1380915"
        },
        "date": 1612826855074,
        "tool": "ndjson",
        "benches": [
          {
            "name": "icu_datetime/work_log – Total Heap Allocations",
            "value": 16173,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_datetime/work_log – Heap at Global Memory Max",
            "value": 9610,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_datetime/work_log – Heap at End of Program Execution",
            "value": 1112,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/syntatically_canonicalize_locales – Total Heap Allocations",
            "value": 514,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/syntatically_canonicalize_locales – Heap at Global Memory Max",
            "value": 305,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/syntatically_canonicalize_locales – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/filter_langids – Total Heap Allocations",
            "value": 1223,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/filter_langids – Heap at Global Memory Max",
            "value": 702,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/filter_langids – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/unread_emails – Total Heap Allocations",
            "value": 12739,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/unread_emails – Heap at Global Memory Max",
            "value": 8984,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/unread_emails – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/elevator_floors – Total Heap Allocations",
            "value": 13797,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/elevator_floors – Heap at Global Memory Max",
            "value": 9069,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/elevator_floors – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_uniset/unicode_bmp_blocks_selector – Total Heap Allocations",
            "value": 976,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_uniset/unicode_bmp_blocks_selector – Heap at Global Memory Max",
            "value": 388,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_uniset/unicode_bmp_blocks_selector – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "fixed_decimal/permyriad – Total Heap Allocations",
            "value": 7,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "fixed_decimal/permyriad – Heap at Global Memory Max",
            "value": 7,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "fixed_decimal/permyriad – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable/writeable_message – Total Heap Allocations",
            "value": 11,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable/writeable_message – Heap at Global Memory Max",
            "value": 11,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable/writeable_message – Heap at End of Program Execution",
            "value": 0,
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
          "id": "efba26f9e066c514a2f1bfce9a8020e7e0a2b653",
          "message": "Optimize likely subtags (#469)\n\nThis breaks up the current monolithic likely subtags data into multiple tables\r\nbased upon the subtags which are used for searching by the maximize algorithm.\r\nBecause of this change, searching can be done using references to the subtags\r\nof the input Locale rather than creating a copy of it.\r\n\r\nThe result of a search always shares data with the search key. This change\r\ntakes advantage of this fact to only store the delta between the search key\r\nand the result.",
          "timestamp": "2021-02-09T14:33:46-05:00",
          "tree_id": "31b1a7acbb99eaaa9981f887f97fca38ff479ed9",
          "url": "https://github.com/unicode-org/icu4x/commit/efba26f9e066c514a2f1bfce9a8020e7e0a2b653"
        },
        "date": 1612899883495,
        "tool": "ndjson",
        "benches": [
          {
            "name": "icu_datetime/work_log – Total Heap Allocations",
            "value": 16173,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_datetime/work_log – Heap at Global Memory Max",
            "value": 9610,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_datetime/work_log – Heap at End of Program Execution",
            "value": 1112,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/syntatically_canonicalize_locales – Total Heap Allocations",
            "value": 514,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/syntatically_canonicalize_locales – Heap at Global Memory Max",
            "value": 305,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/syntatically_canonicalize_locales – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/filter_langids – Total Heap Allocations",
            "value": 1223,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/filter_langids – Heap at Global Memory Max",
            "value": 702,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/filter_langids – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/unread_emails – Total Heap Allocations",
            "value": 12739,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/unread_emails – Heap at Global Memory Max",
            "value": 8984,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/unread_emails – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/elevator_floors – Total Heap Allocations",
            "value": 13797,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/elevator_floors – Heap at Global Memory Max",
            "value": 9069,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/elevator_floors – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_uniset/unicode_bmp_blocks_selector – Total Heap Allocations",
            "value": 976,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_uniset/unicode_bmp_blocks_selector – Heap at Global Memory Max",
            "value": 388,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_uniset/unicode_bmp_blocks_selector – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "fixed_decimal/permyriad – Total Heap Allocations",
            "value": 7,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "fixed_decimal/permyriad – Heap at Global Memory Max",
            "value": 7,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "fixed_decimal/permyriad – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable/writeable_message – Total Heap Allocations",
            "value": 11,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable/writeable_message – Heap at Global Memory Max",
            "value": 11,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable/writeable_message – Heap at End of Program Execution",
            "value": 0,
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
          "id": "324d9edd2def40e0d000646748172cce370d92db",
          "message": "Re-write DateTimeType trait with DateTimeInput (#445)",
          "timestamp": "2021-02-12T11:40:51-06:00",
          "tree_id": "5882c86bb80a06b39e3b924ddded92a7d6d33b64",
          "url": "https://github.com/unicode-org/icu4x/commit/324d9edd2def40e0d000646748172cce370d92db"
        },
        "date": 1613152172572,
        "tool": "ndjson",
        "benches": [
          {
            "name": "icu_datetime/work_log – Total Heap Allocations",
            "value": 16173,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_datetime/work_log – Heap at Global Memory Max",
            "value": 9610,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_datetime/work_log – Heap at End of Program Execution",
            "value": 1112,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/syntatically_canonicalize_locales – Total Heap Allocations",
            "value": 514,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/syntatically_canonicalize_locales – Heap at Global Memory Max",
            "value": 305,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/syntatically_canonicalize_locales – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/filter_langids – Total Heap Allocations",
            "value": 1223,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/filter_langids – Heap at Global Memory Max",
            "value": 702,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/filter_langids – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/unread_emails – Total Heap Allocations",
            "value": 12739,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/unread_emails – Heap at Global Memory Max",
            "value": 8984,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/unread_emails – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/elevator_floors – Total Heap Allocations",
            "value": 13797,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/elevator_floors – Heap at Global Memory Max",
            "value": 9069,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/elevator_floors – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_uniset/unicode_bmp_blocks_selector – Total Heap Allocations",
            "value": 976,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_uniset/unicode_bmp_blocks_selector – Heap at Global Memory Max",
            "value": 388,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_uniset/unicode_bmp_blocks_selector – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "fixed_decimal/permyriad – Total Heap Allocations",
            "value": 7,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "fixed_decimal/permyriad – Heap at Global Memory Max",
            "value": 7,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "fixed_decimal/permyriad – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable/writeable_message – Total Heap Allocations",
            "value": 11,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable/writeable_message – Heap at Global Memory Max",
            "value": 11,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable/writeable_message – Heap at End of Program Execution",
            "value": 0,
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
          "id": "3be597cdf89432e493e7777bb4dec54854a47f97",
          "message": "Update string type recommendations in style_guide.md (#490)",
          "timestamp": "2021-02-12T14:07:12-06:00",
          "tree_id": "3516e2db54f25c315b8e63f51df7cb8fc3f4de38",
          "url": "https://github.com/unicode-org/icu4x/commit/3be597cdf89432e493e7777bb4dec54854a47f97"
        },
        "date": 1613161134706,
        "tool": "ndjson",
        "benches": [
          {
            "name": "icu_datetime/work_log – Total Heap Allocations",
            "value": 16173,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_datetime/work_log – Heap at Global Memory Max",
            "value": 9610,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_datetime/work_log – Heap at End of Program Execution",
            "value": 1112,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/syntatically_canonicalize_locales – Total Heap Allocations",
            "value": 514,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/syntatically_canonicalize_locales – Heap at Global Memory Max",
            "value": 305,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/syntatically_canonicalize_locales – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/filter_langids – Total Heap Allocations",
            "value": 1223,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/filter_langids – Heap at Global Memory Max",
            "value": 702,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/filter_langids – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/unread_emails – Total Heap Allocations",
            "value": 12739,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/unread_emails – Heap at Global Memory Max",
            "value": 8984,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/unread_emails – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/elevator_floors – Total Heap Allocations",
            "value": 13797,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/elevator_floors – Heap at Global Memory Max",
            "value": 9069,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/elevator_floors – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_uniset/unicode_bmp_blocks_selector – Total Heap Allocations",
            "value": 976,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_uniset/unicode_bmp_blocks_selector – Heap at Global Memory Max",
            "value": 388,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_uniset/unicode_bmp_blocks_selector – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "fixed_decimal/permyriad – Total Heap Allocations",
            "value": 7,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "fixed_decimal/permyriad – Heap at Global Memory Max",
            "value": 7,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "fixed_decimal/permyriad – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable/writeable_message – Total Heap Allocations",
            "value": 11,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable/writeable_message – Heap at Global Memory Max",
            "value": 11,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable/writeable_message – Heap at End of Program Execution",
            "value": 0,
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
          "id": "fdd3a0b4773388f2df8cdc3c6cb5bf4230ac79cf",
          "message": "Add litemap crate (TupleVecMap) (#496)\n\n* Add terrain crate\r\n\r\n* Add doctests\r\n\r\n* Add license header, Cargo manifest keys\r\n\r\n* More review fixes\r\n\r\n* Run rustfmt\r\n\r\n* Add simple readme\r\n\r\n* Rename to litemap\r\n\r\n* VecMap -> LiteMap\r\n\r\n* Add K: Borrow<Q>",
          "timestamp": "2021-02-22T16:59:24-08:00",
          "tree_id": "31fa9eb6d25b32050bf2d6b629f45179818d8261",
          "url": "https://github.com/unicode-org/icu4x/commit/fdd3a0b4773388f2df8cdc3c6cb5bf4230ac79cf"
        },
        "date": 1614042544489,
        "tool": "ndjson",
        "benches": [
          {
            "name": "icu_datetime/work_log – Total Heap Allocations",
            "value": 16173,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_datetime/work_log – Heap at Global Memory Max",
            "value": 9610,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_datetime/work_log – Heap at End of Program Execution",
            "value": 1112,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/syntatically_canonicalize_locales – Total Heap Allocations",
            "value": 514,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/syntatically_canonicalize_locales – Heap at Global Memory Max",
            "value": 305,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/syntatically_canonicalize_locales – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/filter_langids – Total Heap Allocations",
            "value": 1223,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/filter_langids – Heap at Global Memory Max",
            "value": 702,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/filter_langids – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/unread_emails – Total Heap Allocations",
            "value": 12739,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/unread_emails – Heap at Global Memory Max",
            "value": 8984,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/unread_emails – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/elevator_floors – Total Heap Allocations",
            "value": 13797,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/elevator_floors – Heap at Global Memory Max",
            "value": 9069,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/elevator_floors – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_uniset/unicode_bmp_blocks_selector – Total Heap Allocations",
            "value": 976,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_uniset/unicode_bmp_blocks_selector – Heap at Global Memory Max",
            "value": 388,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_uniset/unicode_bmp_blocks_selector – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "fixed_decimal/permyriad – Total Heap Allocations",
            "value": 7,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "fixed_decimal/permyriad – Heap at Global Memory Max",
            "value": 7,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "fixed_decimal/permyriad – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable/writeable_message – Total Heap Allocations",
            "value": 11,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable/writeable_message – Heap at Global Memory Max",
            "value": 11,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable/writeable_message – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
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
          "id": "d1e746357f79f9fcf2a63e50db15f365ab9b063b",
          "message": "Switch locid::subtags::Language to be TinyStr4 (#506)\n\n* Switch locid::subtags::Language to be TinyStr4\r\n\r\n* Apply reviewers feedback\r\n\r\n* Fix cargo fmt linter error",
          "timestamp": "2021-02-25T15:34:57-08:00",
          "tree_id": "c40fb8c842f2cfe18053c1f7f1773a77d88017f1",
          "url": "https://github.com/unicode-org/icu4x/commit/d1e746357f79f9fcf2a63e50db15f365ab9b063b"
        },
        "date": 1614296622342,
        "tool": "ndjson",
        "benches": [
          {
            "name": "icu_datetime/work_log – Total Heap Allocations",
            "value": 16173,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_datetime/work_log – Heap at Global Memory Max",
            "value": 9610,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_datetime/work_log – Heap at End of Program Execution",
            "value": 1112,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/syntatically_canonicalize_locales – Total Heap Allocations",
            "value": 514,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/syntatically_canonicalize_locales – Heap at Global Memory Max",
            "value": 305,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/syntatically_canonicalize_locales – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/filter_langids – Total Heap Allocations",
            "value": 1223,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/filter_langids – Heap at Global Memory Max",
            "value": 702,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_locid/filter_langids – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/unread_emails – Total Heap Allocations",
            "value": 12739,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/unread_emails – Heap at Global Memory Max",
            "value": 8984,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/unread_emails – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/elevator_floors – Total Heap Allocations",
            "value": 13797,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/elevator_floors – Heap at Global Memory Max",
            "value": 9069,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_plurals/elevator_floors – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_uniset/unicode_bmp_blocks_selector – Total Heap Allocations",
            "value": 976,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_uniset/unicode_bmp_blocks_selector – Heap at Global Memory Max",
            "value": 388,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "icu_uniset/unicode_bmp_blocks_selector – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "fixed_decimal/permyriad – Total Heap Allocations",
            "value": 7,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "fixed_decimal/permyriad – Heap at Global Memory Max",
            "value": 7,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "fixed_decimal/permyriad – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable/writeable_message – Total Heap Allocations",
            "value": 11,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable/writeable_message – Heap at Global Memory Max",
            "value": 11,
            "unit": "bytes",
            "biggerIsBetter": false
          },
          {
            "name": "writeable/writeable_message – Heap at End of Program Execution",
            "value": 0,
            "unit": "bytes",
            "biggerIsBetter": false
          }
        ]
      }
    ]
  }
}