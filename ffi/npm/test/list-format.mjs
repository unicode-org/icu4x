// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import test from 'ava';

import { Locale, ListFormatter, ListLength, Parts, PartKind } from 'icu';

const locale = Locale.fromString('en');
const formatter = ListFormatter.createAndWithLength(locale, ListLength.Wide);

test('format list to parts with UTF-16 indices and reuse Parts', t => {
  const parts = new Parts();
  t.truthy(parts.isEmpty);
  t.is(parts.length, 0);
  t.is(parts.get(0), null);

  // Includes non-BMP U+1F973 (surrogate pair = 2 UTF-16 code units)
  const out = formatter.formatToParts(['Alice', 'Bob', 'Céline 🥳'], parts);
  t.is(out, 'Alice, Bob, and Céline 🥳');
  t.falsy(parts.isEmpty);
  t.is(parts.length, 5);
  t.is(parts.get(5), null);

  const expected = [
    { start: 0, end: 5, kind: PartKind.ListElement, text: 'Alice' },
    { start: 5, end: 7, kind: PartKind.ListLiteral, text: ', ' },
    { start: 7, end: 10, kind: PartKind.ListElement, text: 'Bob' },
    { start: 10, end: 16, kind: PartKind.ListLiteral, text: ', and ' },
    { start: 16, end: 25, kind: PartKind.ListElement, text: 'Céline 🥳' },
  ];

  for (let i = 0; i < expected.length; i++) {
    const span = parts.get(i);
    t.truthy(span);
    t.is(span.start, expected[i].start);
    t.is(span.end, expected[i].end);
    t.is(span.kind, expected[i].kind);
    t.is(out.slice(span.start, span.end), expected[i].text);
  }

  // Reusing the same `Parts` instance clears previously recorded spans
  const out2 = formatter.formatToParts(['One', 'Two'], parts);
  t.is(out2, 'One and Two');
  t.is(parts.length, 3);
  t.is(parts.get(3), null);
  t.is(out2.slice(parts.get(0).start, parts.get(0).end), 'One');
  t.is(out2.slice(parts.get(1).start, parts.get(1).end), ' and ');
  t.is(out2.slice(parts.get(2).start, parts.get(2).end), 'Two');
});
