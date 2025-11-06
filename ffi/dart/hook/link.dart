// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import 'dart:convert';
import 'dart:io';

import 'package:code_assets/code_assets.dart'
    show HookConfigCodeConfig, LinkInputCodeAssets, OS;
import 'package:collection/collection.dart' show IterableExtension;
import 'package:hooks/hooks.dart' show LinkInput;
import 'package:hooks/hooks.dart' show link;
import 'package:icu4x/src/hook_helpers/shared.dart' show assetId, package;
import 'package:logging/logging.dart' show Level, Logger;
import 'package:native_toolchain_c/native_toolchain_c.dart'
    show CLinker, LinkerOptions;
import 'package:record_use/record_use.dart' as record_use;

/// Run the linker to turn a static into a treeshaken dynamic library.
Future<void> main(List<String> args) async {
  await link(args, (input, output) async {
    print('Start linking');
    final staticLib = input.assets.code.firstWhereOrNull(
      (asset) => asset.id == 'package:$package/$assetId',
    );

    if (staticLib == null) {
      // No static lib built, so assume a dynamic one was already bundled.
      return;
    }

    final usedSymbols = input.usages
        ?.constantsOf(_diplomatFfiUseIdentifier)
        .map((instance) => instance['symbol'] as String);

    print('''
### Using symbols:
  ${usedSymbols?.join('\n')}
### End using symbols
''');

    await CLinker.library(
      name: 'icu4x',
      packageName: 'icu4x',
      assetName: assetId,
      sources: [staticLib.file!.toFilePath()],
      libraries:
          // On Windows, icu4x.lib is lacking /DEFAULTLIB directives to advise
          // the linker on what libraries to link against. To make up for that,
          // the libraries used have to be provided to the linker explicitly.
          input.config.code.targetOS == OS.windows
          ? const ['MSVCRT', 'ws2_32', 'userenv', 'ntdll']
          : const [],
      linkerOptions: LinkerOptions.treeshake(symbolsToKeep: usedSymbols),
    ).run(
      input: input,
      output: output,
      logger: Logger('')
        ..level = Level.ALL
        ..onRecord.listen((record) => print(record.message)),
    );
  });
}

const _diplomatFfiUseIdentifier = record_use.Identifier(
  importUri: 'package:icu4x/src/bindings/lib.g.dart',
  name: '_DiplomatFfiUse',
);

extension on LinkInput {
  record_use.RecordedUsages? get usages {
    // the hooks package is pinned
    // ignore: experimental_member_use
    final records = recordedUsagesFile;
    if (records == null) {
      return null;
    }
    final usagesContent = File.fromUri(records).readAsStringSync();
    final usagesJson = jsonDecode(usagesContent) as Map<String, dynamic>;
    return record_use.RecordedUsages.fromJson(usagesJson);
  }
}
