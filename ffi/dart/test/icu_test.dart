import 'package:icu/icu.dart';
import 'package:path/path.dart' as path;
import 'package:test/test.dart';

void main() {
  init(path.absolute('test/libicu_capi.so'));

  test('FixedDecimal.toString', () {
    final x = FixedDecimal.fromDoubleWithLowerMagnitude(1.49403, -7);
    expect(x.toString(), '1.4940300');
  });

  test('LocaleFallbacker', () {
    final iterator = LocaleFallbacker(DataProvider.compiled())
        .forConfig(LocaleFallbackConfig()
          ..extensionKey = 'ca'
          ..priority = LocaleFallbackPriority.region)
        .fallbackForLocale(Locale.fromString('de-CH-u-ca-japanese'));
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
        LocaleFallbackConfig()
          ..extensionKey = 'ca'
          ..priority = LocaleFallbackPriority.region,
        LocaleFallbackConfig()
          ..extensionKey = 'ca'
          ..priority = LocaleFallbackPriority.region);
    expect(
        LocaleFallbackConfig()
          ..extensionKey = 'nu'
          ..priority = LocaleFallbackPriority.region,
        isNot(LocaleFallbackConfig()
          ..extensionKey = 'ca'
          ..priority = LocaleFallbackPriority.region));
  });

  test('Runes', () {
    Rune a = 'a'.runes.first;
    Rune emoji = '💡'.runes.first;

    final emojiSet = CodePointSetData.loadEmoji(DataProvider.compiled());
    expect(emojiSet.contains(a), false);
    expect(emojiSet.contains(emoji), true);

    Rune upperA = CaseMapper(DataProvider.compiled()).simpleUppercase(a);
    expect(String.fromCharCode(upperA), 'A');
  });
}
