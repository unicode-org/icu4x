// Copyright (c) 2023, the Dart project authors.  Please see the AUTHORS file
// for details. All rights reserved. Use of this source code is governed by a
// BSD-style license that can be found in the LICENSE file.

import 'dart:io';

import 'package:native_assets_cli/native_assets_cli.dart';

void main(List<String> args) async {
  // Parse the build configuration passed to this CLI from Dart or Flutter.
  final buildConfig = await BuildConfig.fromArgs(args);

  // await Process.run('cp', [
  //   '-f',
  //   '/home/mosum/projects/icu4x_robert/ffi/capi/dart/package/test/libicu_capi_cdylib.so',
  //   file.path
  // ]);

  await Process.run('make', ['test']);

  // final request = await HttpClient().getUrl(Uri.parse('http://github.com/unicode-org/icu4x/artifacts'));
  // final response = await request.close();
  // await response.pipe(file.openWrite());

  final buildOutput = BuildOutput(
    timestamp: DateTime.now(),
    assets: [
      Asset(
        id: 'package:icu/lib.g.dart',
        linkMode: LinkMode.dynamic,
        target: buildConfig.target,
        path:
            AssetAbsolutePath(File('test/libicu_capi_cdylib.so').absolute.uri),
      ),
    ],
  );

  // Write the output according to the native assets protocol so that Dart or
  // Flutter can find the native assets produced by this script.
  await buildOutput.writeToFile(outDir: buildConfig.outDir);
}
