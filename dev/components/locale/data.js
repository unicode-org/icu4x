window.BENCHMARK_DATA = {
  "lastUpdate": 1602266058855,
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
      }
    ]
  }
}