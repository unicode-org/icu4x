window.BENCHMARK_DATA = {
  "lastUpdate": 1602737795278,
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
          "id": "88783ab4a36ea6e9e1b8aca4e75ba871f67b7df2",
          "message": "Add per-component benchmark tests in CI (#353)",
          "timestamp": "2020-10-14T21:52:15-07:00",
          "tree_id": "a2f3a475ec905d5abe931e0c3e3eeea42cf96860",
          "url": "https://github.com/unicode-org/icu4x/commit/88783ab4a36ea6e9e1b8aca4e75ba871f67b7df2"
        },
        "date": 1602737794847,
        "tool": "cargo",
        "benches": [
          {
            "name": "plurals/operands/overview",
            "value": 1430,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "plurals/parser/overview",
            "value": 6590,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "plurals/pluralrules/overview",
            "value": 113076,
            "range": "± 437",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}