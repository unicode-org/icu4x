# CLDR "Hunkspace"

2020-07-09

Authors: 

* [filmil (Filip Filmar) · GitHub][ff]
* [zbraniecki (Zibi Braniecki) · GitHub][zb]

*tl;dr:* Generalize the [ICU4X Data Provider][dp] key space to include
localized data from sources other than [CLDR.][cldr] More detail is in the
[Detailed Proposal](#h.us1rqruwdp1h) section, after the proposal is motivated
by a few use cases.

This document proposes a global, shared *hunkspace* (better names always
welcome) of localized "[hunk]" data for the ICU4X Data Provider.  It
doesn't necessarily affect the ICU4X data provider implementation, but does
affect the data model.   Specifically the proposal is to make the data model
more general and open to independent third party extensions.

> The terminology proposed in [ICU4X Locale Data Pipeline][icudp] is used.  Any
> inconsistent word use is a bug, and should be fixed.

[cldr]: http://cldr.unicode.org
[dp]: https://github.com/unicode-org/icu4x/blob/main/docs/design/data_pipeline.md
[ff]: https://github.com/filmil
[hunk]: https://github.com/unicode-org/icu4x/blob/main/docs/design/data_pipeline.md
[icudp]: https://github.com/unicode-org/icu4x/blob/main/docs/design/data_pipeline.md
[zb]: https://github.com/zbraniecki

## Prior Art

The *ICU4X Locale Data Pipeline* document proposes a [data model][dm] that fits
the current use case of CLDR data. The data storage method is abstracted from
the API by using *Data Providers*.  The API is using the
`key:request/value:response` pattern.  See [Request][rq] in the source code for
the example key.

[dm]: https://github.com/unicode-org/icu4x/blob/main/docs/design/data_pipeline.md
[rq]: https://github.com/unicode-org/icu4x/blob/main/provider/cldr/src/lib.rs

## Use cases

> This section lists a few use cases where a shared hunkspace are helpful.

### Third party language packs

The use case comes from [Fuchsia OS][fx], though it is not a
unique need. The explanation below gives the context of the use case, and ends
with the description of the case itself.

Fuchsia software is *ephemeral* (downloaded on demand) and is
*trusted* (verified prior to execution). So, if a process runs on Fuchsia,
there is a cryptographic proof that one can use to verify the provenance of the
software. See the [overview of Fuchsia][fxov] for the reasons behind this
design choice, as it is out of scope here.

[fx]: https://fuchsia.dev
[fxov]: https://fuchsia.dev/fuchsia-src/concepts

Fuchsia's software delivery is at odds with the usual approach of
building and distributing software language packs for large software
products. Here, a software product is "large" if it satisfies the
following conditions:

1.  The volume of locale-specific customization is large.

    The volume is estimated with respect to some common human measure;
    e.g. having tens of thousands of localizable resources is likely a
    large volume of messages, corresponding to a large volume of
    pertinent use cases.

2.  The user base speaks 100+ languages.

    This means there will always be languages which will be impractical
    for the publisher to support. And there will always be languages for
    which the user community could offer a superior user
    experience.

Examples of products expected to fit this definition are: Firefox,
Chromium, Thunderbird, Gmail.

The implications for language packs are:

1.  Language packs are distributed separately from the "main" body of
    the software package.  This is done to control the software
    distribution size.  Distribution size is a known
    \[\[citation\_needed\]\] factor to product adoption. Developer teams
    usually monitor it closely for this reason.
2.  Third party language pack contributions are common and to be expected. 
	Different approaches exist that make this possible.   For examples, see
	[Mozilla Localization][ml10n] or [ᎭᎴᎾ ᏗᏓᏴᎳᏛᏍᎩ ᎬᏗ Gmail ᏣᎳᎩ][gmailch].

[gmailch]: https://gmail.googleblog.com/2012/11/gmail-get-started-with-gmail-in-cherokee.html
[ml10n]: https://l10n.mozilla.org

Item (2) has a consequence that a language pack for a software package may come
from a place that the software distributor doesn't even know exists.  In the
simplest form, you can download a language pack from a website of your choice,
register it with your piece of software and you get a localized version of your
program. This is approximately how [Mozilla's
XPI](https://developer.mozilla.org/en-US/docs/Mozilla/XPI) works with
localizations.

In a world of verified execution, the XPI drop-in approach is no longer
viable. Since only verified data can be downloaded, it is no longer
possible for a piece of software to allow drop-in localization
replacements. So, for a third party contribution to be usable with a
software package, it will have to be verified at the platform level to
ever stand a chance of appearing as a piece of software
configuration.

It would be impractical to require every piece of software to implement its own
verification method. This means, language pack distribution and verification
falls into the purview of the underlying software platform. In Fuchsia's case,
that platform is the operating system.  One can imagine a centralized
[localization] clearinghouse service, which takes care of the provision of
localized assets, similar to how a software delivery framework (say
[TUF](https://theupdateframework.io)) provides packages and files.

A data provider is a natural abstraction for a language pack.  A piece of
software can request a hunk of data from the provider, such as a slice of CLDR.

But, since we centralized the data provision, we could use exactly the
same mechanism to request other types of localized data that is
not a part of CLDR.  Such as, for example, a translation of the
string "Firefox" into Urdu.  While everyone could roll their own
approach to localization (as was the case historically) , having a
common specification could go a long way towards a common solution that
works well in many use cases.  Similar to how the industry has mostly
used the ICU library for handling internationalization matters.

### Delegated localization

The language pack approach works reasonably well to support a known set
of localizable resources.  There are use cases, however, for which a
program does not know the closed set of localizable resources.

This use case arose recently, again, in Fuchsia OS, related to the work on
Accessibility Screen Reader. Fuchsia encourages a modular approach to building
programs, which means that what we consider "apps" internally consists of a
number of communicating services.  The discussion of
[FIDL](https://fuchsia.dev/fuchsia-src/development/languages/fidl), the IPC
method used, is out of scope of this document.

The general pattern that arises is shown in the diagram below, showing
the collaboration diagram of an actual interaction a user has when using
audio output from the Accessibility library.

[![Diagram showing interaction with users](pr002/image1.png)]

The Voice UI delegates the choice of "Message!" to a service it does
not know about in advance, but has to render the message. In the case of
the voice UI, "rendering of a message" is turning the text to synthetic
speech; but one can see how a similar situation can arise in other
aspects of the program lifecycle.  For example, a spell checker service
needs to report an error to the user.

Let's call the pattern "delegated localization": the localization
is produced by a component which is different from the component that
renders the localized resource to the user. The renderer of the
message does not have access to the to-be rendered message ahead of
time.

Another instance of delegated localization is an application using
platform-level localizations for common user interface elements ("OK",
"Cancel", "Edit"\...).   Mechanisms to do this are reinvented in each
independent piece of software.  A localization clearinghouse would allow
the OS to centralize localization decisions.

A way to resolve is to devise some form of a mini "rendering protocol"
where the screen reader provides rendering hints, and the voice UI
renderer follows the hints.  The claim here is that this specific
pattern is common enough that localization could benefit from a common
specification of that rendering protocol.

## Detailed Proposal

Allow independent third parties to put new non-overlapping points into
a common hunkspace, such that for each platform a single data provider
can be used to provide any localized resource.

Specify a mechanism by which this can be done, e.g. by adopting an
URI-like encoding of hunkspace points, and providing a two-way mapping
between an URI and an "unpacked" version of a hunkspace key.

**Why this complication?** It allows third party contributions to localized
software in trusted computing environments.  For example, third party provision
of localized resources, as described in the Fuchsia use case.

**Why should ICU4X care?** Strictly speaking perhaps it doesn't need to. One
can build a "hunkspace" data provider given a cldr data provider, and fixing
the "source" dimension to be "org.unicode.cldr" (see below).  An advantage of a
commonly understood specification is that it may motivate software authors to
provide mutually compatible data provider interfaces.

**Do we even need common schemas like this?** In a world where localizations
are islands of functionality, maybe the answer is no.  A CLDR data provider can
exist on its own.  Firefox has its own elaborate L10N framework.  Fuchsia OS
has use cases where platform-level I18N/L10N data provision is needed, but
these use cases can live inside Fuchsia indefinitely.  It starts to matter if
these worlds get to interact at some level. For example Firefox L10N can not
exist in its current extensible form on Fuchsia, due to the tight controls that
the OS puts in place on the runtime. (You'd have to download a prebuilt version
for your locale, from Mozilla.)  Having a cross-platform data provider
specification could make that a non-issue.]

### Unique IDs

Every localizable resource should get a unique identifier.  This allows
a platform-level localization clearinghouse (from the example above) to
be a one-stop data provider for localization needs.

For example, the [current proposal of the icu4x data provider for
CLDR][dpcldr] uses a 4-dimensional key space consisting of:

- A Language identifier;
- a Category;
- a Key;
- a Payload.

[dpcldr]: https://github.com/unicode-org/icu4x/blob/main/provider/cldr/src/lib.rs

This key space is intended to fit any CLDR resource according to the data
mapping which is internally defined by CLDR rules.

It does not represent a fictitious resource string `Hello, {user}` from
Firefox v143, for example.  So we allow the key space to have more
dimensions than the current 4 of CLDR, to allow us to address both
resources uniquely.  Call that key space a "hunkspace".

At minimum this means adding a "source" dimension to the key space,
allowing for example "cldr.unicode.org" to be used as a (fixed)
dimension for all CLDR data.  We could have a "fuchsia.com" source that
could use a custom data schema like so:

```
Key: {
  source = "fuchsia.dev",
  app="weirdy-birds",
  message_id=42,
  locale="en-US"
}
Value: "Hello, {user}"
```

A note is in order on what constitutes "identical" messages.
Two messages are identical if and only if they map to the same point in
the hunkspace, which means that they are interchangeable no matter what
the context.

Two messages that happen to have identical strings, such as "Print" (1)
and "Print" (2) won't necessarily be interchangeable, if one is used to
mean "Go print this!", while the other is used to stand for "something
which is printed".  This is because these two messages could be
displayed differently in other languages. "Штампај", "Одштампано" would
be the respective translations in another language. The hope here is
that it would always be possible to split a keyspace point into two by
providing finer-grained context where needed, but discussions of that
are out of scope.

A CLDR-only data provider would be a data provider which fixes the
"source" string to the value "cldr.unicode.org".

### Requirements

Some requirements distilled from the discussion above:

1.  There must be a commonly understood data schema for each data set.
     For example "CLDR unicode data",
1.  It must be possible for that data to come from multiple sources.
     E.g. "mozilla.org", vs "google.com", vs. "unicode.org".  Every
    source that provides a specific data schema can have own data.
1.  The key space is multi-dimensional. Each dimension is named, and
    the set of dimensions is specific to each data schema.  Example
    dimension name: `message_id`.
1.  The dimensions of the key space are [not]{.c1} ordered.

## Key encoding schema

Taking into account the needs for the shape and form of the keys
discussed above, and the requirements, here are proposed mappings.

It is allowed for the key encoding to have different equivalent
schemas, which can be used where convenient.  Two proposed schemas are
below, one for URI, another for JSON.

### URI

The following mapping specifies how the dimensions of the "hunkspace"
key are mapped to an URI.

* Data schema (always required):
  * Key: `scheme`
  * URI: "scheme"
  * Remark: Perhaps we could sync up the namings so that we don't need to
	maintain an active mapping like this.  Every scheme must define the data
	schema for each value under a specific key. The data user is supposed to be
	familiar with the data schema and able to parse the returned conforming
	data.
* Source (always required):
  * Key: `source`
  * URI: "authority"
  * Remark:  We propose here the authority should be using the [FQDN][fqdn] form such
    as `icu4x.unicode.org`, as this may be more immediately recognizable due
	to its prevalent use on the Internet.
* Dimension
  * Key: (a simple string)
  * URI: path component such as `...locale_id/42/...`
  * Remark: Every path component becomes a pair of path elements separated by
  * a forward slash (`/`).

[fqdn]: https://en.wikipedia.org/wiki/Fully_qualified_domain_name

For a multi-dimensional key, there may be several equivalent URI
mappings, corresponding to different possible orderings of the
dimensions and their values.  This allows us to omit any of the
dimensions when specifying the URI, which can prove to be useful in
query formulation.   It eliminates the guesswork, as every URI can be
readily parsed into a key, and eliminates the need to introduce a
hierarchical ordering in the URI, if such a hierarchy does not need to
exist.

> It is unclear at the moment if any of the encodings should be the
"canonical" one.

> We may need to either keep all dimensions flat.

Example encodings are given below.

```
Key: {
  source = "fuchsia.dev",
  app="weirdy-birds",
  message_id=42,
  locale="en-US"
}
Value: "Hello, {user}"
```

#### Example 1: Unicode data

Key:

```
{
  "scheme" = "udata"
  "source" = "icu4x.unicode.org"
  "langid" = "en-US"
  "category" = "Decimal"
  "key" = "symbols-v1"
}
```

URI encoding:

```
udata://icu4x.unicode.org/langid/en-US/category/decimal/key/symbols-v1
udata://icu4x.unicode.org/category/decimal/key/symbols-v1/langid/en-US
udata://icu4x.unicode.org/key/symbols-v1/langid/en-US/category/decimal
```

> Any permutations of dimensions with the same values are equivalent.  

> Perhaps we want to specify a canonical ordering of dimensions, for simpler
> comparison.

#### Example 2: Localization

Key:

```
{
  "scheme" = "fuchsia-l10n"
  "source" = "fuchsia.dev"
  "app" = “program-name"
  "locale" = "en-US"
  "message_id" = "42"
}
```


URI encoding:

```
fuchsia-l10n://fuchsia.dev/app/program-name/locale/en-US/message-id/42
```

> “fuchsia-l10n” scheme means “localization, in Fuchsia style”.  “mozilla.org”
> may define a different scheme that matches the localization process.
> Other acceptable permutations are omitted for brevity.

### JSON

#### Example 1: Unicode data

Key:

```
{
  "scheme" = "udata"
  "source" = "icu4x.unicode.org"
  "langid" = "en-US"
  "category" = "Decimal"
  "key" = "symbols-v1"
}
```

JSON encoding:

```json
{
  "scheme": "udata",
  "source": "icu4x.unicode.org",
  "langid":  "en-US",
  "category": "decimal",
  "key": "symbols-v1"
}
```

#### Example 2: Localization

Key:

```
{
  "scheme" = "fuchsia-l10n"
  "source" = "fuchsia.dev"
  "app" = "program-name”
  "locale" = "en-US"
  "message_id" = "42”
}
```

JSON encoding:

```json
{
  "scheme": "fuchsia-l10n",
  "source": "fuchsia.dev",
  "app": "program-name",
  "locale": "en-US",
  "message_id": "42"
}
```

#### Example 3: Firefox Localization and localization contexts

This section gives an example of query use in a software product with complex
localization requirements.  See the [Mozilla documentation][ffl10n] for details
on Firefox localization. The summary is that all the Firefox user interface is
declarative, which is achieved using a mix of SGML-based technologies.  This
includes localization.

* **Note.** The intention of this section is not to give a definitive design
  for the feature.  Instead, it is given to demonstrate how complex localization
  requirements *could* be encoded in a hunkspace.  The precise form of the
  requests and responses is subject to specific needs and design constraints.

Some specific points of Mozilla's localization approach that are pertinent to
the exposition below, as they provide requirements and constraints that an 
implementor must apply.

1. The localization is context-sensitive (see below), which means that the
   requester needs to be able to initialize and maintain this context through
   the lifeetime of the program.

2. The volume of localization text may be large, and ferrying it as a sequence
   of request-response pairs between the program and the provider may contend
   with the latency budget of the program's user interface.

[ffl10n]: https://firefox-source-docs.mozilla.org/intl/index.html

Here is [an example UI declaration][sfx], quoted below in a condensed form, of
the file `//source/browser/components/preferences/preferences.xhtml`.

[sfx]: https://searchfox.org/mozilla-central/source/browser/components/preferences/preferences.xhtml

```html
<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml"
        xmlns:html="http://www.w3.org/1999/xhtml"
        role="document"
        id="preferences-root">
<head>
  <title data-l10n-id="pref-page-title"></title>
  <link rel="localization" href="branding/brand.ftl"/>
  <link rel="localization" href="browser/branding/brandings.ftl"/>
  <!-- … -->
</head>
<!-- … -->
<html:body/>
```

In turn, for example the file [`browser/branding/brandings.ftl`][brandings] has
the following, with some details omitted for brevity:

[brandings]: https://searchfox.org/mozilla-central/source/browser/locales/en-US/browser/branding/brandings.ftl

```
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.
-facebook-container-brand-name = Facebook Container
-lockwise-brand-name = Firefox Lockwise
-lockwise-brand-short-name = Lockwise
-monitor-brand-name = Firefox Monitor
-monitor-brand-short-name = Monitor
-pocket-brand-name = Pocket
-send-brand-name = Firefox Send
-screenshots-brand-name = Firefox Screenshots
```

One will notice that the declaration of `preferences.xhtml` admits
the UI author to declare the source for the bindings, i.e. the values
for specific message identifiers, by specifying a path to a file that
contains the bindings.

This, in turn, means that the value to which a certain message
identifier is bound, for example `-monitor-brand-name` (which can be
seen in `brandings.ftl`), may depend based on the binding decision
taken in the context of the UI widget.  This also means that another UI
file may decide to use the same message identifier in a different UI
widget context.

While the decision to go to such lengths is at the discretion of the
implementor, it nevertheless compels us to provide a way to address such
a binding to the data provider.  That is, the data provider must be able
to address a specific message.

##### Context specifiers

When fewer messages are needed, I propose using "context specifiers", which
allow adding "auxiliary" data that allows disambiguation of the context.
 Since the aim of the data provider is to form a flat key space, and the nature
of the auxiliary data may have arbitrary internal structure, it compels us to
move the auxiliary data outside of the "regular" key dimensions.

For this reason, the "auxiliary" data can be specified as URI query parameters,
or with JSON arguments that start with a "?" symbol.   The resulting URI and
JSON forms are given below.

URI:

```
mozilla-l10n://mozilla.org/app/firefox/locale/en-us/message-id/-monitor-brand-name⮒
    ?from=browser/branding/brandings.ftl⮒
    &to=source/browser/components/preferences/preferences.xhtml
```

JSON:

```json
{
  "scheme": "mozilla-l10n",
  "source": "mozilla.org",
  "app": "firefox",
  "locale": "en-us",
  "message-id": "-monitor-brand-name",
  "?from": "browser/branding/brandings.ftl",
  "?to": "source/browser/components/preferences/preferences.xhtml"
}
```

The keywords `?from` and `?to` are implementation defined, and a data
provider serving the scheme "mozilla-l10n" will need to understand the
intended semantics.

Going a step further, it is also possible to obtain a "context token" from the
proider, which encodes an opaque local state to which the query needs to be
applied:

URI:

```
mozilla-l10n://mozilla.org/app/firefox/locale/en-us/message-id/-monitor-brand-name⮒
    ?token=0xdeadbeef
```

JSON:

```json
{
  "scheme": "mozilla-l10n",
  "source": "mozilla.org",
  "app": "firefox",
  "locale": "en-us",
  "message-id": "-monitor-brand-name",
  "?from": "browser/branding/brandings.ftl"
}
```

##### Complex context

Performance concerns may dictate that batches of messages need to
be transmitted in a limited number of requests, to handle the bandwidth and
latency constraints imposed by other parts of the system, such as the user
interface or the throughput of the connection used to ferry the communication.

In this case, we may find a situation where a single URI will not encode 
all the context information in a practical manner.

JSON request:
```
{
  "scheme": "mozilla-l10n",
  "source": "mozilla.org",
  "app": "firefox",
  "locales": ["ru", "fr", "en-US"],
  "resource_ids": [
    "browser/preferences.ftl",
    "toolkit/menu.ftl",
    "branding/brands.ftl"
  ],
  "message_keys": [
    {"id": "prefs-document-title", "args": null},
    {"id": "prefs-open", "args": null},
    {"id": "prefs-cancel", "args": null},
    {"id": "prefs-header", "args": { "user": "John" }},
  ]
}
```

A major difference with respect to the previous approach, it is now impractical
to encode the entire set of needed information as a URI, so will need to be
formulated in terms of a message sent to the endpoint:

```
mozilla-l10n://mozilla.org/firefox
{
  "locales": ["ru", "fr", "en-US"],
  "resource_ids": [
    "browser/preferences.ftl",
    "toolkit/menu.ftl",
    "branding/brands.ftl"
  ],
  "message_keys": [
    {"id": "prefs-document-title", "args": null},
    {"id": "prefs-open", "args": null},
    {"id": "prefs-cancel", "args": null},
    {"id": "prefs-header", "args": { "user": "John" }},
  ]
}
```

The response:

```
{
  "messages": [
    {  "value": "Your Preferences", "attrs": [], "locale": "ru" },
    {  "value": "Open", "attrs": { "accesskey": "o" }, "locale": "ru" },
    {  "value": "Cancel", "attrs": { "accesskey": "c" }, "locale": "ru" },
    {  "value": null, "attrs": { "label": "Hello, John" }, "locale": "en-US" },
  ],
  "errors": [],
}
```

This can alternatively be encoded as a two-step process:

```
{
  "scheme": "mozilla-l10n",
  "source": "mozilla.org",
  "app": "firefox",
  "locales": ["ru", "fr", "en-US"],
  "resource_ids": [
    "browser/preferences.ftl",
    "toolkit/menu.ftl",
    "branding/brands.ftl"
  ],
}
```

Response:

```
{
  "context_id": "34jf2n#3j3n1",
  "errors": [],
}
```

Then the request will specify the `context_id` to refer back to the information
already transferred:

```
{
  "scheme": "mozilla-l10n",
  "source": "mozilla.org",
  "app": "firefox",
  "context_id": "34jf2n#3j3n1",
  "message_keys": [
    {"id": "prefs-document-title", "args": null},
    {"id": "prefs-open", "args": null},
    {"id": "prefs-cancel", "args": null},
    {"id": "prefs-header", "args": { "user": "John" }},
  ]
}
```

This proposal glosses over the gritty detail of setting up the two-way
communication over a common context, such as cache invalidation, context
cleaning and availability concerns.  The designers would neeed to take them
into account when developing a production version of this proposal.  The URI
encoding approach still favors statically defined messaging where such
messaging is viable.

The upside remains, which is the ability to handle a reference to a repository
of localized data in an uniform manner.

## Conclusion

This proposal describes an encoding of localization contexts to URIs as well as
JSON requests.  The proposed benefit is the ability to identify uniquely
individual localized messages, or batches of such messages in a uniform manner.
The hope is that this approach would make it easier to produce generalized data
providers that are able to serve pluggable, cross-application, localizations.

