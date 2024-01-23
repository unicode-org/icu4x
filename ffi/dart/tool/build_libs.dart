// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import 'package:native_assets_cli/native_assets_cli.dart';
import 'dart:io';

const linuxTargets = [
  Target.linuxArm,
  Target.linuxArm64,
  Target.linuxIA32,
  // There's no stable linker for this target
  // Target.linuxRiscv32,
  Target.linuxRiscv64,
  Target.linuxX64,
];

const androidTargets = [
  Target.androidArm,
  Target.androidArm64,
  Target.androidIA32,
  Target.androidX64,
  // There's no stable linker for this target
  // Target.androidRiscv64,
];

const fuchsiaTargets = [
  // Target.fuchsiaArm64,
  // Target.fuchsiaX64,
];

const macTargets = [
  Target.macOSArm64,
  Target.macOSX64,
];

const iosTargets = [
  // This target is meant to be 32-bit iOS, but that is not supported anymore.
  // We're abusing it for 64-bit simulated, as that does not have a separate Dart
  // target.
  Target.iOSArm,
  Target.iOSArm64,
  Target.iOSX64,
];

const windowsTargets = [
  Target.windowsArm64,
  Target.windowsIA32,
  Target.windowsX64,
];

Future<void> main(List<String> args) async {
  final binDir = args[0];

  final List<Target> targets;
  if (Platform.isLinux) {
    targets = [...androidTargets, ...linuxTargets, ...fuchsiaTargets];
  } else if (Platform.isMacOS) {
    targets = [...macTargets, ...iosTargets];
  } else if (Platform.isWindows) {
    targets = windowsTargets;
  } else {
    throw 'Unsupported host';
  }

  await Directory(binDir).create();

  for (var target in targets) {
    print('Building $target...');
    for (final linkMode in LinkMode.values) {
      try {
        await buildLib(target, linkMode, binDir);
      } catch (e) {
        exitCode++;
        print(e);
      }
    }
  }
}

Future<String> buildLib(Target target, LinkMode linkMode, String outDir) async {
  const dartToRustTargets = {
    Target.androidArm: 'armv7-linux-androideabi',
    Target.androidArm64: 'aarch64-linux-android',
    Target.androidIA32: 'i686-linux-android',
    Target.androidRiscv64: 'riscv64-linux-android',
    Target.androidX64: 'x86_64-linux-android',
    Target.fuchsiaArm64: 'aarch64-unknown-fuchsia',
    Target.fuchsiaX64: 'x86_64-unknown-fuchsia',
    Target.iOSArm: 'aarch64-apple-ios-sim',
    Target.iOSArm64: 'aarch64-apple-ios',
    Target.iOSX64: 'x86_64-apple-ios',
    Target.linuxArm: 'armv7-unknown-linux-gnueabihf',
    Target.linuxArm64: 'aarch64-unknown-linux-gnu',
    Target.linuxIA32: 'i686-unknown-linux-gnu',
    Target.linuxRiscv32: 'riscv32gc-unknown-linux-gnu',
    Target.linuxRiscv64: 'riscv64gc-unknown-linux-gnu',
    Target.linuxX64: 'x86_64-unknown-linux-gnu',
    Target.macOSArm64: 'aarch64-apple-darwin',
    Target.macOSX64: 'x86_64-apple-darwin',
    Target.windowsArm64: 'aarch64-pc-windows-msvc',
    Target.windowsIA32: 'i686-pc-windows-msvc',
    Target.windowsX64: 'x86_64-pc-windows-msvc',
  };

  const noStdTargets = [
    Target.androidRiscv64,
    Target.linuxRiscv32,
  ];

  final rustTarget = dartToRustTargets[target]!;
  final isNoStd = noStdTargets.contains(target);

  if (!isNoStd) {
    var rustup = await Process.run('rustup', ['target', 'add', rustTarget]);

    if (rustup.exitCode != 0) {
      throw rustup.stderr;
    }
  }

  final cargo = await Process.run('cargo', [
    if (isNoStd) '+nightly',
    'rustc',
    '-p=icu_capi',
    '--crate-type=${linkMode == LinkMode.static ? 'staticlib' : 'cdylib'}',
    '--release',
    '--config=profile.release.panic="abort"',
    '--config=profile.release.codegen-units=1',
    '--no-default-features',
    if (!isNoStd)
      '--features=default_components,compiled_data,buffer_provider,logging,simple_logger',
    if (isNoStd)
      '--features=default_components,compiled_data,buffer_provider,libc-alloc,panic-handler',
    if (isNoStd) '-Zbuild-std=core,alloc',
    if (isNoStd) '-Zbuild-std-features=panic_immediate_abort',
    '--target=$rustTarget',
  ]);

  if (cargo.exitCode != 0) {
    throw cargo.stderr;
  }

  final outName = '$outDir/$target-$linkMode';

  final targetDir = '${Platform.script.toFilePath().split('ffi')[0]}target';

  await File(
          '$targetDir/$rustTarget/release/${target.os.dylibFileName('icu_capi')}')
      .copy(outName);

  return outName;
}
