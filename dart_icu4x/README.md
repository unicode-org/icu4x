# dart_icu4x

A Flutter plugin that provides Unicode and internationalization (i18n) functionality using ICU4X (International Components for Unicode) through Rust bindings. This project leverages Flutter Rust Bridge to seamlessly integrate Rust's ICU4X library with Flutter applications.

## Overview

ICU4X is a modern, modular internationalization library that provides Unicode and localization support. This Flutter plugin makes ICU4X's powerful features available to Flutter applications, including:

- **Unicode Character Properties**: Access detailed information about Unicode characters including general category, script, block, and various properties
- **Case Mapping**: Convert text between different cases (uppercase, lowercase, titlecase)
- **Internationalization Support**: Foundation for building fully localized applications

## Project Structure

```
dart_icu4x/
├── rust/                          # Rust backend with ICU4X integration
│   ├── src/
│   │   ├── api/                   # API definitions
│   │   │   ├── mod.rs            # Module declarations
│   │   │   └── simple.rs         # Core Unicode functionality
│   │   ├── lib.rs                # Library entry point
│   │   └── frb_generated.rs      # Auto-generated Flutter Rust Bridge code
│   └── Cargo.toml                # Rust dependencies
├── lib/                          # Dart library code
│   ├── src/rust/                 # Auto-generated Dart bindings
│   │   ├── api/                  # Dart API classes
│   │   │   ├── simple.dart       # Unicode character properties
│   │   │   └── model_helper.dart # Helper classes
│   │   └── frb_generated.dart    # Core bridge functionality
│   └── dart_icu4x.dart          # Main library exports
├── example/                      # Example Flutter application
├── android/                      # Android platform configuration
├── ios/                         # iOS platform configuration
├── linux/                       # Linux platform configuration
├── macos/                       # macOS platform configuration
├── windows/                     # Windows platform configuration
├── cargokit/                    # Build tools for native code
├── test_driver/                 # Integration test configuration
├── flutter_rust_bridge.yaml     # Flutter Rust Bridge configuration
├── pubspec.yaml                 # Flutter package configuration
└── Makefile                     # Build automation
```

## Key Components

### Rust Backend (`rust/`)
- **ICU4X Integration**: Uses ICU4X crates for Unicode and internationalization
- **API Layer**: Provides clean interfaces for Unicode character analysis and case mapping
- **Flutter Rust Bridge**: Automatically generates bindings between Rust and Dart

### Dart Library (`lib/`)
- **Auto-generated Bindings**: Dart classes and functions generated from Rust code
- **Type-safe API**: Full type safety with proper Dart classes for all data structures
- **Cross-platform Support**: Works on all Flutter supported platforms

### Platform Support
- **Android**: Native ARM64 and x86_64 support
- **iOS**: Native ARM64 and x86_64 support  
- **Linux**: Native x86_64 support
- **macOS**: Native ARM64 and x86_64 support
- **Windows**: Native x86_64 support

## Features

### Unicode Character Properties
Get comprehensive information about Unicode characters:

```dart
List<UnicodeCharProperties> properties = getUnicodeCharProperties(
  offset: BigInt.from(0),
  limit: BigInt.from(10),
);

for (var prop in properties) {
  print('Character: ${prop.character}');
  print('Code Point: ${prop.codePoint}');
  print('General Category: ${prop.generalCategory}');
  print('Script: ${prop.script}');
  print('Block: ${prop.block}');
  print('Is Alphabetic: ${prop.isAlphabetic}');
  print('Is Emoji: ${prop.isEmoji}');
}
```

### Case Mapping
Convert text between different cases:

```dart
CaseMappingResult result = getCharacterCaseMapping(character: 'a');
print('Original: ${result.original}');
print('Mapped: ${result.mapped}');
print('Has Mapping: ${result.hasMapping}');
```

### Search Functionality
Search for specific Unicode characters:

```dart
List<UnicodeCharProperties> results = getUnicodeCharProperties(
  search: 'emoji',
  offset: BigInt.from(0),
  limit: BigInt.from(20),
);
```

## Installation

### Prerequisites
- Flutter SDK (>=3.3.0)
- Dart SDK (^3.8.1)
- Rust toolchain (for development)
- Platform-specific build tools (Android NDK, Xcode, etc.)

### Adding to Your Project

1. **Add dependency** to your `pubspec.yaml`:
```yaml
dependencies:
  dart_icu4x:
    path: ../icu4x/dart_icu4x
```

2. **Initialize the library** in your app:
```dart
import 'package:dart_icu4x/dart_icu4x.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(MyApp());
}
```

3. **Use the API**:
```dart
// Get Unicode character properties
final properties = getUnicodeCharProperties(
  offset: BigInt.from(0),
  limit: BigInt.from(10),
);

// Get case mapping
final caseResult = getCharacterCaseMapping(character: 'A');
```

## Development

### Building from Source

1. **Clone the repository**:
```bash
git clone <repository-url>
cd dart_icu4x
```

2. **Install dependencies**:
```bash
flutter pub get
```

3. **Build Rust code**:
```bash
make cargo-build
```

4. **Generate bindings**:
```bash
make generate
```

5. **Run the example**:
```bash
cd example
flutter run
```

### Available Make Commands

- `make help` - Show all available commands
- `make fresh` - Clean build and regenerate everything
- `make clean` - Clean Flutter project and rebuild Rust
- `make cargo-build` - Build Rust code only
- `make generate` - Generate Flutter Rust Bridge bindings
- `make analyze` - Run Flutter analysis
- `make format` - Format Dart code
- `make test` - Run unit tests

### Code Generation

The project uses Flutter Rust Bridge for automatic code generation:

1. **Rust API**: Define functions in `rust/src/api/simple.rs`
2. **Configuration**: Configure in `flutter_rust_bridge.yaml`
3. **Generation**: Run `make generate` to create Dart bindings

## Example Usage

See the `example/` directory for a complete Flutter application demonstrating:

- Unicode character property display
- Character search functionality
- Case mapping operations
- Integration with Flutter UI

## Dependencies

### Rust Dependencies
- `icu` - Core ICU4X functionality
- `icu_casemap` - Case mapping support
- `icu_collections` - Data structures
- `icu_properties` - Unicode properties
- `flutter_rust_bridge` - Flutter-Rust integration

### Dart Dependencies
- `flutter_rust_bridge` - Runtime bridge support
- `plugin_platform_interface` - Plugin interface support

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Run `make test` to ensure everything works
6. Submit a pull request

## License

This project is licensed under the same license as the parent ICU4X project.

## Related Projects

- [ICU4X](https://github.com/unicode-org/icu4x) - The underlying Unicode library
- [Flutter Rust Bridge](https://github.com/fzyzcjy/flutter_rust_bridge) - Flutter-Rust integration framework
- [Flutter FFI](https://docs.flutter.dev/development/platform-integration/c-interop) - Flutter's foreign function interface

## Support

For issues and questions:
- Check the [example application](example/) for usage patterns
- Review the [ICU4X documentation](https://docs.rs/icu/)
- Open an issue on the project repository

