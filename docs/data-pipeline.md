ICU4X Locale Data Pipeline
==========================

One of the key design principles of ICU4X is to make locale data small and portable, allowing it to be pulled from multiple sources depending on the needs of the application.  This document explains how that goal can be achieved.

## Definitions

The following terms are used throughout this document.

- **Data Provider:** An object that returns machine-readable data in a well-specified way.  A data provider provides a link between ICU4X and the raw data.
- **Data Version:** A version reflecting the data itself, abstracted away from the format version and schema version.  For example, CLDR 37 may be a data version.
- **Format Version:** A version of the file format, abstracted away from the schema version and data version.  For example, Protobuf 2 and Protobuf 3 are format versions.
- **Format:** How the data is stored on disk or provided from a service.  Examples of data formats: JSON, YAML, Memory-Mapped, Protobuf.  The format is internal to the data provider.
- **Hunk:** A small piece of locale data relating to a specific task.  For example, the name of January might be a hunk, and a list of all month names could also be considered a hunk.  A data piece is expected to reflect a specific type.
- **LangID:** A tuple of language, script, and region.  LangID is a request variable.  Unicode Locale Extensions should be handled by the ICU4X code rather than the data provider.
- **Key:** An identifier corresponding to a specific hunk.
- **Mapping:** A mechanism that a data provider should follow to read from the schema and serve a hunk that may have a different type.
- **Request Variables:** Metadata that is sent along with a key when requesting data from a data provider.
- **Response Variables:** Metadata that is sent along with a hunk when a data provider responds to a request.
- **Schema Version:** A version of the schema, abstracted away from the format version and data version.  For example, data may be reorganied within the JSON file between schema versions.
- **Schema:** The structure of locale data, abstracted away from the hunk types.  Data is stored in a particular format according to the schema.
- **Type:** The structure of a hunk.  The type may be, for example, a number, string, list of numbers or strings, or another data type discussed in detail later in the document.

## Design

### Data Provider Architecture

The most central piece of the data pipeline architecture is the data provider.  A data provider is an object that can read data from a JSON file, a service, a memory blob, etc.

A JSON data provider might look something like this:

![JSON Data Provider](assets/json-data-provider.svg)

In the above diagram, ICU4X requests a specific key from the data provider.  The data provider uses a mapping to figure out what path to load from the JSON file corresponding to the key.  It then may use a mapping to convert the JSON object to the type expected for the hunk, and then finally it sends the hunk back to the ICU4X implementation.

Requests to the data provider consist not only of a key, but also additional request variables, such as a requested LangID and possibly other metadata. Responses consist not only of a hunk, but also additional response variables, such as a data version, actual LangID, and possibly other metadata.

Data providers can delegate to other data providers for specific tasks.  For example, one might have a data provider that performs caching, or a data provider that delegates to one of several other data providers.  A complex setup might look something like:

![JSON Data Provider](assets/multi-provider-architecture.svg)

In this example, data requests from ICU4X first go through an LRU cache, before going to a locale-sensitive provider, which forks to one of three other providers depending on the requested locale.

The system data provider pulls data for *en* and *zh* from an OS RPC service.  The interaction between the data provider and that service are internal to the data provider.

The REST data provider may wait a certain amount of time for several requests to come in, and then perform a batch request to a cloud service that provides data for *hi* and *my*.  It uses a mapping to figure out how to translate from keys to paths that it can send in batch to the cloud service.  This interaction also is internal to the REST data provider.

The JSON data provider works like the simpler one shown earlier: it has a mapping that translates from keys to JSON schema paths, then from JSON objects to hunks.

### Keys

A key is an integer from an enumeration.  Each key has a corresponding type, which is the type of hunks returned for that specific key.  The type corresponding to a key is stable and never changes; by convention, a version number is built into the key name.  For example:

| Key Name | Key Integer | Type | Comments |
|---|---|---|---|
| NUM_SYM_DECIMAL_V1 | 0x1000 | string |
| NUM_SYM_GROUP_V1 | 0x1001 | i32 | Code point, used in older ICU4Xs |
| NUM_GRP_SIZES_V1 | 0x1002 | tuple(i8, i8) |
| NUM_SYM_GROUP_V2 | 0x1003 | string | String, used in newer ICU4Xs |
| DATE_SYM_JAN_V1 | 0x2000 | string |
| DATE_SYM_FEB_V1 | 0x2001 | string |
| DATE_SYM_MONTHS_V1 | 0x2002 | string list | Same data, different key/type |
| CURR_SYM_V1 | 0x3000 | string | Currency code is a request variable |
| CURR_LOCAL_CODE_V1 | 0x3001 | string | The locale's currency code |
| CURR_LOCAL_SYM_V1 | 0x3002 | string | The symbol for that currency |

*Note:* Above, `i8` and `i32` signify an 8-bit or 32-bit signed integer.  The exact types might differ based on the host language.

*Note:* The keys above are for illustrative purposes only.  The actual data hunks will likely be on the larger side, such as "all number symbols for this locale and numbering system".

