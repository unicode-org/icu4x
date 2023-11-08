Build these data files as follows:

```
$ icu4x-datagen --keys all --locales ccp --format blob --out crates/interactive/data/ccp_all.blob
$ icu4x-datagen --keys-for-bin target/debug/gregory_blob --locales ccp --format blob --out crates/interactive/data/ccp_gregory.blob
```