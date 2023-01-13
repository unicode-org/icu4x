# Generating microbench data

The full versions of these files are located
[in another part of the repository](https://github.com/unicode-org/icu/tree/main/icu4j/perf-tests/data).

## Sanitizing the file

```shell
sed -i '/^#/d' ${filename}
sed -i '/^$/d' ${filename}
```

## Shuffling the file

```shell
shuf -n 20 ${filename} -o ${filename}
```