*Open Question:* How do you map from an enum/integer to a type in a type-safe way in Rust?  In C++/Java, this would entail some sort of cast, which I imagine is possible in Rust but might require an unsafe block.  Main issue: [#8](https://github.com/unicode-org/omnicu/issues/8)

*Open Question:* Due to ongoing developments in [wrapper-layer.md](wrapper-layer.md), the above list of example keys may be more fine-grained than we will need in the final product.  It may be better to have more coarse-grained hunks, like "all decimal format symbols" instead of "grouping separator" and "decimal separator".  Main issue: [#26](https://github.com/unicode-org/omnicu/issues/26)

### Data Key Struct Definitions

The actual data keys and the structs to which they correspond should be defined in a central location in the repository: [components/data_provider/src](https://github.com/unicode-org/icu4x/tree/master/components/data_provider/src).  Follow conventions of existing data provider struct definitions when adding a new one.

There should generally be a 1-to-1 relationship between components (number formatter, plural rules, date format) and modules in the data provider crate.  However, this is not strictly enforced; use your best judgement.

### Request Variables

Requests made to data providers consist of a key and additional *request variables*.  The variables are:

1. Requested LangID
2. Optional String Identifier (explained below)
3. String Encoding (UTF-8 or UTF-16, explained below)

The optional string identifier should be a string corresponding to the key, such as the currency code when requesting `CURR_SYM_V1`.  Most keys will not require an optional string identifier.

The string encoding corresponds to the encoding of the string identifier and also the preferred encoding of strings in the response object.  In other words, it is expected that the string encoding in the request should equal the string encoding in the response.

### Static Data Slicing

Static analysis of ICU4X code can determine which keys are required for a particular build.  This information can be used to automatically generate a minimal JSON file, for example.

The examples `CURR_SYM_V1` and `CURR_LOCAL_SYM_V1` illustrate an important aspect of data slicing.  If the ICU4X code only ever formats the locale's preferred currency symbol, it can use `CURR_LOCAL_SYM_V1`, and the static data slicer can include only that small piece of data.  However, if ICU4X is compiled to accept any arbitrary currency code, then it should use `CURR_SYM_V1`, and the static data slicer knows that it must include symbols for all currencies.

*Note: For the currency example specifically, this may also involve making ICU4X configurable for whether or not it accepts the `-u-cu-` Unicode extension in the locale string, since supporting that extension may require loading full currency data.*

### Mappings

The data on disk may not reflect exactly the types required for a particular key.  For example, imagine we have the following data file:

```json
{
    "gregorian_months": [
        "January", "February", "March", "April", "May", "June",
        "July", "August", "September", "October", "November", "December"
    ]
}
```

The mapping is responsible for translating that schema into the hunk type required for a particular key.  For example, the pseudocode of a mapping might be:

```
switch (key) {
    case DATE_SYM_JAN_V1:
        return json["gregorian_months"][0];
    case DATE_SYM_FEB_V1:
        return json["gregorian_months"][1];
    case DATE_SYM_MONTHS_V1:
        return json["gregorian_months"];
}
```

### Supported Keys

A data provider need not implement a mapping for all keys.  For example, if ICU4X migrates from `NUM_SYM_GROUP_V1` to `NUM_SYM_GROUP_V2`, a data provider might choose to support both keys for a few releases, and at some point drop support for the older key.  This allows the same data provider to be used across multiple ICU4X versions.

### Schema Version

A data file (such as JSON) should represent data conforming to a certain schema, and the data provider's mapping should map from a specific schema version to a specific set of supported keys.  The data file can be rebuilt with different data versions so long as it conforms to the same schema version.

The expectation is that an offline tool transforms CLDR data into a specific data schema version and writes out a data file that the data provider uses at runtime.  It is also possible that a data provider could read from CLDR's raw XML or JSON files directly, although this might not be as efficient at runtime.

### Types

The set of supported types is limited so that ICU4X implementations can be written to support them in a consistent way.  The supported types are:

- i8
- i32 (includes code points)
- double
- string (either UTF-8 or UTF-16 according to request/response)
- byte array
- tuple of one of these types (fixed-length)
- list of one of these types (variable-length)
- struct with fixed keys and values of one of these types

An open-ended map with string keys is not supported because hash map implementations vary from platform to platform, and they do not deliver guaranteed performance.  Instead, a struct with fixed fields can be used.

The data type should be appropriate for fast evaluation at runtime.  For example, it is not recommended for ICU4X to request a pattern string such as `#,##0.00` for number formatting; instead, it should request a struct corresponding to the parsed version of that pattern string, such that ICU4X doesn't need to parse it at runtime.

### Response Variables

Along with the hunk, the data provider sends multiple *response variables*.  The variables are:

1. Supported LangID (explained below)
2. Data Version (explained below)
3. String Encoding (UTF-8 or UTF-16)

The supported LangID is expected to be the most specific LangID that had any data whatsoever, even if it is just an alias.  For example, if en_GB is present but empty, and the data is actually loaded from en_001, the Supported LangID should still be en_GB.  In other words, the fallback mechanism is considered an internal detail.

If the data provider is unable to return data based on a certain request, it may return an error in a form corresponding to the host language's convention.

### Data Version

The data version is expected to be a well-defined, namespaced identifier for the origin of the data.  For example, when represented as a string, the following might be data versions:

- `CLDR_37_alpha1` → Vanilla CLDR 37 alpha1
- `CLDR_37_alpha1_goog2020a` → Google patch 2020a on top of CLDR 37 alpha1
- `FOO_1_1` → Version 1.1 of data from a hypothetical data source named Foo

The first data version subtag, or namespace, defines the syntax for the remainder of the identifier.  For example, the `CLDR` namespace might accept two or three subtags: major version (`37`), minor version (`alpha1`), and optional patch version (`goog2020a`).

*Note:* The syntax for the data version is undefined at this time.  What is shown above is merely a strawman example.
