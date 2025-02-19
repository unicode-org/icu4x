// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import 'package:native_assets_cli/code_assets.dart';
import 'package:path/path.dart' as path;
import 'dart:io';

const crateName = 'icu_capi';

// Copied from Dart's package:intl4x build.dart, see
// https://github.com/dart-lang/i18n/blob/main/pkgs/intl4x/hook/build.dart
Future<Uri> buildLib(BuildInput input, String workingDirectory) async {
  final crateNameFixed = crateName.replaceAll('-', '_');
  final libFileName = input.config.filename(crateNameFixed);
  final libFileUri = input.outputDirectory.resolve(libFileName);

  final code = input.config.code;
  final targetOS = code.targetOS;
  final targetArchitecture = code.targetArchitecture;

  final isNoStd = _isNoStdTarget((targetOS, targetArchitecture));

  if (!isNoStd) {
    final rustArguments = ['target', 'add', asRustTarget(input)];
    await runProcess(
      'rustup',
      rustArguments,
      workingDirectory: workingDirectory,
    );
  }
  final tempDir = await Directory.systemTemp.createTemp();
  final stdFeatures = [
    'default_components',
    'icu_collator,icu_datetime,icu_list,icu_decimal,icu_plurals',
    'compiled_data',
    'buffer_provider',
    'logging',
    'simple_logger',
    'experimental_components',
  ];
  final noStdFeatures = [
    'default_components',
    'icu_collator,icu_datetime,icu_list,icu_decimal,icu_plurals',
    'compiled_data',
    'buffer_provider',
    'libc_alloc',
    'panic_handler',
    'experimental_components',
  ];
  final linkModeType = input.config.buildStatic ? 'staticlib' : 'cdylib';
  final arguments = [
    if (input.config.buildStatic || isNoStd) '+nightly',
    'rustc',
    '-p=$crateName',
    '--crate-type=$linkModeType',
    '--release',
    '--config=profile.release.panic="abort"',
    '--config=profile.release.codegen-units=1',
    '--no-default-features',
    isNoStd
        ? '--features=${noStdFeatures.join(',')}'
        : '--features=${stdFeatures.join(',')}',
    if (isNoStd) '-Zbuild-std=core,alloc',
    if (input.config.buildStatic || isNoStd) ...[
      '-Zbuild-std=std,panic_abort',
      '-Zbuild-std-features=panic_immediate_abort',
    ],
    '--target=${asRustTarget(input)}',
    '--target-dir=${tempDir.path}'
  ];
  await runProcess(
    'cargo',
    arguments,
    workingDirectory: workingDirectory,
  );

  final builtPath = path.join(
    tempDir.path,
    asRustTarget(input),
    'release',
    libFileName,
  );
  final file = File(builtPath);
  if (!(await file.exists())) {
    throw FileSystemException('Building the dylib failed', builtPath);
  }
  await file.copy(libFileUri.toFilePath(windows: Platform.isWindows));
  await tempDir.delete(recursive: true);
  return libFileUri;
}

String asRustTarget(BuildInput input) {
  final rustTarget = _asRustTarget(
    input.config.code.targetOS,
    input.config.code.targetArchitecture,
    input.config.code.targetOS == OS.iOS &&
        input.config.code.iOS.targetSdk == IOSSdk.iPhoneSimulator,
  );
  return rustTarget;
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

extension on BuildConfig {
  bool get buildStatic =>
      code.linkModePreference == LinkModePreference.static || linkingEnabled;

  String Function(String) get filename => buildStatic
      ? code.targetOS.staticlibFileName
      : code.targetOS.dylibFileName;
}

Future<void> runProcess(
  String executable,
  List<String> arguments, {
  String? workingDirectory,
  bool dryRun = false,
}) async {
  print('----------');
  print('Running `$executable $arguments` in $workingDirectory');
  if (!dryRun) {
    final processResult = await Process.run(
      executable,
      arguments,
      workingDirectory: workingDirectory,
    );
    print('stdout:');
    print(processResult.stdout);
    if ((processResult.stderr as String).isNotEmpty) {
      print('stderr:');
      print(processResult.stderr);
    }
    if (processResult.exitCode != 0) {
      throw ProcessException(executable, arguments, '', processResult.exitCode);
    }
  } else {
    print('Not running, as --dry-run is set.');
  }
  print('==========');
}
