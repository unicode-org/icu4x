// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import 'package:icu4x/icu4x.dart'
    show DateFormatter, DateTimeLength, IsoDate, Locale;

void main(List<String> arguments) {
  final locale = Locale.fromString('de-CH');

  final date = IsoDate(2025, 1, 2);

  final formatter = DateFormatter.ymde(locale, length: DateTimeLength.medium);

  print(formatter.formatIso(date));
}
