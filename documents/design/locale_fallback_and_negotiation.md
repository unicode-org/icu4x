# ICU4X: Handling fallbacks and locale negotiation

## Introduction

When several components / layers talk about “locales” or “locale data” we will (more often than not) notice mismatches.

So most libraries need to somehow address these mismatches and gracefully, if possible without the user noticing.

The following is an overview of the various levels where this can / should be addressed, with the names used. These names are not “industry standard”, or even common, but there is no known commonly agreed-upon terminology.
This will also clarify what we mean by “horizontal fallback”, and narrow down the scope of this document.

## Terminology

### Horizontal fallback

Used to load similar resources from inside the same locale.

Some examples are given in [issue #259](https://github.com/unicode-org/icu4x/issues/259). The issue also describes pretty well the options.

### Vertical fallback

This is usually done without changing the language + script part of the locale. But the region might change. This is the most common type of fallback, one that most OSes and frameworks implement today, and what most developers understand when they say “locale fallback.”

It is often used to load “locale dependent resources” (localized messages, month names, sounds, themes).

A common option is a very simple “cuts from the right” strategy (language + region → language → root, which has the risk of ignoring the script if missing in the locale ID). Although simple, I would not recommend we implement this, as it has problems with languages using more than one script (Chinese Simplified / Traditional, or Serbian Cyrillic / Latin).

More sophisticated algorithms would go from Spanish Argentina (`es-AR`) to Spanish-Latin America (`es-419`) to other Spanish locales in Latin America (`es-MX`, or `es-US`) to generic Spanish (`es`) to “OK, give me any kind of Spanish you might have,” and finally to root.
(`es-AR` → `es-419` → `es-MX` / `es-US` → `es` → `es-*` → `root`)

There are good arguments for the fallback to the next locale in the user list before going to `root` (or ultimate fallback). This would of course mean the whole mechanism being applied to each locale in the chain (so imagine a user listing Italian Swiss + French Swiss would go through `it-CH` → `it` → `it-IT` → `it-*` → **<span style="color:red">fr-CH → fr → fr-FR → fr-*</span>** → `root`. 

There are also good arguments against this strategy.

The case described above, with different users desiring different resolution behavior, makes a good example where we can’t make a good decision as library owners, and will have to provide an option for the developers using the library to configure the behavior.

### Language negotiation

At the highest level, usually application or screen (sometimes “panel”) we need to agree on what language to use. The typical requirement at this level is to take an ordered list of locales that the user wants / understands, a list of locales that the component has available, and return the best match. Similar to two people who met for the very first time and are trying to figure out what language to use for a conversation.

At the end of this the whole application / screen / panel will end up using one language, not a mixture. There might be small “disagreements” (for example French-France, even if “we agreed” on French Canadian), but not across languages (so no German if we agreed French).

## Horizontal fallback

Like in most situations, there is no good answer to this problem. We will need to compromise.

The comment in issue #259 summarizes the options pretty well, and it is reproduced here for convenience:

> The two drives that are extremes are:
>
> * We want to make ICU4X DataProvider easy to use. Handling fallbacking in data provider means the user doesn't have to worry about missing data and fallback decisions.
> * We want to make ICU4X DataProvider flexible. Being opaque about missing data means that the user has no flexibility in choosing how to handle fallbacks (do we always want to fallback from `short` to `long` if it's missing? Maybe in some scenarios users will want to fallback differently?
>
> If we go with `fallback in the provider`, we're likely going to have to add ability to get data without the fallback. If we go with fallback in the hands of consumer, we will need to help consumers know how to fallback and there's going to be a number of repetitive pieces of fallbacking in consumers.
>
> Finally, expanding all data in the provider means more data to carry over the wire. A good example is the second one - for locales where `stand_alone` matches `format` contexts, we could just carry over `format` and set `stand_alone` to `null` and the consumer then can say "if there is `stand_alone`, use it, if not, fallback on format".
> But if we expand all fallbacks in Provider, we'll have to for all locales carry duplicated data so that users can infallibly take `stand_alone`.
>
> I believe that, as in most cases, we'll be looking for the "third path" somewhere between those two extremes, and we'll need to resolve it on per-case bases, but I think that encoding in wiki (https://github.com/unicode-org/icu4x/wiki/Hooking-up-a-DataProvider) or docs some guidelines on how to generally make decisions on what is going to be expanded in Provider vs. in consumer, is in our best interest, so let's discuss it!

In most cases the provider is in the best position to decide how to do this kind of horizontal fallback, because the provider “knows” the most about all the bits and pieces of the machinery.

The user might want some flexibility, but (more often than not) does not know the “internals.” If we ask for a “reasonably long date, something similar to ‘March 29, 2021’ in English” then what we can ask for is “a full month name, and numeric day / year.” But we won’t know anything about genitive month names, for example.

Worse, when we format a date there are so many “pieces” that come together, each with its own fallback rules.

Things might (potentially) interfere with the vertical fallback. For example if one cannot find a medium date pattern for English Great Britain, but there is a long date pattern (`"d MMMM y"`). Should we do a vertical fallback to generic English (`en`, which is actually US English), and get `"MMMM d, y"` (which changes the month-day order) or alter the long pattern, algorithmically replacing `"MMMM"` with `"MMM"` and getting `"d MMM y"`?

And some rules can influence each other. What happens if we do vertical fallback for a date pattern, but horizontal fallback for the month names? Can we do this, or should we try to use the same fallback strategy for all “pieces” that come together in the final format?

So the best we can do is:

1. Let the data provider decided on when, how, and between what “pieces” to do horizontal fallback
1. Offer some “rough knobs” (configuration controls for high-level functionality) for the end user for flexibility. But it may not be possible to give full control. These controls would be there to guide the general strategy (“favour vertical fallback over horizontal fallback”), not low level (“disable genitive-to-nominative fallback”)
1. Offer some “rules of thumb” for the implementers on when to do fallback, what kind, and what kind of control to expose for the end user

### Rules of thumb

#### A. Favor vertical fallback (with one caveat)

CLDR does vertical fallback. So in general if something is missing we can assume that the “parent” locale has the proper data. Otherwise, it should be considered a bug.

So we should favor vertical fallback by default, **but only as long as that fallback does not go to root**.

Specifically, what does that mean? It means that between the last step of vertical fallback, from a language to root, we should “insert” a horizontal fallback.

So a Spanish-Argentina locale would try `es-AR` → `es-419` → `es` → `es-*` → `root` (or whatever level or “smartness” we put in the vertical fallback) should become `es-AR` → `es-419` → `es` → `es-*` → (horizontal to `es-AR`, `es-419`, `es`, `es-*`) → `root`.

#### B. Should not try to “fix” CLDR bugs with “smartness” in horizontal fallback

In general there is such a thing as “too smart for your own good.”

Oftentimes, applications try to “massage” the locale data in code in order to get the result they expected.  
That risks damaging the correct results in other locales.

And it is a lot easier to fix the data and download it on demand with the next opportunity, rather than fix code that tried horizontal fallback and got it wrong.

#### C. Don’t do horizontal fallbacks when there is a risk of confusion

In general horizontal fallback between two numeric forms (`"d"` ↔ `"dd"` or `"M"` ↔ `"MM"`) or between two alpha-forms (`"MMM"` ↔ `"MMMM"`) is safer than horizontal fallback between these categories (`"MM"` ↔ `"MMM"`). For example “03/04/2021” ↔ “3/4/2021” or “March 4” ↔ “Mar 4” does not carry a risk of confusion. But “Mar 4, 2021” → “3 4, 2021” is bad. Worse, in Canadian English the field order also changes (`"MMM d, y"` ↔ `"y-MM-dd"`, MDY vs YMD).

Or between format / standalone forms. You can safely go from format to standalone because it is basically using sentence casing (“septembrie” → “Septembrie”, in Romanian).

That is fine and works for all locales. (And don't forget: you also need to perform sentence casing correctly).

But we can’t algorithmically go from standalone to format, because that implies lowercasing. And not all locales do that (ex: in English, the formatted month name is still in sentence case), and because lowercasing can be ambiguous (more than one mapping).

From all these examples, it might seem that one actually needs to know **a lot** about what they are trying to fallback between. And not only about the kind of data, but also about all kinds of locales.

So overlaps nicely with the next rule of thumb.

#### D. Don’t do horizontal fallback

Although this may be controversial, in the end, it is the safest option.

There are very few safe mappings, and most of them are more on the “let’s save memory / data size” side of things, rather than “let’s fix things / get better results.”

So one can drop standalone month / day names if they are 100% identical to the formatting ones. Or drop genitive / nominative month names if they are 100% identical.

We don’t even save that much. The 12 long month names in English, when compressed, end up to just 91 bytes. The code added for horizontal fallback might just “cost” more than those 91 bytes (size, complexity, maintenance, bugs).

Trying to save data makes sense for big data, like collation (where there is not much horizontal fallback to do), or time zones, or country / language names, where (again) we can’t even do much horizontal fallback.

So let’s go back to the older “rule of thumb”: “premature optimization is the root of all evil.”

### Summary

The provider should behave as if there is no fallback of any kind, vertical or horizontal.

The implementer is free to decide if some form of fallback can save some data, and decide where to resolve it.

There are several opportunities for one to “flatten” a complex vertical fallback chain.  
It can be done server side, before the locale data is sent to the client.  
Or it can be done client side, at the persistence layer.  
Or the elements of the chain can be persisted unresolved, resolved on the first use, and the result cached.  
All of these options should be implementation details.

It does not mean we completely drop the idea of horizontal fallback.

But we should consider it case by case, when it looks like a low hanging fruit.  
Treat it like any optimization: looks promising for this case, estimate it, prototype, measure, discuss, before doing it.

And (like most optimizations) does not expose anything to the user (developers, or end-users).  
Things “just get better” (smaller, faster), no changes required.

## Vertical fallback

There are already quite a few examples on how this would work, in the “Horizontal fallback” section, which already fairly clearly illustrate how it works.

But to summarize things, vertical fallback should:

### Do what ICU does

This will work best with the CLDR data, which we use.

This means that we don’t get any major surprises.

It is a beaten path, and would match what most OSes do, since all major OSes already use ICU (macOS, iOS, Android, Windows)

This API does not have to be private, at least in the first iteration, but we should expose it at some point, because it is useful in some cases.  
ICU exposes it, JavaScript / iOS / macOS do not, and so on. So probably not critical.

I don’t know if it is useful to document here (in a design document) what the exact ICU behavior is. We shall “dig it out” for implementation.

**Note:** Improve CLDR / ICU behavior, and keep up with that behavior in ICU4X.  
But not diverge, or invent our own algorithm.

### Optional functionality (via flags?)

We might use the matching list returned by the locale matcher (proposed API).

With the list of available i18n locale data that list would contain the right vertical fallback list.

Would be similar to the ICU vertical fallback, but can go across languages:  
if the user listed [ `ar-DZ`, `fr-DZ` ] then the fallback would not go to root / stop after Arabic.

Instead of `ar-DZ` → `ar` → `root` it might go `ar-DZ` → `ar` → `ar-*` → `fr-DZ` → `fr` → `fr-*` → `root`.

## Language negotiation

### Introduction

Language negotiation (or language matching) is a known problem, and over the years it has been addressed in proprietary ways by various frameworks, OSes, applications.

In recent years Unicode recognized the need and the complexity of the problem and addressed it in the standard: [Unicode Technical Standard #35 (UTS #35) – 4.4 Language Matching](https://unicode.org/reports/tr35/#LanguageMatching).

For ICU4X we have already explained the three kinds of operations that requires one to go through a list of languages to find a match: vertical fallback, horizontal fallback, and language matching.

The core difference between the matching described here and what is called above as “fallback” (vertical and horizontal) is that the matching affects big sections of the UI, and cannot be “hidden” under the cover, the way fallback can. It has to be public functionality, exposed to developers.

It is used to determine the locale of the whole application, or just a dialog / screen / panel.
Something composed of several UI elements (text content, labels, lists, buttons, and so on), some with locale-dependent fragments (dates / times / numbers, and so on).
It is common expectation that all these elements are rendered in the same language.

Since ICU4X is a framework that is relatively low level it has no way to “understand” how things come together: what text comes from resources, what text comes as external content, how they are “assembled” in the same screen / panel, what buttons or labels belong to the panel, etc.

That is something that only the developer can know, or some higher level library / framework, one that actually builds that complex UI (think a templating system, or a LayoutManager, or Dialog, or HTML with a special binding mechanism).

### TLDR

ICU4X needs to provide public APIs / classes, usually invoked by something other than ICU4X.

It would take:

* A list of desired locales (the ones the user understands, ordered by user preference)
* A list of available locales (unordered)
* Various “knobs” (parameters / flags) that are used to tweak the behavior **<span style="color:red">(not ICU)</span>**

And will return either one locale (the one that matches best), or a list of matching locales, ordered according to the user preferences.

The main difference between this and the ICU [LocaleMatcher](https://unicode-org.github.io/icu-docs/apidoc/dev/icu4j/com/ibm/icu/util/LocaleMatcher.html) is

1. the addition of more “control knobs” to disable fallback across languages not specified by the user
2. The option to return an ordered list of all locales matching, not just the top match

### The proposal: mostly the Unicode / ICU Locale matching, with some extras

#### Control knobs, part 1

The current ICU implementation is trying really hard to match languages that the user will very likely understand.

For that it uses CLDR data on mutually intelligibility, and country + language data.

And at the first look this gives good results: for a user that requests Maori, there is a good chance that they understand English New Zealand, so that would be a reasonable match.

Similar for Cherokee → English US, or Quechua → Spanish Latin America, or Catalan → Spanish.

But we can also get somewhat controversial results. Fallbacks between Bosnian / Croatian / Montenegrin / Serbian might be reasonable if we look at the mutual intelligibility. Same for Bulgarian / Macedonian.

If we look at the percentages of people speaking a second language in a certain country we can assume that Ukrainian → Russian, Moldovan → Russian, Flemish → French, Hungarian-Romania → Romanian (and we can keep going), might seem reasonable.

But some of these are problematic because of local sensitivities, community conflicts, history. And these kind matches can anger end-users (_“if I wanted LanguageX, I would have listed LanguageX as being acceptable”_)

These aspects are not captured by simple measures like linguistic distance, majority language of a country, and so on.

So we should be able to fine tune these kinds of fallbacks. Ideally with more than an on / off switch, but by customizing the data.

#### Control knobs, part 2

The second type of “control knobs” would make the language matcher in ICU4X behave more in tune with the existing matches as implemented by the host OS, or framework.

ICU4X is designed to be reusable across operating systems and across programming languages. We will call ICU4X implementations from Java on low resource Android devices. From Dart, running on various operating systems, desktop and mobile. From JavaScript, Go, etc. The sky is the limit.

But if our negotiations and fallbacks differ too much from the ones on the host environment then the users will get unexpected results: the whole application running in one locale, with one panel in a different one, or just dates and times in a different locale.

So we should have a way to emulate OS specific matching.

It does not have to be there in the first implementation, and does not have to be “control knobs,” it can very well be alternate implementations. Some might even call the underlying OS.

But we should design with this direction in mind. Working with interfaces, the ICU-style matching being just the first of several. We can implement a very flexible matcher that is mostly data driven, and create OS specific ones by swapping in / out various data “packages.”

ICU4X might create these customizations, and / or by the developers using ICU4X. The ability to replace the default implementation should be there.

#### Option to return an ordered list of all locales matching, not just the top match

**Use case**

I am a developer using ICU4X and using the formatting features, and also the matcher.

That means that for formatting I can have access to 40+ French variants, 20+ Spanish and Arabic regional variants, 100+ English, etc.

I translate my application into a small subset: [ `fr`, `es`, `es-419`, `en`, `en-GB`, `ar`, `zh` ], but (let’s say) not `zh-Hant`. The order doesn't matter.

Let’s say that the user desires [ `ro-RO`, `es-CL` ] (in this order).
So what can I pass to the matcher? The desired list is pretty clear: what the end-user requested. But what should pass as the list of available locales?

Developers usually have to deal with two lists of locales:

* the list of locales I translate my app into (UI)
* the list of formatting locales available to ICU4X (I18N)

But neither of these give me perfect behavior.

**1. Pass the ICU4X comprehensive list of locales.**

The problem here is that the matcher might return a locale that is supported by ICU4X, but lacks translation for UI.

So the “panel” locale is declared to be `ro-RO` (because ICU4X has data), dates and times will be Romanian, but because there are no Romanian strings most of the UI will be either Spanish or “the ultimate fallback” (English, or maybe the language of the developer team, or the locale declared “default” for the matcher). It does not matter, the result would be a mixture of Romanian dates + something else.

Latin America Spanish for everything would have been consistent.

**2. Pass the list of translated languages for my application**

In this case the best match will be `es-419`, which is Latin American Spanish.

The locale of the panel would be `es-419`, the UI strings would be `es-419`, and the dates / times / etc. would be formatted as LatAm Spanish.

It is consistent, but we could do better. Spanish Chile dates / times might be different from the more “country-neutral” LatAm Spanish, but we ignored all the country specific differences.

If the locale of the panel would be `es-CL` then we would be able to format dates / times the way the user prefers, and would still show UI strings in `es-419`, because the vertical fallback would take care to go from `es-CL` → `es-419` → `es` (somewhere in the string loader / resource manager / whatever component loads UI strings)

Similar problems for all the languages used in many countries (date formats for “generic” Arabic, French, English, when there are tens of countries using these languages)

I learned that the ICU matcher can return two results, one for the UI locale, one for i18n (formatting). ([`LocaleMatcher.Result.makeResolvedLocale`](https://unicode-org.github.io/icu-docs/apidoc/released/icu4j/com/ibm/icu/util/LocaleMatcher.Result.html#makeResolvedLocale--))

That is, if you pass in:

**Available UI:** [`es-419`, `es`]  
**Desired:** [`es-CL`]

You can get:

**UI:** `es-419`  
**i18n:** `es-CL`

This is really nice, but still does not cover some valid situations well enough.

For example Android would return `es-CL` in this case, even for UI. And the resource manager will do the fallback to `es-419` when loading the resources.

The benefit: an application might have very-very few resources for `es-CL`. But they are still present.

Some apps use this to keep things like country specific stuff (GPS coordinates, license agreement). But the bulk of the strings would be `es-419`.

If you tell the resource manager to use `es-419` (the UI locale) then those country specific resources are not accessible.

One option would be to pass every locale that has even one resource as “Available UI”:

**Available UI:** [`es-419`, `es`, `es-CL`, `es-AR`, ...]  
**Desired:** [`es-CL`]

Then the result would be:

**UI:** `es-CL`  
**i18n:** `es-CL`

And the resource manager will do the `es-CL` → `es-419` → `es` (vertical) fallback.

But having a `LanguageMatcher` API that returns [`es-CL`, `es-419`, `es-AR`, `es`] can allow for a simpler resource manager implementation.
The resource manager can use that API to implement that fallback on top of all the smartness that LanguageMatcher already has.

**Conclusion**

None of the two options is ideal, and I can’t think of a way to fix this with the public APIs provided by the current ICU LocaleMatcher.

By providing a filtering method, similar to the current matching one, but returning a list of matches, then it would be possible to implement the more advanced behavior:

* Take the comprehensive list of ICU4X supported locales  
[ `af`, `af-NA`, `af-ZA`, …, **<span style="color:red">es, es-419, es-AR, …, es-CL, …, es-VE</span>**, …, **<span style="color:blue">ro, ro-MD, ro-RO</span>**, …, `zu`, `zu-ZA` ]
* Filter it using the UI locale list (the current match, but returning a list)  
[ `ar`, `es`, `es-419`, `en`, `en-GB`, `fr`, `zh` ]  
⇒ [ `ar`, `ar-*`, `en`, `en-*`, `es`, `es-419`, `es-AR`, …, `es-CL`, …, `es-VE`, …, `fr`, `fr-*`, …, `zh`, `zh-Hans-*` ]  
(the order of the result does not matter, but note how `zh-Hant` is not there)
* Use this resulted (filtered) list as supported list, with the desired user list:  
**supported:** [ `ar`, …, `es`, `es-419`, `es-AR`, …, `es-CL`, …, `es-VE`, …, `ro`, `ro-MD`, `ro-RO`, ... ]  
**desired:** [ `ro-RO`, `es-CL` ]  
a. ⇒ [ `es-CL` ] (best match, one locale)  
b. ⇒ or return a list ordered by match score:
[ `es-CL`, `es-419`, `es-AR`, … `es-VE`, ..., `es`, `es-ES` ]  
Note how `es-CL` is first, `es-419` next, a “laundry list” of LatAm countries follow, and how `es` and `es-ES` are at the very end.
* The panel locale would be set to `fr-CL` (or the longer list?), the formatting will use `es-CL`, and the strings will be `es-419` because the (vertical) fallback would take care of it for UI.
And if the API returns a list (case b.) then we can use that exact list for vertical fallback.

We can provide “syntactic sugar” APIs that can wrap all this and solve the problem of multiple lists of “available locales” (formatting data, UI, TTS, speech recognition, subtitles, you name it).

But we should also allow access to the lower level methods that return an ordered matching list.

This can become an even more interesting / challenging problem if we want even more sources of locale-aware services to match: UI strings, locale-aware formatting (ICU4X), TTS, Speech Recognition, available keyboards, maybe more. And they should all match as best as we can.

We can “chain” intersections of lists like so:

User desired ∩ ICU4X locales ∩ Available UI ⇒ [ `es-CL`, `es-419`, `es-AR`, `es` ]  
[`es-CL`, `es-419`, `es-AR`, `es` ] ∩ Available TTS: [ `es`, `es-419` ] ⇒ [ `es-419`, `es-CL` ]  
[ `es-CL`, `es-419`, `es-AR`, `es` ] ∩ Available Speech Recognition: [ `es` ] ⇒ [ `es` ]

#### Expose an API / class to get locale suggestions

Acknowledgement: this was proposed by Zibi.

In many cases the developers have to implement language pickers where they have to suggest a few likely locales to put at the top of the list.

This is often based on existing “hints” like country codes (for example from the physical location, and / or the SIM provider) and already selected locales.

For example Android looks at the country of the SIM provider, the country of the radio tower connected (~location), the top locale already selected (current UI locale).

So if I set my locale to French Belgium, I buy a SIM in US, and I am currently in Canada, the suggestions are:

* Dutch (Belgium): because of the country of the current UI locale (BE)
* English (Canada): because of the current location
* English (US): because of the SIM provider
* French (Canada): because of the current location
* Spanish (US): because of the SIM provider

Cherokee / Hawaiian / Lakota (all US) are excluded because Android is not localized into these locales. French Belgium is excluded because it is already selected (as UI locale)

The locale excluding some locales or not might be controlled with options.

This does not have to be in the Language Matcher proper, but a lot of the know-how and a lot of the data is common between such functionality and the matcher.

So even if we choose to expose this as a separate class instead of methods in the matcher, we should look at language matcher + suggestions as one single component.

---

# I18n Design Meeting Notes

Date: 2021-09-28

**Markus:** There are 3 parts that we're discussing here

1. Language notation / language matcher
1. Let's say that we've settled on a language to load but haven't specified which files to load that from. We need to figure out what to do. Ex: `de-Latn-LU` - do we look at `de-Latn`, `de-LU`? (when `de-Latn-LU` doesn't have an exact file for it)
1. How do we store the data so that we don't duplicate the data? Ex: `en-GB` and `en-001` is very similar, so how do we avoid storing those data twice

**Shane:** There are interesting things that can be done on step 3, but we have to first define what to do for step 2

**Mark:** CLDR already defines parent-child relationships for locales (see: `common/supplemental/supplementalData.xml`)

**Shane:** This parent locale data is a separate thing from the containment graph.

**Mark:** We used to use the containment graph for language matching, but it turned  out to give bad results. It would show that 2 variants of `en` in Africa would be said to be closer to each other than to the parent `en-GB`

**Markus:** Similarly, `en-AU` would be said to be far from `en-GB` because of geographical distance, but they are close culturally. But we could use containment data for region-in-region data, ex: `en-LU` could resolve to `en-150` because we don't already have explicit parent relationship data for `en-LU` and we know that the `LU` region is contained within region `150`

**Shane:** Let's prioritize improving the ICU/CLDR vertical fallback algorithm. We agree that it can be improved and should be improved in ICU+CLDR+ICU4X

**Mark:** Keep in mind that `de-Latn-LU` is not a common case in practice because de rarely gets tagged with a `Latn` script. But the `en-LU` to `en-150` problem is an example of cases that can be improved.

**Markus:** In ICU, if we don't find an `en-GB` bundle, we just chop off the subtags to look for a matching resource bundle. The `en-GB` bundle has a `%%parent` entry, and we resolve that by looking up the parent (`en-001`) to find a specific entry recursively. But we don't look up the `en-001` bundle if the `en-GB` is not available (?).

**Shane:** Yes, we can have this parent data stored separately in ICU4X in an engine that we can expose (externally?) as a separate step, which can allow it to be used both for improved language negotiation as well as resource lookup. (?)
