import 'package:icu/lib.dart';
import 'package:path/path.dart' as path;
import 'package:test/test.dart';

void main() {
  init(path.absolute("test/libicu_capi_cdylib.so"));

  test('ICU4XFixedDecimal.toString', () {
    final x = ICU4XFixedDecimal.createFromF64WithLowerMagnitude(1.49403, -7);
    expect(x.toString(), "1.4940300");
    print(x.toString());
  });

  test('ICU4XLocaleFallbacker', () {
    final iterator = ICU4XLocaleFallbacker.create(
            ICU4XDataProvider.createCompiled())
        .forConfig(ICU4XLocaleFallbackConfig()
          ..extensionKey = "ca"
          ..priority = ICU4XLocaleFallbackPriority.Region)
        .fallbackForLocale(ICU4XLocale.createFromString("de-CH-u-ca-japanese"));
    while (iterator.get().toString() != "und") {
      print(iterator.get().toString());
      iterator.step();
    }
  });
}
