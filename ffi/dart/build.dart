// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import 'package:native_assets_cli/native_assets_cli.dart';

import 'tool/build_libs.dart' show buildLib;

void main(List<String> args) async {
  final config = await BuildConfig.fromArgs(args);

  final target = (config.target == Target.iOSArm64 &&
          config.targetIOSSdk == IOSSdk.iPhoneSimulator)
      // This stands for Target.iOSArm64Simulator, see build_libs.dart
      ? Target.iOSArm
      : config.target;

  final linkMode = config.linkModePreference.preferredLinkMode;

  final path = '${config.outDir.path}/icu4x';

  await buildLib(
    target,
    linkMode,
    ['default_components', 'experimental_components', 'compiled_data'],
    path,
  );

  await BuildOutput(
    assets: [
      Asset(
          id: 'package:icu/src/lib.g.dart',
          linkMode: linkMode,
          target: target,
          path: AssetAbsolutePath(Uri.file(path)))
    ],
    dependencies: Dependencies([config.packageRoot.resolve('build.rs')]),
  ).writeToFile(outDir: config.outDir);
}
