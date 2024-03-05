import 'package:icu/icu.dart';
import 'package:test/test.dart';

void main() {
  Logger.initSimpleLogger();

  test('FixedDecimal.toString', () {
    final x = FixedDecimal.fromDoubleWithLowerMagnitude(1.49403, -7);
    expect(x.toString(), '1.4940300');
  });

  test('LocaleFallbacker', () {
    final iterator = LocaleFallbacker(DataProvider.compiled())
        .forConfig(LocaleFallbackConfig(
            extensionKey: 'ca',
            priority: LocaleFallbackPriority.region,
            fallbackSupplement: LocaleFallbackSupplement.none))
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

  test('Properties', () {
    Rune a = 'a'.runes.first;
    Rune emoji = '💡'.runes.first;

    final emojiSet = CodePointSetData.loadEmoji(DataProvider.compiled());
    expect(emojiSet.contains(a), false);
    expect(emojiSet.contains(emoji), true);

    Rune upperA = CaseMapper(DataProvider.compiled()).simpleUppercase(a);
    expect(String.fromCharCode(upperA), 'A');
  });

  test('ListFormatter', () {
    final formatter = ListFormatter.andWithLength(
        DataProvider.compiled(), Locale.fromString('es'), ListLength.wide);
    final list = List()
      ..push('España')
      ..push('Francia')
      ..push('Suiza')
      ..push('Italia');

    expect(formatter.format(list), 'España, Francia, Suiza e Italia');
  });
}
