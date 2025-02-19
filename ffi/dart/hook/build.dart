// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import 'package:native_assets_cli/code_assets.dart';

import '../tool/build_libs.dart' show buildLib;

void main(List<String> args) async {
  await build(
    args,
    (input, output) async {
      var isIOSSimulator =
          input.config.code.iOS.targetSdk == IOSSdk.iPhoneSimulator;
      final target = isIOSSimulator
          // This stands for Target.iOSArm64Simulator, see build_libs.dart
          ? (Architecture.arm, OS.iOS)
          : (input.config.code.targetArchitecture, input.config.code.targetOS);

      final outputPath = input.outputDirectory.resolve('icu4x');

      await buildLib(
        target.$2,
        target.$1,
        input.config.code.linkModePreference,
        isIOSSimulator,
        ['default_components', 'experimental_components', 'compiled_data'],
        outputPath.path,
      );

      output.assets.code.add(CodeAsset(
        package: 'icu',
        name: 'src/lib.g.dart',
        linkMode: DynamicLoadingBundled(),
        os: input.config.code.targetOS,
        architecture: input.config.code.targetArchitecture,
        file: outputPath,
      ));

      output.addDependency(input.packageRoot.resolve('build.rs'));
    },
  );
}
