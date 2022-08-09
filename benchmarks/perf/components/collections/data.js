window.BENCHMARK_DATA = {
  "lastUpdate": 1660086805715,
  "repoUrl": "https://github.com/unicode-org/icu4x",
  "entries": {
    "Rust Benchmark": [
      {
        "commit": {
          "author": {
            "email": "elango@unicode.org",
            "name": "Elango",
            "username": "echeran"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "937911cf374cb2b8cc1b1a73cbe5b1a356aa409f",
          "message": "Move icu_uniset into collections component as codepointinvlist module (#2328)\n\n* Initial move of icu_uniset into collections\r\n\r\n* Update other Cargo.toml files\r\n\r\n* Update fully qualified paths in code in use statements, tests, etc; rerun datagen, diplomat-gen\r\n\r\n* Rename 'uniset' module name to 'codepointinvlist', rerun testdata and diplomat-gen\r\n\r\n* Re-enable benchmarks, update CI config and README dashboard links\r\n\r\n* Re-enable examples from icu_uniset using full path field in Cargo.toml",
          "timestamp": "2022-08-03T22:26:43Z",
          "tree_id": "9989f423fec3a91cbd834eb125833c54f358231d",
          "url": "https://github.com/unicode-org/icu4x/commit/937911cf374cb2b8cc1b1a73cbe5b1a356aa409f"
        },
        "date": 1659566331359,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 883,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 635,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 697,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 806,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1173,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1283,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 652,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 724,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 817,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 754,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1297,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 50482785,
            "range": "± 1691087",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/best",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/worst",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/best",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/worst",
            "value": 3,
            "range": "± 0",
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
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "77e5f339eeb9988971a92eb55786e621d7eeef0f",
          "message": "Fix up blob provider constructors (WIP) (#2330)\n\n* Fix up blob provider constructors (WIP)\r\n\r\n* Update new_from_... to try_new_from... in provider\r\n\r\nCo-authored-by: Craig Cornelius <ccornelius@google.com>",
          "timestamp": "2022-08-03T23:30:13Z",
          "tree_id": "a951ec443d5f9d7be558b8fda3af8640553f74e1",
          "url": "https://github.com/unicode-org/icu4x/commit/77e5f339eeb9988971a92eb55786e621d7eeef0f"
        },
        "date": 1659570101636,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 705,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 499,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 514,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 573,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 874,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 941,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 499,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 514,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 573,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 617,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 941,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 54973824,
            "range": "± 780919",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/best",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/worst",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/best",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/worst",
            "value": 3,
            "range": "± 0",
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
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5a2560fe59a15dd284fe8a3fc5abc7e219bfdfcb",
          "message": "Remove HourCycle from the public Lengths API (#2331)",
          "timestamp": "2022-08-03T22:39:04-07:00",
          "tree_id": "ce406a0224313b6a6b632c93c303eaf4cbdb45a1",
          "url": "https://github.com/unicode-org/icu4x/commit/5a2560fe59a15dd284fe8a3fc5abc7e219bfdfcb"
        },
        "date": 1659592237658,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 703,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 499,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 515,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 573,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 871,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 941,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 499,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 514,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 580,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 617,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 941,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 54973838,
            "range": "± 748216",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/best",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/worst",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/best",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/worst",
            "value": 3,
            "range": "± 0",
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
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "2f95f22e804b3d75f7dddeb6e7d9f7fa3fa2f470",
          "message": "icu_datetime constructor polish (#2332)\n\n* TimeZoneFormatter constructors\r\n\r\n* make TimeZoneDataPayloads private\r\n\r\n* DateTimeFormatOptions by value\r\n\r\n* add Any/Buffer across [Typed][Date][Time]Formatter\r\n\r\n* Call site cleanup and fmt\r\n\r\n* generate-readmes\r\n\r\n* cleanups",
          "timestamp": "2022-08-04T05:21:22-07:00",
          "tree_id": "5dbc914b1a8a3b72b452e455ac24d5e060701cd5",
          "url": "https://github.com/unicode-org/icu4x/commit/2f95f22e804b3d75f7dddeb6e7d9f7fa3fa2f470"
        },
        "date": 1659616418524,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 940,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 640,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 709,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 812,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1224,
            "range": "± 52",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1282,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 657,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 718,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 817,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 790,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1302,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 52916690,
            "range": "± 1995825",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/best",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/worst",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/best",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/worst",
            "value": 3,
            "range": "± 0",
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
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "4ae59e13339589daa7c253192f679fe5885a2ca1",
          "message": "icu_normalizer constructor polish (#2333)",
          "timestamp": "2022-08-04T08:26:10-07:00",
          "tree_id": "225cdaebc04792f10dfaf934c31fb692da686f2b",
          "url": "https://github.com/unicode-org/icu4x/commit/4ae59e13339589daa7c253192f679fe5885a2ca1"
        },
        "date": 1659627527601,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 858,
            "range": "± 61",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 680,
            "range": "± 61",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 721,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 810,
            "range": "± 80",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1111,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1172,
            "range": "± 78",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 643,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 712,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 827,
            "range": "± 70",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 826,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1213,
            "range": "± 93",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 41137391,
            "range": "± 2041037",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/best",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/worst",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/best",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/worst",
            "value": 2,
            "range": "± 0",
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
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "8e6289931a594401ca9ce0422d4386190ff5b98a",
          "message": "ListFormatter and LocaleFallbacker constructor fixups (#2334)",
          "timestamp": "2022-08-04T09:21:16-07:00",
          "tree_id": "fca7d81c9a1cc6d4a926be689e3e20e2d5a31ad1",
          "url": "https://github.com/unicode-org/icu4x/commit/8e6289931a594401ca9ce0422d4386190ff5b98a"
        },
        "date": 1659630830696,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 792,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 549,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 622,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 692,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1018,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1088,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 550,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 621,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 691,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 669,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1088,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 43470794,
            "range": "± 719662",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/best",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/worst",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/best",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/worst",
            "value": 2,
            "range": "± 0",
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
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "1b72ec61eae0e7ca437b676ac06340dae06f7bc5",
          "message": "Make private Normalizer constructor private again (#2335)",
          "timestamp": "2022-08-04T14:43:14-05:00",
          "tree_id": "f47aa8be835a0f5ef845529f2e4d786fd0596bdf",
          "url": "https://github.com/unicode-org/icu4x/commit/1b72ec61eae0e7ca437b676ac06340dae06f7bc5"
        },
        "date": 1659642884953,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 704,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 499,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 514,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 573,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 873,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 942,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 499,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 514,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 573,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 629,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 941,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 55184171,
            "range": "± 883012",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/best",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/worst",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/best",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/worst",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "elango@unicode.org",
            "name": "Elango",
            "username": "echeran"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5f6ce805d5a6099ef1b61af2a7f3cf62ef02ce50",
          "message": "Move experimental/char16trie to component/collections (#2336)",
          "timestamp": "2022-08-04T14:29:04-07:00",
          "tree_id": "6486181e2b64d8356935915aa019ca292621d15d",
          "url": "https://github.com/unicode-org/icu4x/commit/5f6ce805d5a6099ef1b61af2a7f3cf62ef02ce50"
        },
        "date": 1659649239730,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 696,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 498,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 513,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 573,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 881,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 941,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 498,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 515,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 573,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 619,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 951,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 52434176,
            "range": "± 1078073",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/best",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/worst",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/best",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/worst",
            "value": 3,
            "range": "± 0",
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
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "94e1ac588ecb256fee269679bd90e1bdc623bfe7",
          "message": "Assorted public API tweaks (#2339)\n\n* Make icu_decimal::error private\n\n* Make EnumeratedProperty private\n\n* try_set_metazone -> maybe_set_metazone\n\n* impl AnyProvider for DatagenProvider",
          "timestamp": "2022-08-04T15:19:09-07:00",
          "tree_id": "97ab27c7d650834b9fc64785a6a8ab2622dd8099",
          "url": "https://github.com/unicode-org/icu4x/commit/94e1ac588ecb256fee269679bd90e1bdc623bfe7"
        },
        "date": 1659652305477,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 860,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 694,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 752,
            "range": "± 59",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 872,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1166,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1227,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 676,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 750,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 892,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 865,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1308,
            "range": "± 72",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 42078447,
            "range": "± 1706021",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/best",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/worst",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/best",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/worst",
            "value": 3,
            "range": "± 0",
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
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "0d7ee289291526f96fa7c5bf1e98a68207ae5749",
          "message": "LocaleCanonicalizer/LocaleExpander refactor (#2338)",
          "timestamp": "2022-08-04T16:08:56-07:00",
          "tree_id": "ee794b7a96df89574ba56ce957a97a9f9e405359",
          "url": "https://github.com/unicode-org/icu4x/commit/0d7ee289291526f96fa7c5bf1e98a68207ae5749"
        },
        "date": 1659655547169,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 914,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 631,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 697,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 807,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1131,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1212,
            "range": "± 60",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 634,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 686,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 810,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 740,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1231,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 46239394,
            "range": "± 1983101",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/best",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/worst",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/best",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/worst",
            "value": 3,
            "range": "± 0",
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
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "507f606ebd4e04e269cb4e4066456d394425b264",
          "message": "Change export of CodePointTrieError (#2341)",
          "timestamp": "2022-08-04T16:35:52-07:00",
          "tree_id": "70436103a10634108d1731444fb8c1a3f63515ec",
          "url": "https://github.com/unicode-org/icu4x/commit/507f606ebd4e04e269cb4e4066456d394425b264"
        },
        "date": 1659656929604,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 907,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 602,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 685,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 772,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1088,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1132,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 577,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 652,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 747,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 684,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1105,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 42920479,
            "range": "± 1705219",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/best",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/worst",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/best",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/worst",
            "value": 3,
            "range": "± 0",
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
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f6585f66493ab21059261534475613a4986b77d7",
          "message": "Optimize TimeVariant (#2340)\n\n* Make TimeVariant 2 bytes\r\n\r\n* Fix test failure",
          "timestamp": "2022-08-04T23:41:04Z",
          "tree_id": "ef525bfd4b24e4ef15ab94467a3352af8dae4919",
          "url": "https://github.com/unicode-org/icu4x/commit/f6585f66493ab21059261534475613a4986b77d7"
        },
        "date": 1659657301308,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 776,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 550,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 613,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 701,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 983,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1061,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 550,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 613,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 701,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 648,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1060,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 40039645,
            "range": "± 106446",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/best",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/worst",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/best",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/worst",
            "value": 2,
            "range": "± 0",
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
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "20c66b705b5f0e76e51419147a2a262fd1aa792d",
          "message": "Update utils versions (#2343)",
          "timestamp": "2022-08-04T16:55:38-07:00",
          "tree_id": "97ff4e3b0b29ec81ac41e56f17deaf0a6972bb65",
          "url": "https://github.com/unicode-org/icu4x/commit/20c66b705b5f0e76e51419147a2a262fd1aa792d"
        },
        "date": 1659658135610,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 865,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 585,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 633,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 688,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 987,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1071,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 516,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 633,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 688,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 660,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1215,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 38369512,
            "range": "± 142622",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/best",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/worst",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/best",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/worst",
            "value": 2,
            "range": "± 0",
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
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "df53387e3806e490a88e8fdb4ffd7590eda014b3",
          "message": "Update components and provider versions (#2345)",
          "timestamp": "2022-08-04T17:39:32-07:00",
          "tree_id": "40b6c36271a23ee554888eec034c24f81db8db07",
          "url": "https://github.com/unicode-org/icu4x/commit/df53387e3806e490a88e8fdb4ffd7590eda014b3"
        },
        "date": 1659660729891,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 975,
            "range": "± 65",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 791,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 920,
            "range": "± 56",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 947,
            "range": "± 64",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1226,
            "range": "± 54",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1336,
            "range": "± 71",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 812,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 919,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 965,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 964,
            "range": "± 53",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1377,
            "range": "± 55",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 42385539,
            "range": "± 1948100",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/best",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/worst",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/best",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/worst",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "57224050+QnnOkabayashi@users.noreply.github.com",
            "name": "Quinn",
            "username": "QnnOkabayashi"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "e4a82942ba55612f83458c88a025996430892c82",
          "message": "Wasm demo improved (#2337)\n\n* moved bindings to `ffi/diplomat/wasm/icu4x`\r\n\r\n* demo app\r\n\r\n* actions install cargo-make\r\n\r\n* ci install rust-src nightly\r\n\r\n* actions fix wasm-demo path\r\n\r\n* update cors, cp index.html to gh-pages\r\n\r\n* temp fix don't do cors\r\n\r\n* only delete wasm-demo if it exists\r\n\r\n* think its fixed now\r\n\r\n* tmp point to quinn fork\r\n\r\n* typo\r\n\r\n* another typo\r\n\r\n* don't remove dir?\r\n\r\n* don't copy index.html?\r\n\r\n* test set cors\r\n\r\n* try copying from main\r\n\r\n* try copying from main\r\n\r\n* pull from quinn fork again\r\n\r\n* change name to `wasm-demo-app`\r\n\r\n* store index.html in /tmp/\r\n\r\n* commit to unicode-org gh-pages?\r\n\r\n* don't specify repo?\r\n\r\n* fix bad rebase\r\n\r\n* put html on gcs\r\n\r\n* print url correctly\r\n\r\n* fix ci\r\n\r\n* explain commented code, nicer link to artifacts\r\n\r\n* Demo preview in artifacts-info\r\n\r\n* `ICU4XLocale` returns result instead of option\r\n\r\n* build with big provider\r\n\r\n* fdf and dtf fancy demos\r\n\r\n* segmenter demo\r\n\r\n* regen c/c++ bindings\r\n\r\n* fix c/c++ tests\r\n\r\n* fix\r\n\r\n* Fix broken HourCycle reference\r\n\r\nCo-authored-by: Manish Goregaokar <manishsmail@gmail.com>\r\nCo-authored-by: Shane F. Carr <shane@unicode.org>",
          "timestamp": "2022-08-05T05:10:43Z",
          "tree_id": "e400d06e43b4a6c20fb0903be7fc2cfada6bc42f",
          "url": "https://github.com/unicode-org/icu4x/commit/e4a82942ba55612f83458c88a025996430892c82"
        },
        "date": 1659676980844,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 825,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 586,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 721,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 757,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1084,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1177,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 586,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 725,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 755,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 688,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1180,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 40191186,
            "range": "± 101524",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/best",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/worst",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/best",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/worst",
            "value": 2,
            "range": "± 0",
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
          "id": "0afcfbca54c8f00b268b919d4f2a3258a00ca518",
          "message": "Cargo.toml fixups for ICU4X 1.0.0-beta1",
          "timestamp": "2022-08-05T22:29:21-07:00",
          "tree_id": "1099c3826bef02a39415cc751cedd5f3f7545fd5",
          "url": "https://github.com/unicode-org/icu4x/commit/0afcfbca54c8f00b268b919d4f2a3258a00ca518"
        },
        "date": 1659764589851,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 885,
            "range": "± 65",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 625,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 787,
            "range": "± 64",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 773,
            "range": "± 54",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1007,
            "range": "± 66",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1100,
            "range": "± 70",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 625,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 721,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 753,
            "range": "± 63",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 765,
            "range": "± 66",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1041,
            "range": "± 48",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 39163475,
            "range": "± 2002157",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/best",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/worst",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/best",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/worst",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "90648681+snktd@users.noreply.github.com",
            "name": "snktd",
            "username": "snktd"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "11dc2bbfc4bcbf2b7ebccf39f019af753b66be0c",
          "message": "Make RcWrap to have a type parameter. (#2270)\n\n* Make RcWrap to have a type parameter.\r\n\r\n* Fixing a minor error.\r\n\r\n* Addressing review comments.\r\n\r\n* Minor fmt fix.\r\n\r\n* Fixing minor issues and merging upstream.\r\n\r\n* Fixing comment.\r\n\r\n* Adding a From<T> constructor",
          "timestamp": "2022-08-08T19:52:09Z",
          "tree_id": "72e0475a68c183303b3513bbdaafbee75b6d329a",
          "url": "https://github.com/unicode-org/icu4x/commit/11dc2bbfc4bcbf2b7ebccf39f019af753b66be0c"
        },
        "date": 1659989260298,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 997,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 735,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 881,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 894,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1292,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1352,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 729,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 871,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 908,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 860,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1381,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 50848633,
            "range": "± 1014508",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/best",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/worst",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/best",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/worst",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "elango@unicode.org",
            "name": "Elango",
            "username": "echeran"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "692972bb70c4fc288fef8f5fb944202d93a1163d",
          "message": "Update CI error msg that instructs fixing diplomat-gen data mismatch (#2354)",
          "timestamp": "2022-08-08T20:54:24Z",
          "tree_id": "baad60fd8c3ab6f6288ce7190914a9d33b911310",
          "url": "https://github.com/unicode-org/icu4x/commit/692972bb70c4fc288fef8f5fb944202d93a1163d"
        },
        "date": 1659992754854,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 732,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 602,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 637,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 619,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 942,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1010,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 602,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 631,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 619,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 674,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1010,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 54238423,
            "range": "± 788781",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/best",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/worst",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/best",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/worst",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "martin@geisler.net",
            "name": "Martin Geisler",
            "username": "mgeisler"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "439de7304052363e232996e0f53c8e6de501730a",
          "message": "Fix typo: \"crate\" -> \"create\" (#2349)\n\nFixes a common typo after prolonged exposure to the Rust ecosystem :-)",
          "timestamp": "2022-08-08T21:47:40-05:00",
          "tree_id": "72e1aacecc5f8b798300a05014a4dae5dd45a790",
          "url": "https://github.com/unicode-org/icu4x/commit/439de7304052363e232996e0f53c8e6de501730a"
        },
        "date": 1660014002930,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 733,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 602,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 587,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 618,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 944,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1027,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 603,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 587,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 618,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 674,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1010,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 54210152,
            "range": "± 760719",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/best",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/worst",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/best",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/worst",
            "value": 3,
            "range": "± 0",
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
          "id": "783ec9e4bbe0d28db4c832760fc9bdf58e569691",
          "message": "Add basic AnyDTF FFI (#2344)\n\n* Add calendar construction bindings\r\n\r\n* wip\r\n\r\n* regen\r\n\r\n* add test\r\n\r\n* rebase fix\r\n\r\n* update datetime demo\r\n\r\n* Default datetime demo input to now\r\n\r\n* Wire up newer errors\r\n\r\n* regen\r\n\r\n* git add regenned files\r\n\r\n* autoset to other\r\n\r\n* Fix usize imports\r\n\r\n* Assorted demo fixes and cleanups\r\n\r\n* fix fdf locale not giving error sometimes\r\nremove import scss from app.ts\r\n\r\n* remove unused dep\r\n\r\n* regen\r\n\r\n* update diplomat\r\n\r\n* regen\r\n\r\n* fixup docs\r\n\r\n* Add set_ns()\r\n\r\n* regen\r\n\r\n* fix select element input\r\n\r\nCo-authored-by: Shane F. Carr <shane@unicode.org>\r\nCo-authored-by: Quinn Okabayashi <qokabay1@swarthmore.edu>",
          "timestamp": "2022-08-09T21:36:32Z",
          "tree_id": "8f14bea429ec6dc139495d23eed9d5ed8729dfea",
          "url": "https://github.com/unicode-org/icu4x/commit/783ec9e4bbe0d28db4c832760fc9bdf58e569691"
        },
        "date": 1660081751071,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 845,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 616,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 741,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 754,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1083,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1150,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 615,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 738,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 756,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 716,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1151,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 43611775,
            "range": "± 342866",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/best",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/worst",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/best",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/worst",
            "value": 2,
            "range": "± 0",
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
          "id": "67d10fcf6bc90b63cb696bdaecbe6311c1810b5c",
          "message": "Grey out time styles (#2358)",
          "timestamp": "2022-08-09T22:59:30Z",
          "tree_id": "9269b7b71eacba5641d1f99af211d2cb16fb7f9f",
          "url": "https://github.com/unicode-org/icu4x/commit/67d10fcf6bc90b63cb696bdaecbe6311c1810b5c"
        },
        "date": 1660086764032,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 845,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 616,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 742,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 770,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1083,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1151,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 617,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 741,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 759,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 716,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1151,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 43534471,
            "range": "± 246663",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/best",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/worst",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/best",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/worst",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}