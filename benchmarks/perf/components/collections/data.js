window.BENCHMARK_DATA = {
  "lastUpdate": 1661273843417,
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
      },
      {
        "commit": {
          "author": {
            "email": "robertbastian@users.noreply.github.com",
            "name": "Robert Bastian",
            "username": "robertbastian"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "005c5ec23e0939d20570a3b2844070d127024fb7",
          "message": "Touching up icu_collections docs (#2346)",
          "timestamp": "2022-08-10T14:45:08+02:00",
          "tree_id": "2ce612b0dd7c727fe01125d7f68e2f8ff75b9a14",
          "url": "https://github.com/unicode-org/icu4x/commit/005c5ec23e0939d20570a3b2844070d127024fb7"
        },
        "date": 1660136284158,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 955,
            "range": "± 56",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 796,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 895,
            "range": "± 51",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 917,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1203,
            "range": "± 76",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1311,
            "range": "± 62",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 766,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 888,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 924,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 889,
            "range": "± 52",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1307,
            "range": "± 57",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 48719696,
            "range": "± 1794576",
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
          "id": "ea35cb4578e5b5c460515392188301c4129f3b20",
          "message": "Fix root Readme example to work (#2353)",
          "timestamp": "2022-08-10T10:10:38-07:00",
          "tree_id": "5bbd3ed498f350305e39fc80581c1cc08bf77a3f",
          "url": "https://github.com/unicode-org/icu4x/commit/ea35cb4578e5b5c460515392188301c4129f3b20"
        },
        "date": 1660152127743,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 845,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 615,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 737,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 754,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1083,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1150,
            "range": "± 0",
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
            "value": 754,
            "range": "± 1",
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
            "value": 1150,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 43585517,
            "range": "± 170649",
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
          "id": "084fca357ef6287b57206fcb2c953caf27045c94",
          "message": "Fix link generated in CI (#2361)",
          "timestamp": "2022-08-10T12:25:37-07:00",
          "tree_id": "005a0cd717388b75d7f46282fdea329a76a8ed28",
          "url": "https://github.com/unicode-org/icu4x/commit/084fca357ef6287b57206fcb2c953caf27045c94"
        },
        "date": 1660160294639,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 1066,
            "range": "± 73",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 765,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 952,
            "range": "± 80",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 906,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1294,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1446,
            "range": "± 410",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 760,
            "range": "± 64",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 889,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 908,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 861,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1423,
            "range": "± 72",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 54833806,
            "range": "± 2686875",
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
            "email": "robertbastian@users.noreply.github.com",
            "name": "Robert Bastian",
            "username": "robertbastian"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "07f945db67a9b142f5708757bab8c65db56db3a4",
          "message": "FFI completeness test (#2275)",
          "timestamp": "2022-08-11T11:03:27-07:00",
          "tree_id": "c2d14fc71cd9fe4d7132eecf9d966823ea30b9c7",
          "url": "https://github.com/unicode-org/icu4x/commit/07f945db67a9b142f5708757bab8c65db56db3a4"
        },
        "date": 1660241715062,
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
            "value": 603,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 588,
            "range": "± 5",
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
            "value": 940,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1028,
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
            "value": 587,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 619,
            "range": "± 3",
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
            "value": 54218408,
            "range": "± 629773",
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
            "email": "robertbastian@users.noreply.github.com",
            "name": "Robert Bastian",
            "username": "robertbastian"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "8a3bb378cb6d73ba74ec94fd406fde6f123a80cc",
          "message": "Singleton testdata providers (#2363)",
          "timestamp": "2022-08-11T20:29:17+02:00",
          "tree_id": "bdb5ce57d435cbbad96b2804eac658d2ee296243",
          "url": "https://github.com/unicode-org/icu4x/commit/8a3bb378cb6d73ba74ec94fd406fde6f123a80cc"
        },
        "date": 1660243309988,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 845,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 615,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 737,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 754,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1083,
            "range": "± 0",
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
            "value": 615,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 737,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 761,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 717,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1150,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 43438755,
            "range": "± 133594",
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
          "id": "d0cb377b5a49274d043d1823f806b0c145657ce1",
          "message": "Comments about ci-job-full-data (#2365)",
          "timestamp": "2022-08-11T19:00:13Z",
          "tree_id": "fd39fdd5f85c889fea38dd5372e09bfccf0d4866",
          "url": "https://github.com/unicode-org/icu4x/commit/d0cb377b5a49274d043d1823f806b0c145657ce1"
        },
        "date": 1660245110261,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 732,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 603,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 589,
            "range": "± 2",
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
            "value": 943,
            "range": "± 4",
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
            "range": "± 12",
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
            "value": 619,
            "range": "± 3",
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
            "value": 54419870,
            "range": "± 712566",
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
          "id": "eca59540880856789c0fe29f4fe165aabbbf4ad4",
          "message": "Delete binary of datetime FFI example (#2367)",
          "timestamp": "2022-08-11T15:04:47-07:00",
          "tree_id": "27741f9c31f378112320b921bf1209d04c2ee429",
          "url": "https://github.com/unicode-org/icu4x/commit/eca59540880856789c0fe29f4fe165aabbbf4ad4"
        },
        "date": 1660256394334,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 844,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 614,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 738,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 754,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1083,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1150,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 615,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 739,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 757,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 717,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1150,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 43611453,
            "range": "± 113365",
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
          "id": "18946bb86f9c0ddbd05e819a66aaab1df2e0d356",
          "message": "Pick default calendar based off of locale in AnyCalendar (#2369)\n\n* Handle defaults in AnyCalendar\r\n\r\n* Fall back to default calendar in DTF\r\n\r\n* docs and test\r\n\r\n* clippy\r\n\r\n* rm TryFrom\r\n\r\n* remove extra methods\r\n\r\n* use macro\r\n\r\n* readd from_locale\r\n\r\n* fix\r\n\r\n* fix",
          "timestamp": "2022-08-12T20:27:03Z",
          "tree_id": "2c5d48f8557c1dcea704b2d4c748bfeed0b6b5fc",
          "url": "https://github.com/unicode-org/icu4x/commit/18946bb86f9c0ddbd05e819a66aaab1df2e0d356"
        },
        "date": 1660336775106,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 735,
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
            "value": 940,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1010,
            "range": "± 1",
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
            "value": 587,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 645,
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
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 54222793,
            "range": "± 726490",
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
          "id": "606d98c9e49f052db01d522b06521db5df7e0027",
          "message": "Logging in FFI (#2368)\n\n* Add features for logging in FFI\r\n\r\n* Add ICU4XLogger\r\n\r\n* regen\r\n\r\n* Add cpp_default feature\r\n\r\n* init logger in all cpp examples\r\n\r\n* Add logging to ICU4XError conversion\r\n\r\n* gen\r\n\r\n* fix features",
          "timestamp": "2022-08-12T23:42:41Z",
          "tree_id": "3b8bcc4ff25be8e9a30e3e141c3dc44a9df1a103",
          "url": "https://github.com/unicode-org/icu4x/commit/606d98c9e49f052db01d522b06521db5df7e0027"
        },
        "date": 1660348490743,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 973,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 710,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 859,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 892,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1254,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1320,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 715,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 839,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 880,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 827,
            "range": "± 56",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1316,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 49619467,
            "range": "± 2031342",
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
            "email": "cad97@cad97.com",
            "name": "Christopher Durham",
            "username": "CAD97"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5f6dd9d0982d4170d44cf58ed9cb0838e785c40b",
          "message": "impl EncodeAsVarULE for Cow (#2376)",
          "timestamp": "2022-08-14T23:35:23-07:00",
          "tree_id": "80165192bb57dd70e4d65cfa452e3a2c2d8fe357",
          "url": "https://github.com/unicode-org/icu4x/commit/5f6dd9d0982d4170d44cf58ed9cb0838e785c40b"
        },
        "date": 1660546067572,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 856,
            "range": "± 56",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 715,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 849,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 836,
            "range": "± 56",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1064,
            "range": "± 63",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1194,
            "range": "± 87",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 685,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 791,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 843,
            "range": "± 57",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 852,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1194,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 44098756,
            "range": "± 2098362",
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
            "email": "35614614+pdogr@users.noreply.github.com",
            "name": "Pawan Dogra",
            "username": "pdogr"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "dff842eb4c68299f5a22ea4a72bf498a5be1f4e2",
          "message": "Enable ShortVec as a backend for LiteMap (#2356)",
          "timestamp": "2022-08-16T11:55:57+02:00",
          "tree_id": "94c73f2449f5deb111929cbcad5dcb1597dcb38f",
          "url": "https://github.com/unicode-org/icu4x/commit/dff842eb4c68299f5a22ea4a72bf498a5be1f4e2"
        },
        "date": 1660644465977,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 732,
            "range": "± 1",
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
            "value": 619,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 939,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1010,
            "range": "± 1",
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
            "value": 620,
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
            "value": 54219317,
            "range": "± 740203",
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
            "email": "robertbastian@users.noreply.github.com",
            "name": "Robert Bastian",
            "username": "robertbastian"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ed6a46de959cea469f05e6ab4154c886f3709fbe",
          "message": "Rename icu::locale_canonicalizer (#2381)",
          "timestamp": "2022-08-16T19:48:43+02:00",
          "tree_id": "dc05fe44e1e18c4725719f64fadf2dcb6a203349",
          "url": "https://github.com/unicode-org/icu4x/commit/ed6a46de959cea469f05e6ab4154c886f3709fbe"
        },
        "date": 1660672914545,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 734,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 600,
            "range": "± 2",
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
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 939,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1010,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 597,
            "range": "± 2",
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
            "value": 619,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 676,
            "range": "± 2",
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
            "value": 54304582,
            "range": "± 751558",
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
            "email": "cad97@cad97.com",
            "name": "Christopher Durham",
            "username": "CAD97"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "47b289e60253050d3933ddd61b851d00317c1caa",
          "message": "Generalize impl EncodeAsVarULE for Cow (#2377)\n\nFixes: 5f6dd9d0982d4170d44cf58ed9cb0838e785c40b",
          "timestamp": "2022-08-16T15:25:54-07:00",
          "tree_id": "cda464b395e5c348f6f098916d57461b116c85dd",
          "url": "https://github.com/unicode-org/icu4x/commit/47b289e60253050d3933ddd61b851d00317c1caa"
        },
        "date": 1660689470066,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 844,
            "range": "± 0",
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
            "value": 738,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 755,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1083,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1150,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 616,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 737,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 755,
            "range": "± 5",
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
            "value": 1150,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 43509914,
            "range": "± 105613",
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
          "id": "a6987d11fc296a69c0d54e2ab7c6ea5381d8478d",
          "message": "Put date skeletons behind a feature (#2370)",
          "timestamp": "2022-08-16T16:32:50-07:00",
          "tree_id": "8343996ca5da3c8c3b199f503d1c6e1b45bc82d2",
          "url": "https://github.com/unicode-org/icu4x/commit/a6987d11fc296a69c0d54e2ab7c6ea5381d8478d"
        },
        "date": 1660693474131,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 815,
            "range": "± 2",
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
            "value": 716,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 749,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1052,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1015,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 516,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 723,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 660,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 608,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1015,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 41468601,
            "range": "± 82416",
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
            "email": "hsivonen@hsivonen.fi",
            "name": "Henri Sivonen",
            "username": "hsivonen"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "14038a5ae761bfb01c702d2903b93161d459f841",
          "message": "Check if the Swedish reformed collation exists (#2387)\n\nForward compatibility with https://unicode-org.atlassian.net/browse/CLDR-15603\r\nSee https://github.com/unicode-org/cldr/commit/aca740fb9c59efa1f1717bee682d98bded5d0428\r\nand https://github.com/unicode-org/cldr/commit/5b1423acc49c6b539e0cfbc69ae38c9cf044b1ca\r\n\r\nCloses #2183",
          "timestamp": "2022-08-17T14:58:02+03:00",
          "tree_id": "d4b9f2031b33a77cae838fc5bcbce9af0e05d7af",
          "url": "https://github.com/unicode-org/icu4x/commit/14038a5ae761bfb01c702d2903b93161d459f841"
        },
        "date": 1660738206244,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 831,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 586,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 725,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 749,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1051,
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
            "value": 586,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 711,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 749,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 689,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1150,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 46848908,
            "range": "± 106384",
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
            "email": "hsivonen@hsivonen.fi",
            "name": "Henri Sivonen",
            "username": "hsivonen"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5a6471da1068173ad9172d0f9e2d15ee5b73063f",
          "message": "Add a remark about inversion list  worst-case performance (#2388)",
          "timestamp": "2022-08-17T14:35:17Z",
          "tree_id": "e6428e5bcb15e31bfcd127c6db1b513ee9518467",
          "url": "https://github.com/unicode-org/icu4x/commit/5a6471da1068173ad9172d0f9e2d15ee5b73063f"
        },
        "date": 1660747655647,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 741,
            "range": "± 346",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 607,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 587,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 616,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 937,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1012,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 607,
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
            "value": 617,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 679,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1012,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 54033683,
            "range": "± 733031",
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
          "id": "aca08bae12b6db27995da89f68a92c9d9f6ce4fc",
          "message": "Don't save the locale in the DateTimeFormatter (#2389)",
          "timestamp": "2022-08-17T14:18:42-07:00",
          "tree_id": "6504341ec68608939668261946f3dcbdcfe7322e",
          "url": "https://github.com/unicode-org/icu4x/commit/aca08bae12b6db27995da89f68a92c9d9f6ce4fc"
        },
        "date": 1660771830643,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 718,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 517,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 642,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 668,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 928,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1015,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 517,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 630,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 660,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 608,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1015,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 41639356,
            "range": "± 65797",
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
          "id": "f66fff7a1107465836615fecb007e1fb31f210b5",
          "message": "Apply missing_docs to icu_provider_adapters (#2392)",
          "timestamp": "2022-08-17T14:55:05-07:00",
          "tree_id": "b1afb6f8d64ebb6bb39c78c1c9d8c3a830b5ae8e",
          "url": "https://github.com/unicode-org/icu4x/commit/f66fff7a1107465836615fecb007e1fb31f210b5"
        },
        "date": 1660774248710,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 927,
            "range": "± 59",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 723,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 827,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 885,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1152,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1232,
            "range": "± 53",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 726,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 831,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 885,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 877,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1252,
            "range": "± 57",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 42057543,
            "range": "± 1816510",
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
          "id": "5e65593a7eb69490c98254b63f7943ccb9605319",
          "message": "Apply missing_docs to icu_provider_blob (#2393)",
          "timestamp": "2022-08-17T16:55:13-07:00",
          "tree_id": "680c9cae5b222f4d730b6b5d8bb7e04be8fa464c",
          "url": "https://github.com/unicode-org/icu4x/commit/5e65593a7eb69490c98254b63f7943ccb9605319"
        },
        "date": 1660781442050,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 814,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 586,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 712,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 749,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1051,
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
            "value": 586,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 717,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 761,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 689,
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
            "value": 46998865,
            "range": "± 88972",
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
          "id": "b48b05e066e538f08b6859a450bac78e1e61f8e0",
          "message": "Update boilerplate.md (#2390)",
          "timestamp": "2022-08-17T16:55:03-07:00",
          "tree_id": "bcfd3c44d49a005e675f746022844e925d59a983",
          "url": "https://github.com/unicode-org/icu4x/commit/b48b05e066e538f08b6859a450bac78e1e61f8e0"
        },
        "date": 1660781625828,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 996,
            "range": "± 88",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 794,
            "range": "± 53",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 943,
            "range": "± 98",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 959,
            "range": "± 235",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1269,
            "range": "± 78",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1382,
            "range": "± 79",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 787,
            "range": "± 51",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 927,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 950,
            "range": "± 64",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 959,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1398,
            "range": "± 95",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 45715500,
            "range": "± 2641788",
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
          "id": "cac7266c7c72386f747eb647fa9769d78d7f41d0",
          "message": "Apply missing_docs to icu_provider_fs (#2395)",
          "timestamp": "2022-08-17T16:58:20-07:00",
          "tree_id": "2fcde2e8b162cde51fd17186f427ea2d2128c0dd",
          "url": "https://github.com/unicode-org/icu4x/commit/cac7266c7c72386f747eb647fa9769d78d7f41d0"
        },
        "date": 1660781868508,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 980,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 703,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 867,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 899,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1263,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1383,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 704,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 866,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 900,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 830,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1383,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 56451834,
            "range": "± 135162",
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
          "id": "598899c2c9236505ae337c885b9b7a33c2505357",
          "message": "Apply missing_docs to icu_provider_macros (#2394)",
          "timestamp": "2022-08-17T16:59:41-07:00",
          "tree_id": "93e3fba4791043f1ca62baab7c913c5cff0a2197",
          "url": "https://github.com/unicode-org/icu4x/commit/598899c2c9236505ae337c885b9b7a33c2505357"
        },
        "date": 1660781981977,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 739,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 607,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 587,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 617,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 937,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1012,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 608,
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
            "value": 617,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 681,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1011,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 54083325,
            "range": "± 844755",
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
          "id": "834b8b4e4db71a30a488c2229f522c74991dd736",
          "message": "Apply missing_docs to icu_provider_testdata (#2397)",
          "timestamp": "2022-08-17T18:26:50-07:00",
          "tree_id": "4ed8da21fdba07534b2637de69bbc923c423d5eb",
          "url": "https://github.com/unicode-org/icu4x/commit/834b8b4e4db71a30a488c2229f522c74991dd736"
        },
        "date": 1660786815683,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 921,
            "range": "± 63",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 729,
            "range": "± 63",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 849,
            "range": "± 99",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 878,
            "range": "± 56",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1126,
            "range": "± 69",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1252,
            "range": "± 108",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 766,
            "range": "± 69",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 847,
            "range": "± 100",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 893,
            "range": "± 79",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 816,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1133,
            "range": "± 75",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 38753973,
            "range": "± 2436781",
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
          "id": "9a85a1d488c6cd4eb267cab72a3ae1343eeba220",
          "message": "Avoid cloning locales in DateTimeFormatter (#2398)",
          "timestamp": "2022-08-18T10:57:05-05:00",
          "tree_id": "9bb901a3ac7f7bd2f41dd89b72cce7475561b109",
          "url": "https://github.com/unicode-org/icu4x/commit/9a85a1d488c6cd4eb267cab72a3ae1343eeba220"
        },
        "date": 1660838959874,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 816,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 586,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 724,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 749,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1052,
            "range": "± 4",
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
            "value": 586,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 725,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 750,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 689,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1151,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 47070223,
            "range": "± 127276",
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
            "email": "robertbastian@users.noreply.github.com",
            "name": "Robert Bastian",
            "username": "robertbastian"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "610bca6e0fcee4f48f831c859ed44bba32cac881",
          "message": "Adding metacrate support to databake (#2380)",
          "timestamp": "2022-08-18T18:09:16+02:00",
          "tree_id": "203f5242f728091a046079b0525fe442cf651622",
          "url": "https://github.com/unicode-org/icu4x/commit/610bca6e0fcee4f48f831c859ed44bba32cac881"
        },
        "date": 1660839762772,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 735,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 606,
            "range": "± 4",
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
            "value": 617,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 938,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1011,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 606,
            "range": "± 2",
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
            "value": 617,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 681,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1012,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 54103667,
            "range": "± 893720",
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
            "email": "35614614+pdogr@users.noreply.github.com",
            "name": "Pawan Dogra",
            "username": "pdogr"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "56eaf09d44be6b3684137ad624a11055e50bd73e",
          "message": "Enable `locale` macro to support single unicode key value pair extension (#2382)",
          "timestamp": "2022-08-18T12:02:33-07:00",
          "tree_id": "01703bd5d752c662aec4a5683c0db35ee4b1815e",
          "url": "https://github.com/unicode-org/icu4x/commit/56eaf09d44be6b3684137ad624a11055e50bd73e"
        },
        "date": 1660850084681,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 937,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 675,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 831,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 866,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1196,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1318,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 696,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 822,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 854,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 803,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1325,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 54747637,
            "range": "± 780312",
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
            "email": "hsivonen@hsivonen.fi",
            "name": "Henri Sivonen",
            "username": "hsivonen"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "a5ac45a4028238afc9c0ef15c4b784e95e22db5f",
          "message": "Consolidate the two auxiliary tries to the main NFD trie (#2371)",
          "timestamp": "2022-08-18T20:23:16Z",
          "tree_id": "3ea9b783f8c2f326a9816f48f3455dcb767679ba",
          "url": "https://github.com/unicode-org/icu4x/commit/a5ac45a4028238afc9c0ef15c4b784e95e22db5f"
        },
        "date": 1660854954502,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 815,
            "range": "± 2",
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
            "value": 719,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 750,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1052,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1151,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 590,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 717,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 747,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 689,
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
            "value": 47075878,
            "range": "± 75896",
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
          "id": "c2be4f22269ae42302a60ce92a7b307fd13cd55c",
          "message": "Add missing doc for segmenter, and turn on the missing_docs warning (#2366)\n\n- Move top-level doc in lib.rs into individual structs.\r\n- Run `cargo make generate-readmes` to update README.md.",
          "timestamp": "2022-08-18T16:09:50-07:00",
          "tree_id": "cfac546ba2e49c9d82fbf17e4e9047ca2466b5af",
          "url": "https://github.com/unicode-org/icu4x/commit/c2be4f22269ae42302a60ce92a7b307fd13cd55c"
        },
        "date": 1660864934529,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 815,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 586,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 714,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 749,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1051,
            "range": "± 0",
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
            "value": 586,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 726,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 749,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 689,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1154,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 46959218,
            "range": "± 65521",
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
          "id": "948628fa3b3a8c79ac5c86cdd56b99b80568275b",
          "message": "Add properties FFI for all types (#2405)\n\n* add try_into_converted\r\n\r\n* Use in ffi\r\n\r\n* rm result type\r\n\r\n* Add ffi for remaining map properties\r\n\r\n* regen\r\n\r\n* some more tests\r\n\r\n* fmt\r\n\r\n* clippy\r\n\r\n* rm CCC\r\n\r\n* check in added files",
          "timestamp": "2022-08-19T16:53:48Z",
          "tree_id": "0bd196f007a6676b39bf7a19861cf394581a4aa9",
          "url": "https://github.com/unicode-org/icu4x/commit/948628fa3b3a8c79ac5c86cdd56b99b80568275b"
        },
        "date": 1660928771037,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 756,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 606,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 588,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 620,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 938,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1012,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 606,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 588,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 617,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 678,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1014,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 54189369,
            "range": "± 1076767",
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
          "id": "b829ced17bf9de7be832acffee757b6b0b23d00a",
          "message": "Document icu_calendar (#2407)\n\n* doc types\r\n\r\n* various docs\r\n\r\n* docs for arithmetic\r\n\r\n* docs for any\r\n\r\n* Document calendars\r\n\r\n* Finish up\r\n\r\n* Move week_of into own module\r\n\r\n* lib.rs\r\n\r\n* fmt\r\n\r\n* nits\r\n\r\n* fix",
          "timestamp": "2022-08-19T18:06:17Z",
          "tree_id": "7268705a87e7a8f152c1ba3cf33a8049c5c2f31a",
          "url": "https://github.com/unicode-org/icu4x/commit/b829ced17bf9de7be832acffee757b6b0b23d00a"
        },
        "date": 1660933086988,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 737,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 608,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 588,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 617,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 937,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1012,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 607,
            "range": "± 4",
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
            "value": 617,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 681,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1031,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 54196243,
            "range": "± 749226",
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
            "email": "yzhang1994@gmail.com",
            "name": "Yvonne Z",
            "username": "yzhang1994"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "8c497c3955d5cd32548af134b9c9599c305eaa95",
          "message": "Make offset_date handle wraparounds for months (#2373)\n\n* Make offset_date handle wraparounds for months\r\n\r\n* Handle edge case where original day is invalid in future month.",
          "timestamp": "2022-08-19T23:44:47Z",
          "tree_id": "0e9d4c7a0c2d48713ad1981a03137ef8ce9b228b",
          "url": "https://github.com/unicode-org/icu4x/commit/8c497c3955d5cd32548af134b9c9599c305eaa95"
        },
        "date": 1660953935856,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 976,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 703,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 866,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 879,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1238,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1390,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 698,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 865,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 895,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 817,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1357,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 54590330,
            "range": "± 1265477",
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
          "id": "5e96059c75c707a3d20d26147384f21460f0bb28",
          "message": "Rename ethiopic -> ethiopian (#2416)\n\n* Rename ethiopic -> ethiopian",
          "timestamp": "2022-08-20T01:11:51Z",
          "tree_id": "3b4cc3958936721c7b4da0f784f6416dd8158ec8",
          "url": "https://github.com/unicode-org/icu4x/commit/5e96059c75c707a3d20d26147384f21460f0bb28"
        },
        "date": 1660958707653,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 938,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 658,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 846,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 884,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1205,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1321,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 668,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 829,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 850,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 792,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1318,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 54736077,
            "range": "± 2617491",
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
          "id": "0485e61824cb3b1710bcedaf4accc43f5ef1737f",
          "message": "TimeVariant -> ZoneVariant with a few more docs (#2427)",
          "timestamp": "2022-08-19T22:21:00-05:00",
          "tree_id": "09839bcd6cb66272e6a289dfefb106404b0f6bd3",
          "url": "https://github.com/unicode-org/icu4x/commit/0485e61824cb3b1710bcedaf4accc43f5ef1737f"
        },
        "date": 1660966414102,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 817,
            "range": "± 2",
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
            "value": 720,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 748,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1051,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1150,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 590,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 710,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 750,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 689,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1150,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 46807519,
            "range": "± 73377",
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
          "id": "64e4c1e38b227fe6a68aa9233c7d88b08243ad64",
          "message": "Hide duration stuff, rename IncludedInAnyCalendar (#2426)\n\n* Hide duration stuff\r\n\r\n* hidden\r\n\r\n* rename IncludedInAnyCalendar\r\n\r\n* save test\r\n\r\n* readme\r\n\r\n* more",
          "timestamp": "2022-08-19T22:49:20-07:00",
          "tree_id": "3c65ddd683399dd0b481c8513450d556173433ce",
          "url": "https://github.com/unicode-org/icu4x/commit/64e4c1e38b227fe6a68aa9233c7d88b08243ad64"
        },
        "date": 1660975456747,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 993,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 796,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 951,
            "range": "± 88",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 961,
            "range": "± 71",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1277,
            "range": "± 68",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1390,
            "range": "± 52",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 790,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 911,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 963,
            "range": "± 152",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 944,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1380,
            "range": "± 107",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 47231359,
            "range": "± 2084078",
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
          "id": "35b33e934de5c2e8a996b27267170ecd1f3fc05a",
          "message": "Add docs and enable missing_docs lint in icu_provider (#2400)",
          "timestamp": "2022-08-20T09:23:02+02:00",
          "tree_id": "6cef63368f5b4712b96d247751c5f9f9a3c64a36",
          "url": "https://github.com/unicode-org/icu4x/commit/35b33e934de5c2e8a996b27267170ecd1f3fc05a"
        },
        "date": 1660980877834,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 748,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 607,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 587,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 620,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 937,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1011,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 607,
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
            "value": 617,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 677,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1011,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 54033173,
            "range": "± 758688",
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
            "email": "shane@unicode.org",
            "name": "Shane F. Carr",
            "username": "sffc"
          },
          "distinct": true,
          "id": "19dd573757f95fa808aa5ea0c4175b2fd17e85c3",
          "message": "tinystr 0.6.2: Add zerovec 0.7 compatibility",
          "timestamp": "2022-08-21T23:06:13-05:00",
          "tree_id": "512d6a7f5fdf945ae95d6bda246884d5e957311c",
          "url": "https://github.com/unicode-org/icu4x/commit/19dd573757f95fa808aa5ea0c4175b2fd17e85c3"
        },
        "date": 1661142064100,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 823,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 614,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 760,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 783,
            "range": "± 7",
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
            "value": 1087,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 614,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 757,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 782,
            "range": "± 7",
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
            "value": 1065,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 41629371,
            "range": "± 211539",
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
            "email": "robertbastian@users.noreply.github.com",
            "name": "Robert Bastian",
            "username": "robertbastian"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "d3bc694b7b8bf2c49bb54d3fd1a6778e8e9357d2",
          "message": "First pass at lints (#2411)",
          "timestamp": "2022-08-22T17:53:23+02:00",
          "tree_id": "d3fcca55550473f6d7610734166f0ab8400605cb",
          "url": "https://github.com/unicode-org/icu4x/commit/d3bc694b7b8bf2c49bb54d3fd1a6778e8e9357d2"
        },
        "date": 1661184475774,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 726,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 542,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 671,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 696,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 872,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 940,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 542,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 669,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 688,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 632,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 940,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 42555694,
            "range": "± 197172",
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
          "id": "f1a6fdb9ba62978b65b2a67edc218139658df32f",
          "message": "Mention some unwrap reasons (#2435)\n\n* Comment on unwrap in ethiopic\r\n\r\n* Fix zerovec comments",
          "timestamp": "2022-08-22T22:22:38Z",
          "tree_id": "fc36bc9af65da9a91b0e043d6c03e6a9f4460a59",
          "url": "https://github.com/unicode-org/icu4x/commit/f1a6fdb9ba62978b65b2a67edc218139658df32f"
        },
        "date": 1661207663576,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 823,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 615,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 757,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 780,
            "range": "± 8",
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
            "value": 1066,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 614,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 757,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 780,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 717,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1065,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 48263287,
            "range": "± 184699",
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
            "email": "robertbastian@users.noreply.github.com",
            "name": "Robert Bastian",
            "username": "robertbastian"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "7c6b6487277c2faef308a40ebad381ee35493203",
          "message": "Fixing ecma402 crate (#2434)",
          "timestamp": "2022-08-23T10:14:05+02:00",
          "tree_id": "64ee2c9edeb08614ebbe9ad71390379a221c9434",
          "url": "https://github.com/unicode-org/icu4x/commit/7c6b6487277c2faef308a40ebad381ee35493203"
        },
        "date": 1661243216131,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 871,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 719,
            "range": "± 92",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 847,
            "range": "± 103",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 859,
            "range": "± 53",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1120,
            "range": "± 90",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1280,
            "range": "± 68",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 691,
            "range": "± 79",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 813,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 874,
            "range": "± 51",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 823,
            "range": "± 56",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1224,
            "range": "± 89",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 42596724,
            "range": "± 2373277",
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
            "email": "35614614+pdogr@users.noreply.github.com",
            "name": "Pawan Dogra",
            "username": "pdogr"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "59b17253f117e0a927e7e55ce39feecd016a32e0",
          "message": "Make `Option<TinyAsciiStr>` be the same size as `TinyAsciiStr` (#2430)",
          "timestamp": "2022-08-23T10:57:57+02:00",
          "tree_id": "f27e745fa03b05a7ff3caef8dd5951bf76ece3ee",
          "url": "https://github.com/unicode-org/icu4x/commit/59b17253f117e0a927e7e55ce39feecd016a32e0"
        },
        "date": 1661245799843,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 824,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 615,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 759,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 780,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 983,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1066,
            "range": "± 1",
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
            "value": 760,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 778,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 716,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1084,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 48265060,
            "range": "± 3557380",
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
          "id": "7abc21b21f1ff358712234f172bb2331b19d7f28",
          "message": "Expose RuleBreakIterator as a public interface (#2408)\n\nAll the UAX 29 break iterators (grapheme cluster, sentence, and word) are type\r\ndefines of RuleBreakIterator. Expose RuleBreakIterator so that users can refer\r\nto RuleBreakIterator's documentation for APIs.",
          "timestamp": "2022-08-23T09:43:34-07:00",
          "tree_id": "aab46c93818a4bc744e6b2d4806ee85541ef7a91",
          "url": "https://github.com/unicode-org/icu4x/commit/7abc21b21f1ff358712234f172bb2331b19d7f28"
        },
        "date": 1661273797354,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 824,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 615,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 760,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 780,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 999,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1067,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 616,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 748,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 780,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 717,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1066,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 48479138,
            "range": "± 116554",
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