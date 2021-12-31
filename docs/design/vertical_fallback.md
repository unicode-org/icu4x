Vertical Fallback in Data Provider
==================================

This document discusses *vertical fallback*, or fallbacks within the same data key but across locales.

## Background

Pre-requisite reading:

- [data_pipeline.md](data_pipeline.md): How the data pipeline works.
- [locale_fallback_and_negotiation.md](locale_fallback_and_negotiation.md): How we should approach the problem sof horizontal fallback, vertical fallback, and language negotiation.

This document assumes the reader is already familiar with the terminology introduced above.

### Goals

When performing vertical fallback, we should:

1. Be understandable/predictable.
2. Never cross language boundaries.
3. Avoid crossing script boundaries.
4. Reduce data size.

### Edge Cases

Edge cases for vertical fallback fall into three main groups:

- Aliases (`iw` → `he`, `nb` → `no`, `ars` → `ar-SA`, `sh` → `sr-Latn`)
- Multi-script languages (sr, zh)
- Meta regions (001, 150, 419)

## Design

Vertical fallbacking should follow a tree: every locale should have exactly one parent, except the root locale, which has no parent.

In the absence of overrides, a locale's parent is the locale with one subtag removed from the end of a locale: `pr-Latn-PT` → `pt-Latn` → `pt` → `und`.

However, the subtag removal process fails to account for the Edge Cases: subtag aliases and multi-script languages. This is where *inheritance overrides* come in: the parent of certain locales is overridden.

### Alias Overrides

Some aliases apply to a single subtag, whereas others apply to multiple subtags. For example, ICU supports the `ars` language, which is an alias for `ar-SA`. The algorithm and data for performing aliases are fairly complex.

ICU4C supports some aliases in vertical fallback, like `ars` → `ar-SA` and `iw-IL` → `he-IL`, but not others, like `sh` → `sr-Latn`.

I would like to make the following controversial position: *vertical fallbacking will not handle aliases*. Instead, locales should be normalized through `LocaleCanonicalizer` before being passed into the data provider. It is not the data provider's job to decide which aliases to support and which ones to ignore, and supporting all aliases amounts to a nontrivial data size and performance penalty.

Not handling aliases within the data provider's vertical fallbacking infrastructure will contribute to the modularity of ICU4X. For example, if a client knows for sure that it is only going to be processing a fixed list of locales that are already in canonical form, then it does not need to carry the alias code and data.

To make alias overrides easier for users who want them, we could allow `LocaleCanonicalizer` to join the data provider chain and normalize the locales before they get to vertical fallback.

### Script Overrides

**Principle:** Every language should have *exactly one* default script.

With this principle in mind, *for the purposes of data loading,* locales that use the default script should not have a script subtag.

Script handling, therefore, proceeds as follows:

1. If the locale does *not* have a script subtag, check if the language/region pair uses a non-default script. If so, add a script subtag. Otherwise, if the locale *does* have a script subtag, check if the script is the default script for the language. If so, remove the script subtag.
2. Perform vertical fallbacking.
3. If the locale uses a non-default script subtag, *do not* fall back to the base language; fall back directly to root instead.

**Example 1:** `sr` has `Cyrl` as its default script, but `sr-ME` prefers `Latn`. Therefore, if `sr-ME` is the input locale, we first maximize it to `sr-Latn-ME`, then fall back to `sr-Latn`. Finally, since `Latn` is not the default script, we fall back to root. The full vertical fallback chain is: `sr-ME` (empty) → `sr-Latn-ME` → `sr-Latn` → `und`.

**Example 2:** The input locale is `de-Latn-LI`. Since `Latn` is the default script for `de`, we minimize the locale to `de-LI`, then fall back to `de` and then to root. Note that `de-Latn` will never have its own data according to the principle stated at the top of this section. The full vertical fallback chain is: `de-Latn-LI` (empty) → `de-LI` → `de` → `und`.

### Meta Regions

CLDR ships data with a small number of meta-regions, which include:

- `en-001` (parent of `en-GB`, among others)
- `en-150` (parent of `en-DE`, among others)
- `es-419` (parent of `es-AR`, among others)
- `pt-PT` (parent of `pt-MO`, among others)
- `zh-Hant-HK` (parent of `zh-Hant-MO`)

Unlike aliases, meta regions represent well-formed, canonicalized locales, which CLDR declares to have relationships with one another.

On the outside, we should support meta-regions. However, the method we use to support them is discussed below.

## Implementation

There are two types of approaches we could take here.

1. Resolve vertical fallbacks at runtime.
2. Resolve vertical fallbacks at build time.

### Runtime Approach

With the runtime approach, we ship the code and data to perform script fallbacks and meta regions at runtime. When we get a locale, we check for data in that locale, and if there is none, we walk up the tree until we find data.

**Pros:**

- No need to pre-minimize or pre-maximize the locale
- Leverages CLDR's heuristics for what vertical fallbacks are most efficient

**Cons:**

- Requires shipping the nontrivial vertical fallback override code and data

### Build-Time Approach

With the build-time approach, we only perform naive vertical fallbacking. All locales for which naive inheritance does not work must therefore have a pointer to the correct data, which are all resolved at build time.

**Pros:**

- Runtime vertical fallback code is extremely simple

**Cons:**

- Larger index arrays, which *may* degrade performance for some locales

There are two ways to resolve the locales, and they both require additional pre-processing on the locale before passing it into the data provider: *minimizing* or *maximizing*.

#### Build-Time Option 1: Minimized Locales

With this option, we assume that locales passed into the data provider are in minimal form. For example, `sr-Latn-ME` should be `sr-ME` and `de-Latn-LI` should be `de-LI`. This means that *every single data key* in `sr-ME` needs to have a direct pointer to the Latin version, but `de-LI` can directly fall back to `de` for most keys.

**Pros:**

- The industry tends to use minimized locales by default (`zh-TW`, `sr-ME`, …), so this option requires less pre-processing on locales at runtime.

**Cons:**

- Footgun: If a non-minimized locale is passed at runtime, it could have the incorrect behavior. For example, `sr-Latn-ME` or `de-Latn-LI` may not find the correct data.
- Struggles to scale when there are many data keys

#### Build-Time Option 2: Manually Maximized Locales

With this option, we assume that locales are in maximal form. For example, `sr-ME` should be `sr-Latn-ME`, and `de-LI` should be `de-Latn-LI`. Note that every language still has a default script, so one of the scripts will always pass through during vertical fallback.

Since maximized locales always have at least three subtags (language, script, and region), we can (and should) check in the data provider whether the input locale is maximized, and if it is not, we can return an error.

**Pros:**

- Scales well to many data keys

**Cons:**

- Not ergonomic; most users will need to call `.maximize()` before giving their locale to ICU4X

#### Build-Time Option 3: Automatically Maximized Locales

This is the same as option 2, except that instead of returning an error if the input locale is not maximized, we automatically maximize it, using the `LocaleCanonicalizer`.

It is likely that if we shipped this option, we would give ICU4X users the ability to turn off the auto-maximization if they prefer.

**Pros:**

- Scales well to many data keys
- Shares code and data with an existing component, rather than introducing a novel chunk of code and data as in the Runtime Approach

**Cons:**

- Requires shipping the likely subtags code and data
- All lookups incur a small but nonempty penalty when maximization occurs
