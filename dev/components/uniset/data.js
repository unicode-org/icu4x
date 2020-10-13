window.BENCHMARK_DATA = {
  "lastUpdate": 1602616948708,
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
      }
    ]
  }
}