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

## Add back the header (if you plan on submitting the files)

```
#/**
# * © 2023 and later: Unicode, Inc. and others.
# * License & terms of use: http://www.unicode.org/copyright.html
# *******************************************************************************
# * Copyright (C) 2002-2023, International Business Machines Corporation and    *
# * others. All Rights Reserved.                                                *
# *******************************************************************************
# */
```
