# ICU4X: Handling horizontal fallbacks
([Issue #259](https://github.com/unicode-org/icu4x/issues/259))

## Introduction

When several components / layers talk about “locales” or “locale data” we will (more often than not) notice mismatches.

So most libraries need to somehow address these mismatches and gracefully, if possible without the user noticing.

I will give a quick overview of the various levels where this can / should be addressed, with the names used. These names are not “industry standard”, or even common, but there is no commonly agreed terminology that I am aware of.
This will also clarify what we mean by “horizontal fallback”, and narrow down the scope of this document.

### Language negotiation

At the highest level, usually application or screen (sometimes “panel”) we need to agree on what language to use. The typical requirement at this level is to take an ordered list of locales that the user wants / understands, a list of locales that the component has available, and return the best match. Similar to two people who met for the very first time and are trying to figure out what language to use for a conversation.

At the end of this the whole application / screen / panel will end up using one language, not a mixture. There might be small “disagreements” (for example French-France, even if “we agreed” on French Canadian), but not across languages (so no German if we agreed French).

### Language fallback (or “vertical fallback”)

This is usually done without changing the language + script part of the locale. But the region might change. This is the most common kind of fallback, one that most OSes and frameworks implement today, and what most developers understand when they say “locale fallback.”

It is often used to load “locale dependent resources” (localized messages, month names, sounds, themes).

A common option is a very simple “cuts from the right” strategy (language+region → language → root, which has the risk of ignoring the script if missing in the locale ID). Although simple, I would not recommend we implement this, as it has problems with languages using more than one script (Chinese Simplified / Traditional, or Serbian Cyrillic / Latin).

More sophisticated algorithms would go from Spanish Argentina (`es-AR`) to Spanish-Latin America (`es-419`) to other Spanish locales in Latin America (`es-MX`, or `es-US`) to generic Spanish (`es`) to “OK, give me any kind of Spanish you might have,” and finally to root.
(`es-AR` → `es-419` → `es-MX` / `es-US` → `es` → `es-*` → `root`)

I’ve also heard good arguments for the fallback to the next locale in the user list before going to `root` (or ultimate fallback). This would of course mean the whole mechanism being applied to each locale in the chain (imagine so a user listing Italian Swiss + French Swiss would go through `it-CH` → `it` → `it-IT` → `it-*` **→ `fr-CH` → `fr` → `fr-FR` → `fr-*`** → `root`.

I’ve also heard good arguments against.

I think this makes a good example where we can’t make a good decision as library owners, and will have to provide an option for the developers using the library to configure the behavior.

### Horizontal fallback

Used to load similar resources from inside the same locale.

Some examples are given in [issue #259](https://github.com/unicode-org/icu4x/issues/259). The issue also describes pretty well the options.

This kind of fallback is the scope of this document.

## Horizontal fallback

Like in most situations, there is no good answer to this problem. We will need to compromise.

The issue summarizes the options pretty well, and I will copy them here for convenience, with the risk of duplication / getting out of sync:

> The two drives that are extremes are:
>
> We want to make ICU4X DataProvider easy to use. Handling fallbacking in data provider means the user doesn't have to worry about missing data and fallback decisions.
>
> We want to make ICU4X DataProvider flexible. Being opaque about missing data means that the user has no flexibility in choosing how to handle fallbacks (do we always want to fallback from short to long if it's missing? Maybe in some scenarios users will want to fallback differently?
>
> If we go with fallback in the provider, we're likely going to have to add ability to get data without the fallback.
>
> If we go with fallback in the hands of consumer, we will need to help consumers know how to fallback and there's going to be a number of repetitive pieces of fallbacking in consumers.
>
> Finally, expanding all data in the provider means more data to carry over the wire. A good example is the second one - for locales where stand_alone matches format contexts, we could just carry over format and set stand_alone to null and the consumer then can say "if there is stand_alone, use it, if not, fallback on format".
>
> But if we expand all fallbacks in Provider, we'll have to for all locales carry duplicated data so that users can infallibly take stand_alone.
>
> I believe that, as in most cases, we'll be looking for the "third path" somewhere between those two extremes, and we'll need to resolve it on per-case bases, but I think that encoding in wiki (https://github.com/unicode-org/icu4x/wiki/Hooking-up-a-DataProvider) or docs some guidelines on how to generally make decisions on what is going to be expanded in Provider vs. in consumer, is in our best interest, so let's discuss it!

I think that in most cases the provider is in the best position to decide how to do this kind of horizontal fallback, because the provider “knows” the most about all the bits and pieces of the machinery.

The user might want some flexibility, but (more often than not) does not know the “internals.” If I ask for a “reasonably long date, something similar to ‘March 29, 2021’ in English” then what I can ask for is “a full month name, and numeric day / year.” But I don’t know anything about genitive month names, for example.

Worse, when you format a date there are so many “pieces” that come together, each with its own fallback rules.

Things might (potentially) interfere with the vertical fallback. For example if one cannot find a medium date pattern for English Great Britain, but there is a long date pattern (`"d MMMM y"`). Should we do a vertical fallback to “generic English (`en`, which is actually US English), and get `"MMMM d, y"` (which changes the month-day order) or alter the long pattern, algorithmically replacing `"MMMM"` with `"MMM"` and getting `"d MMM y"`?

And some rules can influence each other. What happens if we do vertical fallback for a date pattern, but horizontal fallback for the month names? Can we do this, or should we try to use the same fallback strategy for all “pieces” that come together in the final format?

So I think that the best we can do is:

1. Let the data provider decided on when, how, and between what “pieces” to do horizontal fallback
2. Offer some “rough knobs” for the end user for flexibility. But I don’t really see how we can give full control. So they would be there to guide the general strategy (“favour vertical fallback over horizontal fallback”), not low level (“disable genitive-to-nominative fallback”)
3. Offer some “rules of thumb” for the implementers on when to do fallback, what kind, and what kind of control to expose for the end user

## Rules of thumb

### A. Favor vertical fallback (with one caveat)

CLDR does vertical fallback. So in general if something is missing we can assume that the “parent” locale has the proper data. Or is a bug.

So we should favor vertical fallback by default, **but only as long as that fallback does not go to root**.

What I mean by that? That between the last step of vertical fallback, from a language to root, we should “insert” a horizontal fallback.

So a Spanish-Argentina locale would try `es-AR` → `es-419` → `es` → `es-*` → `root` (or whatever level or “smartness” we put in the vertical fallback) should become `es-AR` → `es-419` → `es` → `es-*` → (horizontal to `es-AR`, `es-419`, `es`, `es-*`) → `root`

### B. Should not try to “fix” CLDR bugs with “smartness” in horizontal fallback

In general there is such a thing as “too smart for your own good.”

I have often seen applications trying to “massage” the locale data in code in order to get the result they expected.  
That risks damaging the correct results in other locales.

And it is a lot easier to fix the data and download it on demand with the next opportunity, rather than fix code that tried horizontal fallback and got it wrong.

### C. Don’t do horizontal fallbacks when there is a risk of confusion

In general horizontal fallback between two numeric forms (`"d"` ↔ `"dd"` or `"M"` ↔ `"MM"`) or between two alpha-forms (`"MMM"` ↔ `"MMMM"`) is safer than horizontal fallback between these categories (`"MM"` ↔ `"MMM"`). For example `"03/04/2021"` ↔ `"3/4/2021"` or `"March 4"` ↔ `"Mar 4"` does not carry a risk of confusion. But `"Mar 4, 2021"` → `"3 4, 2021"` is bad. Worse, in Canadian English the field order also changes (`"MMM d, y"` ↔ `"y-MM-dd"`, `MDY` vs `YMD`).

Or between format / standalone forms. You can safely go from format to standalone because it is basically using sentence casing (`"septembrie"` → `"Septembrie"`, in Romanian)

That is fine and works for all locales (warning: assuming you know how to do sentence casing correctly :-).

But you can’t algorithmically go from standalone to format, because that implies lowercasing. And not all locales do that (think English, where the formatting month name is still sentence case), and because lowercasing can be ambiguous (more than one mapping).

You might note from all these examples that you actually need to know A LOT about what you are trying to fallback between. And not only about the kind of data, but also about all kinds of locales.

So this takes me to the next rule of thumb 

### D. Don’t do horizontal fallback

I know this will be controversial… :-)

But in the end it is the safest option.
There are very few safe mappings, and most of them are more on the “let’s save memory / data size” side of things, rather than “let’s fix things / get better results”

So one can drop standalone month / day names if they are 100% identical to the formatting ones. Or drop genitive / nominative month names if they are 100% identical.

We don’t even save that much. The 12 long month names in English, when compressed, end up to just 91 bytes. The code added for horizontal fallback might just “cost” more than those 91 bytes (size, complexity, maintenance, bugs).

Trying to save data makes sense for big data, like collation (where there is not much horizontal fallback to do), or time zones, or country / language names, where (again) we can’t even do much horizontal fallback.

So let’s go back to the older “rule of thumb”:
> “Rules of Optimization:  
> Rule 1: Don't do it.  
> Rule 2 (for experts only): Don't do it yet.”  
>  
> *― Michael A. Jackson*  
> *premature optimization is the root of all evil.*

## Summary

I think that the provider should behave as if there is no fallback of any kind, vertical or horizontal.

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
