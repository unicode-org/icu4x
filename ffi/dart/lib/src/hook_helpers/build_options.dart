// Copyright (c) 2025, the Dart project authors. Please see the AUTHORS file
// for details. All rights reserved. Use of this source code is governed by a
// BSD-style license that can be found in the LICENSE file.

import 'dart:convert' show JsonEncoder;
import 'dart:io';

import 'package:collection/collection.dart';
import 'package:hooks/hooks.dart' show HookInputUserDefines;
import 'package:path/path.dart' as path;

enum BuildModeEnum { local, checkout, fetch }

class BuildOptions {
  final BuildModeEnum buildMode;
  final Uri? localDylibPath;
  final Uri? checkoutPath;
  final bool? treeshake;
  final String? packageWithLinker;

  BuildOptions({
    required this.buildMode,
    this.localDylibPath,
    this.checkoutPath,
    this.treeshake,
    this.packageWithLinker,
  });

  Map<String, dynamic> toMap() {
    return {
      'buildMode': buildMode.name,
      if (localDylibPath != null) 'localDylibPath': localDylibPath.toString(),
      if (checkoutPath != null) 'checkoutPath': checkoutPath.toString(),
      if (treeshake != null) 'treeshake': treeshake.toString(),
      if (packageWithLinker != null) 'packageWithLinker': packageWithLinker,
    };
  }

  factory BuildOptions.fromDefines(HookInputUserDefines defines) {
    return BuildOptions(
      buildMode:
          BuildModeEnum.values.firstWhereOrNull(
            (element) => element.name == defines['buildMode'],
          ) ??
          BuildModeEnum.fetch,
      localDylibPath: defines.path('localDylibPath'),
      checkoutPath: defines.path('checkoutPath'),
      treeshake: (defines['treeshake'] ?? true) == true,
      packageWithLinker: defines['packageWithLinker'] as String?,
    );
  }

  static String getPath(Directory dir, String p) =>
      path.canonicalize(path.absolute(dir.path, p));

  String toJson() => const JsonEncoder.withIndent('  ').convert(toMap());
}
