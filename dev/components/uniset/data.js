window.BENCHMARK_DATA = {
  "lastUpdate": 1602734895535,
  "repoUrl": "https://github.com/unicode-org/icu4x",
  "entries": {
    "Rust Benchmark": [
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
          "id": "30ca95f5759b4e7718de9d7782fa3e7cae40d5ed",
          "message": "Fix API docs and benchmark dashboards (#314)",
          "timestamp": "2020-10-09T12:46:07-05:00",
          "tree_id": "6e6f52c286d1182a46a78f9a7f80e4998e932c00",
          "url": "https://github.com/unicode-org/icu4x/commit/30ca95f5759b4e7718de9d7782fa3e7cae40d5ed"
        },
        "date": 1602266075868,
        "tool": "cargo",
        "benches": [
          {
            "name": "isize/smaller",
            "value": 28870,
            "range": "± 2421",
            "unit": "ns/iter"
          },
          {
            "name": "isize/larger",
            "value": 68594,
            "range": "± 6178",
            "unit": "ns/iter"
          },
          {
            "name": "to_string/908070605040302010",
            "value": 259,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "write_to/908070605040302010",
            "value": 129,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "from_string/1000000001",
            "value": 88,
            "range": "± 9",
            "unit": "ns/iter"
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
          "id": "fa8e1f40afd7a650a5900615eab8c741a24767a7",
          "message": "Add ICU Meta Package for 0.1 (#306)\n\n* Add ICU Meta Package for 0.1\r\n\r\n* Move to use shared testdata\r\n\r\n* Switch to testdata",
          "timestamp": "2020-10-09T11:51:42-07:00",
          "tree_id": "fe9dacbc4c5b993db0c3324a3e057a4ee3f9563f",
          "url": "https://github.com/unicode-org/icu4x/commit/fa8e1f40afd7a650a5900615eab8c741a24767a7"
        },
        "date": 1602269983533,
        "tool": "cargo",
        "benches": [
          {
            "name": "isize/smaller",
            "value": 31724,
            "range": "± 880",
            "unit": "ns/iter"
          },
          {
            "name": "isize/larger",
            "value": 77278,
            "range": "± 1995",
            "unit": "ns/iter"
          },
          {
            "name": "to_string/908070605040302010",
            "value": 346,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "write_to/908070605040302010",
            "value": 122,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "from_string/1000000001",
            "value": 84,
            "range": "± 3",
            "unit": "ns/iter"
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
          "id": "5f9af945d26a668ff3898b44db74a35fa44a1280",
          "message": "Use testdata everywhere (#311)\n\n* Use testdata everywhere\r\n\r\n* Use icu_testdata\r\n\r\n* Apply feedback\r\n\r\n* Fix missing semicolon\r\n\r\n* Fix fs-data-provider test",
          "timestamp": "2020-10-09T13:10:47-07:00",
          "tree_id": "c2a3953a0a719bee0d4c840804618a9febff96f2",
          "url": "https://github.com/unicode-org/icu4x/commit/5f9af945d26a668ff3898b44db74a35fa44a1280"
        },
        "date": 1602274741960,
        "tool": "cargo",
        "benches": [
          {
            "name": "isize/smaller",
            "value": 33934,
            "range": "± 510",
            "unit": "ns/iter"
          },
          {
            "name": "isize/larger",
            "value": 80498,
            "range": "± 1464",
            "unit": "ns/iter"
          },
          {
            "name": "to_string/908070605040302010",
            "value": 293,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "write_to/908070605040302010",
            "value": 142,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "from_string/1000000001",
            "value": 91,
            "range": "± 2",
            "unit": "ns/iter"
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
          "id": "e3cfe0076e1090be65c1d45f9828945e3d14b6fb",
          "message": "Rename icu-pluralrules -> icu-plurals and icu-unicodeset -> icu-uniset (#323)",
          "timestamp": "2020-10-10T16:18:26-07:00",
          "tree_id": "33247e15156fabf01a869386f41e2b7c604ea977",
          "url": "https://github.com/unicode-org/icu4x/commit/e3cfe0076e1090be65c1d45f9828945e3d14b6fb"
        },
        "date": 1602372434842,
        "tool": "cargo",
        "benches": [
          {
            "name": "isize/smaller",
            "value": 32724,
            "range": "± 1629",
            "unit": "ns/iter"
          },
          {
            "name": "isize/larger",
            "value": 77723,
            "range": "± 4830",
            "unit": "ns/iter"
          },
          {
            "name": "to_string/908070605040302010",
            "value": 287,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "write_to/908070605040302010",
            "value": 143,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "from_string/1000000001",
            "value": 89,
            "range": "± 5",
            "unit": "ns/iter"
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
          "id": "b8b68b61b3372debda6911547f8fafd5bfcc08e2",
          "message": "Run CI bench command per component (#328)",
          "timestamp": "2020-10-12T12:16:42-07:00",
          "tree_id": "1c1a87a7f25c4274b33bb9240b355444586557db",
          "url": "https://github.com/unicode-org/icu4x/commit/b8b68b61b3372debda6911547f8fafd5bfcc08e2"
        },
        "date": 1602530414410,
        "tool": "cargo",
        "benches": [
          {
            "name": "uniset/overview",
            "value": 36909455,
            "range": "± 1075061",
            "unit": "ns/iter"
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
          "id": "cc50a71267baf23db4b636b2f9fcc147dcc8eb38",
          "message": "Fix locale macros to remove dependency on TinyStr (#337)",
          "timestamp": "2020-10-13T11:42:33-05:00",
          "tree_id": "e635af6a44159c7986364c1b293856de5518ec7a",
          "url": "https://github.com/unicode-org/icu4x/commit/cc50a71267baf23db4b636b2f9fcc147dcc8eb38"
        },
        "date": 1602607589211,
        "tool": "cargo",
        "benches": [
          {
            "name": "uniset/overview",
            "value": 37965092,
            "range": "± 101675",
            "unit": "ns/iter"
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
          "id": "dabd9f259ce3f9cddc0407ebbdaa721098d36238",
          "message": "Use proc-macro-crate to handle icu vs icu_locale (#338)",
          "timestamp": "2020-10-13T10:00:42-07:00",
          "tree_id": "698bee642e47433193a1a86757f0141699f13a49",
          "url": "https://github.com/unicode-org/icu4x/commit/dabd9f259ce3f9cddc0407ebbdaa721098d36238"
        },
        "date": 1602608630939,
        "tool": "cargo",
        "benches": [
          {
            "name": "uniset/overview",
            "value": 28785664,
            "range": "± 1395091",
            "unit": "ns/iter"
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
          "id": "24b22c9992e3ac8666bf6c267a07d5711d058b78",
          "message": "remove cache step in coverage check in CI (#331)",
          "timestamp": "2020-10-13T10:31:25-07:00",
          "tree_id": "3698779d37499e0588c3c551ef4687511a2d242d",
          "url": "https://github.com/unicode-org/icu4x/commit/24b22c9992e3ac8666bf6c267a07d5711d058b78"
        },
        "date": 1602610472589,
        "tool": "cargo",
        "benches": [
          {
            "name": "uniset/overview",
            "value": 30059995,
            "range": "± 1759078",
            "unit": "ns/iter"
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
            "email": "zibi@braniecki.net",
            "name": "Zibi Braniecki",
            "username": "zbraniecki"
          },
          "distinct": true,
          "id": "0db49de698fa174a86d745ec7f331639f2fefaf3",
          "message": "Fix clippy warning",
          "timestamp": "2020-10-13T11:20:42-07:00",
          "tree_id": "34cae13baf2a3ff2d8ab09c8acaa0cf93fe94bff",
          "url": "https://github.com/unicode-org/icu4x/commit/0db49de698fa174a86d745ec7f331639f2fefaf3"
        },
        "date": 1602613510594,
        "tool": "cargo",
        "benches": [
          {
            "name": "uniset/overview",
            "value": 34188674,
            "range": "± 1555026",
            "unit": "ns/iter"
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
          "id": "f05a276e1126f330deaa014e298f9771da90696c",
          "message": "Use langid macros everywhere. (#341)",
          "timestamp": "2020-10-13T12:06:42-07:00",
          "tree_id": "fe77378dc2aca94cca1d5ff1a3d2064cfac61c26",
          "url": "https://github.com/unicode-org/icu4x/commit/f05a276e1126f330deaa014e298f9771da90696c"
        },
        "date": 1602616238817,
        "tool": "cargo",
        "benches": [
          {
            "name": "uniset/overview",
            "value": 34270426,
            "range": "± 1600228",
            "unit": "ns/iter"
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
          "id": "88d531bf2d28ff57e2229fa177d68f03a41a161d",
          "message": "temporarily disable benchmark test for utils/fixed-decimal (#344)",
          "timestamp": "2020-10-13T12:17:48-07:00",
          "tree_id": "838a6a93cba406411330ccb2db23ea3141a82b0d",
          "url": "https://github.com/unicode-org/icu4x/commit/88d531bf2d28ff57e2229fa177d68f03a41a161d"
        },
        "date": 1602616944511,
        "tool": "cargo",
        "benches": [
          {
            "name": "uniset/overview",
            "value": 34875478,
            "range": "± 3585160",
            "unit": "ns/iter"
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
          "id": "9bd2b9ff50247c760e7bf99d6ccaca70277a0021",
          "message": "Fix coverage CI (#343)",
          "timestamp": "2020-10-13T12:30:02-07:00",
          "tree_id": "c69bf53421e6a78f41381c065983a45cdc20261a",
          "url": "https://github.com/unicode-org/icu4x/commit/9bd2b9ff50247c760e7bf99d6ccaca70277a0021"
        },
        "date": 1602617637004,
        "tool": "cargo",
        "benches": [
          {
            "name": "uniset/overview",
            "value": 33492127,
            "range": "± 1036892",
            "unit": "ns/iter"
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
          "id": "b0f9de3c1d255650db5c0273c727d9def978b26c",
          "message": "temporarily disable flaky unit test (#345)",
          "timestamp": "2020-10-13T12:50:28-07:00",
          "tree_id": "d5ee40a306f3767cbc1d03bf5044d4b53378bc89",
          "url": "https://github.com/unicode-org/icu4x/commit/b0f9de3c1d255650db5c0273c727d9def978b26c"
        },
        "date": 1602618836790,
        "tool": "cargo",
        "benches": [
          {
            "name": "uniset/overview",
            "value": 36420917,
            "range": "± 130500",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "filmil@gmail.com",
            "name": "Filip Filmar",
            "username": "filmil"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "04478b52eaf8b68bd1c901bfeb74ef1f8efb49c7",
          "message": "Implements ecma402_traits with icu4x. (#271)\n\nThis allows (almost) seamless switching between `rust_icu` and\r\nicu4x for plural formatting.\r\n\r\nThe API surface is a bit different with ecma402 than icu4x, which\r\nmay be a necessary concession to API uniformity.\r\n\r\nFor the time being, the significant digits in the formatted number\r\nare not handled completely, so, for example, excessive precision will\r\nnot be handled correctly.\r\n\r\nAlso for the time being, no other APIs defined in `ecma402_traits` have\r\na icu4x implementation.  Plural rules formatting was chosen as an easy\r\ndemo of the common ECMA402 API.",
          "timestamp": "2020-10-13T13:19:05-07:00",
          "tree_id": "3f58fd776d2526c5384ac9bb4ed7a072062165d7",
          "url": "https://github.com/unicode-org/icu4x/commit/04478b52eaf8b68bd1c901bfeb74ef1f8efb49c7"
        },
        "date": 1602620554337,
        "tool": "cargo",
        "benches": [
          {
            "name": "uniset/overview",
            "value": 36984109,
            "range": "± 63084",
            "unit": "ns/iter"
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
          "id": "99b68b1b9d1e086b515026473adb457299050b6d",
          "message": "Initial commit with bincode support (#321)\n\n* Initial commit with bincode support\r\n\r\n* Running cargo fmt\r\n\r\n* Fixing bincode errors.",
          "timestamp": "2020-10-13T13:30:42-07:00",
          "tree_id": "5a4498c6922aa1ce18230540b03394bfac8d392d",
          "url": "https://github.com/unicode-org/icu4x/commit/99b68b1b9d1e086b515026473adb457299050b6d"
        },
        "date": 1602621246735,
        "tool": "cargo",
        "benches": [
          {
            "name": "uniset/overview",
            "value": 30829614,
            "range": "± 1293178",
            "unit": "ns/iter"
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
            "email": "zibi@braniecki.net",
            "name": "Zibi Braniecki",
            "username": "zbraniecki"
          },
          "distinct": true,
          "id": "f8de2adf11e3dd8964bc3b536811f2b0883a5e38",
          "message": "Fix fs-data-provider bincode",
          "timestamp": "2020-10-13T13:31:35-07:00",
          "tree_id": "c2fc0dae9ff48c49a12ad2a7418eb449c11b5eb0",
          "url": "https://github.com/unicode-org/icu4x/commit/f8de2adf11e3dd8964bc3b536811f2b0883a5e38"
        },
        "date": 1602621337988,
        "tool": "cargo",
        "benches": [
          {
            "name": "uniset/overview",
            "value": 34141805,
            "range": "± 1321295",
            "unit": "ns/iter"
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
            "email": "zibi@braniecki.net",
            "name": "Zibi Braniecki",
            "username": "zbraniecki"
          },
          "distinct": true,
          "id": "887743d7479e6de7167a99da79293b376af51697",
          "message": "Rename icu-locale to icu-locid",
          "timestamp": "2020-10-13T13:49:19-07:00",
          "tree_id": "144cff6e20b2e9cd076076ec43f1d85e431cdf48",
          "url": "https://github.com/unicode-org/icu4x/commit/887743d7479e6de7167a99da79293b376af51697"
        },
        "date": 1602622380929,
        "tool": "cargo",
        "benches": [
          {
            "name": "uniset/overview",
            "value": 30505172,
            "range": "± 1941997",
            "unit": "ns/iter"
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
            "email": "zibi@braniecki.net",
            "name": "Zibi Braniecki",
            "username": "zbraniecki"
          },
          "distinct": true,
          "id": "6fddde66b8f481ae480d988f9664773f1de8d80e",
          "message": "Hotfix github workflow",
          "timestamp": "2020-10-13T13:51:51-07:00",
          "tree_id": "66b95d6457876f8388c27c053225371a3e838a30",
          "url": "https://github.com/unicode-org/icu4x/commit/6fddde66b8f481ae480d988f9664773f1de8d80e"
        },
        "date": 1602622548651,
        "tool": "cargo",
        "benches": [
          {
            "name": "uniset/overview",
            "value": 33755331,
            "range": "± 1006799",
            "unit": "ns/iter"
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
            "email": "zibi@braniecki.net",
            "name": "Zibi Braniecki",
            "username": "zbraniecki"
          },
          "distinct": true,
          "id": "28674c5a4a27ffba89ef4f195bdc5c4d64df4c98",
          "message": "Replace - with _ in crate and directory names",
          "timestamp": "2020-10-13T14:05:47-07:00",
          "tree_id": "f6994abc00a3c1148b74a74916da5d41dc8b874f",
          "url": "https://github.com/unicode-org/icu4x/commit/28674c5a4a27ffba89ef4f195bdc5c4d64df4c98"
        },
        "date": 1602623367716,
        "tool": "cargo",
        "benches": [
          {
            "name": "uniset/overview",
            "value": 35538389,
            "range": "± 776198",
            "unit": "ns/iter"
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
          "id": "764c9695a018fdfba767c03d0a8426384b93ba1b",
          "message": "Replace icu_data_provider and friends with icu_provider (#347)",
          "timestamp": "2020-10-13T18:43:15-05:00",
          "tree_id": "a47e616ac505743a8abda324438826b409f02739",
          "url": "https://github.com/unicode-org/icu4x/commit/764c9695a018fdfba767c03d0a8426384b93ba1b"
        },
        "date": 1602632825227,
        "tool": "cargo",
        "benches": [
          {
            "name": "uniset/overview",
            "value": 33020873,
            "range": "± 1962672",
            "unit": "ns/iter"
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
            "email": "zibi@braniecki.net",
            "name": "Zibi Braniecki",
            "username": "zbraniecki"
          },
          "distinct": true,
          "id": "8725ff5a766fb406619601115d1eaf7816caf9c0",
          "message": "Rename icu::locale to icu::locid",
          "timestamp": "2020-10-13T18:05:20-07:00",
          "tree_id": "bdbe2c5b62df9626384dc180c7863049492aa722",
          "url": "https://github.com/unicode-org/icu4x/commit/8725ff5a766fb406619601115d1eaf7816caf9c0"
        },
        "date": 1602637722298,
        "tool": "cargo",
        "benches": [
          {
            "name": "uniset/overview",
            "value": 38067712,
            "range": "± 1349823",
            "unit": "ns/iter"
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
            "email": "zibi@braniecki.net",
            "name": "Zibi Braniecki",
            "username": "zbraniecki"
          },
          "distinct": true,
          "id": "33ef9644f0cde5f7a44ea2b0de43d19a41750e81",
          "message": "Hot-fix for a proc macro",
          "timestamp": "2020-10-13T18:12:55-07:00",
          "tree_id": "6bf5822bf7a2d9bc2543a0c5d6c81fdab0d64de7",
          "url": "https://github.com/unicode-org/icu4x/commit/33ef9644f0cde5f7a44ea2b0de43d19a41750e81"
        },
        "date": 1602638260429,
        "tool": "cargo",
        "benches": [
          {
            "name": "uniset/overview",
            "value": 36625581,
            "range": "± 102531",
            "unit": "ns/iter"
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
            "email": "zibi@braniecki.net",
            "name": "Zibi Braniecki",
            "username": "zbraniecki"
          },
          "distinct": true,
          "id": "9013960948d6d3251e880969e0fee933bc1a7681",
          "message": "Clippy nursery and pedantic lints - almost all are Self and docs.",
          "timestamp": "2020-10-14T09:18:05-07:00",
          "tree_id": "be801716393c689d188d64a89fd9d90f8ad0a26a",
          "url": "https://github.com/unicode-org/icu4x/commit/9013960948d6d3251e880969e0fee933bc1a7681"
        },
        "date": 1602692793059,
        "tool": "cargo",
        "benches": [
          {
            "name": "uniset/overview",
            "value": 34760595,
            "range": "± 1299520",
            "unit": "ns/iter"
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
            "email": "zibi@braniecki.net",
            "name": "Zibi Braniecki",
            "username": "zbraniecki"
          },
          "distinct": true,
          "id": "038531324236e1b45a4b7c4a47c30aef9f1b91bd",
          "message": "Remove Error postfix from Errors.",
          "timestamp": "2020-10-14T09:41:37-07:00",
          "tree_id": "98a6ee4f8f23eec30e4fd63ecac6587f24040fbf",
          "url": "https://github.com/unicode-org/icu4x/commit/038531324236e1b45a4b7c4a47c30aef9f1b91bd"
        },
        "date": 1602693914328,
        "tool": "cargo",
        "benches": [
          {
            "name": "uniset/overview",
            "value": 34389659,
            "range": "± 1112064",
            "unit": "ns/iter"
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
            "email": "zibi@braniecki.net",
            "name": "Zibi Braniecki",
            "username": "zbraniecki"
          },
          "distinct": true,
          "id": "8c81d6dfcf270533c2836b9ecd32a98dc5e8124b",
          "message": "API doc cleanups for main components",
          "timestamp": "2020-10-14T10:36:33-07:00",
          "tree_id": "aa36b41e675c8815ee1d4e4f90bfa4ea4e18f537",
          "url": "https://github.com/unicode-org/icu4x/commit/8c81d6dfcf270533c2836b9ecd32a98dc5e8124b"
        },
        "date": 1602697200941,
        "tool": "cargo",
        "benches": [
          {
            "name": "uniset/overview",
            "value": 25925438,
            "range": "± 986759",
            "unit": "ns/iter"
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
            "email": "zibi@braniecki.net",
            "name": "Zibi Braniecki",
            "username": "zbraniecki"
          },
          "distinct": true,
          "id": "4025a90052299ee6fa8f35465611ccf3f63fe9ae",
          "message": "Hot-fix to benchmark",
          "timestamp": "2020-10-14T10:50:09-07:00",
          "tree_id": "c7cd05cbb8a9b5488ecf0ace78dcf605e6508ce3",
          "url": "https://github.com/unicode-org/icu4x/commit/4025a90052299ee6fa8f35465611ccf3f63fe9ae"
        },
        "date": 1602697999111,
        "tool": "cargo",
        "benches": [
          {
            "name": "uniset/overview",
            "value": 25089836,
            "range": "± 1755288",
            "unit": "ns/iter"
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
            "email": "zibi@braniecki.net",
            "name": "Zibi Braniecki",
            "username": "zbraniecki"
          },
          "distinct": true,
          "id": "3aa5b0ea32d019342fddf51f6f07be0704ce57d7",
          "message": "API docs for providers",
          "timestamp": "2020-10-14T11:28:20-07:00",
          "tree_id": "58626ef5c81c8b18856431f87b37501d9b90a016",
          "url": "https://github.com/unicode-org/icu4x/commit/3aa5b0ea32d019342fddf51f6f07be0704ce57d7"
        },
        "date": 1602700320498,
        "tool": "cargo",
        "benches": [
          {
            "name": "uniset/overview",
            "value": 36527957,
            "range": "± 559184",
            "unit": "ns/iter"
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
          "id": "4f0790dd12aca14305e2519deff172f3d8c35469",
          "message": "Update README.md",
          "timestamp": "2020-10-14T13:48:16-07:00",
          "tree_id": "49ec68de555d39c3529f13d810424720d39868f9",
          "url": "https://github.com/unicode-org/icu4x/commit/4f0790dd12aca14305e2519deff172f3d8c35469"
        },
        "date": 1602708708330,
        "tool": "cargo",
        "benches": [
          {
            "name": "uniset/overview",
            "value": 36424299,
            "range": "± 732208",
            "unit": "ns/iter"
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
          "id": "a960563d6ad2dd5d76a5e73af7bb33f0a1877bf8",
          "message": "Refactor the DateTime pattern parser to handle quotes. (#336)\n\n* Refactor the DateTime pattern parser to handle quotes.\r\n\r\n* Refactor Parser\r\n\r\n* Switch several places to use Self",
          "timestamp": "2020-10-14T18:39:36-07:00",
          "tree_id": "5505bb1791e9c3454a1afe53751fd0f42b134182",
          "url": "https://github.com/unicode-org/icu4x/commit/a960563d6ad2dd5d76a5e73af7bb33f0a1877bf8"
        },
        "date": 1602726199815,
        "tool": "cargo",
        "benches": [
          {
            "name": "uniset/overview",
            "value": 36583721,
            "range": "± 98517",
            "unit": "ns/iter"
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
          "id": "b1fac4565d04cf5c55c689c449b359ffea10c411",
          "message": "Add author line to LICENSE (#350)",
          "timestamp": "2020-10-14T20:40:31-05:00",
          "tree_id": "3dfe935bd859a464cbfc6356952d8981c6aad96e",
          "url": "https://github.com/unicode-org/icu4x/commit/b1fac4565d04cf5c55c689c449b359ffea10c411"
        },
        "date": 1602726245754,
        "tool": "cargo",
        "benches": [
          {
            "name": "uniset/overview",
            "value": 30729236,
            "range": "± 1664513",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "zbraniecki@mozilla.com",
            "name": "Zibi Braniecki",
            "username": "zbraniecki"
          },
          "committer": {
            "email": "zbraniecki@mozilla.com",
            "name": "Zibi Braniecki",
            "username": "zbraniecki"
          },
          "distinct": true,
          "id": "16345fe929d03167a5e2dd11c6dc13a593ee1e74",
          "message": "More clippy cleanups",
          "timestamp": "2020-10-14T19:56:50-07:00",
          "tree_id": "99cf4a3ad4f72ac27152dd2827ae235b0fbbe5aa",
          "url": "https://github.com/unicode-org/icu4x/commit/16345fe929d03167a5e2dd11c6dc13a593ee1e74"
        },
        "date": 1602730832577,
        "tool": "cargo",
        "benches": [
          {
            "name": "uniset/overview",
            "value": 36396623,
            "range": "± 381433",
            "unit": "ns/iter"
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
          "id": "77328b4e2dd785e49904d10b577646a248f8ae63",
          "message": "Update README.md",
          "timestamp": "2020-10-14T20:10:50-07:00",
          "tree_id": "5a9103ac82121f1af27d656489cdda4beaf35890",
          "url": "https://github.com/unicode-org/icu4x/commit/77328b4e2dd785e49904d10b577646a248f8ae63"
        },
        "date": 1602731650651,
        "tool": "cargo",
        "benches": [
          {
            "name": "uniset/overview",
            "value": 26931338,
            "range": "± 1901735",
            "unit": "ns/iter"
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
          "id": "d7f0dea144b03040199a7049f91f41d51a8ff4b9",
          "message": "Update README.md",
          "timestamp": "2020-10-14T20:15:52-07:00",
          "tree_id": "21dab36e350f57f56773782d3e00f41424fe1c81",
          "url": "https://github.com/unicode-org/icu4x/commit/d7f0dea144b03040199a7049f91f41d51a8ff4b9"
        },
        "date": 1602731982102,
        "tool": "cargo",
        "benches": [
          {
            "name": "uniset/overview",
            "value": 31471400,
            "range": "± 1734225",
            "unit": "ns/iter"
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
          "id": "d1d0124bd522f96dae72653e4fb97a76181a5d35",
          "message": "Update README.md",
          "timestamp": "2020-10-14T20:17:36-07:00",
          "tree_id": "aed1c8033b5298fa4504b0e62e987356471b117b",
          "url": "https://github.com/unicode-org/icu4x/commit/d1d0124bd522f96dae72653e4fb97a76181a5d35"
        },
        "date": 1602732043054,
        "tool": "cargo",
        "benches": [
          {
            "name": "uniset/overview",
            "value": 31980537,
            "range": "± 1257946",
            "unit": "ns/iter"
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
          "id": "57d505091bc9d54aa481bb0331ad4d1e022a96c7",
          "message": "Update README.md",
          "timestamp": "2020-10-14T20:20:10-07:00",
          "tree_id": "038ca4c016f92df0200943b0debee88f94ec082a",
          "url": "https://github.com/unicode-org/icu4x/commit/57d505091bc9d54aa481bb0331ad4d1e022a96c7"
        },
        "date": 1602732213832,
        "tool": "cargo",
        "benches": [
          {
            "name": "uniset/overview",
            "value": 36241567,
            "range": "± 369132",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "zbraniecki@mozilla.com",
            "name": "Zibi Braniecki",
            "username": "zbraniecki"
          },
          "committer": {
            "email": "zbraniecki@mozilla.com",
            "name": "Zibi Braniecki",
            "username": "zbraniecki"
          },
          "distinct": true,
          "id": "697c0b09694ea848c0da3a0582edf326a7b7e0f6",
          "message": "Remove doc links and add read more",
          "timestamp": "2020-10-14T20:24:33-07:00",
          "tree_id": "735eb719a346a390f162cbba69fd4442fa825a42",
          "url": "https://github.com/unicode-org/icu4x/commit/697c0b09694ea848c0da3a0582edf326a7b7e0f6"
        },
        "date": 1602732480197,
        "tool": "cargo",
        "benches": [
          {
            "name": "uniset/overview",
            "value": 34348244,
            "range": "± 1044927",
            "unit": "ns/iter"
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
          "id": "7388c4d05b8ed3b457a42e947632aeffb384f628",
          "message": "Update README.md",
          "timestamp": "2020-10-14T20:40:08-07:00",
          "tree_id": "aabd4a302e8c13f800ed182cfa638bb14bce72b8",
          "url": "https://github.com/unicode-org/icu4x/commit/7388c4d05b8ed3b457a42e947632aeffb384f628"
        },
        "date": 1602733394573,
        "tool": "cargo",
        "benches": [
          {
            "name": "uniset/overview",
            "value": 31241301,
            "range": "± 1460701",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "zbraniecki@mozilla.com",
            "name": "Zibi Braniecki",
            "username": "zbraniecki"
          },
          "committer": {
            "email": "zbraniecki@mozilla.com",
            "name": "Zibi Braniecki",
            "username": "zbraniecki"
          },
          "distinct": true,
          "id": "9cb2aa2fa6944982a1534e3bf1580bbbd7055979",
          "message": "Populate all readmes.",
          "timestamp": "2020-10-14T21:04:22-07:00",
          "tree_id": "1e75aeb92e0255d7a3c82936d3c15169958d9ae5",
          "url": "https://github.com/unicode-org/icu4x/commit/9cb2aa2fa6944982a1534e3bf1580bbbd7055979"
        },
        "date": 1602734893226,
        "tool": "cargo",
        "benches": [
          {
            "name": "uniset/overview",
            "value": 38979726,
            "range": "± 1551180",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}