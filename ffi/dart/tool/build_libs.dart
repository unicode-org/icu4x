// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import 'package:native_assets_cli/native_assets_cli.dart';
import 'dart:io';

Future<void> main(List<String> args) async {
  if (args.isEmpty) {
    print(
        'Usage: ${Platform.script.pathSegments.last} <out_dir> <target:${Target.values}> <link mode: ${LinkMode.values}> <cargo features>');
    exit(1);
  }
  final out = Uri.file(args[0]).toFilePath();
  final target = Target.values.firstWhere((t) => t.toString() == args[1]);
  final linkMode = LinkMode.values.firstWhere((l) => l.toString() == args[2]);
  final cargoFeatures = args.elementAtOrNull(3) ?? 'default_components';

  await buildLib(target, linkMode, cargoFeatures, out);
}

Future<void> buildLib(
  Target target,
  LinkMode linkMode,
  String cargoFeatures,
  String outName,
) async {
  var root = Platform.script.toFilePath().split('ffi')[0];
  root = root.substring(0, root.length - 1); // trim trailing slash

  print('Building $outName');

  final rustTarget = dartToRustTargets[target]!;

  final isNoStd = [
    Target.androidRiscv64,
    Target.linuxRiscv32,
  ].contains(target);

  final rustupArguments = ['target', 'add', rustTarget];
  final rustup = await Process.start('rustup', rustupArguments);
  stdout.addStream(rustup.stdout);
  stderr.addStream(rustup.stderr);

  final rustupExitCode = await rustup.exitCode;
  if (rustupExitCode != 0) {
    throw ProcessException(
      'rustup',
      rustupArguments,
      'Exited with a non-zero exit code',
      rustupExitCode,
    );
  }

  final features = [
    if (linkMode == LinkMode.dynamic) 'compiled_data',
    'buffer_provider',
    if (isNoStd) ...['libc-alloc', 'panic-handler'],
    if (!isNoStd) ...['logging', 'simple_logger'],
  ].join(',');
  final arguments = [
    if (isNoStd) '+nightly',
    'rustc',
    '--manifest-path=$root/ffi/capi/Cargo.toml',
    '--crate-type=${linkMode == LinkMode.static ? 'staticlib' : 'cdylib'}',
    '--release',
    '--config=profile.release.panic="abort"',
    '--config=profile.release.codegen-units=1',
    '--no-default-features',
    '--features=$features,$cargoFeatures',
    if (isNoStd) '-Zbuild-std=core,alloc',
    if (isNoStd) '-Zbuild-std-features=panic_immediate_abort',
    '--target=$rustTarget',
  ];
  final cargo = await Process.start('cargo', arguments);
  stdout.addStream(cargo.stdout);
  stderr.addStream(cargo.stderr);

  final cargoExitCode = await cargo.exitCode;
  if (cargoExitCode != 0) {
    throw ProcessException(
      'cargo',
      arguments,
      'Exited with a non-zero exit code',
      cargoExitCode,
    );
  }

  final filename = linkMode == LinkMode.dynamic
      ? target.os.dylibFileName
      : target.os.staticlibFileName;
  final uri =
      Uri.file('$root/target/$rustTarget/release/${filename(('icu_capi'))}');
  await File.fromUri(uri).copy(outName);
}

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
