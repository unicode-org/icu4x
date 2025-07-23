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

  BuildOptions({
    required this.buildMode,
    this.localDylibPath,
    this.checkoutPath,
    this.treeshake,
  });

  Map<String, dynamic> toMap() {
    return {
      'buildMode': buildMode.name,
      if (localDylibPath != null) 'localDylibPath': localDylibPath.toString(),
      if (checkoutPath != null) 'checkoutPath': checkoutPath.toString(),
      if (treeshake != null) 'treeshake': treeshake.toString(),
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
    );
  }

  static String getPath(Directory dir, String p) =>
      path.canonicalize(path.absolute(dir.path, p));

  String toJson() => const JsonEncoder.withIndent('  ').convert(toMap());
}
