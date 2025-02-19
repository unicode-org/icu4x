// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import 'package:native_assets_cli/code_assets.dart';
import 'dart:io';

Future<void> main(List<String> args) async {
  if (args.isEmpty) {
    print(
        'Usage: ${Platform.script.pathSegments.last} <out_dir> <os:${OS.values}> <architecture:${Architecture.values}> <linkMode:${LinkModePreference.values}> <ios-sim:true,false> <cargo features>');
    exit(1);
  }
  final out = Uri.file(args[0]).toFilePath();
  final os = OS.values.firstWhere((t) => t.toString() == args[1]);
  final architecture =
      Architecture.values.firstWhere((t) => t.toString() == args[2]);
  final linkModePreference =
      LinkModePreference.values.firstWhere((t) => t.toString() == args[3]);
  final iOSSimulator = args.elementAtOrNull(4) == 'true';

  final defaultFeatures = ['default_components', 'compiled_data'];
  final cargoFeatures = args.elementAtOrNull(3)?.split(',') ?? defaultFeatures;

  await buildLib(
      os, architecture, linkModePreference, iOSSimulator, cargoFeatures, out);
}

Future<void> buildLib(
  OS os,
  Architecture architecture,
  LinkModePreference linkModePreference,
  bool iOSSimulator,
  Iterable<String> cargoFeatures,
  String outName,
) async {
  var root = Platform.script.toFilePath().split('ffi')[0];
  root = root.substring(0, root.length - 1); // trim trailing slash

  print('Building $outName');

  final rustTarget = _asRustTarget(os, architecture, iOSSimulator);

  final isNoStd = _isNoStdTarget((os, architecture));

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

  final features = {
    'buffer_provider',
    if (isNoStd) ...['libc-alloc', 'panic-handler'],
    if (!isNoStd) ...['logging', 'simple_logger'],
    ...cargoFeatures,
  };
  final arguments = [
    if (isNoStd) '+nightly',
    'rustc',
    '--manifest-path=$root/ffi/capi/Cargo.toml',
    '--crate-type=${linkModePreference == LinkModePreference.static ? 'staticlib' : 'cdylib'}',
    '--release',
    '--config=profile.release.panic="abort"',
    '--config=profile.release.codegen-units=1',
    '--no-default-features',
    '--features=${features.join(',')}',
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

  final filename = linkModePreference == LinkModePreference.static
      ? os.staticlibFileName
      : os.dylibFileName;
  final uri =
      Uri.file('$root/target/$rustTarget/release/${filename(('icu_capi'))}');
  await File.fromUri(uri).copy(outName);
}

String _asRustTarget(OS os, Architecture? architecture, bool isSimulator) {
  if (os == OS.iOS && architecture == Architecture.arm64 && isSimulator) {
    return 'aarch64-apple-ios-sim';
  }
  return switch ((os, architecture)) {
    (OS.android, Architecture.arm) => 'armv7-linux-androideabi',
    (OS.android, Architecture.arm64) => 'aarch64-linux-android',
    (OS.android, Architecture.ia32) => 'i686-linux-android',
    (OS.android, Architecture.riscv64) => 'riscv64-linux-android',
    (OS.android, Architecture.x64) => 'x86_64-linux-android',
    (OS.fuchsia, Architecture.arm64) => 'aarch64-unknown-fuchsia',
    (OS.fuchsia, Architecture.x64) => 'x86_64-unknown-fuchsia',
    (OS.iOS, Architecture.arm64) => 'aarch64-apple-ios',
    (OS.iOS, Architecture.x64) => 'x86_64-apple-ios',
    (OS.linux, Architecture.arm) => 'armv7-unknown-linux-gnueabihf',
    (OS.linux, Architecture.arm64) => 'aarch64-unknown-linux-gnu',
    (OS.linux, Architecture.ia32) => 'i686-unknown-linux-gnu',
    (OS.linux, Architecture.riscv32) => 'riscv32gc-unknown-linux-gnu',
    (OS.linux, Architecture.riscv64) => 'riscv64gc-unknown-linux-gnu',
    (OS.linux, Architecture.x64) => 'x86_64-unknown-linux-gnu',
    (OS.macOS, Architecture.arm64) => 'aarch64-apple-darwin',
    (OS.macOS, Architecture.x64) => 'x86_64-apple-darwin',
    (OS.windows, Architecture.arm64) => 'aarch64-pc-windows-msvc',
    (OS.windows, Architecture.ia32) => 'i686-pc-windows-msvc',
    (OS.windows, Architecture.x64) => 'x86_64-pc-windows-msvc',
    (_, _) => throw UnimplementedError(
        'Target ${(os, architecture)} not available for rust'),
  };
}

bool _isNoStdTarget((OS os, Architecture? architecture) arg) => [
      (OS.android, Architecture.riscv64),
      (OS.linux, Architecture.riscv64)
    ].contains(arg);
