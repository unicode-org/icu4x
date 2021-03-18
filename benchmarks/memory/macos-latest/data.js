window.BENCHMARK_DATA = {
  "lastUpdate": 1616101282569,
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
          "id": "df7b6edf44c3360d3ad4127c12ed03c5f039ff6b",
          "message": "Update data_pipeline.md to discuss caching (#497)",
          "timestamp": "2021-02-27T04:39:28-06:00",
          "tree_id": "4d625493d15543672a3f6413db21e02a8d3e0c6e",
          "url": "https://github.com/unicode-org/icu4x/commit/df7b6edf44c3360d3ad4127c12ed03c5f039ff6b"
        },
        "date": 1614422866566,
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
          "id": "3f9e74b767a215c490007361b23f9c45cf2d1b82",
          "message": "Add serde to litemap (#514)",
          "timestamp": "2021-03-01T16:24:22-08:00",
          "tree_id": "dc24eebcd24cdd1bc8fe1b680e0d02c7d509ce7a",
          "url": "https://github.com/unicode-org/icu4x/commit/3f9e74b767a215c490007361b23f9c45cf2d1b82"
        },
        "date": 1614645447094,
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
          "id": "c308b40dbdb5018dd6c3ffec429e2dd002bb4066",
          "message": "Add CLDR JSON data to testdata and remove from provider_cldr (#513)",
          "timestamp": "2021-03-01T21:38:11-06:00",
          "tree_id": "ade81acb58382bed3e53eeaf91f1eb688b93feb7",
          "url": "https://github.com/unicode-org/icu4x/commit/c308b40dbdb5018dd6c3ffec429e2dd002bb4066"
        },
        "date": 1614656788748,
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
            "email": "kpozin@google.com",
            "name": "Konstantin Pozin",
            "username": "kpozin"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "64c143f402a3c2956f4aa172e903e1e159598b6d",
          "message": "Fix date pattern parsing for non-ASCII literals (#503) (#515)\n\nFixes #503",
          "timestamp": "2021-03-01T22:17:01-08:00",
          "tree_id": "b171fbf1030491ae586af21032d1c76eaffe293b",
          "url": "https://github.com/unicode-org/icu4x/commit/64c143f402a3c2956f4aa172e903e1e159598b6d"
        },
        "date": 1614666304528,
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
          "id": "b15c0b36901095406a67e3620a3d1d0e294b7692",
          "message": "Make LanguageIdentifier a field of Locale (#492)\n\nMake LanguageIdentifier a field of Locale.\r\n\r\nFixes #447",
          "timestamp": "2021-03-02T07:03:43-05:00",
          "tree_id": "8c6d75a7dbcdd0015e8419882389c04863145644",
          "url": "https://github.com/unicode-org/icu4x/commit/b15c0b36901095406a67e3620a3d1d0e294b7692"
        },
        "date": 1614687281810,
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
          "id": "40c5dd3205c8fa4beee91a9ac47b18f628a7e550",
          "message": "Update tinytemplate to fix nightly breakage (#529)",
          "timestamp": "2021-03-04T09:33:16-08:00",
          "tree_id": "a408baa9d9867ae1aafe7557813ceaa087254972",
          "url": "https://github.com/unicode-org/icu4x/commit/40c5dd3205c8fa4beee91a9ac47b18f628a7e550"
        },
        "date": 1614879881741,
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
          "id": "9b0fa09152aea4de2f8a37628d0bebb9ebb50f3d",
          "message": "litemap: Serialize to maps, add tests (#516)",
          "timestamp": "2021-03-04T12:08:48-06:00",
          "tree_id": "fac0d611c10d52bfd9390ec23ca617338358fd7e",
          "url": "https://github.com/unicode-org/icu4x/commit/9b0fa09152aea4de2f8a37628d0bebb9ebb50f3d"
        },
        "date": 1614881870393,
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
          "id": "2d622075ee5b17ce15f22c9ef2388ea4df930710",
          "message": "Include data in icu_testdata crate (#527)",
          "timestamp": "2021-03-04T12:22:52-06:00",
          "tree_id": "c613b9f025246813565511cb22eaf08afc4a8bc6",
          "url": "https://github.com/unicode-org/icu4x/commit/2d622075ee5b17ce15f22c9ef2388ea4df930710"
        },
        "date": 1614883823055,
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
          "id": "68d6b2bdcfbf36c7d25848a71009a2159ea3d780",
          "message": "Separate Patterns and Symbols trait methods in DateTimeFormat. (#517)",
          "timestamp": "2021-03-04T10:25:02-08:00",
          "tree_id": "6b5351a233b4d758cc95b5cd352a9bca13818898",
          "url": "https://github.com/unicode-org/icu4x/commit/68d6b2bdcfbf36c7d25848a71009a2159ea3d780"
        },
        "date": 1614883976226,
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
          "id": "ba3ea1deb32bbd6c5d7177d68db4b6f51531baad",
          "message": "Rename master branch to main (#520)",
          "timestamp": "2021-03-04T13:19:38-06:00",
          "tree_id": "07c2754e5e1c5c81c3c8fb1545c77baa5c4d115a",
          "url": "https://github.com/unicode-org/icu4x/commit/ba3ea1deb32bbd6c5d7177d68db4b6f51531baad"
        },
        "date": 1614886323241,
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
          "id": "2179f6fee8764aec332b2d251514ac30970ec021",
          "message": "Add a cargo make command and a CI check for bincode (#521)",
          "timestamp": "2021-03-04T14:42:19-06:00",
          "tree_id": "e3ca34ef3a4d4f167fe1688db94643a4d44d8ed7",
          "url": "https://github.com/unicode-org/icu4x/commit/2179f6fee8764aec332b2d251514ac30970ec021"
        },
        "date": 1614891231400,
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
          "id": "2d6664a376df209f86d4af47f0a686b91b8172e0",
          "message": "Move DTF to only store DateSymbols and a single pattern. (#518)",
          "timestamp": "2021-03-05T07:40:03-08:00",
          "tree_id": "c6e88347fda54bd0e73aba4c94510803800e4025",
          "url": "https://github.com/unicode-org/icu4x/commit/2d6664a376df209f86d4af47f0a686b91b8172e0"
        },
        "date": 1614959428715,
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
          "id": "c5104b3ab7cce22e11a71f9797d1dbaff9b40f2c",
          "message": "Small quality-of-life fixes (#543)",
          "timestamp": "2021-03-12T15:01:43-06:00",
          "tree_id": "01203783bcee98b2dc8dd6e86af26e01f3a2da42",
          "url": "https://github.com/unicode-org/icu4x/commit/c5104b3ab7cce22e11a71f9797d1dbaff9b40f2c"
        },
        "date": 1615583400077,
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
          "id": "a4a8e4a68a1e9c3b3b0517068bd46707f410bf2e",
          "message": "Pin the nightly version in CI (#526)",
          "timestamp": "2021-03-12T15:19:41-06:00",
          "tree_id": "d6e37aaaa114fcc11a47423215a3457177fa66bb",
          "url": "https://github.com/unicode-org/icu4x/commit/a4a8e4a68a1e9c3b3b0517068bd46707f410bf2e"
        },
        "date": 1615584639206,
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
          "id": "ed9db541404a2cc36c0c1f813f0b365a136ac085",
          "message": "Update testing instructions on CI doc (#482)\n\n* Update testing instructions on CI doc\r\n\r\n* Respond to PR feedback\r\n\r\n* Add section on caching to help ICU",
          "timestamp": "2021-03-15T10:23:30-07:00",
          "tree_id": "8641e96858fdad8258a651439f28f8a30b188fa4",
          "url": "https://github.com/unicode-org/icu4x/commit/ed9db541404a2cc36c0c1f813f0b365a136ac085"
        },
        "date": 1615829580504,
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
          "id": "43360de1e53004d9a56d7f29a2573de03dd145fc",
          "message": "Update Rust nightly version in CI (#554)\n\nUpdate to nightly-2021-03-15",
          "timestamp": "2021-03-16T12:19:38-05:00",
          "tree_id": "2dd2ddda3eb75c73dad705596b764977c5352b9c",
          "url": "https://github.com/unicode-org/icu4x/commit/43360de1e53004d9a56d7f29a2573de03dd145fc"
        },
        "date": 1615916488094,
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
          "id": "b6ed6f058a0c3b6566eac78a58f47303bf48830f",
          "message": "Implement FromIterator for LiteMap (#544)\n\n* Implement FromIterator for LiteMap\r\n\r\n* Fix ordering of size-hint match to be more intuitive\r\n\r\n* Update comments\r\n\r\n* Clean up function names\r\n\r\n* Update comments\r\n\r\n* Rename extend_from_sorted to try_extend_from_sorted\r\n\r\n- This better self-documents its fallibility\r\n\r\n* Make new try_extend_from_sorted() match try_append()\r\n\r\n- Rather than returning a result with Self, the function now\r\n  returns `None` on success and the reamaining elements on failure.\r\n- Make the extend functions take self by mut reference.\r\n- Make the extend functions public.\r\n- Add documentation tests.\r\n\r\n* Update documentation tests\r\n\r\n* Update try_append() to fail on equal keys\r\n\r\n* Add #[must_use] to try_append and try_extend_from_sorted\r\n\r\n* Run cargo fmt\r\n\r\n* Update try_append comment\r\n\r\n* Remove extend_from methods in favor using try_append directly\r\n\r\n* Add try_append duplicate key assertion to example\r\n\r\n* Add test for FromIterator\r\n\r\n* Remove redundant \"test\" from function name\r\n\r\n* Fix typo in test\r\n\r\n* Make try_append doc test more explicit",
          "timestamp": "2021-03-16T11:00:03-07:00",
          "tree_id": "f3444d407a7ff607c187d15f4aa9a6d452dd1045",
          "url": "https://github.com/unicode-org/icu4x/commit/b6ed6f058a0c3b6566eac78a58f47303bf48830f"
        },
        "date": 1615918094048,
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
          "id": "4dae1fa1ce36e35ea48d41343890a311dfb7ee49",
          "message": "Re-organize the benchmark data (#546)",
          "timestamp": "2021-03-18T15:49:59-05:00",
          "tree_id": "74d02b07f4f7c8e5a8057a5200b61f4230ef4969",
          "url": "https://github.com/unicode-org/icu4x/commit/4dae1fa1ce36e35ea48d41343890a311dfb7ee49"
        },
        "date": 1616101280773,
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