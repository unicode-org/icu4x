window.BENCHMARK_DATA = {
  "lastUpdate": 1670268739935,
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
          "id": "eedfa41b65cad8e646f07d692fad8638538e306b",
          "message": "inline(always) trie accessors that are macros in ICU4C (#2410)",
          "timestamp": "2022-08-23T20:31:54+03:00",
          "tree_id": "cc8f2b7d5058272d8f907ca9f3d3e412a3f0599f",
          "url": "https://github.com/unicode-org/icu4x/commit/eedfa41b65cad8e646f07d692fad8638538e306b"
        },
        "date": 1661276617071,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 637,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 372,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 391,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 441,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 927,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 939,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 371,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 388,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 442,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 489,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 940,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 54094109,
            "range": "± 684075",
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
          "id": "b01a716fd0a953bb9acb6ac1db8558750e8b8137",
          "message": "Update postcard to 1.0 (#2438)",
          "timestamp": "2022-08-23T13:54:04-07:00",
          "tree_id": "7e8dc672cc6043e49f276457209e748a3f6801c9",
          "url": "https://github.com/unicode-org/icu4x/commit/b01a716fd0a953bb9acb6ac1db8558750e8b8137"
        },
        "date": 1661288764385,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 865,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 444,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 479,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 635,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1093,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1182,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 444,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 478,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 635,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 594,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1179,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 44360007,
            "range": "± 168701",
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
          "id": "64a4d74411aeaa92e91a41027ca35cb4503f2576",
          "message": "Add comment about inline(always) in CodePointTrie (#2443)",
          "timestamp": "2022-08-24T07:19:41-07:00",
          "tree_id": "f7cbef6a919a1cbdfe969a573e98fa72423fb9fa",
          "url": "https://github.com/unicode-org/icu4x/commit/64a4d74411aeaa92e91a41027ca35cb4503f2576"
        },
        "date": 1661351495342,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 684,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 453,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 449,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 481,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 961,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 989,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 447,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 443,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 482,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 498,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 990,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 56387422,
            "range": "± 922758",
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
          "id": "d8bee1999b6266a5c24dadbacb39d70abf0a3088",
          "message": "Second pass at lints (#2432)",
          "timestamp": "2022-08-24T17:30:12+02:00",
          "tree_id": "b3994dbde847d013418589b3c975d3e6e78f270f",
          "url": "https://github.com/unicode-org/icu4x/commit/d8bee1999b6266a5c24dadbacb39d70abf0a3088"
        },
        "date": 1661355731443,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 683,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 449,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 449,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 481,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 961,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 989,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 448,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 442,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 481,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 498,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 986,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 56319713,
            "range": "± 807346",
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
          "id": "6f6a9cc2e3fb9fbba2772a64389682a93163a9f5",
          "message": "Add more missing docs (#2399)",
          "timestamp": "2022-08-24T17:47:03Z",
          "tree_id": "b4a63060a14d41d73f2f8c31ab21414f8c491e95",
          "url": "https://github.com/unicode-org/icu4x/commit/6f6a9cc2e3fb9fbba2772a64389682a93163a9f5"
        },
        "date": 1661363986968,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 682,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 446,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 445,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 480,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 962,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 985,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 445,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 446,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 479,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 497,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 993,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 56535079,
            "range": "± 789052",
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
          "id": "283a775986ec0a8c666768f897b21820aa8d6a80",
          "message": "Enable missing docs lint in timezone (#2441)",
          "timestamp": "2022-08-24T17:47:14Z",
          "tree_id": "4fe3fb3ebfdc7d72c0e0a54ce576078257df82c0",
          "url": "https://github.com/unicode-org/icu4x/commit/283a775986ec0a8c666768f897b21820aa8d6a80"
        },
        "date": 1661364030289,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 847,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 468,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 510,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 617,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1066,
            "range": "± 60",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1150,
            "range": "± 59",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 444,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 481,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 685,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 607,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1151,
            "range": "± 65",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 42691126,
            "range": "± 1649671",
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
          "id": "58e0bfa5aef0774581a388070fdcfaacb3155f93",
          "message": "Move mock datetime parsing code to test modules (#2436)\n\n* rm parse_gregorian usage\r\n\r\n* update work_log.wasm\r\n\r\n* Move mock over to tests\r\n\r\n* gregorian -> iso\r\n\r\n* revert work_log\r\n\r\n* fmt\r\n\r\n* readme\r\n\r\n* fix",
          "timestamp": "2022-08-24T20:33:24Z",
          "tree_id": "f14945dfe3610e0eb7ac69435d626368a7addb28",
          "url": "https://github.com/unicode-org/icu4x/commit/58e0bfa5aef0774581a388070fdcfaacb3155f93"
        },
        "date": 1661374013817,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 681,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 450,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 444,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 480,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 963,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 988,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 447,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 442,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 480,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 499,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 989,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 56537382,
            "range": "± 836594",
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
          "id": "df54446afc4b9f154edc92882e6e42dd26b05a5c",
          "message": "Better rust_link handling; add module-based allowlist (#2440)\n\n* update diplomat\r\n\r\n* Update smallvec\r\n\r\n* Update completeness test\r\n\r\n* Add module allowlist to completeness test\r\n\r\n* prepopulate\r\n\r\n* diplomatup again\r\n\r\n* regen\r\n\r\n* fix list\r\n\r\n* name",
          "timestamp": "2022-08-24T20:33:49Z",
          "tree_id": "fbf38a655fbd52e910e62b3603d4179cc4574242",
          "url": "https://github.com/unicode-org/icu4x/commit/df54446afc4b9f154edc92882e6e42dd26b05a5c"
        },
        "date": 1661374080248,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 855,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 556,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 582,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 687,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1232,
            "range": "± 53",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1344,
            "range": "± 181",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 552,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 583,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 683,
            "range": "± 55",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 666,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1320,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 45891593,
            "range": "± 1891414",
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
          "id": "832401abdb7f0d878f94949d9075504110796407",
          "message": "Optimize normalizers for contiguous-memory input/output case (#2378)",
          "timestamp": "2022-08-25T09:34:48+03:00",
          "tree_id": "7ef0559b171700fc26fc23a7c0957d9abbd18f7e",
          "url": "https://github.com/unicode-org/icu4x/commit/832401abdb7f0d878f94949d9075504110796407"
        },
        "date": 1661410064422,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 586,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 366,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 375,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 426,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 781,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 814,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 365,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 373,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 426,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 461,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 806,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 54196439,
            "range": "± 832499",
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
          "id": "6b98f3b5eb35b6b02ba33ec778a45f5a2ec58ec1",
          "message": "Switch from runtime to compile time Locale parsing (#2406)",
          "timestamp": "2022-08-25T10:27:44+02:00",
          "tree_id": "34b4141348260d09f8c4b7d09f2f6d9b3ba70d91",
          "url": "https://github.com/unicode-org/icu4x/commit/6b98f3b5eb35b6b02ba33ec778a45f5a2ec58ec1"
        },
        "date": 1661416832107,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 643,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 476,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 506,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 579,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 819,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 913,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 476,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 506,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 579,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 610,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 914,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 43622141,
            "range": "± 108396",
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
          "id": "66f13f6985f9389ca9e4b5e1360fd3d4c436f8f2",
          "message": "Check in FFI coverage list (#2446)\n\n* checkin\r\n\r\n* Split CI tasks into ffi and verify-ffi\r\n\r\n* regen\r\n\r\n* fix task name\r\n\r\n* remove diplomat-gen deps\r\n\r\n* path\r\n\r\n* e\r\n\r\n* nightly\r\n\r\n* nightly\r\n\r\n* print output\r\n\r\n* rm py\r\n\r\n* undo\r\n\r\n* unpin nightly\r\n\r\n* undo\r\n\r\n* reset rustdoc-types\r\n\r\n* Make new completeness module\r\n\r\n* no need for nightly when building\r\n\r\n* fix path\r\n\r\n* check in cargo\r\n\r\n* log\r\n\r\n* Move generated file\r\n\r\n* license\r\n\r\n* publish = false\r\n\r\n* publish\r\n\r\n* lockfile\r\n\r\nCo-authored-by: Robert Bastian <robertbastian@users.noreply.github.com>",
          "timestamp": "2022-08-25T20:02:11Z",
          "tree_id": "11181601e466674553562acf5d20c8da04f6a092",
          "url": "https://github.com/unicode-org/icu4x/commit/66f13f6985f9389ca9e4b5e1360fd3d4c436f8f2"
        },
        "date": 1661458541263,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 728,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 545,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 580,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 651,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 966,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1049,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 552,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 572,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 644,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 691,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1031,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 48856135,
            "range": "± 1617101",
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
          "id": "e17ab458aa39467946dadab33d2d3cb37a622646",
          "message": "FFI coverage whack-a-mole part 1 (#2455)\n\n* Make fixed_decimal::decimal private (fixes #2449)\r\n\r\n* regen coverage\r\n\r\n* fix insert_ty\r\n\r\n* Handle errors and duration apis\r\n\r\n* Improve datetime docs display\r\n\r\n* Fixes\r\n\r\n* Update to newer nightly\r\n\r\n* regen\r\n\r\n* regen diplomat\r\n\r\n* fix imports\r\n\r\n* fmt",
          "timestamp": "2022-08-26T00:35:34Z",
          "tree_id": "330d545ed674bac5071780a2f83549710fb3063e",
          "url": "https://github.com/unicode-org/icu4x/commit/e17ab458aa39467946dadab33d2d3cb37a622646"
        },
        "date": 1661474881854,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 643,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 476,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 506,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 579,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 819,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 913,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 476,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 506,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 579,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 610,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 913,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 43508420,
            "range": "± 108839",
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
          "id": "259049454dfd5b9d4cbc7d0ded1753b93a8ebb91",
          "message": "FFI coverage for FixedDecimal, TimeZone, TinyStr (#2442)",
          "timestamp": "2022-08-25T22:07:16-05:00",
          "tree_id": "182f4b60ea5d797904b1f5e3b3eb2e1ead8cca3d",
          "url": "https://github.com/unicode-org/icu4x/commit/259049454dfd5b9d4cbc7d0ded1753b93a8ebb91"
        },
        "date": 1661483947536,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 567,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 420,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 447,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 511,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 722,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 806,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 420,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 447,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 511,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 538,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 806,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 38401453,
            "range": "± 88756",
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
          "id": "c2bb80b85fd57db13c3f570f4cde1fe20a7744ef",
          "message": "FFI coverage whack-a-mole part 2 (#2457)\n\n* Update diplomat\r\n\r\n* include calendar trait\r\n\r\n* Add AnyCalendarKind; calendar cleanups\r\n\r\n* Mention broken links\r\n\r\n* regen\r\n\r\n* fix tests\r\n\r\n* Fixup fixed decimal links\r\n\r\n* regen diplomat\r\n\r\n* bump diplomat to main\r\n\r\n* remove broken links test (it doesn't work since IGNORED_TRAITS bails early",
          "timestamp": "2022-08-26T19:37:01Z",
          "tree_id": "92efaab472136d7d269ede672d69c85f4935106d",
          "url": "https://github.com/unicode-org/icu4x/commit/c2bb80b85fd57db13c3f570f4cde1fe20a7744ef"
        },
        "date": 1661543386167,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 772,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 572,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 607,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 693,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 981,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1087,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 570,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 601,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 686,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 726,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1088,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 51575839,
            "range": "± 445910",
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
          "id": "dd09074425b825f4ca696922e58280722d986454",
          "message": "Add test and create_empty on CustomTimeZone (#2456)",
          "timestamp": "2022-08-26T12:49:56-07:00",
          "tree_id": "03c41df6277d3db3fcf1fd8390c2912d83007d01",
          "url": "https://github.com/unicode-org/icu4x/commit/dd09074425b825f4ca696922e58280722d986454"
        },
        "date": 1661544268530,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 771,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 571,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 608,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 694,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 983,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1096,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 571,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 608,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 695,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 732,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1096,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 52207114,
            "range": "± 173042",
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
          "id": "bf9012336256c01512374518f2ff90f25d45a998",
          "message": "Add icu_testdata::versions module and fix metadata lookup (#2463)",
          "timestamp": "2022-08-26T16:48:04-07:00",
          "tree_id": "b3050de32c50319d1bbaa67b37aaf415515ec963",
          "url": "https://github.com/unicode-org/icu4x/commit/bf9012336256c01512374518f2ff90f25d45a998"
        },
        "date": 1661558456588,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 584,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 420,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 447,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 511,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 722,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 806,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 420,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 446,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 511,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 538,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 805,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 38531591,
            "range": "± 133899",
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
          "id": "af635e223132df2671ce7004b992835b26f6f271",
          "message": "week_of refactoring (#2462)\n\n* Add week_of_month and week_of_year to Date\r\n* Rename CalendarInfo\r\n* Move the provider struct into icu_calendar and make runtime struct non_exhaustive",
          "timestamp": "2022-08-27T15:01:05-07:00",
          "tree_id": "48f4463323ac3489ccba72194095ba86de8d0c91",
          "url": "https://github.com/unicode-org/icu4x/commit/af635e223132df2671ce7004b992835b26f6f271"
        },
        "date": 1661638368868,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 584,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 367,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 375,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 425,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 781,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 809,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 366,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 373,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 425,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 462,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 807,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 54001451,
            "range": "± 627058",
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
            "email": "sipasseuth.daniel@gmail.com",
            "name": "Sipasseuth Daniel",
            "username": "dsipasseuth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "1ad3fb5eae2c319b71b385729922b2db202f32ca",
          "message": "split_first_u16/split_first_u24 -> split_first (#2459)\n\n* refactor(issue_2056): split_first_u16/split_first_u24 -> split_first\r\n\r\n* Apply suggestions from code review\r\n\r\nCo-authored-by: Robert Bastian <robertbastian@users.noreply.github.com>\r\n\r\n* cargo fmt error fix\r\n\r\nCo-authored-by: Daniel Sipasseuth <dsipasseuth@google.com>\r\nCo-authored-by: Robert Bastian <robertbastian@users.noreply.github.com>",
          "timestamp": "2022-08-29T19:31:32Z",
          "tree_id": "e56556ee389cf523bd23bfe8c64209b0445beab6",
          "url": "https://github.com/unicode-org/icu4x/commit/1ad3fb5eae2c319b71b385729922b2db202f32ca"
        },
        "date": 1661802384327,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 592,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 366,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 373,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 426,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 781,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 808,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 365,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 374,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 425,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 461,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 807,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 54214803,
            "range": "± 757973",
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
          "id": "ebedd726e407960aafec8c7932b802250162e4be",
          "message": "Make AnyPayload sync by using RcWrap #2176 (#2465)\n\n* Merge remote-tracking branch 'uptstream/main'\r\n\r\n* Make AnyPayload sync by using RcWrap\r\n\r\n* Changing the imports and running the fmt.\r\n\r\n* Moving RcWrapBounds to response.rs\r\n\r\n* Minor fix for comment.\r\n\r\n* Running cargo make testdata\r\n\r\n* Minor change to fix the tests.\r\n\r\n* Removing carg-if dependency.",
          "timestamp": "2022-08-29T19:30:48Z",
          "tree_id": "80aecfa9c26ef29ee9293a8fe2347704ff784003",
          "url": "https://github.com/unicode-org/icu4x/commit/ebedd726e407960aafec8c7932b802250162e4be"
        },
        "date": 1661802456372,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 643,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 476,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 506,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 579,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 819,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 914,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 476,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 506,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 579,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 610,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 913,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 43546310,
            "range": "± 121950",
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
          "id": "9949cdc883c1c780faf81a1e06ae7888e2758d24",
          "message": "Whack-a-mole 3: Date, DateTime, Time, and Formatters (#2464)\n\n* fix docs\r\n\r\n* Finish all AnyCalendarKind methods\r\n\r\n* datetime -> datetime_format\r\n\r\n* Split into datetime and calendar modules\r\n\r\n* Add ffi objects for Date\r\n\r\n* Add lots of methods to Date and DateTime\r\n\r\n* regen\r\n\r\n* remove dangerous mutable set_ns api\r\n\r\n* regen\r\n\r\n* Add Time module\r\n\r\n* hook Time type into everything\r\n\r\n* regen\r\n\r\n* Add Date formatters\r\n\r\n* rename\r\n\r\n* add DateTime ctor from parts\r\n\r\n* more datetime coverage whackamole\r\n\r\n* regen\r\n\r\n* clip\r\n\r\n* fix test\r\n\r\n* Update ffi/diplomat/src/date.rs\r\n\r\nCo-authored-by: Shane F. Carr <shane@unicode.org>\r\n\r\n* Update ffi/diplomat/src/date.rs\r\n\r\nCo-authored-by: Shane F. Carr <shane@unicode.org>\r\n\r\n* Update ffi/diplomat/src/date.rs\r\n\r\nCo-authored-by: Shane F. Carr <shane@unicode.org>\r\n\r\n* Update ffi/diplomat/src/datetime_formatter.rs\r\n\r\nCo-authored-by: Shane F. Carr <shane@unicode.org>\r\n\r\n* Update ffi/diplomat/src/datetime.rs\r\n\r\nCo-authored-by: Shane F. Carr <shane@unicode.org>\r\n\r\n* fixes\r\n\r\n* format_gregorian_datetime\r\n\r\n* add more formatters\r\n\r\n* regen\r\n\r\n* test\r\n\r\n* Move over to ICU4XIsoDate[Time] in gregorian formatter\r\n\r\n* Remove ICU4XGregorianDateTime\r\n\r\n* regen\r\n\r\n* tests\r\n\r\n* copy paste Date methods\r\n\r\n* Add methods to datetime\r\n\r\n* copy paste to iso\r\n\r\n* regen\r\n\r\n* clippy\r\n\r\n* clip\r\n\r\n* Update components/calendar/src/date.rs\r\n\r\nCo-authored-by: Shane F. Carr <shane@unicode.org>\r\n\r\n* Update components/calendar/src/datetime.rs\r\n\r\nCo-authored-by: Shane F. Carr <shane@unicode.org>\r\n\r\n* Update ffi/diplomat/src/datetime_formatter.rs\r\n\r\nCo-authored-by: Shane F. Carr <shane@unicode.org>\r\n\r\n* fixes\r\n\r\n* Update ffi/diplomat/src/date.rs\r\n\r\nCo-authored-by: Shane F. Carr <shane@unicode.org>\r\n\r\n* Update ffi/diplomat/src/date.rs\r\n\r\nCo-authored-by: Shane F. Carr <shane@unicode.org>\r\n\r\n* Update ffi/diplomat/src/datetime.rs\r\n\r\nCo-authored-by: Shane F. Carr <shane@unicode.org>\r\n\r\n* doc\r\n\r\n* gen\r\n\r\n* fmt\r\n\r\n* fix\r\n\r\n* link\r\n\r\nCo-authored-by: Shane F. Carr <shane@unicode.org>",
          "timestamp": "2022-08-29T22:04:41Z",
          "tree_id": "9bf610175077042d5d41e3d90ac8bdad19fc7aa0",
          "url": "https://github.com/unicode-org/icu4x/commit/9949cdc883c1c780faf81a1e06ae7888e2758d24"
        },
        "date": 1661811487898,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 626,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 466,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 498,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 595,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 982,
            "range": "± 62",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1006,
            "range": "± 60",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 467,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 493,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 582,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 561,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 988,
            "range": "± 77",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 45396690,
            "range": "± 2659110",
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
          "id": "a1dff1e5c7fe680bc466a3bf2c60ab4c2851f802",
          "message": "Fix docs for XID_Start and XID_Continue in properties sets API docs (#2469)",
          "timestamp": "2022-08-29T23:18:45Z",
          "tree_id": "32f2f970bf185ce357f3e2bcd78a7b7b805b5e5b",
          "url": "https://github.com/unicode-org/icu4x/commit/a1dff1e5c7fe680bc466a3bf2c60ab4c2851f802"
        },
        "date": 1661815921155,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 687,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 493,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 558,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 613,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1058,
            "range": "± 82",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1097,
            "range": "± 64",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 501,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 548,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 614,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 639,
            "range": "± 51",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1108,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 44830722,
            "range": "± 1102705",
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
          "id": "84ffc4a2400105162d6a09cc6f7a8a57b74c88b2",
          "message": "Whack-a-mole part 4: PluralRules, some properties (#2471)\n\n* Some pluralrules fixes\r\n\r\n* regen\r\n\r\n* tests\r\n\r\n* regen\r\n\r\n* pare down properties",
          "timestamp": "2022-08-30T04:58:52Z",
          "tree_id": "96ecb18ad3209c5588387b4a857f47953c5ca2eb",
          "url": "https://github.com/unicode-org/icu4x/commit/84ffc4a2400105162d6a09cc6f7a8a57b74c88b2"
        },
        "date": 1661836306106,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 612,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 444,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 484,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 551,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 959,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 974,
            "range": "± 172",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 447,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 482,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 544,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 571,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 977,
            "range": "± 52",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 39629393,
            "range": "± 1836302",
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
          "id": "9e95b1dd21c0f6bfe58f3d2a426ff89633c3658b",
          "message": "Reducing `locid_id` API surface (#2484)",
          "timestamp": "2022-08-30T18:55:00+02:00",
          "tree_id": "9f5074b538f57209568fa0bcdb5f2cf8c36335de",
          "url": "https://github.com/unicode-org/icu4x/commit/9e95b1dd21c0f6bfe58f3d2a426ff89633c3658b"
        },
        "date": 1661879462454,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 772,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 571,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 608,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 695,
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
            "value": 1097,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 571,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 608,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 695,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 732,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1096,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 52191883,
            "range": "± 172064",
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
            "email": "younies.mahmoud@gmail.com",
            "name": "Younies Mahmoud",
            "username": "younies"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "8f902cbc1f62204bef1fd0f086988f971257c239",
          "message": "Fix clippy for the non_exhaustive enums. (#2483)\n\nFix clippy for the non_exhaustive enums.",
          "timestamp": "2022-08-30T19:36:53+02:00",
          "tree_id": "e0396ade2e346f889fa49d7a52554fbb1764f0f3",
          "url": "https://github.com/unicode-org/icu4x/commit/8f902cbc1f62204bef1fd0f086988f971257c239"
        },
        "date": 1661881797938,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 794,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 565,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 607,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 682,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 958,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1076,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 561,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 591,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 693,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 739,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1088,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 51885670,
            "range": "± 545347",
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
          "id": "72424a5b15026844341ccf762bb2d10231eb4755",
          "message": "TimeZoneFormatter FFI and CustomTimeZone cleanup (#2470)",
          "timestamp": "2022-08-30T10:44:26-07:00",
          "tree_id": "025f143248d33866c853f73d0fb9e0cd7323d53e",
          "url": "https://github.com/unicode-org/icu4x/commit/72424a5b15026844341ccf762bb2d10231eb4755"
        },
        "date": 1661882293818,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 761,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 547,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 615,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 703,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 975,
            "range": "± 82",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1073,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 548,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 579,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 658,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 692,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1071,
            "range": "± 57",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 51674340,
            "range": "± 1529802",
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
          "id": "1b560dff118b8894200688b594d124522947dfc0",
          "message": "Stop returning error on mismatched locale and type calendar (#2477)",
          "timestamp": "2022-08-30T13:40:42-07:00",
          "tree_id": "080e57a9ba6b8153b86d1e7f92aa7e17cbfda279",
          "url": "https://github.com/unicode-org/icu4x/commit/1b560dff118b8894200688b594d124522947dfc0"
        },
        "date": 1661892923413,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 760,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 575,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 597,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 689,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 965,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1080,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 560,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 596,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 686,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 721,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1108,
            "range": "± 48",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 51367466,
            "range": "± 535304",
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
          "id": "74d3983b3080933cf755f0659d44c444530e2f55",
          "message": "Fix arithmetic in Indian calendar (#2479)\n\n* Fix indian date conversion\r\n\r\n* Optimize days_in_year for solar calendars",
          "timestamp": "2022-08-30T21:58:28Z",
          "tree_id": "326c4c4698ae577985e7e928cfcae8dab3b1180a",
          "url": "https://github.com/unicode-org/icu4x/commit/74d3983b3080933cf755f0659d44c444530e2f55"
        },
        "date": 1661897443558,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 643,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 476,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 506,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 579,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 819,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 914,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 476,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 506,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 582,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 610,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 914,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 43524387,
            "range": "± 103268",
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
          "id": "5e6141a710a913db0139ab63e7e2364f493dbff6",
          "message": "Mark generated files as generated (#2487)\n\n* Mark generated files as generated",
          "timestamp": "2022-08-30T21:58:14Z",
          "tree_id": "ae622768b54e57e53aadc36ad35cffb8f94e704d",
          "url": "https://github.com/unicode-org/icu4x/commit/5e6141a710a913db0139ab63e7e2364f493dbff6"
        },
        "date": 1661897611697,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 678,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 467,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 537,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 580,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1031,
            "range": "± 80",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 975,
            "range": "± 117",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 448,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 493,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 648,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 537,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1036,
            "range": "± 73",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 45531720,
            "range": "± 3443341",
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
          "id": "780a04bd4c4127f21607b0e2d70d4487116f99d2",
          "message": "Update collator examples (#2447)",
          "timestamp": "2022-08-30T17:13:29-07:00",
          "tree_id": "eeaa627ddfca59daa2f9fcfa341087187877c995",
          "url": "https://github.com/unicode-org/icu4x/commit/780a04bd4c4127f21607b0e2d70d4487116f99d2"
        },
        "date": 1661905533160,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 643,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 476,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 506,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 579,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 819,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 914,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 476,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 506,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 579,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 610,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 914,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 43555728,
            "range": "± 135432",
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
          "id": "90db7c6a5ef3d64e969850c2655e8b9e45948c81",
          "message": "Use char instead of U24 in normalizer data (#2481)\n\n* Use char instead of U24 in normalizer data\r\n\r\nchar now has the same 3-byte ULE representation as U24, so the postcard and\r\nthe baked form do not change. (The JSON form changes, though.)",
          "timestamp": "2022-08-31T08:51:46+03:00",
          "tree_id": "e09bb860703df76840e87b9c870285f830cb12e2",
          "url": "https://github.com/unicode-org/icu4x/commit/90db7c6a5ef3d64e969850c2655e8b9e45948c81"
        },
        "date": 1661925907051,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 681,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 503,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 546,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 613,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1038,
            "range": "± 55",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1088,
            "range": "± 57",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 503,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 550,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 614,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 640,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1173,
            "range": "± 127",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 45597094,
            "range": "± 1737978",
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
          "id": "95d977f393ff7f46ce358b9fa12b548cd41b0625",
          "message": "Fix CI (#2494)",
          "timestamp": "2022-08-31T17:13:16+02:00",
          "tree_id": "bbf24ec2a11c86de12303355294f84a2a7351917",
          "url": "https://github.com/unicode-org/icu4x/commit/95d977f393ff7f46ce358b9fa12b548cd41b0625"
        },
        "date": 1661959651683,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 701,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 520,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 562,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 638,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1104,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1140,
            "range": "± 53",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 515,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 559,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 632,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 661,
            "range": "± 51",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1136,
            "range": "± 113",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 45878675,
            "range": "± 1939893",
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
          "id": "cd0f1fe2b14e914e62896b4e3f60a8855f339856",
          "message": "Whack-a-mole part 4: properties and locale (#2488)\n\n* Add all sets FFI\n\n* Add general category group\n\n* links and stuff\n\n* Add get_set_for_value\n\n* link up bidi_class\n\n* locale paring\n\n* regen\n\n* update tests\n\n* some review fixes\n\n* rename\n\n* gen\n\n* tests\n\n* remove docs\n\n* regen\n\n* add 32\n\n* regen\n\n* post merge coverage regen\n\n* Fix name; fix position in ignorelist\n\n* module doesn't exist anymore\n\n* also filter out properties collections (for now)\n\n* rename",
          "timestamp": "2022-08-31T12:02:11-07:00",
          "tree_id": "a020ec0bd06f759c0ab2cef41e50c9e37b08ca9c",
          "url": "https://github.com/unicode-org/icu4x/commit/cd0f1fe2b14e914e62896b4e3f60a8855f339856"
        },
        "date": 1661973234627,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 582,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 369,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 374,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 425,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 781,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 806,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 366,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 376,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 426,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 462,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 805,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 54020832,
            "range": "± 747261",
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
          "id": "faec6f178c3becaa6a6b903eef1cd087f3a51bad",
          "message": "Create options bag for CollatorOptions (#2475)",
          "timestamp": "2022-09-01T14:30:45Z",
          "tree_id": "2262a265c71c3514bfdd2157fde350b10e0395c1",
          "url": "https://github.com/unicode-org/icu4x/commit/faec6f178c3becaa6a6b903eef1cd087f3a51bad"
        },
        "date": 1662043372333,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 567,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 420,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 506,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 579,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 722,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 914,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 476,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 446,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 511,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 538,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 806,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 38573058,
            "range": "± 91558",
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
          "id": "237e9211c74394342ec42eae7cde565687ea005e",
          "message": "Whack-a-mole part 5: WeekCalculator, ignorelisting assoc items (#2499)\n\n* Track which trait you are in\r\n\r\n* Add support for ignoring associated items\r\n\r\n* Add WeekCalculator\r\n\r\n* add week_of functions\r\n\r\n* Update diplomat",
          "timestamp": "2022-09-02T17:30:07Z",
          "tree_id": "02a82a9a5bebff1135dc894a044b696d46dfa2f3",
          "url": "https://github.com/unicode-org/icu4x/commit/237e9211c74394342ec42eae7cde565687ea005e"
        },
        "date": 1662140542342,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 662,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 476,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 446,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 511,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 722,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 806,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 420,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 446,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 511,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 538,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 806,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 38553740,
            "range": "± 97969",
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
          "id": "7ca3a778149dfdc493480eac35971c42a087f464",
          "message": "TimeZone FFI fixes (#2466)",
          "timestamp": "2022-09-02T11:38:53-07:00",
          "tree_id": "1d41f0ec1a9c006e2bc1a7b453f15ad8d012fd44",
          "url": "https://github.com/unicode-org/icu4x/commit/7ca3a778149dfdc493480eac35971c42a087f464"
        },
        "date": 1662144766340,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 746,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 544,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 577,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 679,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 972,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1069,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 536,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 594,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 652,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 695,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1068,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 51106821,
            "range": "± 969147",
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
          "id": "19ce9fea2a752adfd3b4d4cc0b7434aa253bef6a",
          "message": "Clean up FFFD magic numbers in Collator with REPLACEMENT_CHAR (#2496)",
          "timestamp": "2022-09-02T21:16:27Z",
          "tree_id": "7bffd49ec8cb11205da6d816f8bef818586907db",
          "url": "https://github.com/unicode-org/icu4x/commit/19ce9fea2a752adfd3b4d4cc0b7434aa253bef6a"
        },
        "date": 1662154233813,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 763,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 549,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 586,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 663,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 954,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1081,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 562,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 580,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 685,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 721,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1057,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 53172719,
            "range": "± 2523559",
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
          "id": "81b3c7ecd66219f6b62bc3ed248cafdccb81505e",
          "message": "Assorted TimeZone fixes (#2478)",
          "timestamp": "2022-09-02T14:18:44-07:00",
          "tree_id": "36a4c7793d78bf85a50383803a6a40d87fbf9621",
          "url": "https://github.com/unicode-org/icu4x/commit/81b3c7ecd66219f6b62bc3ed248cafdccb81505e"
        },
        "date": 1662154344999,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 772,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 572,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 607,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 695,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 985,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1102,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 571,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 608,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 695,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 732,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1096,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 52120347,
            "range": "± 163877",
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
          "id": "47c8b13a32a824562e1261b1b0877365985ab972",
          "message": "ZonedDateTimeFormatter FFI (#2500)",
          "timestamp": "2022-09-02T14:19:15-07:00",
          "tree_id": "820e83020c1cac149b4d15faa214250a5ca0f14f",
          "url": "https://github.com/unicode-org/icu4x/commit/47c8b13a32a824562e1261b1b0877365985ab972"
        },
        "date": 1662154402391,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 417,
            "range": "± 124",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 259,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 275,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 304,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 558,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 578,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 258,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 266,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 304,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 329,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 580,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 38856783,
            "range": "± 2745273",
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
          "id": "799297f0b31abb5cd8e04bcb756d16788b35f04f",
          "message": "Split experimental and non-experimental DateTimeFormatter constructors (#2495)",
          "timestamp": "2022-09-02T14:19:04-07:00",
          "tree_id": "9c406843873644a1141f0a8fb5d1e358150f5ce1",
          "url": "https://github.com/unicode-org/icu4x/commit/799297f0b31abb5cd8e04bcb756d16788b35f04f"
        },
        "date": 1662154528020,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 643,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 476,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 506,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 579,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 819,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 914,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 476,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 506,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 579,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 610,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 924,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 43625332,
            "range": "± 105971",
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
          "id": "6e6ea16c1ead141900defedd1ede50a671684b1d",
          "message": "Generate consistent Locale subtag APIs (#1932)\n\n* wip\r\n\r\n* comments\r\n\r\n* recover macros mod\r\n\r\n* fix",
          "timestamp": "2022-09-02T15:15:51-07:00",
          "tree_id": "51503c59bef5e92faeb7799dc7f94db3a2e1e7cf",
          "url": "https://github.com/unicode-org/icu4x/commit/6e6ea16c1ead141900defedd1ede50a671684b1d"
        },
        "date": 1662157777412,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 643,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 476,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 506,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 578,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 818,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 913,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 476,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 506,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 579,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 609,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 913,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 43321974,
            "range": "± 125143",
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
          "id": "ca4020bf5c7a412bd9b8f7849d96ef270455e33d",
          "message": "Collator: Add traditional spanish and plumbing to make it work (#2497)",
          "timestamp": "2022-09-02T15:18:12-07:00",
          "tree_id": "add0eb3a06be48e4921cba0f853378f4720fd414",
          "url": "https://github.com/unicode-org/icu4x/commit/ca4020bf5c7a412bd9b8f7849d96ef270455e33d"
        },
        "date": 1662157851382,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 795,
            "range": "± 70",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 572,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 608,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 694,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1015,
            "range": "± 54",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1093,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 568,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 607,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 722,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 732,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1098,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 53849424,
            "range": "± 3463547",
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
          "id": "aa7480a6869214956cae821e4a7a385a7df1351d",
          "message": "Change IO in icu_codepointtrie_builder (#2503)",
          "timestamp": "2022-09-05T12:49:52+02:00",
          "tree_id": "f210dd5c6c199f6177673f820047b00ebd7a6463",
          "url": "https://github.com/unicode-org/icu4x/commit/aa7480a6869214956cae821e4a7a385a7df1351d"
        },
        "date": 1662375693098,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 558,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 476,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 444,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 507,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 818,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 797,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 418,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 506,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 578,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 536,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 806,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 38253016,
            "range": "± 2819417",
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
          "id": "a03ec6f259bf2256ffcddca1a20589f9c59d38d8",
          "message": "Revert \"Add a remark about inversion list  worst-case performance (#2388)\" (#2515)\n\nThis reverts commit 5a6471da1068173ad9172d0f9e2d15ee5b73063f.",
          "timestamp": "2022-09-05T14:04:19+03:00",
          "tree_id": "008c28de29f4dfb5c5d981aa76080e4c11bad50f",
          "url": "https://github.com/unicode-org/icu4x/commit/a03ec6f259bf2256ffcddca1a20589f9c59d38d8"
        },
        "date": 1662376711213,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 794,
            "range": "± 60",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 572,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 609,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 696,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 984,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1098,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 572,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 608,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 696,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 733,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1129,
            "range": "± 73",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 52367568,
            "range": "± 487211",
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
            "email": "younies.mahmoud@gmail.com",
            "name": "Younies Mahmoud",
            "username": "younies"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "0b85d15e625fcd37f768b9b5e0d1d7df62e9c9bf",
          "message": "Fix indexing_slicing clippy issue. (#2493)\n\nFix indexing_slicing clippy issues.",
          "timestamp": "2022-09-05T15:04:46+02:00",
          "tree_id": "c7a609b255f1a972b2fe5f38a890354a28bd1352",
          "url": "https://github.com/unicode-org/icu4x/commit/0b85d15e625fcd37f768b9b5e0d1d7df62e9c9bf"
        },
        "date": 1662383871963,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 674,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 497,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 542,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 614,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1064,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1086,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 500,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 539,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 610,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 640,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1098,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 44228778,
            "range": "± 1481522",
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
          "id": "4e9ba58d0288c90a5aa04b2982af4eb2aa476f4b",
          "message": "Fix typo (#2516)",
          "timestamp": "2022-09-05T15:40:03+02:00",
          "tree_id": "a733e96f614841f4dee9d35d94f225f2f8721da4",
          "url": "https://github.com/unicode-org/icu4x/commit/4e9ba58d0288c90a5aa04b2982af4eb2aa476f4b"
        },
        "date": 1662385943768,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 643,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 476,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 506,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 580,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 819,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 914,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 476,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 446,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 511,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 538,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 913,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 43359041,
            "range": "± 193248",
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
          "id": "03f8e10dbaa7202df390c2457eb03d53d59097e1",
          "message": "Remove auto-deserialize implementations from {Blob,Static,Fs}DataProvider and restructure testdata (#2364)",
          "timestamp": "2022-09-06T14:36:42-07:00",
          "tree_id": "2ae47ece70256e7ab69ce0f38c975a0ad9a80e80",
          "url": "https://github.com/unicode-org/icu4x/commit/03f8e10dbaa7202df390c2457eb03d53d59097e1"
        },
        "date": 1662500906132,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 567,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 476,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 447,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 510,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 722,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 914,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 419,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 446,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 511,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 538,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 806,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 38230275,
            "range": "± 2442902",
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
          "id": "d4c5f6d61686dc35d163d18c4413345d89bb8ef7",
          "message": "Add canary tests for `core::slice::from_raw_parts`'s constness (#2521)",
          "timestamp": "2022-09-07T01:12:18+02:00",
          "tree_id": "b3962c9a0748c8617841e0d20ecf307e5e58586e",
          "url": "https://github.com/unicode-org/icu4x/commit/d4c5f6d61686dc35d163d18c4413345d89bb8ef7"
        },
        "date": 1662506677172,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 643,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 476,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 506,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 579,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 819,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 913,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 476,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 506,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 579,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 610,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 914,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 43533884,
            "range": "± 106383",
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
          "id": "3651ac087dbe1ec49d590c67592b6be0c9183153",
          "message": "Hide fields of certain non-provider library structs (#2524)",
          "timestamp": "2022-09-06T22:07:42-05:00",
          "tree_id": "4bf13796f8338c21e124ed4ada0827eaa7aaba3e",
          "url": "https://github.com/unicode-org/icu4x/commit/3651ac087dbe1ec49d590c67592b6be0c9183153"
        },
        "date": 1662520847046,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 678,
            "range": "± 100",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 504,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 550,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 624,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1071,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1151,
            "range": "± 125",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 504,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 545,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 618,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 639,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1094,
            "range": "± 69",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 45517814,
            "range": "± 2227112",
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
          "id": "5cf865d5047d02b78496427da9a50fc9745ed591",
          "message": "Adding `Display` for our `Writeable` types (#2518)",
          "timestamp": "2022-09-07T18:37:07+02:00",
          "tree_id": "e85100ffd3efb400e06607bb429bba5cc96da09b",
          "url": "https://github.com/unicode-org/icu4x/commit/5cf865d5047d02b78496427da9a50fc9745ed591"
        },
        "date": 1662569359191,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 588,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 353,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 371,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 429,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 781,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 805,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 354,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 371,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 426,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 461,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 805,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 54370310,
            "range": "± 961755",
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
          "id": "feaa5c01d4e2cfb180343bd7697eedb350ac154f",
          "message": "Add FFI bindings for Collator (#2498)",
          "timestamp": "2022-09-07T18:57:10Z",
          "tree_id": "882764af47263c6400baa5cbb3090d69fc8f2760",
          "url": "https://github.com/unicode-org/icu4x/commit/feaa5c01d4e2cfb180343bd7697eedb350ac154f"
        },
        "date": 1662577989976,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 624,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 475,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 510,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 573,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 988,
            "range": "± 55",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1059,
            "range": "± 73",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 462,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 509,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 582,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 630,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 996,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 42339822,
            "range": "± 2382676",
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
          "id": "4f1ee6951068465fc81c8d5eb1ad9fadb794f2b4",
          "message": "Add UnvalidatedStr and use it in LocaleFallbackParentsV1 (#2502)",
          "timestamp": "2022-09-07T13:58:05-07:00",
          "tree_id": "21e671c883579e419f048453438a4c7a8db6f565",
          "url": "https://github.com/unicode-org/icu4x/commit/4f1ee6951068465fc81c8d5eb1ad9fadb794f2b4"
        },
        "date": 1662585047732,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 772,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 571,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 632,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 696,
            "range": "± 19",
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
            "value": 1097,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 571,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 608,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 695,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 732,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1097,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 52143680,
            "range": "± 219365",
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
          "id": "203ba870c401d5167c16eb550842f921199e9f35",
          "message": "icu::locid_transform rustlink and cleanup (#2523)",
          "timestamp": "2022-09-07T15:15:01-07:00",
          "tree_id": "6bd61654e524c145faebd420b42fe952ae6b37b1",
          "url": "https://github.com/unicode-org/icu4x/commit/203ba870c401d5167c16eb550842f921199e9f35"
        },
        "date": 1662589689305,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 585,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 354,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 370,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 426,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 781,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 806,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 352,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 371,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 424,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 462,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 806,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 54212711,
            "range": "± 909129",
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
            "email": "kelebra20@gmail.com",
            "name": "Oleksii Tkachuk",
            "username": "kelebra"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "84c0ef10caeca73e5df07aa688abd7f320ba0563",
          "message": "Fix feature specification in provider/fs (#2527)",
          "timestamp": "2022-09-08T01:09:59+02:00",
          "tree_id": "30fcdeed845790f3c91da349cf0dc0b1732dc5ec",
          "url": "https://github.com/unicode-org/icu4x/commit/84c0ef10caeca73e5df07aa688abd7f320ba0563"
        },
        "date": 1662593262419,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 692,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 509,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 553,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 625,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1087,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1111,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 509,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 554,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 629,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 651,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1203,
            "range": "± 63",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 45463701,
            "range": "± 599689",
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
            "email": "sipasseuth.daniel@gmail.com",
            "name": "Sipasseuth Daniel",
            "username": "dsipasseuth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5a668e0bb8fe34472585f58c1eb1eb63913e041c",
          "message": "Renaming functions to match agreed conventions (#2460)",
          "timestamp": "2022-09-08T01:30:19+02:00",
          "tree_id": "0e5323b4db463b06becbc9f7fd0be2585efa5924",
          "url": "https://github.com/unicode-org/icu4x/commit/5a668e0bb8fe34472585f58c1eb1eb63913e041c"
        },
        "date": 1662594186561,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 654,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 552,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 553,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 679,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 913,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1044,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 526,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 575,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 662,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 715,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1046,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 51061720,
            "range": "± 1356339",
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
          "id": "0dab3bdc068f2d99e71de7b964e71e2b3c52f64f",
          "message": "Rename `write_len` (#2529)",
          "timestamp": "2022-09-08T10:34:17+02:00",
          "tree_id": "d5397a64a464a11da079339901239d4d86cab0c8",
          "url": "https://github.com/unicode-org/icu4x/commit/0dab3bdc068f2d99e71de7b964e71e2b3c52f64f"
        },
        "date": 1662626838238,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 643,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 476,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 506,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 579,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 819,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 914,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 476,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 506,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 579,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 610,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 914,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 43532938,
            "range": "± 109351",
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
            "email": "kelebra20@gmail.com",
            "name": "Oleksii Tkachuk",
            "username": "kelebra"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "90235ae3c77535bc82adc5896a4e56f0027235fa",
          "message": "Refactor from for loop append to extend (#2531)",
          "timestamp": "2022-09-08T13:37:08+03:00",
          "tree_id": "0f6c685d47479572f3db710b4efc220c7eb98e33",
          "url": "https://github.com/unicode-org/icu4x/commit/90235ae3c77535bc82adc5896a4e56f0027235fa"
        },
        "date": 1662634256852,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 772,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 571,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 608,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 696,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1011,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1098,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 572,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 608,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 696,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 732,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1097,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 52194654,
            "range": "± 171769",
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
          "id": "ae78458675e84d4b8682f139fadbe107a0695972",
          "message": "Configurable DataProvider FFI (#2526)",
          "timestamp": "2022-09-08T16:18:58-07:00",
          "tree_id": "43f0b84a7ab903410e4eb8172c77313524e37250",
          "url": "https://github.com/unicode-org/icu4x/commit/ae78458675e84d4b8682f139fadbe107a0695972"
        },
        "date": 1662679950535,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 643,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 476,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 506,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 579,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 819,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 914,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 476,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 506,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 579,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 610,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 914,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 43627900,
            "range": "± 320554",
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
          "id": "d1b3e0fdb671f4de8f68ecf341a01eeb2c6d1b5a",
          "message": "Fix Segmenter's missing FFI (#2533)\n\n* Fix Segmenter's missing FFI\r\n\r\nChange `icu_segmenter` to `icu::segmenter` in segmenter_grapheme.rs. This\r\nremoves `segment_latin`, `segment_str`, and `segment_utf16` from the\r\nmissing_apis.txt.\r\n\r\nAdd all the types and types aliases that need no FFI to the IGNORED_PATHS.",
          "timestamp": "2022-09-08T16:48:52-07:00",
          "tree_id": "54ea23e7764e55900817419433dac20b3e5dfd43",
          "url": "https://github.com/unicode-org/icu4x/commit/d1b3e0fdb671f4de8f68ecf341a01eeb2c6d1b5a"
        },
        "date": 1662681683980,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 643,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 476,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 507,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 579,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 819,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 914,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 476,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 506,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 579,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 610,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 914,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 43620211,
            "range": "± 135053",
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
            "email": "m_kato@ga2.so-net.ne.jp",
            "name": "Makoto Kato",
            "username": "makotokato"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "1979ec419dccbd5089bf20763002886bed123937",
          "message": "Remove heapless feature from serde-json-core. (#2530)",
          "timestamp": "2022-09-09T14:36:09+02:00",
          "tree_id": "5619f2757115cfa6e0554dd8931ffcd0cd673112",
          "url": "https://github.com/unicode-org/icu4x/commit/1979ec419dccbd5089bf20763002886bed123937"
        },
        "date": 1662727752940,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 765,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 567,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 606,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 693,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 980,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1097,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 568,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 604,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 688,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 723,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1084,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 51704456,
            "range": "± 813939",
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
          "id": "79da612943fd8d18b06836c390b97b9e4c226599",
          "message": "Filter out more data provider traits and fix a rust link (#2538)",
          "timestamp": "2022-09-09T10:59:27-07:00",
          "tree_id": "ebed317d24c3ab25cbd04f755fe55d50a3961d65",
          "url": "https://github.com/unicode-org/icu4x/commit/79da612943fd8d18b06836c390b97b9e4c226599"
        },
        "date": 1662747067055,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 643,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 476,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 506,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 580,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 819,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 914,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 476,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 506,
            "range": "± 0",
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
            "value": 610,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 806,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 38471320,
            "range": "± 94228",
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
          "id": "dbe360cc7a724d048f601c2339c8ba7eb4142184",
          "message": "Add experimental notices on all exported modules, structs, and enums (#2536)",
          "timestamp": "2022-09-09T10:59:50-07:00",
          "tree_id": "b34e1be3d2310844b7255211d75e0e1b23bd9b1a",
          "url": "https://github.com/unicode-org/icu4x/commit/dbe360cc7a724d048f601c2339c8ba7eb4142184"
        },
        "date": 1662747076727,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 643,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 477,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 507,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 579,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 819,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 914,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 476,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 506,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 579,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 610,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 916,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 43520650,
            "range": "± 105089",
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
          "id": "fcbd5aab858b52fb4a887dbb968afab06083f85a",
          "message": "Use GIGO with debug assertion in Char16Trie (#2537)",
          "timestamp": "2022-09-09T23:19:53Z",
          "tree_id": "8890744face6945df216346020d46cc6cb1948a8",
          "url": "https://github.com/unicode-org/icu4x/commit/fcbd5aab858b52fb4a887dbb968afab06083f85a"
        },
        "date": 1662766262424,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 584,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 355,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 374,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 426,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 781,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 805,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 355,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 370,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 426,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 461,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 806,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 54158132,
            "range": "± 813319",
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
          "id": "aeaa9d1ef2fdb423eeb690468da520868d2acc12",
          "message": "Add normalizer FFI (#2542)\n\n* Add normalizer FFI\r\n\r\n* Add normalizer props",
          "timestamp": "2022-09-09T23:18:36Z",
          "tree_id": "ca52abb24fbeb15bab6151e6d28720b8fa98500d",
          "url": "https://github.com/unicode-org/icu4x/commit/aeaa9d1ef2fdb423eeb690468da520868d2acc12"
        },
        "date": 1662766319121,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 707,
            "range": "± 65",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 493,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 525,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 620,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1071,
            "range": "± 57",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1098,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 486,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 522,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 621,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 632,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1096,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 50018612,
            "range": "± 2124055",
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
          "id": "fe331719cbdb2b050b4f67227103f6b0d013d560",
          "message": "Sensibly handle invalid strings across FFI (#2534)\n\n* Move identifier/basic parsing FFI APIs over to using bytes\r\n\r\n* Fix collator APIs\r\n\r\n* Refactor handle_complex_language to be shareable\r\n\r\n* use in segmenter APIs\r\n\r\n* Update segmenter ffi\r\n\r\n* Use Utf8CharIndices",
          "timestamp": "2022-09-09T23:21:26Z",
          "tree_id": "c5f4f4e3f96467018779b5a543d19777d0575c48",
          "url": "https://github.com/unicode-org/icu4x/commit/fe331719cbdb2b050b4f67227103f6b0d013d560"
        },
        "date": 1662766464284,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 842,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 563,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 594,
            "range": "± 61",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 684,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1238,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1327,
            "range": "± 59",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 578,
            "range": "± 63",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 586,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 688,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 670,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1340,
            "range": "± 89",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 46387629,
            "range": "± 3678549",
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
          "id": "56504e048e7d78e83783aef0c789447fba59e84c",
          "message": "Fix rust links and finish FFI coverage for datetime and timezone (#2544)\n\n* Fix rust links and finish FFI coverage for datetime and timezone\r\n\r\n* Add basic ICU4XZonedDateTimeFormatter test",
          "timestamp": "2022-09-10T03:23:58Z",
          "tree_id": "5050e97374e9b10b3146620e92731b3da33491a4",
          "url": "https://github.com/unicode-org/icu4x/commit/56504e048e7d78e83783aef0c789447fba59e84c"
        },
        "date": 1662781019220,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 1024,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 576,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 647,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 743,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1283,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1364,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 574,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 647,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 744,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 697,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1396,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 47124813,
            "range": "± 978076",
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
          "id": "cf958a87745d6133e3c41e0dd863912ecea5eead",
          "message": "Add ForkByKeyProvider FFI (#2546)",
          "timestamp": "2022-09-12T13:33:55-07:00",
          "tree_id": "57e650ad55dd025e725ec4944bf139d452bf7340",
          "url": "https://github.com/unicode-org/icu4x/commit/cf958a87745d6133e3c41e0dd863912ecea5eead"
        },
        "date": 1663015509035,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 615,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 371,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 391,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 466,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 977,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 993,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 372,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 390,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 457,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 499,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 992,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 50227108,
            "range": "± 963050",
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
          "id": "5f984a8dc2de9791cb8db1d2a24934bc5cf7ab79",
          "message": "Whack-a-mole part 6 (#2547)\n\n* Fix collator links\r\n\r\n* blanket-ignore any/buffer provider ctors\r\n\r\n* collator, decimal, casemapping, bidi, calendar",
          "timestamp": "2022-09-12T21:15:22Z",
          "tree_id": "049c9954f4e66939999b682e207e8b394778f92f",
          "url": "https://github.com/unicode-org/icu4x/commit/5f984a8dc2de9791cb8db1d2a24934bc5cf7ab79"
        },
        "date": 1663018070240,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 767,
            "range": "± 61",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 511,
            "range": "± 51",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 532,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 633,
            "range": "± 66",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1108,
            "range": "± 77",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1303,
            "range": "± 136",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 529,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 505,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 604,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 615,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1204,
            "range": "± 91",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 43121146,
            "range": "± 2875642",
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
          "id": "53b5fd5658334e37c9f5b45c5d7e643999be310a",
          "message": "Add ListFormatter FFI (#2545)",
          "timestamp": "2022-09-12T16:29:13-07:00",
          "tree_id": "be58aa9147a3347e873dd90a980c017df98613ad",
          "url": "https://github.com/unicode-org/icu4x/commit/53b5fd5658334e37c9f5b45c5d7e643999be310a"
        },
        "date": 1663026050533,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 855,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 480,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 539,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 620,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1084,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1167,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 480,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 538,
            "range": "± 1",
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
            "value": 581,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1165,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 40565585,
            "range": "± 88300",
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
          "id": "b33c927488a65f98f26c08d060743d007a0349c8",
          "message": "LocaleFallbacker FFI (#2551)",
          "timestamp": "2022-09-12T17:53:59-07:00",
          "tree_id": "a6604c5891edf66608601958722224bd2d7c87d4",
          "url": "https://github.com/unicode-org/icu4x/commit/b33c927488a65f98f26c08d060743d007a0349c8"
        },
        "date": 1663031166907,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 936,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 509,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 594,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 658,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1200,
            "range": "± 51",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1322,
            "range": "± 56",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 500,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 558,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 654,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 613,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1219,
            "range": "± 74",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 42974869,
            "range": "± 2235043",
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
            "email": "m_kato@ga2.so-net.ne.jp",
            "name": "Makoto Kato",
            "username": "makotokato"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ef8d16e20482dd6823b9ded4c79bb86bacb953cc",
          "message": "Add C/C++ headers in the package (#2553)",
          "timestamp": "2022-09-13T02:30:36Z",
          "tree_id": "d4466b5ea8fdf80ccf5959af4dfb2994c23508f8",
          "url": "https://github.com/unicode-org/icu4x/commit/ef8d16e20482dd6823b9ded4c79bb86bacb953cc"
        },
        "date": 1663036992362,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 620,
            "range": "± 76",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 370,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 389,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 454,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 959,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 982,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 370,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 389,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 454,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 497,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 983,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 50101455,
            "range": "± 1536623",
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
          "id": "824f91bcf11b99465e916fedbedc5eb062d1e67d",
          "message": "Whack-a-mole 7: The last whack-a-mole? (#2552)\n\n* Some locale FFI\r\n\r\n* Add scriptextensions\r\n\r\n* ignore PluralOperands::n()\r\n\r\n* char16trie\r\n\r\n* comments\r\n\r\n* Update ffi/diplomat/ffi_coverage/src/main.rs\r\n\r\nCo-authored-by: Shane F. Carr <shane@unicode.org>\r\n\r\nCo-authored-by: Shane F. Carr <shane@unicode.org>",
          "timestamp": "2022-09-13T19:18:41Z",
          "tree_id": "e5271527550ea81df34ca712a45fdfdb8a9b8dbd",
          "url": "https://github.com/unicode-org/icu4x/commit/824f91bcf11b99465e916fedbedc5eb062d1e67d"
        },
        "date": 1663097471153,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 710,
            "range": "± 86",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 455,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 452,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 576,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1077,
            "range": "± 107",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1159,
            "range": "± 72",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 447,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 488,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 578,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 589,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1102,
            "range": "± 104",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 37542755,
            "range": "± 3671147",
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
          "id": "efcc000be1f2ce08111b72760d6e6711cea29d4b",
          "message": "Better API for ScriptWithExtensions (#2555)",
          "timestamp": "2022-09-14T17:09:18-07:00",
          "tree_id": "7923a536fc3304ada17fcd2b70a884823277d1ba",
          "url": "https://github.com/unicode-org/icu4x/commit/efcc000be1f2ce08111b72760d6e6711cea29d4b"
        },
        "date": 1663201370867,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 891,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 586,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 570,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 696,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1255,
            "range": "± 57",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1363,
            "range": "± 63",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 568,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 572,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 687,
            "range": "± 72",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 668,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1332,
            "range": "± 189",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 47364215,
            "range": "± 2203421",
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
            "email": "srl295@gmail.com",
            "name": "Steven R. Loomis",
            "username": "srl295"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "e88803860b3c07db8bc60c8f8118ca1ad652256b",
          "message": "chore(license): fix license in collections/Cargo.toml, others (#2562)\n\n- also the two fuzz components didn’t have a license\r\n\r\nFixes #2559",
          "timestamp": "2022-09-15T20:51:19Z",
          "tree_id": "ecc6e31febe8380c6d0856080fe45de966135f1f",
          "url": "https://github.com/unicode-org/icu4x/commit/e88803860b3c07db8bc60c8f8118ca1ad652256b"
        },
        "date": 1663275888139,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 960,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 573,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 629,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 684,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1219,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1369,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 572,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 638,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 725,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 687,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1373,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 47805761,
            "range": "± 731163",
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
          "id": "ce24346f1a1a64e615b17ce508c7e2f10a3706f2",
          "message": "Finish off icu_provider_adapters FFI (#2561)",
          "timestamp": "2022-09-15T15:26:31-07:00",
          "tree_id": "40df2bcb17f7e0163f2b4ed285cee0d181549542",
          "url": "https://github.com/unicode-org/icu4x/commit/ce24346f1a1a64e615b17ce508c7e2f10a3706f2"
        },
        "date": 1663281620113,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 858,
            "range": "± 88",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 554,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 590,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 681,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1231,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1338,
            "range": "± 56",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 547,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 579,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 687,
            "range": "± 94",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 665,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1320,
            "range": "± 51",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 44678447,
            "range": "± 2405768",
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
          "id": "add4a2b9256ab5905bde14902708b5191fcc2f7c",
          "message": "Add some useful data provider impls; refactor AnyPayloadProvider (#2564)",
          "timestamp": "2022-09-15T15:29:36-07:00",
          "tree_id": "4e3c5e73b02c0418c2622cb286c56522044b4ac8",
          "url": "https://github.com/unicode-org/icu4x/commit/add4a2b9256ab5905bde14902708b5191fcc2f7c"
        },
        "date": 1663281855065,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 756,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 423,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 487,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 547,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 957,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1031,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 423,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 477,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 547,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 512,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1031,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 35916298,
            "range": "± 3289367",
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
          "id": "db24f67aea54b9360a7004295fa2ec9201a39686",
          "message": "Rename DataKey methods and return DataKeyPath (#2565)",
          "timestamp": "2022-09-15T16:45:31-07:00",
          "tree_id": "5d5e977e99d6455444ab79101a680cccd53555d2",
          "url": "https://github.com/unicode-org/icu4x/commit/db24f67aea54b9360a7004295fa2ec9201a39686"
        },
        "date": 1663286335357,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 992,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 542,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 618,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 710,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1233,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1328,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 545,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 630,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 693,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 658,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1339,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 47060949,
            "range": "± 1239975",
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
          "id": "ccad8fa063b734bfc75cbfbea0add57ec561c908",
          "message": "Metazone and Time Zone renaming (#2569)",
          "timestamp": "2022-09-15T21:17:17-07:00",
          "tree_id": "c5ca432a51e6f052206061e000241a7dd0335947",
          "url": "https://github.com/unicode-org/icu4x/commit/ccad8fa063b734bfc75cbfbea0add57ec561c908"
        },
        "date": 1663302515463,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 611,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 370,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 389,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 458,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 954,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 980,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 374,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 389,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 454,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 500,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 984,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 50156930,
            "range": "± 1103606",
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
            "email": "kelebra20@gmail.com",
            "name": "Oleksii Tkachuk",
            "username": "kelebra"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "9b1bc61c69752892c33594011751c0df522ab198",
          "message": "Remove more for push usages in favor of extend (#2539)",
          "timestamp": "2022-09-16T18:10:10+02:00",
          "tree_id": "3b5d4b1c3bc6dc9f2cf67beca796f5165a4d555d",
          "url": "https://github.com/unicode-org/icu4x/commit/9b1bc61c69752892c33594011751c0df522ab198"
        },
        "date": 1663345504951,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 800,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 517,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 564,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 661,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1151,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1249,
            "range": "± 73",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 538,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 541,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 632,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 625,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1222,
            "range": "± 85",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 41368821,
            "range": "± 2461895",
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
          "id": "c25e0e47b4ae3a9379734ab60b492df31ec90228",
          "message": "Add boilerplate to segmenter (#2566)\n\n* Update boilerplate for segmenter\r\n\r\n* Enable clippy::exhaustive_enums\r\n\r\nMark both enum non_exhaustive in case that CSSWG adds new values to the\r\ncorresponding CSS properties in the future.\r\n\r\n* Enable clippy::exhaustive_structs\r\n\r\nIgnore clippy::exhaustive_structs, clippy::exhaustive_enums in segmenter's\r\nprovider mod like other icu4x components.",
          "timestamp": "2022-09-16T15:39:34-07:00",
          "tree_id": "df9f312a6f15f4cd4804bc97a1f405cdebf5b0b1",
          "url": "https://github.com/unicode-org/icu4x/commit/c25e0e47b4ae3a9379734ab60b492df31ec90228"
        },
        "date": 1663368830325,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 834,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 546,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 575,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 670,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1217,
            "range": "± 60",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1330,
            "range": "± 56",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 545,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 573,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 672,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 655,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1304,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 43737499,
            "range": "± 1072158",
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
          "id": "9ecc55e1134420428dd0553092562ea3a85e96a7",
          "message": "Remove `format_to_write`s (#2528)",
          "timestamp": "2022-09-17T09:27:59+02:00",
          "tree_id": "223bfcdbd97a029cd142bdd992eed1141813e434",
          "url": "https://github.com/unicode-org/icu4x/commit/9ecc55e1134420428dd0553092562ea3a85e96a7"
        },
        "date": 1663400537942,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 793,
            "range": "± 62",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 549,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 576,
            "range": "± 56",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 678,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1222,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1307,
            "range": "± 82",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 512,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 547,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 639,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 661,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1326,
            "range": "± 57",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 44332417,
            "range": "± 2937968",
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
            "email": "dsipasseuth@google.com",
            "name": "Sipasseuth Daniel",
            "username": "dsipasseuth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "a2f18db5f482ec62d853b15aa3e94557bb6a69df",
          "message": "Modified doc examples to use icu::* instead of icu_* (#2541)",
          "timestamp": "2022-09-17T09:36:05+02:00",
          "tree_id": "294b3107c70228a8252d255dd5c749c5177c286d",
          "url": "https://github.com/unicode-org/icu4x/commit/a2f18db5f482ec62d853b15aa3e94557bb6a69df"
        },
        "date": 1663400930275,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 763,
            "range": "± 48",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 509,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 532,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 615,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1085,
            "range": "± 60",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1218,
            "range": "± 79",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 494,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 503,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 615,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 579,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1172,
            "range": "± 99",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 42380443,
            "range": "± 2230809",
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
          "id": "a89f542d698bb96fbfd5d5f9064975a8de05404e",
          "message": "Add ExactSizeIterator (#2580)",
          "timestamp": "2022-09-19T15:59:14+02:00",
          "tree_id": "f64560b0a743098963ff15a2ae2309e0e5acf881",
          "url": "https://github.com/unicode-org/icu4x/commit/a89f542d698bb96fbfd5d5f9064975a8de05404e"
        },
        "date": 1663596772725,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 817,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 540,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 569,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 660,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1233,
            "range": "± 235",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1285,
            "range": "± 57",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 557,
            "range": "± 66",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 551,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 654,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 599,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1215,
            "range": "± 57",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 41121447,
            "range": "± 2064408",
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
            "email": "m_kato@ga2.so-net.ne.jp",
            "name": "Makoto Kato",
            "username": "makotokato"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "3cfc9cdb1e186b14aeeeef8d180237232f510c2c",
          "message": "Avoid testdata dependency from icu_capi (#2578)",
          "timestamp": "2022-09-19T16:00:07+02:00",
          "tree_id": "48e4c25083bf77d8d567c07457b75620b96faf84",
          "url": "https://github.com/unicode-org/icu4x/commit/3cfc9cdb1e186b14aeeeef8d180237232f510c2c"
        },
        "date": 1663596792240,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 611,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 369,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 390,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 454,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 961,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 982,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 370,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 391,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 454,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 498,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 982,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 50125716,
            "range": "± 541341",
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
          "id": "4ee18e9da5834190f7c5648fd3c7e9504541951c",
          "message": "Rename postcard feature for 1.0 (#2584)",
          "timestamp": "2022-09-20T16:46:26+02:00",
          "tree_id": "288cd054bc8cdc2d847385de8da3c7394288bf3d",
          "url": "https://github.com/unicode-org/icu4x/commit/4ee18e9da5834190f7c5648fd3c7e9504541951c"
        },
        "date": 1663685994566,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 611,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 370,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 390,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 455,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 961,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 987,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 370,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 389,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 454,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 504,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 986,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 50454505,
            "range": "± 767602",
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
          "id": "9cb743f8445210db627ba8effd7da6f5ec1312d1",
          "message": "Remove metadata from path and add FallbackSupplement (#2567)",
          "timestamp": "2022-09-20T10:24:03-05:00",
          "tree_id": "03ada325295547c8f11ab8edb7549e92a51de418",
          "url": "https://github.com/unicode-org/icu4x/commit/9cb743f8445210db627ba8effd7da6f5ec1312d1"
        },
        "date": 1663688179366,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 855,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 480,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 539,
            "range": "± 2",
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
            "value": 1084,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1168,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 480,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 539,
            "range": "± 1",
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
            "value": 581,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1168,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 40484903,
            "range": "± 74517",
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
          "id": "16c90be4247f0d35852ee6cd55c63e5c4c2680a6",
          "message": "Add NicheBytes trait and NichedOptionULE (#2501)\n\n* Add NicheBytes trait, NichedOption, NichedOptionULE\r\n\r\n* Add ULE types for NonZeroU8, NonZeroI8",
          "timestamp": "2022-09-20T20:27:15Z",
          "tree_id": "7d23cd5982d146c6d09390f0cd7b0f4199bf2856",
          "url": "https://github.com/unicode-org/icu4x/commit/16c90be4247f0d35852ee6cd55c63e5c4c2680a6"
        },
        "date": 1663706360549,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 855,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 480,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 538,
            "range": "± 1",
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
            "value": 1084,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1168,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 480,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 539,
            "range": "± 2",
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
            "value": 581,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1166,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 40599178,
            "range": "± 192987",
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
          "id": "e8a90e4b8273e098f75a0dc6d0510f9c32bafb13",
          "message": "Remove re-export of FormattedFixedDecimal (#2595)",
          "timestamp": "2022-09-20T15:28:35-07:00",
          "tree_id": "a440a26677300ff041752db12328f491e742f6a3",
          "url": "https://github.com/unicode-org/icu4x/commit/e8a90e4b8273e098f75a0dc6d0510f9c32bafb13"
        },
        "date": 1663713877679,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 839,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 545,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 573,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 675,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1220,
            "range": "± 68",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1312,
            "range": "± 112",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 547,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 579,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 675,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 656,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1319,
            "range": "± 100",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 43989507,
            "range": "± 1422824",
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
          "id": "a2efa18f7b05c00e9947044e4ceaf86af433f43b",
          "message": "DateTime: Change default length to medium and add tests for Default (#2596)",
          "timestamp": "2022-09-20T15:28:52-07:00",
          "tree_id": "a9a4745a6105dcf6c90c6fa03484d92f2a11486a",
          "url": "https://github.com/unicode-org/icu4x/commit/a2efa18f7b05c00e9947044e4ceaf86af433f43b"
        },
        "date": 1663714096156,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 856,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 480,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 540,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 620,
            "range": "± 5",
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
            "value": 1168,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 480,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 539,
            "range": "± 1",
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
            "value": 581,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1166,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 40628582,
            "range": "± 98266",
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
          "id": "7bffd2fd4299b98f2a1c614587569d2f622ba152",
          "message": "Improve docs for enum properties without changing API (#2597)",
          "timestamp": "2022-09-20T16:36:50-07:00",
          "tree_id": "b458a78994e710f86bf87fcc5f72d5fc1054c9f9",
          "url": "https://github.com/unicode-org/icu4x/commit/7bffd2fd4299b98f2a1c614587569d2f622ba152"
        },
        "date": 1663717887816,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 832,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 539,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 579,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 679,
            "range": "± 53",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1227,
            "range": "± 60",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1312,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 545,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 584,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 681,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 657,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1308,
            "range": "± 48",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 43970646,
            "range": "± 1440824",
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
          "id": "871bcd5d744bcf9ec752df7f84f4be058817b9c8",
          "message": "Normalizer: Mark function as experimental and fix feature (#2594)",
          "timestamp": "2022-09-20T23:13:35-07:00",
          "tree_id": "1b1438afb385f7ea5490a76f6470260749091689",
          "url": "https://github.com/unicode-org/icu4x/commit/871bcd5d744bcf9ec752df7f84f4be058817b9c8"
        },
        "date": 1663741638123,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 976,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 548,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 602,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 715,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1234,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1342,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 559,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 624,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 704,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 654,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1330,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 46264733,
            "range": "± 964873",
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
          "id": "fbd829c8ad0094db44af41a1b32cea38ee540c5b",
          "message": "Add permutation to ZVL containers (#2605)",
          "timestamp": "2022-09-21T14:08:10+02:00",
          "tree_id": "c3352c9888f60000fa33675e0a186e66034ba241",
          "url": "https://github.com/unicode-org/icu4x/commit/fbd829c8ad0094db44af41a1b32cea38ee540c5b"
        },
        "date": 1663762887962,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 856,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 582,
            "range": "± 83",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 581,
            "range": "± 54",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 678,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 1218,
            "range": "± 60",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1300,
            "range": "± 62",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 556,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 559,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 688,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 654,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1314,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 45173446,
            "range": "± 1316308",
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
          "id": "f6fe4c49e6e89f03c9372d78f67065c4784690bd",
          "message": "Rename FFI constructors (#2601)",
          "timestamp": "2022-09-21T08:52:03-07:00",
          "tree_id": "c1d8d970cb69954a23874673190f01c8aff1a518",
          "url": "https://github.com/unicode-org/icu4x/commit/f6fe4c49e6e89f03c9372d78f67065c4784690bd"
        },
        "date": 1663776229364,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 607,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 370,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 389,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 457,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 961,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 988,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 370,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 390,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 455,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 500,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 984,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 50225953,
            "range": "± 778651",
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
          "id": "f53a87eaed760170e328dffb8705bcdda41657ea",
          "message": "Less building in CI (#2606)",
          "timestamp": "2022-09-21T18:07:56+02:00",
          "tree_id": "eeef469c9992c6fe65fb9f1135532852521a6a77",
          "url": "https://github.com/unicode-org/icu4x/commit/f53a87eaed760170e328dffb8705bcdda41657ea"
        },
        "date": 1663777257936,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 855,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 423,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 540,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 547,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 957,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1031,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 424,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 476,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 547,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 513,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1031,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 40644376,
            "range": "± 215197",
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
          "id": "0e395d9c03f8d98bd3c37a0bba7bced4d5511529",
          "message": "Turn ZeroVec into a struct (#2599)\n\n* Add alternate owned/borrowed ZeroVec methods\n\n* Use everywhere\n\n* Replace with internal pointer\n\n* fix\n\n* fix\n\n* rm old docs\n\n* clip\n\n* fix\n\n* oops\n\n* Add to_mut_slice()\n\n* fix\n\n* merge fix",
          "timestamp": "2022-09-21T11:36:04-07:00",
          "tree_id": "3cc1befc84d22a4b67f1f3fcd3b3b356f0c74d8d",
          "url": "https://github.com/unicode-org/icu4x/commit/0e395d9c03f8d98bd3c37a0bba7bced4d5511529"
        },
        "date": 1663786150344,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 459,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 298,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 321,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 371,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 611,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 648,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 299,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 321,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 371,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 413,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 648,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 54230039,
            "range": "± 809461",
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
          "id": "35ca5455a371af3124e5683c367846f9e6fbbcbc",
          "message": "Move `decimal-bn-en.postcard` (#2607)",
          "timestamp": "2022-09-21T21:20:43+02:00",
          "tree_id": "05d9d48b2562fe66a6471479234a4c48f9181a09",
          "url": "https://github.com/unicode-org/icu4x/commit/35ca5455a371af3124e5683c367846f9e6fbbcbc"
        },
        "date": 1663788789382,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 448,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 319,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 323,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 369,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 647,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 743,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 319,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 333,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 369,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 422,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 742,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 42984135,
            "range": "± 134670",
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
          "id": "abf9d5b3d09cb4bae27d136f873d6c99fab97f96",
          "message": "Removing `StaticDataProvider` (#2582)",
          "timestamp": "2022-09-21T23:08:50+02:00",
          "tree_id": "0402eb060932fbcd18f313bfb337002ed139f527",
          "url": "https://github.com/unicode-org/icu4x/commit/abf9d5b3d09cb4bae27d136f873d6c99fab97f96"
        },
        "date": 1663795403207,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 615,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 479,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 506,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 503,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 919,
            "range": "± 57",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 971,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 481,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 503,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 502,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 568,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 967,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 44935189,
            "range": "± 1179504",
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
          "id": "ab02aade03ce68ec53f90fe4eeb140e5e97b2e01",
          "message": "fix ci (#2610)",
          "timestamp": "2022-09-22T12:30:15-05:00",
          "tree_id": "2d864525020adaddf86f9df5d17fc818bf01e12a",
          "url": "https://github.com/unicode-org/icu4x/commit/ab02aade03ce68ec53f90fe4eeb140e5e97b2e01"
        },
        "date": 1663868675464,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 587,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 475,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 489,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 506,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 850,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 927,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 469,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 478,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 497,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 531,
            "range": "± 68",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 932,
            "range": "± 84",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 48732236,
            "range": "± 1632429",
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
            "email": "srl295@gmail.com",
            "name": "Steven R. Loomis",
            "username": "srl295"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ecc0df29d672febd7ecaa3d2f09fe7b91e9efd61",
          "message": "Update CONTRIBUTING.md to mention the new CLA (#2468)\n\n* Update CONTRIBUTING.md to mention the new CLA\r\n\r\n* Update CONTRIBUTING.md\r\n\r\nlinkify the CLA",
          "timestamp": "2022-09-22T18:16:13Z",
          "tree_id": "cc44c53f532e327f53b08abcaa699b30a24a0d98",
          "url": "https://github.com/unicode-org/icu4x/commit/ecc0df29d672febd7ecaa3d2f09fe7b91e9efd61"
        },
        "date": 1663871301926,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 448,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 299,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 321,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 371,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 611,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 722,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 298,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 321,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 371,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 413,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 648,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 53990383,
            "range": "± 830560",
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
          "id": "a0dab7a3d29529739f5ac6c9f218fb894fe7acf4",
          "message": "Fix wasm demo (#2611)\n\n* Fix wasm demo\r\n\r\n* ethiopian -> ethiopian",
          "timestamp": "2022-09-22T21:20:18Z",
          "tree_id": "c08f81924a2ef3db5a13880d59d91fcddf378a3f",
          "url": "https://github.com/unicode-org/icu4x/commit/a0dab7a3d29529739f5ac6c9f218fb894fe7acf4"
        },
        "date": 1663882436576,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 530,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 381,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 400,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 439,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 783,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 867,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 382,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 398,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 441,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 525,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 869,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 51010545,
            "range": "± 521361",
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
          "id": "a9e2ba4d54db7a3718f7bb849941c6f871c1ecbc",
          "message": "Plurals: Use From instead of TryFrom for signed integers (#2593)",
          "timestamp": "2022-09-22T15:53:08-07:00",
          "tree_id": "3e626c1a0d6ca4adde662f9900de73af42dc829c",
          "url": "https://github.com/unicode-org/icu4x/commit/a9e2ba4d54db7a3718f7bb849941c6f871c1ecbc"
        },
        "date": 1663887917740,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 448,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 298,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 321,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 371,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 611,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 648,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 299,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 321,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 371,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 413,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 648,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 54105539,
            "range": "± 1200629",
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
          "id": "15eb279623daefef4c0fee077eb05af20340f6cd",
          "message": "Initial constructor documentation overhaul (#2573)",
          "timestamp": "2022-09-22T18:11:41-07:00",
          "tree_id": "236370aaf7004fc97dd267d72d9b6521a7ceb17f",
          "url": "https://github.com/unicode-org/icu4x/commit/15eb279623daefef4c0fee077eb05af20340f6cd"
        },
        "date": 1663896327774,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 524,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 420,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 439,
            "range": "± 79",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 438,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 801,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 890,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 425,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 436,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 437,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 504,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 928,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 39099840,
            "range": "± 1743341",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/best",
            "value": 1,
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
            "value": 1,
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
          "id": "42b574d447e952d201136478cafe76311c6245f7",
          "message": "More updates to datetime docs (#2550)",
          "timestamp": "2022-09-23T01:51:51Z",
          "tree_id": "39549412022fadd9bfba4c5d7e5fe6bfa2385f3d",
          "url": "https://github.com/unicode-org/icu4x/commit/42b574d447e952d201136478cafe76311c6245f7"
        },
        "date": 1663898654890,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 450,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 312,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 321,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 371,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 610,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 648,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 298,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 321,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 371,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 413,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 647,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 54057679,
            "range": "± 782480",
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
          "id": "ff1d4b370b834281e3524118fb41883341a7e2bd",
          "message": "Warn against breaking the Pernosco integration of CharacterAndClass (#2604)",
          "timestamp": "2022-09-23T09:11:25+03:00",
          "tree_id": "8b03fd8dc329ce1a6b4612a00bcd8a009a2710a3",
          "url": "https://github.com/unicode-org/icu4x/commit/ff1d4b370b834281e3524118fb41883341a7e2bd"
        },
        "date": 1663914218756,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 462,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 297,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 321,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 371,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 611,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 648,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 297,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 321,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 371,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 413,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 648,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 54146849,
            "range": "± 711784",
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
            "email": "younies.mahmoud@gmail.com",
            "name": "Younies Mahmoud",
            "username": "younies"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "531b59e7f50157f0b6d8242966d726dfe868e4a6",
          "message": "fixed_decimal::Sign should be exhaustive (#2620)",
          "timestamp": "2022-09-23T22:40:13+02:00",
          "tree_id": "78210eaaeb5fcb9c78356d565631b2525a9b083b",
          "url": "https://github.com/unicode-org/icu4x/commit/531b59e7f50157f0b6d8242966d726dfe868e4a6"
        },
        "date": 1663966347375,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 449,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 298,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 321,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 371,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 610,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 648,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 298,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 321,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 371,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 413,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 648,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 54151027,
            "range": "± 889553",
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
          "id": "4cb5ed130c8a7557c310a4fa0fa7ebf4d8e5270c",
          "message": "Add module declaration-level notes about special-casing in Pernosco (#2619)",
          "timestamp": "2022-09-23T22:44:56Z",
          "tree_id": "d341576f93e6094d54d81a320b8eac045a6907a5",
          "url": "https://github.com/unicode-org/icu4x/commit/4cb5ed130c8a7557c310a4fa0fa7ebf4d8e5270c"
        },
        "date": 1663974663330,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 443,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 319,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 332,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 368,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 628,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 727,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 319,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 323,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 368,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 421,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 728,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 42599813,
            "range": "± 407896",
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
            "email": "younies.mahmoud@gmail.com",
            "name": "Younies Mahmoud",
            "username": "younies"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c2e29bd30a695d032bc82fb8606e3f6439991613",
          "message": "Fixes the bidi documentation (#2621)\n\nFix bidi documentation",
          "timestamp": "2022-09-24T01:11:20+02:00",
          "tree_id": "85845f20eba917337c6c8a7704c365a330bcb182",
          "url": "https://github.com/unicode-org/icu4x/commit/c2e29bd30a695d032bc82fb8606e3f6439991613"
        },
        "date": 1663975496009,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 537,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 375,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 396,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 443,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 763,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 872,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 378,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 392,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 431,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 510,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 894,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 52014952,
            "range": "± 1798311",
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
          "id": "2ec486db756badb05f08e85818211437ee0fe6cc",
          "message": "Post-review zerovec fixes (#2622)\n\n* dtor\n\n* docs and tests\n\n* may_dangle\n\n* Add EyepatchHackVector\n\n* Update utils/zerovec/src/zerovec/mod.rs\n\nCo-authored-by: Shane F. Carr <shane@unicode.org>\n\n* review\n\nCo-authored-by: Shane F. Carr <shane@unicode.org>",
          "timestamp": "2022-09-23T18:17:00-07:00",
          "tree_id": "003a21d2688e02eb0a8bd46b04770afea6ace8a3",
          "url": "https://github.com/unicode-org/icu4x/commit/2ec486db756badb05f08e85818211437ee0fe6cc"
        },
        "date": 1663982931983,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 537,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 312,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 333,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 369,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 746,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 869,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 321,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 332,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 370,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 422,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 740,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 42643738,
            "range": "± 377342",
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
          "id": "15afab068be56984abfff0896054c32e17b884d3",
          "message": "Add FFI tutorials and tutorial landing page (#2624)\n\n* Fix example\n\n* icu_capi docs\n\n* Add cpp docs\n\n* Note tutorial in capi docs\n\n* fmt\n\n* add js doc\n\n* add js tutorial\n\n* add index docs\n\n* review\n\n* Update docs/tutorials/cpp.md\n\nCo-authored-by: Shane F. Carr <shane@unicode.org>\n\n* Update docs/tutorials/cpp.md\n\nCo-authored-by: Shane F. Carr <shane@unicode.org>\n\n* Update docs/tutorials/js.md\n\nCo-authored-by: Shane F. Carr <shane@unicode.org>\n\nCo-authored-by: Shane F. Carr <shane@unicode.org>",
          "timestamp": "2022-09-23T21:20:39-07:00",
          "tree_id": "9d2ecef046e277196cc9d8f238e71a6d37203996",
          "url": "https://github.com/unicode-org/icu4x/commit/15afab068be56984abfff0896054c32e17b884d3"
        },
        "date": 1663993969610,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 537,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 320,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 344,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 369,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 746,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 869,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 324,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 342,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 369,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 422,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 739,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 42960457,
            "range": "± 194461",
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
          "id": "e14f99bfcd725d72d85e00eb61c2749d5c16183b",
          "message": "from_tr35_string -> get_for_cldr_string (#2633)",
          "timestamp": "2022-09-24T10:32:06-07:00",
          "tree_id": "a6a6c4003f786a56f99a6d95bdb95cc1a519861a",
          "url": "https://github.com/unicode-org/icu4x/commit/e14f99bfcd725d72d85e00eb61c2749d5c16183b"
        },
        "date": 1664041450017,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 537,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 320,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 333,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 370,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 747,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 869,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 321,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 333,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 370,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 422,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 740,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 42878928,
            "range": "± 171528",
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
          "id": "463c3e6ad24c0764e49d80d06e418bac596d63d1",
          "message": "Propagate \"help choosing\" docs to all unstable constructors (#2634)",
          "timestamp": "2022-09-24T10:31:21-07:00",
          "tree_id": "e889a5ad59e4316dd91f513687a28541ce92a909",
          "url": "https://github.com/unicode-org/icu4x/commit/463c3e6ad24c0764e49d80d06e418bac596d63d1"
        },
        "date": 1664041518481,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 471,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 303,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 316,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 360,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 610,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 644,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 302,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 320,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 360,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 410,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 621,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 52526039,
            "range": "± 747382",
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
          "id": "139e795cc84df7c5adc166beff851052554716e4",
          "message": "Don't export utils functions from codepointinvlist (#2629)",
          "timestamp": "2022-09-24T22:13:13-07:00",
          "tree_id": "406baffbe78bda0d2b02df157b4ae2d29a6f36e9",
          "url": "https://github.com/unicode-org/icu4x/commit/139e795cc84df7c5adc166beff851052554716e4"
        },
        "date": 1664083724081,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 463,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 315,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 321,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 371,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 592,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 647,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 317,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 319,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 371,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 408,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 625,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 55281416,
            "range": "± 797895",
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
          "id": "ee45dc4f3e4cbee4a07f92020434d28e63959fe4",
          "message": "Fix \"help choosing a constructor\" docs (#2631)",
          "timestamp": "2022-09-24T22:12:27-07:00",
          "tree_id": "b865fcadcb08e8f06ad469d64fd407e1cbfdc73a",
          "url": "https://github.com/unicode-org/icu4x/commit/ee45dc4f3e4cbee4a07f92020434d28e63959fe4"
        },
        "date": 1664083793392,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 462,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 317,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 319,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 368,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 590,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 644,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 315,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 319,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 370,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 408,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 623,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 55196900,
            "range": "± 772593",
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
          "id": "b4279d35203e3771542471388697fbfd15896bd3",
          "message": "Add _with_length to TimeFormatter and [Typed]DateFormatter (#2617)",
          "timestamp": "2022-09-24T22:15:17-07:00",
          "tree_id": "72db3406581f4332e2fed7a01abe34c935f8acb4",
          "url": "https://github.com/unicode-org/icu4x/commit/b4279d35203e3771542471388697fbfd15896bd3"
        },
        "date": 1664083938466,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 593,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 373,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 408,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 474,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 775,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 854,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 368,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 403,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 481,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 503,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 856,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 50393146,
            "range": "± 1493275",
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
          "id": "3bd2e1444b62b6884eedc6b3439fcd2f032d7662",
          "message": "Add constructor trio in icu_segmenter (#2616)",
          "timestamp": "2022-09-24T22:16:00-07:00",
          "tree_id": "e073750e671e6f29919abee5e9cef7ca0f9da786",
          "url": "https://github.com/unicode-org/icu4x/commit/3bd2e1444b62b6884eedc6b3439fcd2f032d7662"
        },
        "date": 1664084183298,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 537,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 434,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 470,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 495,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 819,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 882,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 441,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 463,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 496,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 510,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 855,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 47483711,
            "range": "± 1711941",
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
          "id": "dbb02a18b48a63100c748e6ef3f39d5c734810f9",
          "message": "Consistently use try_ or get_for_ prefix on fallible functions (#2615)",
          "timestamp": "2022-09-24T22:16:59-07:00",
          "tree_id": "14174484487c0908eda88c7cdd95bc05c6512f45",
          "url": "https://github.com/unicode-org/icu4x/commit/dbb02a18b48a63100c748e6ef3f39d5c734810f9"
        },
        "date": 1664084377708,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 505,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 319,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 335,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 413,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 715,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 801,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 320,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 335,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 413,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 447,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 798,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 43613704,
            "range": "± 72868",
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
          "id": "e5ee9caff26dea38275ad49065eb1b42f3818276",
          "message": "Copy types by value (#2638)",
          "timestamp": "2022-09-25T12:19:53-07:00",
          "tree_id": "3ba9c0f5642ed34d13717e620ba7753f4f4dc593",
          "url": "https://github.com/unicode-org/icu4x/commit/e5ee9caff26dea38275ad49065eb1b42f3818276"
        },
        "date": 1664134382531,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 607,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 383,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 402,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 494,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 858,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 966,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 382,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 397,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 494,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 537,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 956,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 52568526,
            "range": "± 322004",
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
          "id": "f9e307b8ad5de785184bba12558790602e4970d4",
          "message": "Remove re-export of icu_calendar types (#2636)",
          "timestamp": "2022-09-25T12:19:41-07:00",
          "tree_id": "c87f8ed224e6feac4bc49c57e9e07d540f0421f2",
          "url": "https://github.com/unicode-org/icu4x/commit/f9e307b8ad5de785184bba12558790602e4970d4"
        },
        "date": 1664134439408,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 547,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 434,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 466,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 477,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 771,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 841,
            "range": "± 53",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 413,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 443,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 497,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 514,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 853,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 44884886,
            "range": "± 1944416",
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
          "id": "944c31975137dd2dffea2d2a2fa2ed0abd11bd8a",
          "message": "private::Key and other::Key to ::Subtag (#2632)",
          "timestamp": "2022-09-26T09:43:41-07:00",
          "tree_id": "b51e010dc6e44e26d8ea6d38eace5ce48d6ec94e",
          "url": "https://github.com/unicode-org/icu4x/commit/944c31975137dd2dffea2d2a2fa2ed0abd11bd8a"
        },
        "date": 1664211392712,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 514,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 416,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 468,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 445,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 763,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 808,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 414,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 461,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 444,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 497,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 792,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 40601134,
            "range": "± 2557724",
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
            "value": 1,
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
          "id": "d7f9ce3cfcd390ce811bb244a217b61661b3bb18",
          "message": "Update Diplomat to 0.4.0 (#2642)",
          "timestamp": "2022-09-26T18:22:00Z",
          "tree_id": "a9e4f8ebbe0bfe2b1dc31d9c8fa31d7927308942",
          "url": "https://github.com/unicode-org/icu4x/commit/d7f9ce3cfcd390ce811bb244a217b61661b3bb18"
        },
        "date": 1664217564637,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 576,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 466,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 505,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 537,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 835,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 934,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 458,
            "range": "± 85",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 502,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 525,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 544,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1168,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 48420282,
            "range": "± 1389627",
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
          "id": "dcbefa2377f0459882291cd5cc7ec895d59e7c4b",
          "message": "Infallible from_minutes_since_local_unix_epoch (#2646)",
          "timestamp": "2022-09-26T12:56:26-07:00",
          "tree_id": "1c9f7c7444413164039d07f51b1658c7a67127f7",
          "url": "https://github.com/unicode-org/icu4x/commit/dcbefa2377f0459882291cd5cc7ec895d59e7c4b"
        },
        "date": 1664223009970,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 506,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 320,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 336,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 413,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 631,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 710,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 282,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 325,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 413,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 447,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 827,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 131456627,
            "range": "± 365215",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/best",
            "value": 1,
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
          "id": "0eaff6f63ad74ec7184bad089dd7a67de071ab8c",
          "message": "Make expect_pattern GIGO (#2650)",
          "timestamp": "2022-09-26T19:57:29Z",
          "tree_id": "865e6a8e6b2d6a8c959c23b0b86b7b4addeb0dcd",
          "url": "https://github.com/unicode-org/icu4x/commit/0eaff6f63ad74ec7184bad089dd7a67de071ab8c"
        },
        "date": 1664223039064,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 506,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 320,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 338,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 413,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 715,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 807,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 320,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 348,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 413,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 448,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 798,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 43884223,
            "range": "± 64392",
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
            "email": "zibi@amazon.com",
            "name": "Zibi Braniecki",
            "username": "zbraniecki"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "7929b6e0cd392cbe9026740ee9ad83bcf1f392c5",
          "message": "Document inline clippy lints for datetime and plurals (#2572)",
          "timestamp": "2022-09-26T20:53:40Z",
          "tree_id": "7d57dda3c3614dd72b910f53dae533cc27401613",
          "url": "https://github.com/unicode-org/icu4x/commit/7929b6e0cd392cbe9026740ee9ad83bcf1f392c5"
        },
        "date": 1664226374075,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 506,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 320,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 325,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 413,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 715,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 808,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 319,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 335,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 413,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 448,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 799,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 44049309,
            "range": "± 60387",
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
          "id": "39b0797a56ff36fc82fde6e52093f6219e4c7b5e",
          "message": "ListStyle -> ListLength and add _with_length (#2628)",
          "timestamp": "2022-09-26T16:07:23-07:00",
          "tree_id": "11c6a6b2bd6a507b949f1867e575182590951a91",
          "url": "https://github.com/unicode-org/icu4x/commit/39b0797a56ff36fc82fde6e52093f6219e4c7b5e"
        },
        "date": 1664234392486,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 506,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 320,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 335,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 413,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 715,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 808,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 320,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 335,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 413,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 448,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 799,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 43927028,
            "range": "± 291807",
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
          "id": "774971a31cf76fc126f264ff430326ea4d01b9f4",
          "message": "Make PluralOperands fields private, add static constructor (#2598)",
          "timestamp": "2022-09-26T23:53:46Z",
          "tree_id": "5d90895c0732c35ddebe0f987c58f0f850cf00f8",
          "url": "https://github.com/unicode-org/icu4x/commit/774971a31cf76fc126f264ff430326ea4d01b9f4"
        },
        "date": 1664237283320,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 536,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 420,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 505,
            "range": "± 52",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 478,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 834,
            "range": "± 173",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 895,
            "range": "± 62",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 458,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 521,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 520,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 553,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 907,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 43867769,
            "range": "± 2130222",
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
          "id": "eeed42d3771b29764ad15b9f8a393896408fa4b9",
          "message": "Update utils in prep for 1.0 (#2651)\n\n\r\n* reformat doctests for  bumped utils crates\r\n\r\n* Bump fixed_decimal to 0.5.0\r\n\r\n* Bump zerovec to 0.9\r\n\r\n* Bump tinystr to 0.7\r\n\r\n* Bump litemap to 0.6.0",
          "timestamp": "2022-09-26T23:55:24Z",
          "tree_id": "2c8098b55aaa88a1f60692eed7c74bcfef8df0cf",
          "url": "https://github.com/unicode-org/icu4x/commit/eeed42d3771b29764ad15b9f8a393896408fa4b9"
        },
        "date": 1664237331651,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 592,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 385,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 388,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 444,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 896,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1044,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 385,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 399,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 443,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 508,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 888,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 56536036,
            "range": "± 202596",
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
          "id": "38f41b6e170c432e88d684ebd6b546e4ca837ebf",
          "message": "Remaining utils bumps (writeable, fixed_decimal) (#2652)\n\n* remove version from devdep\r\n\r\n* Bump writeable to 0.5.0",
          "timestamp": "2022-09-27T00:51:08Z",
          "tree_id": "1d60f0ecc0f014285cc780f325a63e65fdc5f897",
          "url": "https://github.com/unicode-org/icu4x/commit/38f41b6e170c432e88d684ebd6b546e4ca837ebf"
        },
        "date": 1664240592473,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 493,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 320,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 331,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 367,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 745,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 868,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 323,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 323,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 371,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 422,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 779,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 42821700,
            "range": "± 191094",
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
          "id": "3391fb8441cd88db2dfaf8fda1c005dc0fcf96e7",
          "message": "Make error enums more consistent (#2649)",
          "timestamp": "2022-09-26T20:38:20-05:00",
          "tree_id": "25108fb038439934f622a80a66a08ed19391ae4b",
          "url": "https://github.com/unicode-org/icu4x/commit/3391fb8441cd88db2dfaf8fda1c005dc0fcf96e7"
        },
        "date": 1664243578655,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 590,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 419,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 444,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 467,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 786,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 824,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 423,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 443,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 468,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 524,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 837,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 47556546,
            "range": "± 1821776",
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
          "id": "897c0a36c42ef211769eef570924124793c963fe",
          "message": "More Copy arguments (#2654)",
          "timestamp": "2022-09-27T10:01:56+02:00",
          "tree_id": "a9cf1ab3cbf044c5b65714d64475d731942eefc4",
          "url": "https://github.com/unicode-org/icu4x/commit/897c0a36c42ef211769eef570924124793c963fe"
        },
        "date": 1664266589544,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 474,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 318,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 316,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 360,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 609,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 645,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 318,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 316,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 360,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 411,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 622,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 52640955,
            "range": "± 788280",
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
          "id": "936082ff14b8d247dd35eca863f1b32cacda4ccb",
          "message": "Less cloning in `BlobDataProvider` (#2608)",
          "timestamp": "2022-09-27T15:56:20Z",
          "tree_id": "52045fe2275e8e5a88d0564d40a4389fe41b8e16",
          "url": "https://github.com/unicode-org/icu4x/commit/936082ff14b8d247dd35eca863f1b32cacda4ccb"
        },
        "date": 1664295108946,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 493,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 320,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 332,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 423,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 746,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 868,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 321,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 333,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 424,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 422,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 738,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 42859945,
            "range": "± 95582",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/best",
            "value": 1,
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
            "value": 1,
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
          "id": "7a471b7e4584bfc165d9e593295742c880369bf7",
          "message": "Update yoke (#2655)",
          "timestamp": "2022-09-27T16:58:32Z",
          "tree_id": "d1531ea3c964ecccfa49869ead2201f0c10e6a66",
          "url": "https://github.com/unicode-org/icu4x/commit/7a471b7e4584bfc165d9e593295742c880369bf7"
        },
        "date": 1664298729463,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 466,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 304,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 321,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 377,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 607,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 654,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 305,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 320,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 376,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 412,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 632,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 52603223,
            "range": "± 806045",
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
          "id": "a64c345427decfa5d36f08bdc9b7c81b2539cfee",
          "message": "Fixing assorted doc test lints (#2656)",
          "timestamp": "2022-09-27T17:03:26Z",
          "tree_id": "04d3d21e1e68d98a295e4ca27049eb138a7624eb",
          "url": "https://github.com/unicode-org/icu4x/commit/a64c345427decfa5d36f08bdc9b7c81b2539cfee"
        },
        "date": 1664298967335,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 492,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 319,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 332,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 372,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 681,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 805,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 319,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 332,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 371,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 452,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 733,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 42820314,
            "range": "± 127553",
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
          "id": "b10859a48f806068461a0138dced72a1a045217b",
          "message": "format docs (#2657)",
          "timestamp": "2022-09-27T17:41:26Z",
          "tree_id": "cfe2db7bd03fdcbaed47637abe607bb425675e66",
          "url": "https://github.com/unicode-org/icu4x/commit/b10859a48f806068461a0138dced72a1a045217b"
        },
        "date": 1664301356293,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 493,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 319,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 323,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 372,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 680,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 804,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 318,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 323,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 371,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 421,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 733,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 42943014,
            "range": "± 141020",
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
          "id": "ee2751298384ddf1ea7da566df541f2962c8f067",
          "message": "fix ecma402 (#2658)",
          "timestamp": "2022-09-27T18:12:38Z",
          "tree_id": "e65b5aae226ce76b210ad022a6ca161955fa4096",
          "url": "https://github.com/unicode-org/icu4x/commit/ee2751298384ddf1ea7da566df541f2962c8f067"
        },
        "date": 1664303088886,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 469,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 307,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 322,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 377,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 607,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 655,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 305,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 321,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 376,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 411,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 631,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 53963400,
            "range": "± 811848",
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
            "email": "zibi@unicode.org",
            "name": "Zibi Braniecki",
            "username": "zbraniecki"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "a59141cc099286b155e20ea707a6b5c49a2d7a78",
          "message": "Expand i18n in docs (#2659)\n\n* Expand i18n in docs\r\n\r\n* One more",
          "timestamp": "2022-09-27T11:32:43-07:00",
          "tree_id": "a1556e093fac7b60006b59f0b50d7584706e0483",
          "url": "https://github.com/unicode-org/icu4x/commit/a59141cc099286b155e20ea707a6b5c49a2d7a78"
        },
        "date": 1664304295698,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 492,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 311,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 323,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 371,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 680,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 804,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 319,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 332,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 371,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 422,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 729,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 42622045,
            "range": "± 146655",
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
          "id": "a17377feda88b4aa6de52906ebd81b20a0b13cba",
          "message": "Update changelog for 1.0 (#2661)",
          "timestamp": "2022-09-27T19:19:06Z",
          "tree_id": "199dbfd0951d6797eb3dd0eac6ab090db5a57a4d",
          "url": "https://github.com/unicode-org/icu4x/commit/a17377feda88b4aa6de52906ebd81b20a0b13cba"
        },
        "date": 1664307093630,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 493,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 319,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 333,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 371,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 681,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 805,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 319,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 333,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 372,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 422,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 729,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 42849229,
            "range": "± 116031",
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
          "id": "ed5f96813c6e9a7366bb6cc848846beacff35c94",
          "message": "Bump all crate versions to 1.0 (#2662)\n\n(except for segmenter and casemapping, they are 0.7)",
          "timestamp": "2022-09-27T19:53:23Z",
          "tree_id": "da72437bce12fdecb696d3ca458129e4e01f8158",
          "url": "https://github.com/unicode-org/icu4x/commit/ed5f96813c6e9a7366bb6cc848846beacff35c94"
        },
        "date": 1664309173773,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 481,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 318,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 323,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 410,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 620,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 720,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 318,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 323,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 410,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 445,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 710,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 47196756,
            "range": "± 162283",
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
          "id": "5f91c4bf32fe62c1011a8ba178b6a2afa9051d42",
          "message": "Don't say index.md (#2665)",
          "timestamp": "2022-09-27T23:13:48Z",
          "tree_id": "227339001a320b8d2f3a8ad7b5244e5fa5949659",
          "url": "https://github.com/unicode-org/icu4x/commit/5f91c4bf32fe62c1011a8ba178b6a2afa9051d42"
        },
        "date": 1664321291741,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 490,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 319,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 322,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 370,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 611,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 645,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 318,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 322,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 370,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 406,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 627,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 52381865,
            "range": "± 705585",
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
          "id": "82ef865e12d81ad6661f2c692dfbd4786e909185",
          "message": "ICU4X version (#2666)",
          "timestamp": "2022-09-27T23:38:55Z",
          "tree_id": "26f8bb3aea568807b4e16a7176a36cb3d5768a96",
          "url": "https://github.com/unicode-org/icu4x/commit/82ef865e12d81ad6661f2c692dfbd4786e909185"
        },
        "date": 1664322719815,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 531,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 423,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 485,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 463,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 781,
            "range": "± 70",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 833,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 420,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 478,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 462,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 515,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 813,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 42447026,
            "range": "± 1797244",
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
          "id": "d7d90cbb6ec9ae3b2efd61b5c55b55bc44d8cdd6",
          "message": "Another tutorial fix (#2667)\n\n\r\n* language as_str",
          "timestamp": "2022-09-27T23:45:37Z",
          "tree_id": "d830a7d2d153e08e7ef7b3e81e4b3a1bccced270",
          "url": "https://github.com/unicode-org/icu4x/commit/d7d90cbb6ec9ae3b2efd61b5c55b55bc44d8cdd6"
        },
        "date": 1664323126877,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 481,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 320,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 323,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 410,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 620,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 720,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 318,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 323,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 410,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 445,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 710,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 47193517,
            "range": "± 70350",
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
          "id": "369623318efe879a103fe4297c863315414c6b85",
          "message": "More tutorial fixes (#2668)\n\n\r\n* More tutorial fixes",
          "timestamp": "2022-09-27T23:54:26Z",
          "tree_id": "74c06864f785654d9af10a1e41a0de327b096a1e",
          "url": "https://github.com/unicode-org/icu4x/commit/369623318efe879a103fe4297c863315414c6b85"
        },
        "date": 1664324360698,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 488,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 311,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 322,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 370,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 612,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 645,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 298,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 320,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 370,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 406,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 626,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 52591915,
            "range": "± 728607",
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
          "id": "28bf527db690f991069533f8fe7c7b2467d84fc2",
          "message": "Fix wording in tutorial (#2669)",
          "timestamp": "2022-09-27T23:57:17Z",
          "tree_id": "fc04bed22f42c91c805b099926f2b46ca41b7019",
          "url": "https://github.com/unicode-org/icu4x/commit/28bf527db690f991069533f8fe7c7b2467d84fc2"
        },
        "date": 1664324553317,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 509,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 413,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 482,
            "range": "± 86",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 457,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 762,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 798,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 409,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 451,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 434,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 501,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 791,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 38536302,
            "range": "± 2002178",
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
          "id": "dd77d559ecd8c8d96f9231271140b213efc18c80",
          "message": "More tutorial fixes (#2670)\n\n* Fix link\r\n\r\n* rm outdated notice",
          "timestamp": "2022-09-28T00:10:38Z",
          "tree_id": "776914ad6657cca0c13d301e93cdb8e97a53e937",
          "url": "https://github.com/unicode-org/icu4x/commit/dd77d559ecd8c8d96f9231271140b213efc18c80"
        },
        "date": 1664327672964,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 516,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 405,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 470,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 441,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 739,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 723,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 371,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 420,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 407,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 435,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 729,
            "range": "± 48",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 38150438,
            "range": "± 2252783",
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
          "id": "6db40e605431b69ea169a27e4d331e78854d2e5a",
          "message": "Datagen fixes in tutorial (#2671)\n\n* Fix link\r\n\r\n* rm outdated notice\r\n\r\n* datagen fixes",
          "timestamp": "2022-09-28T00:25:43Z",
          "tree_id": "34d838fd9e30b50237032b4e7cfdab4bb80f97fb",
          "url": "https://github.com/unicode-org/icu4x/commit/6db40e605431b69ea169a27e4d331e78854d2e5a"
        },
        "date": 1664328220203,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 481,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 319,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 333,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 410,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 622,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 719,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 318,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 323,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 410,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 445,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 704,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 46930266,
            "range": "± 250363",
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
          "id": "4cce0a72f97002782fa9091c4f5c313e91c1539f",
          "message": "icu_provider feature (#2672)",
          "timestamp": "2022-09-28T00:30:28Z",
          "tree_id": "d6a9aeab0772455d7292e012061f99e1f12298b3",
          "url": "https://github.com/unicode-org/icu4x/commit/4cce0a72f97002782fa9091c4f5c313e91c1539f"
        },
        "date": 1664328528942,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 492,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 313,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 319,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 370,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 612,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 645,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 297,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 320,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 370,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 409,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 626,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 52566089,
            "range": "± 726230",
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
          "id": "5da1221896486ca462adb048ab1857a211b622a9",
          "message": "Various tutorial fixes from feedback (#2674)\n\n* Clean up\r\n\r\n* rename path so it's not confusing with icu\r\n\r\n* fixes\r\n\r\n* Fixup data management section\r\n\r\n* more fixes\r\n\r\n* cargo-edit\r\n\r\n* zibi\r\n\r\n* rob\r\n\r\n* fix\r\n\r\n* Update docs/tutorials/intro.md\r\n\r\nCo-authored-by: Robert Bastian <robertbastian@users.noreply.github.com>\r\n\r\nCo-authored-by: Robert Bastian <robertbastian@users.noreply.github.com>",
          "timestamp": "2022-09-28T15:58:56Z",
          "tree_id": "16605ec0ee8762552022ee019fb4afc188e7ff33",
          "url": "https://github.com/unicode-org/icu4x/commit/5da1221896486ca462adb048ab1857a211b622a9"
        },
        "date": 1664382279224,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 489,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 300,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 320,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 370,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 612,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 645,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 315,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 318,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 370,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 409,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 628,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 52383989,
            "range": "± 796331",
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
          "id": "498012a76a52412aa4c56aa4d481b95ce4e6bd6c",
          "message": "`icu_datagen@1.0.1` (#2679)\n\n* including segmenter data\r\n\r\n* lock",
          "timestamp": "2022-09-28T14:34:22-07:00",
          "tree_id": "57bbf21d0923754236ece560135990764f62f4db",
          "url": "https://github.com/unicode-org/icu4x/commit/498012a76a52412aa4c56aa4d481b95ce4e6bd6c"
        },
        "date": 1664401644124,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 436,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 347,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 404,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 381,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 618,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 673,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 342,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 384,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 377,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 419,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 679,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 34184907,
            "range": "± 1664938",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/best",
            "value": 1,
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
            "value": 1,
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
          "id": "f74dc3a64b28c7c45e640bdb752036665a7bfe22",
          "message": "Doc improvements, with separate data management tutorial (#2678)",
          "timestamp": "2022-09-28T21:56:01Z",
          "tree_id": "fb97eea77ca33dc348b42739a5c198412429bb11",
          "url": "https://github.com/unicode-org/icu4x/commit/f74dc3a64b28c7c45e640bdb752036665a7bfe22"
        },
        "date": 1664403231077,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 510,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 298,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 322,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 370,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 612,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 645,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 306,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 317,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 370,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 406,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 628,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 53544846,
            "range": "± 1179859",
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
          "id": "bac631a4c59cced96e637fa381583f243bc71183",
          "message": "Mention in index (#2680)",
          "timestamp": "2022-09-28T22:04:31Z",
          "tree_id": "157a4364387988e9beb980cdc750862aaa866009",
          "url": "https://github.com/unicode-org/icu4x/commit/bac631a4c59cced96e637fa381583f243bc71183"
        },
        "date": 1664405557090,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 481,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 319,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 323,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 410,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 620,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 720,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 318,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 335,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 410,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 445,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 703,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 47127299,
            "range": "± 67301",
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
          "id": "7f22b4677338c848fd42e2a9fb7d28cf546e0d95",
          "message": "Some wasm demo fixes (#2688)\n\n* Change default calendar now that we have fallbacking\r\n\r\n* Handle calendaring when locale is already specified with unicode extensions",
          "timestamp": "2022-09-29T22:24:37Z",
          "tree_id": "46d6a8ea9d632588cde6597d62a1bfe8fec0678f",
          "url": "https://github.com/unicode-org/icu4x/commit/7f22b4677338c848fd42e2a9fb7d28cf546e0d95"
        },
        "date": 1664491015414,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 490,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 314,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 324,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 370,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 612,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 645,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 314,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 321,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 370,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 406,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 629,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 52532923,
            "range": "± 783374",
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
          "id": "f9d456b6387304cf77b620b3ebd7549b02dc0627",
          "message": "Transformer code for DisplayNames component. (#2635)",
          "timestamp": "2022-09-30T18:26:06+02:00",
          "tree_id": "9ad2942718485d2150b146d8b214682d24db1bc6",
          "url": "https://github.com/unicode-org/icu4x/commit/f9d456b6387304cf77b620b3ebd7549b02dc0627"
        },
        "date": 1664556027830,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 491,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 313,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 322,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 370,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 611,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 645,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 314,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 322,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 370,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 407,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 626,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 52486222,
            "range": "± 649787",
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
          "id": "1ff25a693f73a876ad2684d57d6dd9f407371852",
          "message": "Update icu4x placeholder crate (#2681)",
          "timestamp": "2022-09-30T19:58:33+02:00",
          "tree_id": "b030e0a0d4c7985dc4967a8866a80c142bc5983d",
          "url": "https://github.com/unicode-org/icu4x/commit/1ff25a693f73a876ad2684d57d6dd9f407371852"
        },
        "date": 1664561523483,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 536,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 435,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 490,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 497,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 823,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 869,
            "range": "± 52",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 436,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 507,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 468,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 531,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 855,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 43439912,
            "range": "± 2633537",
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
          "id": "43800bf3d86db8df87cf8bc6ac92832c36918ff2",
          "message": "rm (#2694)",
          "timestamp": "2022-09-30T19:27:39Z",
          "tree_id": "1fcc8caea2c82220fbb14380c949db6e7c1c44b5",
          "url": "https://github.com/unicode-org/icu4x/commit/43800bf3d86db8df87cf8bc6ac92832c36918ff2"
        },
        "date": 1664566911022,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 481,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 318,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 323,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 410,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 620,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 719,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 318,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 335,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 410,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 445,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 703,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 47123005,
            "range": "± 305825",
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
          "id": "926e2208597f78017fa6fed477640a348d5d81e3",
          "message": "mention that we support multiple unsized fields in varule (#2695)\n\n* mention that we support multiple unsized fields in varule\r\n\r\n* Update utils/zerovec/src/lib.rs\r\n\r\nCo-authored-by: Robert Bastian <robertbastian@users.noreply.github.com>\r\n\r\nCo-authored-by: Robert Bastian <robertbastian@users.noreply.github.com>",
          "timestamp": "2022-09-30T19:45:16Z",
          "tree_id": "dfa4e4a71acf2e12e338c94117b155725f40b7b9",
          "url": "https://github.com/unicode-org/icu4x/commit/926e2208597f78017fa6fed477640a348d5d81e3"
        },
        "date": 1664567941585,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 425,
            "range": "± 103",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 281,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 293,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 362,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 557,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 640,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 281,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 285,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 362,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 393,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 624,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 41615958,
            "range": "± 2687934",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/best",
            "value": 1,
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
            "value": 1,
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
          "id": "5b7cea454a785c205a4d8e66e7c616731406a3bd",
          "message": "Fixing baked datagen for no keys and keys with no data (#2698)\n\n* fix\n\n* clippy\n\n* fix test\n\n* better code\n\nCo-authored-by: Manish Goregaokar <manishsmail@gmail.com>",
          "timestamp": "2022-09-30T21:21:41-07:00",
          "tree_id": "4e1dc7b98267574eed6ed615034db4890fcf913d",
          "url": "https://github.com/unicode-org/icu4x/commit/5b7cea454a785c205a4d8e66e7c616731406a3bd"
        },
        "date": 1664598880855,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 563,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 372,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 394,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 479,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 739,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 844,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 376,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 396,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 480,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 519,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 821,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 54924362,
            "range": "± 1231695",
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
            "email": "616399+pt2121@users.noreply.github.com",
            "name": "Prat",
            "username": "pt2121"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5a8c31a80ce2a774a1b07f6bb21a79087f92a6c1",
          "message": "Fix Time::from_minute_with_remainder_days to handle negatives (#2643) (#2702)\n\nCo-authored-by: Prat T <pt2121@users.noreply.github.com>",
          "timestamp": "2022-10-02T18:28:56-05:00",
          "tree_id": "3e9eb55dde49b418fd03d44dacfddffdf96847f7",
          "url": "https://github.com/unicode-org/icu4x/commit/5a8c31a80ce2a774a1b07f6bb21a79087f92a6c1"
        },
        "date": 1664754106399,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 461,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 380,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 414,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 398,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 671,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 744,
            "range": "± 54",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 391,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 426,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 392,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 441,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 697,
            "range": "± 60",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 37007109,
            "range": "± 2825698",
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
            "value": 1,
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
            "email": "m_kato@ga2.so-net.ne.jp",
            "name": "Makoto Kato",
            "username": "makotokato"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "774b734dd4993ff4faa77898c584ecc657ab5712",
          "message": "Remove unnecessary langauge check for East Asian languagne (SA property) (#2705)",
          "timestamp": "2022-10-03T17:02:52+09:00",
          "tree_id": "3f252f10560490373b7b0bb5e2cad09cd921ca11",
          "url": "https://github.com/unicode-org/icu4x/commit/774b734dd4993ff4faa77898c584ecc657ab5712"
        },
        "date": 1664784924461,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 493,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 314,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 323,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 370,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 613,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 646,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 311,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 322,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 370,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 407,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 626,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 52634542,
            "range": "± 839419",
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
          "id": "9c82b8ca39bbb195fb94bf7e77d646440cb17409",
          "message": "Rename *BreakSegmenter to *Segmenter (#2707)\n\n* Rename *BreakSegmenter to *Segmenter.\r\n\r\nThis patch is generated via:\r\n\r\n```\r\nrg -l BreakSegmenter experimental/segmenter/ ffi/diplomat/src/ ffi/diplomat/cpp/examples/ ffi/diplomat/wasm/wasm-demo/ | xargs sed -i 's/BreakSegmenter/Segmenter/g'\r\n```\r\n\r\n* Run `cargo fmt`\r\n\r\n* Regenerate readme and diplomat\r\n\r\nThis patch is generated via\r\n\r\n```\r\ncargo make diplomat-gen && cargo make generate-readmes\r\n```",
          "timestamp": "2022-10-03T08:58:03-07:00",
          "tree_id": "be89798e09c73390fde07347780547768a6b09fb",
          "url": "https://github.com/unicode-org/icu4x/commit/9c82b8ca39bbb195fb94bf7e77d646440cb17409"
        },
        "date": 1664813583398,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 578,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 382,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 402,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 493,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 744,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 864,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 383,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 401,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 493,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 534,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 852,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 60132070,
            "range": "± 2877609",
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
          "id": "9b9d14d32d185fadacd2a3847ff6a965728e3698",
          "message": "Add div_rem_euclid and use it in icu_calendar (#2704)\n\n* Add div_rem_euclid and use it in icu_calendar\r\n\r\n* Don't need helper function when dividend is positive",
          "timestamp": "2022-10-03T17:17:34Z",
          "tree_id": "3bdc15aa1d70dcf7c125b29cbac4860fb3b3749b",
          "url": "https://github.com/unicode-org/icu4x/commit/9b9d14d32d185fadacd2a3847ff6a965728e3698"
        },
        "date": 1664818228905,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 540,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 356,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 373,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 450,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 670,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 815,
            "range": "± 48",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 358,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 390,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 454,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 479,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 816,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 52386181,
            "range": "± 2888883",
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
            "email": "dsipasseuth@google.com",
            "name": "Sipasseuth Daniel",
            "username": "dsipasseuth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "664daf32563655e23bd0e94bfff84004d1ba29b2",
          "message": "Update tutorials to documentation testing. (#2645)",
          "timestamp": "2022-10-03T19:07:51Z",
          "tree_id": "996c03f9243ea475c5b6dba610ec63003c5d3810",
          "url": "https://github.com/unicode-org/icu4x/commit/664daf32563655e23bd0e94bfff84004d1ba29b2"
        },
        "date": 1664824833224,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 481,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 318,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 334,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 410,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 670,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 719,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 323,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 328,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 405,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 445,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 705,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 46892869,
            "range": "± 97553",
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
          "id": "359f09b3d99864163bf1c80dd3a42fd002bfa174",
          "message": "More borrowing in locid's `write_to_string` (#2693)",
          "timestamp": "2022-10-03T22:53:19+02:00",
          "tree_id": "d70229caeca04f26ef6e80af8112667601f0492c",
          "url": "https://github.com/unicode-org/icu4x/commit/359f09b3d99864163bf1c80dd3a42fd002bfa174"
        },
        "date": 1664831214102,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 489,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 295,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 322,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 370,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 611,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 645,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 317,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 322,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 369,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 406,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 623,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 52388385,
            "range": "± 811993",
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
          "id": "06c352982e941ca8cdb64ffa47f500f68c80dab1",
          "message": "Clean up dependency specifications so serde isn't pulled in by default (#2696)\n\n* Mark optional deps as optional\r\n\r\n* Make dep optional\r\n\r\n* Rm dep from locid_transform\r\n\r\n* serde feature\r\n\r\n* unconditional litemap",
          "timestamp": "2022-10-03T21:02:58Z",
          "tree_id": "9b88e9cbd64d8e895e075ed3ab1b54a64bb65b33",
          "url": "https://github.com/unicode-org/icu4x/commit/06c352982e941ca8cdb64ffa47f500f68c80dab1"
        },
        "date": 1664831741787,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 537,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 320,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 332,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 423,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 747,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 868,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 321,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 333,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 424,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 423,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 739,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 39703848,
            "range": "± 191838",
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
          "id": "c33971f8d8516eb9bd3cc12a229e815fb76c3af9",
          "message": "Fix math in calendar (#2714)\n\n* Tests and initial work on ISO with negative years\r\n\r\n* Fix coptic division issue\r\n\r\n* fix test\r\n\r\n* add missing test\r\n\r\n\r\nCo-authored-by: Shane F. Carr <shane@unicode.org>",
          "timestamp": "2022-10-03T14:32:04-07:00",
          "tree_id": "04ca4044a126a3d78d03196d712519efb5a08322",
          "url": "https://github.com/unicode-org/icu4x/commit/c33971f8d8516eb9bd3cc12a229e815fb76c3af9"
        },
        "date": 1664833518039,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 469,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 320,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 316,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 361,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 615,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 644,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 320,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 316,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 360,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 414,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 621,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 49500892,
            "range": "± 1087793",
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
          "id": "f0592c39779594403fd7704a8911d44f8315c251",
          "message": "Fix spelling in docs (#2720)",
          "timestamp": "2022-10-05T15:53:35Z",
          "tree_id": "9f452a4a00d5ee0e69809d9f4b020ab23b8e262a",
          "url": "https://github.com/unicode-org/icu4x/commit/f0592c39779594403fd7704a8911d44f8315c251"
        },
        "date": 1664986092755,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 645,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 383,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 399,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 508,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 892,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1037,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 385,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 399,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 508,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 507,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 883,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 47234191,
            "range": "± 276499",
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
            "email": "samchen61661@gmail.com",
            "name": "samchen",
            "username": "samchen61661"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "4f7e7de8058e3497e73cac8c02f2edfcc28b431f",
          "message": "Create DateTimeParser in experimental branch ixdtf (#2480)\n\n* Create DateTimeParser\r\n\r\n* Use Peekable iterator\r\n\r\n* Update Cargo.toml\r\n\r\n* address comments\r\n\r\n* add module\r\n\r\n* cargo make generate-readmes",
          "timestamp": "2022-10-07T14:33:05-07:00",
          "tree_id": "cf222a1840675e0861623f93c5f751e9a5af1d1c",
          "url": "https://github.com/unicode-org/icu4x/commit/4f7e7de8058e3497e73cac8c02f2edfcc28b431f"
        },
        "date": 1665179186941,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 645,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 385,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 400,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 507,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 896,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1043,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 385,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 399,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 509,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 511,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 888,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 47415409,
            "range": "± 134958",
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
          "id": "bc907e68ae7fe7933a01195f31a9377326cfe8b5",
          "message": "Allowing no keys in datagen CLI (#2731)",
          "timestamp": "2022-10-08T01:55:42+02:00",
          "tree_id": "8a7f61f9a448f7307900c6a58a91639b807cd538",
          "url": "https://github.com/unicode-org/icu4x/commit/bc907e68ae7fe7933a01195f31a9377326cfe8b5"
        },
        "date": 1665187752047,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 655,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 394,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 410,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 524,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 909,
            "range": "± 55",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1062,
            "range": "± 79",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 389,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 407,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 519,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 555,
            "range": "± 72",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 900,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 48128092,
            "range": "± 1707691",
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
          "id": "c6f3cb2a272eed1843feb994d8297de7d313fa43",
          "message": "Static blob constructor over FFI (#2724)",
          "timestamp": "2022-10-08T02:37:19+02:00",
          "tree_id": "5da190fb22974796bb65dad8e72e2c05b993c6d6",
          "url": "https://github.com/unicode-org/icu4x/commit/c6f3cb2a272eed1843feb994d8297de7d313fa43"
        },
        "date": 1665190258656,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 538,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 386,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 402,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 447,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 723,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 835,
            "range": "± 51",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 385,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 400,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 435,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 489,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 787,
            "range": "± 69",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 37725735,
            "range": "± 2174675",
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
          "id": "728d3891e37be6256e2c2fc604e50f8f0865fb66",
          "message": "`icu_provider@1.0.1` (#2732)",
          "timestamp": "2022-10-08T03:01:18+02:00",
          "tree_id": "8907d6923a76c85635929d2e7e61ec7755579212",
          "url": "https://github.com/unicode-org/icu4x/commit/728d3891e37be6256e2c2fc604e50f8f0865fb66"
        },
        "date": 1665191786629,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 395,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 279,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 278,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 322,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 526,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 560,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 280,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 277,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 321,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 358,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 576,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 42726502,
            "range": "± 644259",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/best",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/worst",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/best",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/worst",
            "value": 1,
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
          "id": "d5116379029a0e20aa50843503337cc6199ad889",
          "message": "Add notes about collation index generation (#2728)",
          "timestamp": "2022-10-10T09:18:10+03:00",
          "tree_id": "750d57672f793b167e54926ccc366a1e1669030e",
          "url": "https://github.com/unicode-org/icu4x/commit/d5116379029a0e20aa50843503337cc6199ad889"
        },
        "date": 1665383519425,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 458,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 322,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 319,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 368,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 605,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 646,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 330,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 326,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 374,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 416,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 652,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 49626780,
            "range": "± 642216",
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
          "id": "aef683066aa355158f1e37cd843915ab7a49b59c",
          "message": "Adding CNAME in docs CI (#2734)",
          "timestamp": "2022-10-10T18:53:58+02:00",
          "tree_id": "ba0d93a318a858b0d1db0dfb4aad8b431a6e7627",
          "url": "https://github.com/unicode-org/icu4x/commit/aef683066aa355158f1e37cd843915ab7a49b59c"
        },
        "date": 1665421602024,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 502,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 320,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 335,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 472,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 715,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 709,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 320,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 336,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 472,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 447,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 798,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 39618567,
            "range": "± 65284",
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
          "id": "a92d8bc14f64d92ef27e67709b1e2e8bb9e3f541",
          "message": "Update design_doc.md",
          "timestamp": "2022-10-10T19:20:54-05:00",
          "tree_id": "e885194977b4e3b014d9ab9eb68b81f707454744",
          "url": "https://github.com/unicode-org/icu4x/commit/a92d8bc14f64d92ef27e67709b1e2e8bb9e3f541"
        },
        "date": 1665448425775,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 502,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 320,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 335,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 473,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 714,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 803,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 319,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 335,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 473,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 447,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 800,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 39516033,
            "range": "± 51875",
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
          "id": "44adf7e0154be9987c384a642f595b91cd85c577",
          "message": "Update design_doc.md",
          "timestamp": "2022-10-11T01:14:08-05:00",
          "tree_id": "5db99f4f2d774ca258278d527f5b0fd37dce16d8",
          "url": "https://github.com/unicode-org/icu4x/commit/44adf7e0154be9987c384a642f595b91cd85c577"
        },
        "date": 1665469700476,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 461,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 319,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 318,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 370,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 605,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 647,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 319,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 318,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 371,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 418,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 663,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 49534800,
            "range": "± 600355",
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
          "id": "30c7e8466fa28313c1984085463baf008d319851",
          "message": "Use better branch setting for benchmarks CI job (#2737)",
          "timestamp": "2022-10-11T16:27:37Z",
          "tree_id": "2ffadecac23e6a8025fb3f856609f70193ee0c28",
          "url": "https://github.com/unicode-org/icu4x/commit/30c7e8466fa28313c1984085463baf008d319851"
        },
        "date": 1665506458361,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 505,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 320,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 335,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 478,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 715,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 806,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 319,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 335,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 473,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 447,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 797,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 39531357,
            "range": "± 60656",
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
          "id": "5e5e30c5a8f3373a7f6ddcc0370239d220cd048f",
          "message": "Fix sample code to use iso (#2741)",
          "timestamp": "2022-10-13T12:32:24-05:00",
          "tree_id": "59acb8c5c5559eaf1553808108aff0396f379a90",
          "url": "https://github.com/unicode-org/icu4x/commit/5e5e30c5a8f3373a7f6ddcc0370239d220cd048f"
        },
        "date": 1665683121550,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 456,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 319,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 316,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 366,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 606,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 646,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 319,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 317,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 374,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 414,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 658,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 49405317,
            "range": "± 675188",
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
          "id": "9e8fbc08b311d541aa7f0cd70f63d58023fdfff1",
          "message": "Cleaning up ffi/wasm and generating ffi docs previews (#2736)",
          "timestamp": "2022-10-14T18:12:40+02:00",
          "tree_id": "2617db2874aaaab657738706ad44e773ab3c1ac4",
          "url": "https://github.com/unicode-org/icu4x/commit/9e8fbc08b311d541aa7f0cd70f63d58023fdfff1"
        },
        "date": 1665764860185,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 541,
            "range": "± 48",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 421,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 479,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 496,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 804,
            "range": "± 56",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 907,
            "range": "± 51",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 437,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 454,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 471,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 478,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 765,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 37793798,
            "range": "± 2260438",
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
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "e2b6ec966d783b2d4fc78d13c8a2baaaca3fc31a",
          "message": "Bump got and ava in /ffi/diplomat/js/examples/node (#2744)\n\nRemoves [got](https://github.com/sindresorhus/got). It's no longer used after updating ancestor dependency [ava](https://github.com/avajs/ava). These dependencies need to be updated together.\r\n\r\n\r\nRemoves `got`\r\n\r\nUpdates `ava` from 3.15.0 to 4.3.3\r\n- [Release notes](https://github.com/avajs/ava/releases)\r\n- [Commits](https://github.com/avajs/ava/compare/v3.15.0...v4.3.3)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: got\r\n  dependency-type: indirect\r\n- dependency-name: ava\r\n  dependency-type: direct:development\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-10-14T21:08:31Z",
          "tree_id": "32e69c2ded251a1431b15a406df41d1803f49faa",
          "url": "https://github.com/unicode-org/icu4x/commit/e2b6ec966d783b2d4fc78d13c8a2baaaca3fc31a"
        },
        "date": 1665782577483,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 561,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 348,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 382,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 556,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 854,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 964,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 387,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 402,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 570,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 523,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 894,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 45349318,
            "range": "± 954856",
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
          "id": "58a77c95fbbd11e9a2b9f62a0b7e3d04d9c5f6f7",
          "message": "Reducing `to_string`s (#2727)",
          "timestamp": "2022-10-15T01:31:35+02:00",
          "tree_id": "cabc1a1ea926b6dc7b4df54b53da7c29252ed059",
          "url": "https://github.com/unicode-org/icu4x/commit/58a77c95fbbd11e9a2b9f62a0b7e3d04d9c5f6f7"
        },
        "date": 1665791056194,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 456,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 320,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 322,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 365,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 665,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 647,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 321,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 320,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 370,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 412,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 657,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 49524453,
            "range": "± 647735",
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
          "id": "3aaa9bb82875a81b7599528fcd3581818d2f05c4",
          "message": "Revert \"Bump got and ava in /ffi/diplomat/js/examples/node (#2744)\" (#2746)\n\nThis reverts commit e2b6ec966d783b2d4fc78d13c8a2baaaca3fc31a.",
          "timestamp": "2022-10-14T23:33:34Z",
          "tree_id": "6aed7b51d75a52b7c12f2cebd485e429d665295c",
          "url": "https://github.com/unicode-org/icu4x/commit/3aaa9bb82875a81b7599528fcd3581818d2f05c4"
        },
        "date": 1665791243785,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 327,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 235,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 230,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 264,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 435,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 464,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 230,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 229,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 296,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 294,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 470,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 35291184,
            "range": "± 2279211",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/best",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/worst",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/best",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/worst",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsipasseuth@google.com",
            "name": "Sipasseuth Daniel",
            "username": "dsipasseuth"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "177a72889915dbf42b96297666a00991388b5d22",
          "message": "Fixes in execution/paths to smooth the tutorial experience (#2729)",
          "timestamp": "2022-10-15T04:11:54+02:00",
          "tree_id": "102a8950069dbe4c9d44f896a013894134ea85cc",
          "url": "https://github.com/unicode-org/icu4x/commit/177a72889915dbf42b96297666a00991388b5d22"
        },
        "date": 1665800773877,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 566,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 458,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 485,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 519,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 830,
            "range": "± 103",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 905,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 461,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 478,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 525,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 546,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 880,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 45526499,
            "range": "± 1994050",
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
          "id": "e37275fe168440cae5e5ba310ac79c60e3deef9e",
          "message": "Run artifacts-build all the time (#2750)",
          "timestamp": "2022-10-14T22:04:56-05:00",
          "tree_id": "b898d503cdad66432f791e1e49b1c902f97c7b28",
          "url": "https://github.com/unicode-org/icu4x/commit/e37275fe168440cae5e5ba310ac79c60e3deef9e"
        },
        "date": 1665803865533,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 502,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 319,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 348,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 472,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 715,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 808,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 319,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 350,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 478,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 447,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 797,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 39503875,
            "range": "± 57096",
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
          "id": "d654ee0629157b167ec98a0b1f436e800d4dc2fd",
          "message": "Reducing `to_string` in locid (#2745)",
          "timestamp": "2022-10-14T20:48:32-07:00",
          "tree_id": "bf5296cddd74cdc807150efc87c4ad4809218791",
          "url": "https://github.com/unicode-org/icu4x/commit/d654ee0629157b167ec98a0b1f436e800d4dc2fd"
        },
        "date": 1665806618324,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 601,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 380,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 410,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 564,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 858,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 963,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 379,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 414,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 569,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 536,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 954,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 46507697,
            "range": "± 527568",
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
          "id": "48457f5ce2a775094cda3b6431667da14c430ee2",
          "message": "More borrow in list JSON deserialization (#2751)",
          "timestamp": "2022-10-14T21:08:14-07:00",
          "tree_id": "30e85b03d7e8947c6489fba030a4a47880a3fa1d",
          "url": "https://github.com/unicode-org/icu4x/commit/48457f5ce2a775094cda3b6431667da14c430ee2"
        },
        "date": 1665807699912,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 457,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 324,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 321,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 366,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 606,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 646,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 320,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 317,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 366,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 411,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 663,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 49323569,
            "range": "± 681476",
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
          "id": "15e8ca42eab240971291cc785ffd16007a982c6c",
          "message": "NPM with full data (#2747)",
          "timestamp": "2022-10-14T21:07:15-07:00",
          "tree_id": "7243fda130663e0af0db85b890b61501ba4c2e01",
          "url": "https://github.com/unicode-org/icu4x/commit/15e8ca42eab240971291cc785ffd16007a982c6c"
        },
        "date": 1665807750215,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 502,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 319,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 336,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 472,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 715,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 805,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 320,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 348,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 472,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 447,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 797,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 39615079,
            "range": "± 46391",
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
          "id": "8470cdae04636cc7a8e485dce91712ca8dd269c5",
          "message": "Some wasm fixes (#2757)\n\n* Cache full-data in wasm build\r\n\r\n* fix diplomat-wasm import\r\n\r\n* Bump stack size\r\n\r\n* Fix ci\r\n\r\n* More stack size :(\r\n\r\n* Bump diplomat\r\n\r\n* fixup sed script",
          "timestamp": "2022-10-18T19:26:16Z",
          "tree_id": "6344351620af9d2231dda18f4935c453c755c546",
          "url": "https://github.com/unicode-org/icu4x/commit/8470cdae04636cc7a8e485dce91712ca8dd269c5"
        },
        "date": 1666122090505,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 615,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 392,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 429,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 575,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 871,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 984,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 391,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 425,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 582,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 548,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 982,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 48644265,
            "range": "± 1051754",
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
          "id": "c151750827a2a396f020d7823781ae33399e9cee",
          "message": "Post-review wasm fixes (#2759)\n\n* Post-review fixes\r\n\r\n* add more flags",
          "timestamp": "2022-10-18T13:44:19-07:00",
          "tree_id": "841327473e73b91a81f0453870f9410754cd2b1b",
          "url": "https://github.com/unicode-org/icu4x/commit/c151750827a2a396f020d7823781ae33399e9cee"
        },
        "date": 1666126906341,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 607,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 479,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 499,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 546,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 863,
            "range": "± 54",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 940,
            "range": "± 52",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 473,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 497,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 543,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 567,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 932,
            "range": "± 62",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 46665360,
            "range": "± 1411379",
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
          "id": "5faf121440aa6c0701487c2de925a52763745246",
          "message": "Improve collation datagen test assertions (#2766)",
          "timestamp": "2022-10-18T18:21:07-07:00",
          "tree_id": "fb4982e9cdf0e955580d65250b5c7a10fd4d9a95",
          "url": "https://github.com/unicode-org/icu4x/commit/5faf121440aa6c0701487c2de925a52763745246"
        },
        "date": 1666143333112,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 513,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 328,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 364,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 498,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 739,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 818,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 324,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 340,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 484,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 455,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 827,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 41444483,
            "range": "± 1507326",
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
          "id": "ee28e44378b7c5b6f356d3c1e375b2027c7d4c5b",
          "message": "Update Node.js to 16.18.0 and other WASM-related fixes (#2763)",
          "timestamp": "2022-10-18T18:32:03-07:00",
          "tree_id": "5b3fcbae267dae6a6f1e46adb2fd8004bcb146ae",
          "url": "https://github.com/unicode-org/icu4x/commit/ee28e44378b7c5b6f356d3c1e375b2027c7d4c5b"
        },
        "date": 1666143911661,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 502,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 319,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 347,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 472,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 715,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 803,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 319,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 337,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 472,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 447,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 800,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 39526828,
            "range": "± 83133",
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
          "id": "aa8a5ae0671c99a4a3925c220990723963147d85",
          "message": "WASM review feedback (#2767)",
          "timestamp": "2022-10-18T21:51:37-07:00",
          "tree_id": "49e2e52a4920dea9811fa267c9496366679ad8e7",
          "url": "https://github.com/unicode-org/icu4x/commit/aa8a5ae0671c99a4a3925c220990723963147d85"
        },
        "date": 1666155890957,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 503,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 320,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 337,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 473,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 715,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 803,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 320,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 336,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 473,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 447,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 800,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 39678594,
            "range": "± 117923",
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
          "id": "cd3ba6e8b511a2470cadf9504d6a2de8ffae38f3",
          "message": "Add SVG loading spiral to WASM demo (#2772)",
          "timestamp": "2022-10-19T17:00:09-07:00",
          "tree_id": "eb592178ea40128762b6e0175c5426bab7aedb55",
          "url": "https://github.com/unicode-org/icu4x/commit/cd3ba6e8b511a2470cadf9504d6a2de8ffae38f3"
        },
        "date": 1666224800888,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 509,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 336,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 360,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 483,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 756,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 822,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 329,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 365,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 508,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 464,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 915,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 42247841,
            "range": "± 2373310",
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
          "id": "9c99640979e6f943f72162b876587a6bb05bcb54",
          "message": "Remove static_assertions dep from fixed-decimal, bump version (#2773)\n\n* Remove static_assertions dep from fixed_decimal\r\n\r\n* Bump fixed-decimal to 0.5.1\r\n\r\n* remove devdep too\r\n\r\n* fmt",
          "timestamp": "2022-10-20T00:36:48Z",
          "tree_id": "360e1fdfe4a50ae29c709d4aa1a167ed221219a5",
          "url": "https://github.com/unicode-org/icu4x/commit/9c99640979e6f943f72162b876587a6bb05bcb54"
        },
        "date": 1666227086537,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 637,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 456,
            "range": "± 72",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 512,
            "range": "± 521",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 502,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 849,
            "range": "± 196",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 919,
            "range": "± 286",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 468,
            "range": "± 72",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 528,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 521,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 575,
            "range": "± 148",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 896,
            "range": "± 60",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 46374788,
            "range": "± 6970676",
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
          "id": "0acdb50f0726616c5d689890b0321bceb0a519bc",
          "message": "Allow clippy::forget_copy in derive(Yokeable) impl (#2775)\n\nThe attribute was already emitted when proving covariance manually,\r\nbut not in the normal case.",
          "timestamp": "2022-10-19T21:58:31-07:00",
          "tree_id": "2f88440b84462c0ccb4b09cbfd474702a9005890",
          "url": "https://github.com/unicode-org/icu4x/commit/0acdb50f0726616c5d689890b0321bceb0a519bc"
        },
        "date": 1666242611655,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 335,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 215,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 230,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 263,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 436,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 461,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 214,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 229,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 263,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 295,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 443,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 37390362,
            "range": "± 368336",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/best",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/worst",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/best",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/worst",
            "value": 1,
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
          "id": "c6b8ea6eab550d1c9cc0251083fa480c6b352c41",
          "message": "Allow nightly to be overridden by an environment variable (#2760)\n\n* Centralize nightly used by CI jobs, allow env vars to override it\r\n\r\n* Bump makefile min version",
          "timestamp": "2022-10-21T20:35:28Z",
          "tree_id": "6286f0c75f0c5f1a69749b903f81566677783a69",
          "url": "https://github.com/unicode-org/icu4x/commit/c6b8ea6eab550d1c9cc0251083fa480c6b352c41"
        },
        "date": 1666385347744,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 537,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 320,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 323,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 371,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 746,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 866,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 321,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 323,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 370,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 422,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 775,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 47277212,
            "range": "± 157117",
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
          "id": "668d100f2dc4c3e355c043df7c23877ca2b6f8ad",
          "message": "Add GN build rules (#1972)",
          "timestamp": "2022-10-25T12:22:50-07:00",
          "tree_id": "184c12d8a760e3971b91760b5a8a1c20da40f984",
          "url": "https://github.com/unicode-org/icu4x/commit/668d100f2dc4c3e355c043df7c23877ca2b6f8ad"
        },
        "date": 1666726743609,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 599,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 467,
            "range": "± 91",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 511,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 499,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 857,
            "range": "± 61",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 904,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 473,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 521,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 497,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 584,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 899,
            "range": "± 68",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 44276708,
            "range": "± 1325560",
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
            "email": "elango@unicode.org",
            "name": "Elango Cheran",
            "username": "echeran"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ecffce3fc5d3a2fea9fad22000dd6053c0f9e828",
          "message": "Add documentation on CodePointTrie details and perf considerations (#2717)",
          "timestamp": "2022-10-26T00:01:03Z",
          "tree_id": "d1822b1e57b19ae6aaffe915105ebd304518c894",
          "url": "https://github.com/unicode-org/icu4x/commit/ecffce3fc5d3a2fea9fad22000dd6053c0f9e828"
        },
        "date": 1666743290010,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 590,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 347,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 365,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 418,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 816,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 956,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 350,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 354,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 416,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 454,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 852,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 52554547,
            "range": "± 1166991",
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
          "id": "6e748569e3c408cd5b59938d4629af9bfff32cae",
          "message": "Add ixdtf to CI (#2786)",
          "timestamp": "2022-10-25T17:00:41-07:00",
          "tree_id": "1f5a94110c9ff68f999575e0b4b191cf0042657a",
          "url": "https://github.com/unicode-org/icu4x/commit/6e748569e3c408cd5b59938d4629af9bfff32cae"
        },
        "date": 1666743381157,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 673,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 401,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 417,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 465,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 933,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1083,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 411,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 405,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 464,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 529,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 971,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 60720306,
            "range": "± 2713876",
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
            "email": "jfaucher.fr@gmail.com",
            "name": "jlfaucher",
            "username": "jlfaucher"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c1d58ea7bae7246dd4c290a93bfa9cf1412bb6b5",
          "message": "Invalid usage of string_view in FFI C++ example (#2792)",
          "timestamp": "2022-10-27T02:43:44Z",
          "tree_id": "e0c01eca8c3b09b8af0a864a188f96bc7b7d91ee",
          "url": "https://github.com/unicode-org/icu4x/commit/c1d58ea7bae7246dd4c290a93bfa9cf1412bb6b5"
        },
        "date": 1666839428283,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 473,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 319,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 323,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 371,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 614,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 651,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 318,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 322,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 368,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 413,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 621,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 52494180,
            "range": "± 789540",
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
            "email": "elango@unicode.org",
            "name": "Elango Cheran",
            "username": "echeran"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "2db2c5f3b1ee2a44ff8d9356fd4044844bd91564",
          "message": "Update Script property value enums (#2787)",
          "timestamp": "2022-10-27T18:05:15Z",
          "tree_id": "81bd768d4610026bd3610ecf2c7fcebaaf044d55",
          "url": "https://github.com/unicode-org/icu4x/commit/2db2c5f3b1ee2a44ff8d9356fd4044844bd91564"
        },
        "date": 1666894733779,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 537,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 320,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 323,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 371,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 745,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 869,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 320,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 333,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 371,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 424,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 775,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 47162433,
            "range": "± 71109",
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
          "id": "6bf69ba0a5bb6504a18a95829d3ffdbbca414a51",
          "message": "Don't use std postcard in tests (#2795)\n\n* Don't use std postcard in tests\n\n* Bump deduplicating_array\n\n* lock",
          "timestamp": "2022-10-28T08:44:56-07:00",
          "tree_id": "0130b5737a2edbf40da254c1dcdac37223dbc618",
          "url": "https://github.com/unicode-org/icu4x/commit/6bf69ba0a5bb6504a18a95829d3ffdbbca414a51"
        },
        "date": 1666972894630,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 645,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 385,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 388,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 445,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 895,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 1042,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 385,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 388,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 445,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 507,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 931,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 56609570,
            "range": "± 278985",
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
          "id": "55a23db51dd4fed764b3854fcc794245e4e12dff",
          "message": "Exclude certain collations by default and add option to include them (#2789)",
          "timestamp": "2022-11-01T16:23:54-07:00",
          "tree_id": "976335ef4049c618bef11551f0c89cc59df041e9",
          "url": "https://github.com/unicode-org/icu4x/commit/55a23db51dd4fed764b3854fcc794245e4e12dff"
        },
        "date": 1667345866326,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 610,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 365,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 390,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 424,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 829,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 979,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 361,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 364,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 416,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 474,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 879,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 52813120,
            "range": "± 1640146",
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
            "email": "m_kato@ga2.so-net.ne.jp",
            "name": "Makoto Kato",
            "username": "makotokato"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "6b49d6193d165ad3cdc013e62e8c5dff0d043d2c",
          "message": "Use GraphemeClusterSegmenter in DictionarySegmenter and LstmSegmenter (#2716)\n\n* Use GraphemeClusterSegmenter in DictionarySegmenter and LstmSegmenter\r\n\r\n* Fix windows build error.\r\n\r\n* Use extend instead of append\r\n\r\n* Clean up calucatating complex segment per Zibi's review.\r\n\r\n* Rename to grapheme_iter.\r\n\r\n* Add python tool to convert to ICU4X's JSON.\r\n\r\n* Add shebang.",
          "timestamp": "2022-11-02T17:14:53+09:00",
          "tree_id": "1a0ea1a856c5223e1e0358de41510604d3552bb8",
          "url": "https://github.com/unicode-org/icu4x/commit/6b49d6193d165ad3cdc013e62e8c5dff0d043d2c"
        },
        "date": 1667377602593,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 335,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 218,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 230,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 263,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 438,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 462,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 218,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 230,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 263,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 297,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 446,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 37648425,
            "range": "± 419429",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/best",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains/worst",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/best",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/contains_range/worst",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "elango@unicode.org",
            "name": "Elango Cheran",
            "username": "echeran"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "a6a7c0c8c1773fd19059df0ccf14949e5119e67d",
          "message": "Add UnicodeSet that supports strings (#2796)",
          "timestamp": "2022-11-02T23:00:27Z",
          "tree_id": "ec3473233c3419ae6b0bdc08891e294b42ff1ec2",
          "url": "https://github.com/unicode-org/icu4x/commit/a6a7c0c8c1773fd19059df0ccf14949e5119e67d"
        },
        "date": 1667430933888,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 608,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 475,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 515,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 494,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 865,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 917,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 478,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 511,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 497,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 572,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 906,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 44329278,
            "range": "± 1100659",
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
          "id": "ba676cb7ffb9975fffa698f02f458ee5ca0bc75f",
          "message": "Enable experimental feature in icu in segmenter's Cargo.toml (#2803)\n\nThis patch fixed the following doc test error when running\r\n`cargo test --all-features` under `experimental/segmenter`.\r\n\r\n```\r\n---- src/lib.rs - (line 54) stdout ----\r\nerror[E0432]: unresolved import `icu::segmenter`\r\n --> src/lib.rs:55:10\r\n  |\r\n3 | use icu::segmenter::WordSegmenter;\r\n  |          ^^^^^^^^^ could not find `segmenter` in `icu`\r\n```",
          "timestamp": "2022-11-02T17:56:06-07:00",
          "tree_id": "53bebdfa6955d4854ac374a40247f39b7a614f7e",
          "url": "https://github.com/unicode-org/icu4x/commit/ba676cb7ffb9975fffa698f02f458ee5ca0bc75f"
        },
        "date": 1667437791281,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 536,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 320,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 323,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 371,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 745,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 869,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 321,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 323,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 370,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 422,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 775,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 47083769,
            "range": "± 171981",
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
            "email": "elango@unicode.org",
            "name": "Elango Cheran",
            "username": "echeran"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "86db546f2f6ad2a222b832fc8495b3ad511e8151",
          "message": "Add API and testdata for Basic_Emoji property (#2802)",
          "timestamp": "2022-11-03T21:02:24Z",
          "tree_id": "831db04dd77825083d9f231f33d16682ba700025",
          "url": "https://github.com/unicode-org/icu4x/commit/86db546f2f6ad2a222b832fc8495b3ad511e8151"
        },
        "date": 1667510247809,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 599,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 426,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 475,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 461,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 746,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 802,
            "range": "± 72",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 432,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 506,
            "range": "± 48",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 518,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 553,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 826,
            "range": "± 55",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 42138689,
            "range": "± 2026121",
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
            "email": "elango@unicode.org",
            "name": "Elango Cheran",
            "username": "echeran"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "1065865a78fd7b085dc386fca58996ab6bf7ee66",
          "message": "Update version of ICU data via icuexportdata to release-72-1 (#2788)",
          "timestamp": "2022-11-03T22:55:25Z",
          "tree_id": "b9c2d9e32f44f6ca5d6a6925177714ea554bd203",
          "url": "https://github.com/unicode-org/icu4x/commit/1065865a78fd7b085dc386fca58996ab6bf7ee66"
        },
        "date": 1667516937846,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 537,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 320,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 323,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 370,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 746,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 867,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 320,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 333,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 371,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 422,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 775,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 47169822,
            "range": "± 535876",
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
          "id": "a84fa17041b112b883962f4bc61aaa328a57af52",
          "message": "Create why_rust.md (#2811)",
          "timestamp": "2022-11-04T19:28:42-07:00",
          "tree_id": "480b38cc2a680a52628c33057a02e316bb314ff4",
          "url": "https://github.com/unicode-org/icu4x/commit/a84fa17041b112b883962f4bc61aaa328a57af52"
        },
        "date": 1667616132366,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 537,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 320,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 323,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 371,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 745,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 873,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 320,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 333,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 371,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 422,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 774,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 47157767,
            "range": "± 113071",
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
          "id": "88e8edb91585bd441bad0be4b79f871ea7fa391a",
          "message": "Add note about memory safety and code reviews",
          "timestamp": "2022-11-05T10:57:35-07:00",
          "tree_id": "8ed87b969a3ac56bce129492697c123e6a6c7d9a",
          "url": "https://github.com/unicode-org/icu4x/commit/88e8edb91585bd441bad0be4b79f871ea7fa391a"
        },
        "date": 1667671866803,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 544,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 334,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 334,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 388,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 773,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 867,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 319,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 337,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 372,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 416,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 773,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 49908120,
            "range": "± 2904939",
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
            "email": "elango@unicode.org",
            "name": "Elango Cheran",
            "username": "echeran"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "54e45952efebb56cc9ba353da117a9353a449ebc",
          "message": "Fix testdata for Basic_Emoji, `ko` collation data (#2813)\n\n* Fix merge race condition file clobbering by restoring Basic_Emoji.toml in download glob\r\n\r\n* Fix merge race condition file clobbering by regenerating ko locale collation data files",
          "timestamp": "2022-11-06T09:01:31-08:00",
          "tree_id": "49690010b1d50f137aa43eefe97fd6f026fb224f",
          "url": "https://github.com/unicode-org/icu4x/commit/54e45952efebb56cc9ba353da117a9353a449ebc"
        },
        "date": 1667754964337,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 469,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 299,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 322,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 368,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 614,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 647,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 303,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 322,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 368,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 413,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 621,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 52545559,
            "range": "± 933851",
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
          "id": "7d4a5983e0303ff9e8ea87efae0f33ea8da4eeba",
          "message": "Enable more clippy lints in segmenter (#2804)\n\n* Enable clippy::expect_used lint\r\n\r\nThis patch introduces helper methods in rule segementer and line segmenter to\r\neliminates a lot of `unwrap()` usage. This commit shouldn't change behavior.\r\n\r\n* Enable clippy::unwrap_used lint\r\n\r\n* Enable clippy::panic lint",
          "timestamp": "2022-11-06T23:13:40-08:00",
          "tree_id": "315721c95d4b5c6079ac6d0caca861e98d403a40",
          "url": "https://github.com/unicode-org/icu4x/commit/7d4a5983e0303ff9e8ea87efae0f33ea8da4eeba"
        },
        "date": 1667806059253,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 472,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 318,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 322,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 368,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 614,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 647,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 319,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 322,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 370,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 413,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 621,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 52531890,
            "range": "± 790493",
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
          "id": "9844a387f4f0b5eb57a22964d208cc5fe5c52f52",
          "message": "Update diplomat (#2819)\n\n* Update diplomat",
          "timestamp": "2022-11-07T21:34:20Z",
          "tree_id": "efad288efe31383366eec62bb3252ebf71451c75",
          "url": "https://github.com/unicode-org/icu4x/commit/9844a387f4f0b5eb57a22964d208cc5fe5c52f52"
        },
        "date": 1667857696463,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 547,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 350,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 350,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 385,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 798,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 922,
            "range": "± 54",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 338,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 338,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 387,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 441,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 805,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 49513460,
            "range": "± 2626823",
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
          "id": "3762f06ab850c54b629c0d9a7e2f29bda7b4614d",
          "message": "Add tests for zh fallback (#2818)",
          "timestamp": "2022-11-07T14:19:49-08:00",
          "tree_id": "10bf3f93da629dcc333b2050b9c41c5ef29e6d7e",
          "url": "https://github.com/unicode-org/icu4x/commit/3762f06ab850c54b629c0d9a7e2f29bda7b4614d"
        },
        "date": 1667860463171,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 470,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 316,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 323,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 368,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 614,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 651,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 318,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 323,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 368,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 413,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 621,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 52568644,
            "range": "± 808870",
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
          "id": "b6ec7663d15696b567fa5dd3d525f1aaf3541d24",
          "message": "Remove allow(clippy::expect_used) annotations in segmenter (#2817)\n\n* Refactor common pattern to implement iterators\r\n\r\nWe can share code between some of the word, grapheme clusters, and sentence\r\niterators since their differences are only the payload.\r\n\r\n* Remove allow(clippy::expect_used) annotations\r\n\r\nMake segmenter's helper methods return `Option` to eliminate `expect`, and adapt\r\ncallers.",
          "timestamp": "2022-11-09T09:22:40-08:00",
          "tree_id": "f522cd59289b80aa0fafe590c7222ac8390eae6c",
          "url": "https://github.com/unicode-org/icu4x/commit/b6ec7663d15696b567fa5dd3d525f1aaf3541d24"
        },
        "date": 1668015604726,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 621,
            "range": "± 76",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 489,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 527,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 515,
            "range": "± 48",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 893,
            "range": "± 52",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 951,
            "range": "± 90",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 486,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 528,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 512,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 580,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 912,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 44914857,
            "range": "± 1232968",
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
          "id": "7217ef7a23687232e04af4da1798f4d744853e1a",
          "message": "Move ICU4XOrdering to its own module (#2820)",
          "timestamp": "2022-11-09T19:01:31-08:00",
          "tree_id": "e52d0cecfb068cb786c5047659dd0e5868056618",
          "url": "https://github.com/unicode-org/icu4x/commit/7217ef7a23687232e04af4da1798f4d744853e1a"
        },
        "date": 1668050222528,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 543,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 320,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 333,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 368,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 738,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 858,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 318,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 328,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 386,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 420,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 773,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 46331848,
            "range": "± 1914230",
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
          "id": "46aad80e015dd1df5eb0828c9842fb9276c93cbd",
          "message": "Add tinywasm example (#2824)",
          "timestamp": "2022-11-11T11:25:09-08:00",
          "tree_id": "5c7263f93c2a2ddb576c176faaff3cd4799b252e",
          "url": "https://github.com/unicode-org/icu4x/commit/46aad80e015dd1df5eb0828c9842fb9276c93cbd"
        },
        "date": 1668195590480,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 537,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 320,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 323,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 371,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 745,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 873,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 321,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 333,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 371,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 422,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 775,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 47162771,
            "range": "± 64357",
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
          "id": "d418731826aea10cbc72d6a3b73789a981e425bd",
          "message": "Add example for iterating over the windows of a VarZeroVec (#2797)",
          "timestamp": "2022-11-14T15:29:45-08:00",
          "tree_id": "292a1168cb0b382ad84784fc32fceb392d4fa0c8",
          "url": "https://github.com/unicode-org/icu4x/commit/d418731826aea10cbc72d6a3b73789a981e425bd"
        },
        "date": 1668469505233,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 616,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 485,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 526,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 515,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 886,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 925,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 482,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 530,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 512,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 593,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 920,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 45186972,
            "range": "± 1420847",
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
          "id": "cec3f13a7861395aa092e1434a0da4a7242ea3a2",
          "message": "Adding a function to get display-name for a region. (#2816)",
          "timestamp": "2022-11-16T11:01:49-06:00",
          "tree_id": "e695e45c56a3bd281bc1e69d747dab16e78c9a2b",
          "url": "https://github.com/unicode-org/icu4x/commit/cec3f13a7861395aa092e1434a0da4a7242ea3a2"
        },
        "date": 1668619140892,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 518,
            "range": "± 48",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 411,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 454,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 390,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 671,
            "range": "± 75",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 724,
            "range": "± 85",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 370,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 399,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 400,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 465,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 788,
            "range": "± 72",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 37114572,
            "range": "± 3200048",
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
            "email": "post@ralfj.de",
            "name": "Ralf Jung",
            "username": "RalfJung"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "54876538f561785449efea762411259f49e4f404",
          "message": "FlexZeroSlize: implement Eq and PartialEq by hand (#2834)",
          "timestamp": "2022-11-18T21:16:11Z",
          "tree_id": "1f5f7323d81c9c3562b4f90b988ee15943bc1393",
          "url": "https://github.com/unicode-org/icu4x/commit/54876538f561785449efea762411259f49e4f404"
        },
        "date": 1668807097762,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 594,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 471,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 515,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 502,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 897,
            "range": "± 77",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 929,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 482,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 520,
            "range": "± 48",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 500,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 585,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 889,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 44593226,
            "range": "± 1822837",
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
          "id": "f7972c97d2aaaca62f034e3690f0f991a164c7ef",
          "message": "Bump zerovec to 0.9.1 (#2835)",
          "timestamp": "2022-11-20T12:12:50-05:00",
          "tree_id": "03f46def0cc184159a9607a44fe77e09334e1ce6",
          "url": "https://github.com/unicode-org/icu4x/commit/f7972c97d2aaaca62f034e3690f0f991a164c7ef"
        },
        "date": 1668965291544,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 458,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 320,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 325,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 372,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 612,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 646,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 322,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 322,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 372,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 411,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 634,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 50434066,
            "range": "± 949625",
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
          "id": "f23e4f9fa7c62941ba893c876c9266367a3dc734",
          "message": "Nightly cronjob CI (#2836)\n\n* Uniform indentation\r\n\r\n* wip nightly cronjob CI\r\n\r\n* Fix GN task on nightly\r\n\r\n* Pull in appropriate toolchain for memory CI\r\n\r\n* Skip c-tiny/tinywasm tests on forced nightlies\r\n\r\n* Make locale size tests work on forced nightlies\r\n\r\n* Add way to skip doctests using CFGs\r\n\r\n* fix for windows\r\n\r\n* Move to makefile\r\n\r\n* remove PR gating\r\n\r\n* fix memory",
          "timestamp": "2022-11-22T18:23:49Z",
          "tree_id": "db7bc5aa91508d0d822ac02fe02ef456c88898be",
          "url": "https://github.com/unicode-org/icu4x/commit/f23e4f9fa7c62941ba893c876c9266367a3dc734"
        },
        "date": 1669142372562,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 574,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 467,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 491,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 529,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 845,
            "range": "± 51",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 959,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 465,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 486,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 538,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 559,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 908,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 46324393,
            "range": "± 1522694",
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
            "email": "samchen61661@gmail.com",
            "name": "samchen",
            "username": "samchen61661"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b82dfd315bd17f50d23f721090e3efc837542a8f",
          "message": "Error handling in IXDTF parser (#2821)\n\n* Error handling and support time zone\r\n\r\n* error handling only",
          "timestamp": "2022-11-22T18:06:28-08:00",
          "tree_id": "c10340cc1de4c6e8f291fb94d5c5b0f7bd16c444",
          "url": "https://github.com/unicode-org/icu4x/commit/b82dfd315bd17f50d23f721090e3efc837542a8f"
        },
        "date": 1669170088549,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 464,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 320,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 322,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 372,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 609,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 646,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 319,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 322,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 372,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 411,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 634,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 50421983,
            "range": "± 943064",
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
            "email": "m_kato@ga2.so-net.ne.jp",
            "name": "Makoto Kato",
            "username": "makotokato"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "889dc290744dea36f77beaaa20229121811b654e",
          "message": "icu_segmenter: enforce clippy::indexing_slicing. (#2325)\n\n* icu_segmenter denies the clippy::indexing_slicing.\r\n\r\n* Fix clippy error",
          "timestamp": "2022-11-28T01:53:36-06:00",
          "tree_id": "4740cf8f388e26b8c04af2ccca208d605aac723f",
          "url": "https://github.com/unicode-org/icu4x/commit/889dc290744dea36f77beaaa20229121811b654e"
        },
        "date": 1669622917005,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 542,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 463,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 440,
            "range": "± 52",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 472,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 731,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 786,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 443,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 473,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 532,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 550,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 889,
            "range": "± 63",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 43562869,
            "range": "± 1775929",
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
          "id": "0f244a8f5b5c89286214ed77fb90abfa596c76c8",
          "message": "Pin wasmer-cli version (#2842)",
          "timestamp": "2022-11-28T16:56:02+01:00",
          "tree_id": "6f6d99bc30373ab47c212713072f2bf4739b3955",
          "url": "https://github.com/unicode-org/icu4x/commit/0f244a8f5b5c89286214ed77fb90abfa596c76c8"
        },
        "date": 1669651816148,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 503,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 320,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 335,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 407,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 682,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 763,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 320,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 334,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 406,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 446,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 761,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 39492277,
            "range": "± 219971",
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
          "id": "d2b9a1063052daee0c07ab13fc797a3cf12b3fbb",
          "message": "Transformer for RelativeTimeFormat component (#2822)",
          "timestamp": "2022-11-28T18:34:02Z",
          "tree_id": "bb53287be1ccd608b85980b68e9f8deff06148d3",
          "url": "https://github.com/unicode-org/icu4x/commit/d2b9a1063052daee0c07ab13fc797a3cf12b3fbb"
        },
        "date": 1669661380924,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 568,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 467,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 494,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 504,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 887,
            "range": "± 53",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 918,
            "range": "± 61",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 470,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 498,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 502,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 566,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 895,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 43413477,
            "range": "± 1860434",
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
          "id": "12311a43ddcb8a5ac04f1f36ece232c0e77926f2",
          "message": "Don't upload stuff on schedule/triggered tasks (#2841)",
          "timestamp": "2022-11-28T22:03:37Z",
          "tree_id": "e0a2651a4fb77b951cdf2224fddae2fb1101bafd",
          "url": "https://github.com/unicode-org/icu4x/commit/12311a43ddcb8a5ac04f1f36ece232c0e77926f2"
        },
        "date": 1669673915491,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 503,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 417,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 443,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 454,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 771,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 801,
            "range": "± 51",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 410,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 438,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 459,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 513,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 794,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 37936343,
            "range": "± 1323906",
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
          "id": "bd8c66d06b4d0ca0d155aaee43dd141952fbb9ae",
          "message": "Fixing datagen invocation (#2843)",
          "timestamp": "2022-11-29T12:47:03+01:00",
          "tree_id": "4fa6c1feb097b143213e0502be5f65435b90068c",
          "url": "https://github.com/unicode-org/icu4x/commit/bd8c66d06b4d0ca0d155aaee43dd141952fbb9ae"
        },
        "date": 1669723359824,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 538,
            "range": "± 48",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 438,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 462,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 494,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 823,
            "range": "± 261",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 946,
            "range": "± 110",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 470,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 475,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 510,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 538,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 841,
            "range": "± 111",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 44239875,
            "range": "± 2324875",
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
          "id": "169f2248481fcd1d038f4ff4134f553a87d083c5",
          "message": "populate future plural rules mapping (#2848)",
          "timestamp": "2022-11-30T19:22:31Z",
          "tree_id": "0b4537797293dbd263670c0e73197c41d5bc5d32",
          "url": "https://github.com/unicode-org/icu4x/commit/169f2248481fcd1d038f4ff4134f553a87d083c5"
        },
        "date": 1669837079656,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 600,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 382,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 419,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 488,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 816,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 915,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 383,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 423,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 488,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 534,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 913,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 47166119,
            "range": "± 325128",
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
          "id": "ee8f0aba420561083ce345e9979aadfc009c1983",
          "message": "`#[no_std]` for LSTM segmenter (#2845)",
          "timestamp": "2022-12-01T11:44:52+01:00",
          "tree_id": "b43a14e3dc283952b5fabb38e084009f9305db2c",
          "url": "https://github.com/unicode-org/icu4x/commit/ee8f0aba420561083ce345e9979aadfc009c1983"
        },
        "date": 1669892414059,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 567,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 465,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 504,
            "range": "± 64",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 509,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 867,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 869,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 460,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 516,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 508,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 566,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 841,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 43147120,
            "range": "± 1907394",
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
          "id": "fafee0df273ee2b6ff173ce95724eb8205ee942e",
          "message": "Fix top-level readme example (#2858)",
          "timestamp": "2022-12-01T19:34:25Z",
          "tree_id": "f1948ce1bf3de2ae19981023caa62b430cd58cc8",
          "url": "https://github.com/unicode-org/icu4x/commit/fafee0df273ee2b6ff173ce95724eb8205ee942e"
        },
        "date": 1669924106710,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 459,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 320,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 317,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 372,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 595,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 643,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 298,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 316,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 371,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 408,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 627,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 50364362,
            "range": "± 997872",
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
          "id": "7050db4ba86c554520a169bdb3d523e18af12ffc",
          "message": "Readable JSON inversion lists (#2855)",
          "timestamp": "2022-12-02T13:29:33+01:00",
          "tree_id": "36efa6cb879c1bb56a7e752db8b5b5aa2e5868dc",
          "url": "https://github.com/unicode-org/icu4x/commit/7050db4ba86c554520a169bdb3d523e18af12ffc"
        },
        "date": 1669985016754,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 459,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 298,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 316,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 371,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 596,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 643,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 297,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 316,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 371,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 408,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 627,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 50604093,
            "range": "± 877482",
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
          "id": "2c94820406ddc22989c9bf054af474df9b8cbd7e",
          "message": "Fix sv_reformed warning (#2859)",
          "timestamp": "2022-12-02T18:03:43+01:00",
          "tree_id": "8fb721ff0fff518c5b3d23bc577e1686daf448ec",
          "url": "https://github.com/unicode-org/icu4x/commit/2c94820406ddc22989c9bf054af474df9b8cbd7e"
        },
        "date": 1670001540512,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 503,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 320,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 334,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 407,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 682,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 762,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 320,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 334,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 407,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 447,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 761,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 39617128,
            "range": "± 242646",
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
            "email": "elango@unicode.org",
            "name": "Elango Cheran",
            "username": "echeran"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "28a6b68a1d327af33655ff25464eebef971d3eab",
          "message": "Add APIs for returning exemplar characters data (#2812)",
          "timestamp": "2022-12-02T09:07:11-08:00",
          "tree_id": "7ae30537846291ce7161316383d7d7eb5d566e29",
          "url": "https://github.com/unicode-org/icu4x/commit/28a6b68a1d327af33655ff25464eebef971d3eab"
        },
        "date": 1670001772166,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 503,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 320,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 335,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 411,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 682,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 764,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 320,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 335,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 411,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 447,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 1028,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 39593957,
            "range": "± 61370",
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
            "email": "kelebra20@gmail.com",
            "name": "Oleksii Tkachuk",
            "username": "kelebra"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "d474a7997a9a408a45791e29a9308feb63043689",
          "message": "Address last for/vec.push case in favor of vec.extend (#2861)",
          "timestamp": "2022-12-03T18:01:30+02:00",
          "tree_id": "4d8b20bdaa531c376511eb31333a77b2e310466b",
          "url": "https://github.com/unicode-org/icu4x/commit/d474a7997a9a408a45791e29a9308feb63043689"
        },
        "date": 1670084141720,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 460,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 299,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 323,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 371,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 594,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 643,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 297,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 316,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 370,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 409,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 626,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 50546963,
            "range": "± 834390",
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
          "id": "17966ef8e24664a7414e46528095c061b6987e15",
          "message": "Add debug_split_at to style guide (#2854)",
          "timestamp": "2022-12-05T10:48:34+01:00",
          "tree_id": "72263c3975873f3f3e6dcd61d1b3d5936a7c5bf4",
          "url": "https://github.com/unicode-org/icu4x/commit/17966ef8e24664a7414e46528095c061b6987e15"
        },
        "date": 1670234577388,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 504,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 320,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 324,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 407,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 682,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 762,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 320,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 335,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 407,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 447,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 761,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 39356785,
            "range": "± 44909",
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
          "id": "95d1d3736d7d4f2197b11fd813ed29290a402458",
          "message": "Add RelativeTimeFormatter (#2846)\n\n* Add RelativeTimeFormatter\n\n* Minor changes\n\n- Fix negative number checking\n- Move lib test to doc test\n- Fix docs in relativetime/src/error.rs\n- Change format method to take Into<FixedDecimal>\n\n* Changes\n\n- Make \"dateFields.other\" a non-optional field\n- This removes the 'expect' in FormattedRelativeTime.format\n- Replace indexing slicing by 'split_at'\n- Add tests in cldr/relativetime for sentinel index (pattern without\n  placeholder)\n\n* Path for Numeric::Auto\n\n- Handle Numeric::Auto\n- Add doc tests\n\n* Change in deps, RelativeTimeFormatter api\n\n- Remove displaynames component from deps\n- Make RelativeTimeFormatter.format take FixedDecimal arg\n- Fix clippy error\n\n* Remove split_at function\n\n* store is_negative and don't clone in format\n\n* Fix relative time path\n\n* update missing_apis.txt\n\n* update testdata",
          "timestamp": "2022-12-05T16:49:06+05:30",
          "tree_id": "b6e1bdd647c30066cf423d65a4072ac599d95f47",
          "url": "https://github.com/unicode-org/icu4x/commit/95d1d3736d7d4f2197b11fd813ed29290a402458"
        },
        "date": 1670239992344,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 460,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 298,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 323,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 371,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 598,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 643,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 297,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 317,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 371,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 408,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 625,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 50514099,
            "range": "± 952075",
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
            "email": "egg.robin.leroy@gmail.com",
            "name": "Robin Leroy",
            "username": "eggrobin"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "967898f219a221dea3e9525c9ab8d6bb4d085cd2",
          "message": "CompactDecimal and ScientificDecimal (#2847)",
          "timestamp": "2022-12-05T14:39:57+01:00",
          "tree_id": "af6fc03da58b4b75e976a399065f52473d2f226c",
          "url": "https://github.com/unicode-org/icu4x/commit/967898f219a221dea3e9525c9ab8d6bb4d085cd2"
        },
        "date": 1670248596156,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 502,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 320,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 335,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 407,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 682,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 763,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 322,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 335,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 407,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 446,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 761,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 39628252,
            "range": "± 140959",
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
          "id": "583207162d72f50365234e14a80863da1dd372d8",
          "message": "Use rustup run instead of +nightly in ffi-coverage (#2865)",
          "timestamp": "2022-12-05T13:52:23Z",
          "tree_id": "ea80c9403e92e0d2cd261a01bdd2e249a0ab0b3b",
          "url": "https://github.com/unicode-org/icu4x/commit/583207162d72f50365234e14a80863da1dd372d8"
        },
        "date": 1670249263067,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 597,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 389,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 406,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 481,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 819,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 914,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 393,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 406,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 484,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 534,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 912,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 47846434,
            "range": "± 1090236",
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
          "id": "1d9c42cd543c08d2218e1fa99ca7fdd26fafe084",
          "message": "Remove ICU4XCreateDataProviderResult from FFI (#2866)",
          "timestamp": "2022-12-05T16:44:36Z",
          "tree_id": "eb407ee266d0363679c380897741974ee1c080fd",
          "url": "https://github.com/unicode-org/icu4x/commit/1d9c42cd543c08d2218e1fa99ca7fdd26fafe084"
        },
        "date": 1670259678730,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 603,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 382,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 410,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 476,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 731,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 830,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 366,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 378,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 447,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 509,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 826,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 44639791,
            "range": "± 2047948",
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
            "email": "m_kato@ga2.so-net.ne.jp",
            "name": "Makoto Kato",
            "username": "makotokato"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "40835f7721d3c60aa751b9910bfcded6a0ea5103",
          "message": "Store grapheme cluster payload instead of grapheme cluster segmenter. (#2864)",
          "timestamp": "2022-12-05T11:16:36-08:00",
          "tree_id": "2752c6ac5ee234c48b719f0f586f79d70d7bf6a0",
          "url": "https://github.com/unicode-org/icu4x/commit/40835f7721d3c60aa751b9910bfcded6a0ea5103"
        },
        "date": 1670268671239,
        "tool": "cargo",
        "benches": [
          {
            "name": "cpt/overview",
            "value": 502,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/eng",
            "value": 320,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/pcd",
            "value": 335,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ukr",
            "value": 407,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/yue",
            "value": 681,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/small/ccp",
            "value": 763,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/eng",
            "value": 320,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/pcd",
            "value": 335,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ukr",
            "value": 407,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/yue",
            "value": 446,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cpt/get/fast/ccp",
            "value": 761,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "uniset/overview",
            "value": 39613652,
            "range": "± 62581",
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