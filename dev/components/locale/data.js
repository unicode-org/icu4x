window.BENCHMARK_DATA = {
  "lastUpdate": 1602274739416,
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
        "date": 1602266058187,
        "tool": "cargo",
        "benches": [
          {
            "name": "isize/smaller",
            "value": 28798,
            "range": "± 2573",
            "unit": "ns/iter"
          },
          {
            "name": "isize/larger",
            "value": 69389,
            "range": "± 8625",
            "unit": "ns/iter"
          },
          {
            "name": "to_string/908070605040302010",
            "value": 258,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "write_to/908070605040302010",
            "value": 122,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "from_string/1000000001",
            "value": 79,
            "range": "± 10",
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
        "date": 1602269985655,
        "tool": "cargo",
        "benches": [
          {
            "name": "isize/smaller",
            "value": 33153,
            "range": "± 362",
            "unit": "ns/iter"
          },
          {
            "name": "isize/larger",
            "value": 79277,
            "range": "± 908",
            "unit": "ns/iter"
          },
          {
            "name": "to_string/908070605040302010",
            "value": 281,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "write_to/908070605040302010",
            "value": 139,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "from_string/1000000001",
            "value": 89,
            "range": "± 1",
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
        "date": 1602274738994,
        "tool": "cargo",
        "benches": [
          {
            "name": "isize/smaller",
            "value": 35172,
            "range": "± 2032",
            "unit": "ns/iter"
          },
          {
            "name": "isize/larger",
            "value": 83462,
            "range": "± 1410",
            "unit": "ns/iter"
          },
          {
            "name": "to_string/908070605040302010",
            "value": 303,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "write_to/908070605040302010",
            "value": 148,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "from_string/1000000001",
            "value": 96,
            "range": "± 4",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}