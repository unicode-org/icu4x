window.BENCHMARK_DATA = {
  "lastUpdate": 1602738488836,
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
          "id": "174d846a131ca79802b7da3eb004982d442e399d",
          "message": "Update README.md",
          "timestamp": "2020-10-14T22:03:28-07:00",
          "tree_id": "6bc0a97dd78518e59b2560279d89e9afbc749241",
          "url": "https://github.com/unicode-org/icu4x/commit/174d846a131ca79802b7da3eb004982d442e399d"
        },
        "date": 1602738488420,
        "tool": "cargo",
        "benches": [
          {
            "name": "plurals/operands/overview",
            "value": 1272,
            "range": "± 79",
            "unit": "ns/iter"
          },
          {
            "name": "plurals/parser/overview",
            "value": 6251,
            "range": "± 1157",
            "unit": "ns/iter"
          },
          {
            "name": "plurals/pluralrules/overview",
            "value": 112743,
            "range": "± 6966",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}