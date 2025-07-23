import 'package:hooks/hooks.dart';

const _prefix = 'symbols_to_keep_for_icu4x_';

extension SymbolKeeper on LinkOutputBuilder {
  void registerSymbolsToBeKept(
    String callerPackageName,
    Map<String, Set<String>> symbolsToKeep,
  ) {
    metadata.add(
      'icu4x',
      '$_prefix$callerPackageName',
      symbolsToKeep.map((key, value) => MapEntry(key, value.toList())),
    );
  }
}

extension SymbolReader on LinkInput {
  Map<String, Set<String>> get fetchSymbolsToBeKept => metadata.entries
      .where((entry) => entry.key.startsWith(_prefix))
      .map((e) => e.value as Map)
      .map((e) => e.map((key, value) => MapEntry(key, (value as List).toSet())))
      .cast<Map<String, Set<String>>>()
      .fold({}, _mergeMaps);
}

Map<String, Set<String>> _mergeMaps(
  Map<String, Set<String>> base,
  Map<String, Set<String>> additional,
) => base..updateAll((key, value) => {...value, ...additional[key] ?? {}});
