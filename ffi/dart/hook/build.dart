// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import 'dart:io';

import 'package:code_assets/code_assets.dart';
import 'package:hooks/hooks.dart' show build;

import '../tool/build_libs.dart' show buildLib;

void main(List<String> args) async {
  await build(args, (input, output) async {
    final lib = await buildLib(
      input.config.code.targetOS,
      input.config.code.targetArchitecture,
      input.config.code.linkModePreference == LinkModePreference.static ||
          input.config.linkingEnabled,
      input.config.code.targetOS == OS.iOS &&
          input.config.code.iOS.targetSdk == IOSSdk.iPhoneSimulator,
      Directory.fromUri(input.packageRoot),
      [
        'default_components',
        'collator',
        'datetime',
        'list',
        'decimal',
        'plurals',
        'compiled_data',
        'buffer_provider',
        'experimental',
      ],
    );

    // Rebuild if bindings change
    output.addDependencies(
      Directory(
        '${input.packageRoot.path}/lib/src',
      ).listSync().map((e) => Uri.file(e.path)),
    );

    output.assets.code.add(
      CodeAsset(
        package: input.packageName,
        name: 'src/lib.g.dart',
        linkMode: DynamicLoadingBundled(),
        file: lib.uri,
      ),
    );
  });
}
