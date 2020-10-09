window.BENCHMARK_DATA = {
  "lastUpdate": 1602274707601,
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
        "date": 1602266090444,
        "tool": "cargo",
        "benches": [
          {
            "name": "isize/smaller",
            "value": 32521,
            "range": "± 2204",
            "unit": "ns/iter"
          },
          {
            "name": "isize/larger",
            "value": 76162,
            "range": "± 6465",
            "unit": "ns/iter"
          },
          {
            "name": "to_string/908070605040302010",
            "value": 283,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "write_to/908070605040302010",
            "value": 140,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "from_string/1000000001",
            "value": 88,
            "range": "± 5",
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
        "date": 1602269989907,
        "tool": "cargo",
        "benches": [
          {
            "name": "isize/smaller",
            "value": 34242,
            "range": "± 1112",
            "unit": "ns/iter"
          },
          {
            "name": "isize/larger",
            "value": 79432,
            "range": "± 330",
            "unit": "ns/iter"
          },
          {
            "name": "to_string/908070605040302010",
            "value": 289,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "write_to/908070605040302010",
            "value": 141,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "from_string/1000000001",
            "value": 91,
            "range": "± 0",
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
        "date": 1602274707205,
        "tool": "cargo",
        "benches": [
          {
            "name": "isize/smaller",
            "value": 29857,
            "range": "± 1572",
            "unit": "ns/iter"
          },
          {
            "name": "isize/larger",
            "value": 68861,
            "range": "± 3510",
            "unit": "ns/iter"
          },
          {
            "name": "to_string/908070605040302010",
            "value": 252,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "write_to/908070605040302010",
            "value": 118,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "from_string/1000000001",
            "value": 76,
            "range": "± 3",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}