import 'package:dart_icu4x/dart_icu4x.dart';

/// Extension methods for UnicodeCharProperties
extension UnicodeCharPropertiesX on UnicodeCharProperties {
  /// Convert UnicodeCharProperties to a JSON object
  Map<String, dynamic> toJson() => {
    'character': character,
    'codePoint': codePoint,
    'name': name,
    'unicodeValue': unicodeValue,
    'generalCategory': generalCategory,
    'block': block,
    'plane': plane,
    'script': script,
    'bidiClass': bidiClass,
    'eastAsianWidth': eastAsianWidth,
    'lineBreak': lineBreak,
    'wordBreak': wordBreak,
    'sentenceBreak': sentenceBreak,
    'graphemeClusterBreak': graphemeClusterBreak,
    'hangulSyllableType': hangulSyllableType,
    'joiningType': joiningType,
    'isAlphabetic': isAlphabetic,
    'isUppercase': isUppercase,
    'isLowercase': isLowercase,
    'isWhiteSpace': isWhiteSpace,
    'isMath': isMath,
    'isDash': isDash,
    'isDiacritic': isDiacritic,
    'isEmoji': isEmoji,
    'isEmojiPresentation': isEmojiPresentation,
    'isEmojiModifier': isEmojiModifier,
    'isEmojiModifierBase': isEmojiModifierBase,
  };
}

/// Extension methods for UnicodeCharProperties
extension MapX on Map<String, dynamic> {
  /// Convert a JSON object to UnicodeCharProperties
  UnicodeCharProperties fromUnicodeCharPropertiesJson() =>
      UnicodeCharProperties(
        character: this['character'] as String,
        codePoint: this['codePoint'] as int,
        name: this['name'] as String,
        unicodeValue: this['unicodeValue'] as String,
        generalCategory: this['generalCategory'] as String,
        block: this['block'] as String,
        plane: this['plane'] as String,
        script: this['script'] as String,
        bidiClass: this['bidiClass'] as String,
        eastAsianWidth: this['eastAsianWidth'] as String,
        lineBreak: this['lineBreak'] as String,
        wordBreak: this['wordBreak'] as String,
        sentenceBreak: this['sentenceBreak'] as String,
        graphemeClusterBreak: this['graphemeClusterBreak'] as String,
        hangulSyllableType: this['hangulSyllableType'] as String,
        joiningType: this['joiningType'] as String,
        isAlphabetic: this['isAlphabetic'] as bool,
        isUppercase: this['isUppercase'] as bool,
        isLowercase: this['isLowercase'] as bool,
        isWhiteSpace: this['isWhiteSpace'] as bool,
        isMath: this['isMath'] as bool,
        isDash: this['isDash'] as bool,
        isDiacritic: this['isDiacritic'] as bool,
        isEmoji: this['isEmoji'] as bool,
        isEmojiPresentation: this['isEmojiPresentation'] as bool,
        isEmojiModifier: this['isEmojiModifier'] as bool,
        isEmojiModifierBase: this['isEmojiModifierBase'] as bool,
      );
}
