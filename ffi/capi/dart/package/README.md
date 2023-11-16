# Dart bindings for ICU4X
[![Docs](https://img.shields.io/badge/docs-pub.dev-blue)](https://pub.dev/documentation/icu/latest/) [![Build Status](https://github.com/unicode-org/icu4x/actions/workflows/build-test.yml/badge.svg)](https://github.com/unicode-org/icu4x/actions) [![pub package](https://img.shields.io/pub/v/icu.svg)](https://pub.dev/packages/icu) [![package publisher](https://img.shields.io/pub/publisher/icu.svg)](https://pub.dev/packages/icu/publisher)

Experimental Dart bindings for ICU4X.

Example:

```dart
main() {
  //TODO: Implement build.dart, `@Native` annotations, and compiled targets storage to remove this.
  init(path.absolute('path/to/dynamic/library'));

  final x = FixedDecimal.fromDoubleWithLowerMagnitude(1.49403, -7);
  print(x); //prints '1.4940300'
}
```