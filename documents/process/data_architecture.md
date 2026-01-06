# Principles of data architecture

When adding new data to ICU4X, we attempt to design it such that:

 * If a client is not using a piece of *functionality*, the data for that functionality will not be linked in via baked data or `--markers-for-bin`-ed blob data.
 * If a client is not using a particular locale or type of input, the data for that locale can be omitted via appropriate filtering in datagen.
 * Data for multiple pieces of functionality should be shared where possible.

These are broad principles which we cannot always attain, but they form a good north star.

This document attempts to provide guidance on how to structure data, and document policies around that.

## Zero-copy

The only inviolable requirement for ICU4X data is that it must be possible to zero-copy deserialize. The data model may (and usually will) allocate, but only optionally, when e.g. loading from human-readable data sources. The `zerovec` crate contains many abstractions which automatically handle this for you.

At the moment, we do not have a tutorial on writing zero-copy data structures, but we might add one in the future. In the meantime, feel free to ask an ICU4X team member for help with this.

## Filtering data for unused functionality

By and large, we organize data into various "data markers"[^1], which implement the [`DataMarker`] trait. ICU4X types which load data will typically have a `try_new_unstable` function bounded by `P: DataProvider<MyDataMarker>`, which gets internally called by other constructors. Baked data constructors typically load from the internal `provider::Baked` provider.

If a baked constructor is called, this will automatically link in the data it depends on. If any other constructor is called, appropriate breadcrumbs will be placed in the binary so that `icu4x-datagen --markers-for-bin` can automatically figure out which markers were used.

As such, organizing data into different "markers" is the primary mechanism by which data can be sliced for ICU4X-using applications.

## Filtering data for locales

By and large, we have two kinds of data markers. "Singleton" markers, for things like Unicode properties, and locale-based markers (e.g. date time formatting data). For singleton markers there is only one chunk of data associated with it, whereas locale-based markers have one chunk of data per locale.

ICU4X ships with a set of default locales, but developers can use `icu4x-datagen` to customize it. The idea is that data markers slice based on the used functionality (your application either formats dates or does not), and locales slice based on the locales you wish to ship, which is something that you may wish to easily tweak after writing your application, or have some dynamic data loading solution for.

Typically, this maps to *user* locale, since `icu4x-datagen` is a tool that is invoked by the developer. At the moment, we do not have any data that is organized by *content* locale (the locale of the content the user is seeing). Rather, our current situations where content locale is used (segmenters, casemapping) use non-locale-dependent data (either singleton data or data using data marker attributes), and pass content locale in as an option.

## Filtering data via data marker attributes

Sometimes you want locale-like filtering for things that are not locales, or wish to have an additional layer of lookup to data loading so you can have multiple pieces of data per marker/locale.

For this, we have "data marker attributes". They are used a couple different ways:


In date time formatting, they are used to differentiate between different formatting "lengths" (e.g. M vs Mon vs Monday). An application performing date time formatting usually does not know in advance exactly what lengths it will need (since this varies per locale and based on the preferences used). However, we do not want `DateTimeFormatter` to load all of these lengths at once when being constructed: when it is constructed, it *does* know what lengths it needs, and it should just load the requested data. As documented on [`YearNames`], we use a data marker attribute to distinguish.

In unit formatting, we use data marker attributes for units. This allows a unit formatter to load the data for just the unit it needs, and it allows an application to pre-filter based on units via `icu4x-datagen`. A similar thing is done in our display names component, using attributes for the name of the locale/region/etc being formatted.

Note that data marker attributes reduce data load size[^2], but do not automatically filter anything; all filtering must be done by the developer via `icu4x-datagen`.

## Attributes vs separate markers

A common design tradeoff is whether to use attributes or separate markers to slice by a particular property. For example, we have structured units as being separate attributes on the same markers, but formatting information for calendars are separate markers, per calendar.

We do not have a hard and fast rule for which to pick here, but we have some guidelines.

Separate markers have the benefit of being automatically sliceable, which directly leads to less code/data size for people who do not need everything. Therefore, it is ideal to try and see if separate markers can be made to work nicely.

Separate markers have some downsides: code using separate markers usually needs a separate constructor for each marker, and the number of APIs can explode in size. If there are valid use cases involving loading different pieces of data based on runtime settings, you may need some sort of manual dynamic dispatch solution, akin to `AnyCalendar`, to make that work. In such a situation, the manual dynamic dispatch solution will link in all the data.

Separate markers also need the addition of a new marker whenever the property gets a new possible value. This makes them a poor choice for often-expanding things like units, and basically impossible for unbounded sets like locale display names.

## Policy around overlapping data

Data can often be shared between functionality by splitting it down. For example, a lot of the Unicode properties data is used by other components, like casemapping.

If you are writing multiple APIs that use overlapping data, split the data such that the overlapping data is separated and can be loaded by both. If this does not seem possible, or comes in conflict with other goals, this policy can be overridden by a TC decision (policy from [#3022]).


In *general* we prefer splitting data where possible. However, sometimes it happens where related pieces of data are basically only ever loaded together, and there can be scope for merging them into more data-efficient structures. [`BidiMirroringGlyph`] is an example of this: it is a type that represents three related properties, packed together instead of supporting individual data loads for all three.


 [^1]: Occasionally called "data keys" in older documentation.
 [^2]: Data load size is by and large not a problem in the first place when zero-copy deserializing
 [`DataMarker`]: https://unicode-org.github.io/icu4x/rustdoc/icu_provider/trait.DataMarker.html
 [`YearNames`]: https://unicode-org.github.io/icu4x/rustdoc/icu/datetime/provider/neo/enum.YearNames.html
 [`BidiMirroringGlyph`]: https://unicode-org.github.io/icu4x/rustdoc/icu/properties/props/struct.BidiMirroringGlyph.html
 [#3022]: https://github.com/unicode-org/icu4x/issues/3022