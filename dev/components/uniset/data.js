window.BENCHMARK_DATA = {
  "lastUpdate": 1602266076571,
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
      }
    ]
  }
}