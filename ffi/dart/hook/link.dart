// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import 'package:hooks/hooks.dart';
import 'package:icu4x/src/hook_helpers/library.dart';

/// Run the linker to turn a static into a treeshaken dynamic library.
///
/// If linking fails in the default `fetch` build mode, falls back to the
/// pre-built dynamic library.
Future<void> main(List<String> args) async {
  await link(args, (input, output) async {
    await icu4xLibrary.link(input: input, output: output);
  });
}
