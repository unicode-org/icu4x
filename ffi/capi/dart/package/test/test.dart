import 'package:icu/lib.dart';
import 'package:path/path.dart' as path;
import 'package:test/test.dart';

void main() {
  init(path.absolute('test/libicu_capi_cdylib.so'));

  test('ICU4XFixedDecimal.toString', () {
    final x = ICU4XFixedDecimal.fromF64WithLowerMagnitude(1.49403, -7);
    expect(x.toString(), '1.4940300');
  });

  test('ICU4XLocaleFallbacker', () {
    final iterator = ICU4XLocaleFallbacker(ICU4XDataProvider.compiled())
        .forConfig(ICU4XLocaleFallbackConfig()
          ..extensionKey = 'ca'
          ..priority = ICU4XLocaleFallbackPriority.region)
        .fallbackForLocale(ICU4XLocale.fromString('de-CH-u-ca-japanese'));
    expect(iterator.get.toString(), 'de-CH-u-ca-japanese');
    iterator.step();
    expect(iterator.get.toString(), 'de-CH');
    iterator.step();
    expect(iterator.get.toString(), 'und-CH-u-ca-japanese');
    iterator.step();
    expect(iterator.get.toString(), 'und-CH');
    iterator.step();
    expect(iterator.get.toString(), 'und');
  });

  test('Struct equality', () {
    expect(
        ICU4XLocaleFallbackConfig()
          ..extensionKey = 'ca'
          ..priority = ICU4XLocaleFallbackPriority.region,
        ICU4XLocaleFallbackConfig()
          ..extensionKey = 'ca'
          ..priority = ICU4XLocaleFallbackPriority.region);
    expect(
        ICU4XLocaleFallbackConfig()
          ..extensionKey = 'nu'
          ..priority = ICU4XLocaleFallbackPriority.region,
        isNot(ICU4XLocaleFallbackConfig()
          ..extensionKey = 'ca'
          ..priority = ICU4XLocaleFallbackPriority.region));
  });
}
