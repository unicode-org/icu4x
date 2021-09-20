window.BENCHMARK_DATA = {
  "lastUpdate": 1632158835877,
  "repoUrl": "https://github.com/unicode-org/icu4x",
  "entries": {
    "Benchmark": [
      {
        "commit": {
          "author": {
            "email": "41129501+gnrunge@users.noreply.github.com",
            "name": "Norbert Runge",
            "username": "gnrunge"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f19a64b4662d56c2588a592685e87c1e799c7f2a",
          "message": "Add data size benchmark: monitor size of file (#1051)\n\nprovider/testdata/data/testdata.postcard.\r\n\r\nRemove erroneous second print-out of results.\r\n\r\nMoves directory datasize/ with the output data under 'benchmarks/' directory,\r\nfrom 'benchmarks/binsize/'.",
          "timestamp": "2021-09-20T10:25:55-07:00",
          "tree_id": "470c8072eebfc55a2fad062841a147b4b49fdae1",
          "url": "https://github.com/unicode-org/icu4x/commit/f19a64b4662d56c2588a592685e87c1e799c7f2a"
        },
        "date": 1632158832439,
        "tool": "ndjson",
        "benches": [
          {
            "name": "provider/testdata/data/testdata.postcard",
            "value": 582678,
            "unit": "bytes",
            "biggerIsBetter": false
          }
        ]
      }
    ]
  }
}