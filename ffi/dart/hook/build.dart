// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import 'dart:io';

import 'package:native_assets_cli/code_assets.dart';

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
      Directory.fromUri(input.outputDirectory),
      [
        'default_components',
        'icu_collator',
        'icu_datetime',
        'icu_list',
        'icu_decimal',
        'icu_plurals',
        'compiled_data',
        'buffer_provider',
        'experimental_components',
      ],
    );

    output.assets.code.add(
      CodeAsset(
        package: input.packageName,
        name: 'src/lib.g.dart',
        linkMode: DynamicLoadingBundled(),
        os: input.config.code.targetOS,
        architecture: input.config.code.targetArchitecture,
        file: lib.uri,
      ),
    );
  });
}
